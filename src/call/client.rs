// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::grpc_sys;
use futures_executor::block_on;
use futures_util::future::poll_fn;
use futures_util::{ready, Sink, Stream};
use parking_lot::Mutex;

use super::{ShareCall, ShareCallHolder, SinkBase, WriteFlags};
use crate::buf::GrpcSlice;
use crate::call::{check_run, Call, MessageReader, Method};
use crate::channel::Channel;
use crate::codec::{DeserializeFn, SerializeFn};
use crate::error::{Error, Result};
use crate::metadata::{Metadata, UnownedMetadata};
use crate::task::{BatchFuture, BatchType};

/// Update the flag bit in res.
#[inline]
pub fn change_flag(res: &mut u32, flag: u32, set: bool) {
    if set {
        *res |= flag;
    } else {
        *res &= !flag;
    }
}

/// Options for calls made by client.
#[derive(Clone, Default)]
pub struct CallOption {
    timeout: Option<Duration>,
    write_flags: WriteFlags,
    call_flags: u32,
    headers: Option<Metadata>,
}

impl CallOption {
    /// Signal that the call should not return UNAVAILABLE before it has started.
    pub fn wait_for_ready(mut self, wait_for_ready: bool) -> CallOption {
        change_flag(
            &mut self.call_flags,
            grpc_sys::GRPC_INITIAL_METADATA_WAIT_FOR_READY,
            wait_for_ready,
        );
        self
    }

    /// Set write flags.
    pub fn write_flags(mut self, write_flags: WriteFlags) -> CallOption {
        self.write_flags = write_flags;
        self
    }

    /// Set a timeout.
    pub fn timeout(mut self, timeout: Duration) -> CallOption {
        self.timeout = Some(timeout);
        self
    }

    /// Get the timeout.
    pub fn get_timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Set the headers to be sent with the call.
    pub fn headers(mut self, meta: Metadata) -> CallOption {
        self.headers = Some(meta);
        self
    }

    /// Get headers to be sent with the call.
    pub fn get_headers(&self) -> Option<&Metadata> {
        self.headers.as_ref()
    }
}

impl Call {
    pub fn unary_async<Req, Resp>(
        channel: &Channel,
        method: &Method<Req, Resp>,
        req: &Req,
        mut opt: CallOption,
    ) -> Result<ClientUnaryReceiver<Resp>> {
        let call = channel.create_call(method, &opt)?;
        let mut payload = GrpcSlice::default();
        (method.req_ser())(req, &mut payload)?;
        let cq_f = check_run(BatchType::CheckRead, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_unary(
                call.call,
                ctx,
                payload.as_mut_ptr(),
                opt.write_flags.flags,
                opt.headers
                    .as_mut()
                    .map_or_else(ptr::null_mut, |c| c as *mut _ as _),
                opt.call_flags,
                tag,
            )
        });
        Ok(ClientUnaryReceiver::new(call, cq_f, method.resp_de()))
    }

    pub fn client_streaming<Req, Resp>(
        channel: &Channel,
        method: &Method<Req, Resp>,
        mut opt: CallOption,
    ) -> Result<(ClientCStreamSender<Req>, ClientCStreamReceiver<Resp>)> {
        let call = channel.create_call(method, &opt)?;
        let cq_f = check_run(BatchType::CheckRead, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_client_streaming(
                call.call,
                ctx,
                opt.headers
                    .as_mut()
                    .map_or_else(ptr::null_mut, |c| c as *mut _ as _),
                opt.call_flags,
                tag,
            )
        });

        let share_call = Arc::new(Mutex::new(ShareCall::new(call, cq_f)));
        let sink = ClientCStreamSender::new(share_call.clone(), method.req_ser(), opt.call_flags);
        let recv = ClientCStreamReceiver::new(share_call, method.resp_de());
        Ok((sink, recv))
    }

    pub fn server_streaming<Req, Resp>(
        channel: &Channel,
        method: &Method<Req, Resp>,
        req: &Req,
        mut opt: CallOption,
    ) -> Result<ClientSStreamReceiver<Resp>> {
        let call = channel.create_call(method, &opt)?;
        let mut payload = GrpcSlice::default();
        (method.req_ser())(req, &mut payload)?;
        let cq_f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_server_streaming(
                call.call,
                ctx,
                payload.as_mut_ptr(),
                opt.write_flags.flags,
                opt.headers
                    .as_mut()
                    .map_or_else(ptr::null_mut, |c| c as *mut _ as _),
                opt.call_flags,
                tag,
            )
        });

        let headers_f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_recv_initial_metadata(call.call, ctx, tag)
        });

        Ok(ClientSStreamReceiver::new(
            call,
            cq_f,
            method.resp_de(),
            headers_f,
        ))
    }

    pub fn duplex_streaming<Req, Resp>(
        channel: &Channel,
        method: &Method<Req, Resp>,
        mut opt: CallOption,
    ) -> Result<(ClientDuplexSender<Req>, ClientDuplexReceiver<Resp>)> {
        let call = channel.create_call(method, &opt)?;
        let cq_f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_duplex_streaming(
                call.call,
                ctx,
                opt.headers
                    .as_mut()
                    .map_or_else(ptr::null_mut, |c| c as *mut _ as _),
                opt.call_flags,
                tag,
            )
        });

        let headers_f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_recv_initial_metadata(call.call, ctx, tag)
        });

        let share_call = Arc::new(Mutex::new(ShareCall::new(call, cq_f)));
        let sink = ClientDuplexSender::new(share_call.clone(), method.req_ser(), opt.call_flags);
        let recv = ClientDuplexReceiver::new(share_call, method.resp_de(), headers_f);
        Ok((sink, recv))
    }
}

/// A receiver for unary request.
///
/// The future is resolved once response is received.
#[must_use = "if unused the ClientUnaryReceiver may immediately cancel the RPC"]
pub struct ClientUnaryReceiver<T> {
    call: Call,
    resp_f: BatchFuture,
    resp_de: DeserializeFn<T>,
    finished: bool,
    message: Option<T>,
    initial_metadata: UnownedMetadata,
    trailing_metadata: UnownedMetadata,
}

impl<T> ClientUnaryReceiver<T> {
    fn new(call: Call, resp_f: BatchFuture, resp_de: DeserializeFn<T>) -> ClientUnaryReceiver<T> {
        ClientUnaryReceiver {
            call,
            resp_f,
            resp_de,
            finished: false,
            message: None,
            initial_metadata: UnownedMetadata::empty(),
            trailing_metadata: UnownedMetadata::empty(),
        }
    }

    /// Cancel the call.
    #[inline]
    pub fn cancel(&mut self) {
        self.call.cancel()
    }

    #[inline]
    pub fn resp_de(&self, reader: MessageReader) -> Result<T> {
        (self.resp_de)(reader)
    }

    async fn wait_for_batch_future(&mut self) -> Result<()> {
        if self.finished {
            return Ok(());
        }

        let data = Pin::new(&mut self.resp_f).await?;
        self.initial_metadata = data.initial_metadata;
        self.trailing_metadata = data.trailing_metadata;
        self.message = Some(self.resp_de(data.message_reader.unwrap())?);
        self.finished = true;
        Ok(())
    }

    pub async fn message(&mut self) -> Result<T> {
        self.wait_for_batch_future().await?;
        Ok(self.message.take().unwrap())
    }

    /// Get the initial metadata.
    pub async fn headers(&mut self) -> Result<&Metadata> {
        self.wait_for_batch_future().await?;
        // Because we have a reference to call, so it's safe to read.
        Ok(unsafe { self.initial_metadata.assume_valid() })
    }

    pub async fn trailers(&mut self) -> Result<&Metadata> {
        self.wait_for_batch_future().await?;
        // Because we have a reference to call, so it's safe to read.
        Ok(unsafe { self.trailing_metadata.assume_valid() })
    }

    pub fn receive_sync(&mut self) -> Result<(Metadata, T, Metadata)> {
        block_on(async {
            let headers = self.headers().await?.clone();
            let message = self.message().await?;
            let trailer = self.trailers().await?.clone();
            Ok::<(Metadata, T, Metadata), Error>((headers, message, trailer))
        })
    }
}

impl<T: Unpin> Future for ClientUnaryReceiver<T> {
    type Output = Result<T>;

    /// Note this method is conflict with method `message`.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<T>> {
        if self.finished {
            if let Some(message) = self.message.take() {
                return Poll::Ready(Ok(message));
            }
            panic!("future should not be polled twice.");
        }

        let data = ready!(Pin::new(&mut self.resp_f).poll(cx)?);
        self.initial_metadata = data.initial_metadata;
        self.trailing_metadata = data.trailing_metadata;
        self.finished = true;
        Poll::Ready(self.resp_de(data.message_reader.unwrap()))
    }
}

/// A receiver for client streaming call.
///
/// If the corresponding sink has dropped or cancelled, this will poll a
/// [`RpcFailure`] error with the [`Cancelled`] status.
///
/// [`RpcFailure`]: ./enum.Error.html#variant.RpcFailure
/// [`Cancelled`]: ./enum.RpcStatusCode.html#variant.Cancelled
#[must_use = "if unused the ClientCStreamReceiver may immediately cancel the RPC"]
pub struct ClientCStreamReceiver<T> {
    call: Arc<Mutex<ShareCall>>,
    resp_de: DeserializeFn<T>,
    finished: bool,
    message: Option<T>,
    initial_metadata: UnownedMetadata,
    trailing_metadata: UnownedMetadata,
}

impl<T> ClientCStreamReceiver<T> {
    /// Private constructor to simplify code in `impl Call`
    fn new(call: Arc<Mutex<ShareCall>>, resp_de: DeserializeFn<T>) -> ClientCStreamReceiver<T> {
        ClientCStreamReceiver {
            call,
            resp_de,
            finished: false,
            message: None,
            initial_metadata: UnownedMetadata::empty(),
            trailing_metadata: UnownedMetadata::empty(),
        }
    }

    /// Cancel the call.
    pub fn cancel(&mut self) {
        let lock = self.call.lock();
        lock.call.cancel()
    }

    #[inline]
    pub fn resp_de(&self, reader: MessageReader) -> Result<T> {
        (self.resp_de)(reader)
    }

    async fn wait_for_batch_future(&mut self) -> Result<()> {
        if self.finished {
            return Ok(());
        }
        let data = poll_fn(|cx| {
            let mut call = self.call.lock();
            call.poll_finish(cx)
        })
        .await?;

        self.message = Some(self.resp_de(data.message_reader.unwrap())?);
        self.initial_metadata = data.initial_metadata;
        self.trailing_metadata = data.trailing_metadata;
        self.finished = true;
        Ok(())
    }

    pub async fn message(&mut self) -> Result<T> {
        self.wait_for_batch_future().await?;
        Ok(self.message.take().unwrap())
    }

    /// Get the initial metadata.
    pub async fn headers(&mut self) -> Result<&Metadata> {
        self.wait_for_batch_future().await?;
        // We still have a reference in share call.
        Ok(unsafe { self.initial_metadata.assume_valid() })
    }

    pub async fn trailers(&mut self) -> Result<&Metadata> {
        self.wait_for_batch_future().await?;
        // We still have a reference in share call.
        Ok(unsafe { self.trailing_metadata.assume_valid() })
    }
}

impl<T> Drop for ClientCStreamReceiver<T> {
    /// The corresponding RPC will be canceled if the receiver did not
    /// finish before dropping.
    fn drop(&mut self) {
        if !self.finished {
            self.cancel();
        }
    }
}

impl<T: Unpin> Future for ClientCStreamReceiver<T> {
    type Output = Result<T>;

    /// Note this method is conflict with method `message`.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<T>> {
        if self.finished {
            if let Some(message) = self.message.take() {
                return Poll::Ready(Ok(message));
            }
            panic!("future should not be polled twice.");
        }

        let data = {
            let mut call = self.call.lock();
            ready!(call.poll_finish(cx)?)
        };
        self.initial_metadata = data.initial_metadata;
        self.trailing_metadata = data.trailing_metadata;
        self.finished = true;
        Poll::Ready((self.resp_de)(data.message_reader.unwrap()))
    }
}

/// A sink for client streaming call and duplex streaming call.
/// To close the sink properly, you should call [`close`] before dropping.
///
/// [`close`]: #method.close
#[must_use = "if unused the StreamingCallSink may immediately cancel the RPC"]
pub struct StreamingCallSink<Req> {
    call: Arc<Mutex<ShareCall>>,
    sink_base: SinkBase,
    close_f: Option<BatchFuture>,
    req_ser: SerializeFn<Req>,
    call_flags: u32,
}

impl<Req> StreamingCallSink<Req> {
    fn new(
        call: Arc<Mutex<ShareCall>>,
        req_ser: SerializeFn<Req>,
        call_flags: u32,
    ) -> StreamingCallSink<Req> {
        StreamingCallSink {
            call,
            sink_base: SinkBase::new(false),
            close_f: None,
            req_ser,
            call_flags,
        }
    }

    /// By default it always sends messages with their configured buffer hint. But when the
    /// `enhance_batch` is enabled, messages will be batched together as many as possible.
    /// The rules are listed as below:
    /// - All messages except the last one will be sent with `buffer_hint` set to true.
    /// - The last message will also be sent with `buffer_hint` set to true unless any message is
    ///    offered with buffer hint set to false.
    ///
    /// No matter `enhance_batch` is true or false, it's recommended to follow the contract of
    /// Sink and call `poll_flush` to ensure messages are handled by gRPC C Core.
    pub fn enhance_batch(&mut self, flag: bool) {
        self.sink_base.enhance_buffer_strategy = flag;
    }

    pub fn cancel(&mut self) {
        let call = self.call.lock();
        call.call.cancel()
    }
}

impl<P> Drop for StreamingCallSink<P> {
    /// The corresponding RPC will be canceled if the sink did not call
    /// [`close`] before dropping.
    ///
    /// [`close`]: #method.close
    fn drop(&mut self) {
        if self.close_f.is_none() {
            self.cancel();
        }
    }
}

impl<Req> Sink<(Req, WriteFlags)> for StreamingCallSink<Req> {
    type Error = Error;

    #[inline]
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        Pin::new(&mut self.sink_base).poll_ready(cx)
    }

    #[inline]
    fn start_send(mut self: Pin<&mut Self>, (msg, flags): (Req, WriteFlags)) -> Result<()> {
        {
            let mut call = self.call.lock();
            call.check_alive()?;
        }
        let t = &mut *self;
        Pin::new(&mut t.sink_base).start_send(&mut t.call, &msg, flags, t.req_ser, t.call_flags)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        {
            let mut call = self.call.lock();
            call.check_alive()?;
        }
        let t = &mut *self;
        Pin::new(&mut t.sink_base).poll_flush(cx, &mut t.call, t.call_flags)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<()>> {
        let t = &mut *self;
        let mut call = t.call.lock();
        if t.close_f.is_none() {
            ready!(Pin::new(&mut t.sink_base).poll_ready(cx)?);

            let close_f = call.call.start_send_close_client()?;
            t.close_f = Some(close_f);
        }

        if Pin::new(t.close_f.as_mut().unwrap()).poll(cx)?.is_pending() {
            // if call is finished, can return early here.
            call.check_alive()?;
            return Poll::Pending;
        }
        Poll::Ready(Ok(()))
    }
}

/// A sink for client streaming call.
///
/// To close the sink properly, you should call [`close`] before dropping.
///
/// [`close`]: #method.close
pub type ClientCStreamSender<T> = StreamingCallSink<T>;
/// A sink for duplex streaming call.
///
/// To close the sink properly, you should call [`close`] before dropping.
///
/// [`close`]: #method.close
pub type ClientDuplexSender<T> = StreamingCallSink<T>;

enum FutureOrValue<F, V> {
    Future(F),
    Value(V),
}

struct ResponseStreamImpl<H, T> {
    call: H,
    msg_f: Option<BatchFuture>,
    read_done: bool,
    finished: bool,
    resp_de: DeserializeFn<T>,
    headers_f: FutureOrValue<BatchFuture, UnownedMetadata>,
}

impl<H: ShareCallHolder + Unpin, T> ResponseStreamImpl<H, T> {
    fn new(call: H, resp_de: DeserializeFn<T>, headers_f: BatchFuture) -> ResponseStreamImpl<H, T> {
        ResponseStreamImpl {
            call,
            msg_f: None,
            read_done: false,
            finished: false,
            resp_de,
            headers_f: FutureOrValue::Future(headers_f),
        }
    }

    fn cancel(&mut self) {
        self.call.call(|c| c.call.cancel())
    }

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Result<T>>> {
        if !self.finished {
            let t = &mut *self;
            let finished = &mut t.finished;
            let _ = t.call.call(|c| {
                let res = c.poll_finish(cx);
                *finished = c.finished;
                res
            })?;
        }

        let mut bytes = None;
        loop {
            if !self.read_done {
                if let Some(msg_f) = &mut self.msg_f {
                    let batch_result = ready!(Pin::new(msg_f).poll(cx)?);
                    bytes = batch_result.message_reader;
                    if bytes.is_none() {
                        self.read_done = true;
                    }
                }
            }

            if self.read_done {
                if self.finished {
                    return Poll::Ready(None);
                }
                return Poll::Pending;
            }

            // so msg_f must be either stale or not initialised yet.
            self.msg_f.take();
            let msg_f = self.call.call(|c| c.call.start_recv_message())?;
            self.msg_f = Some(msg_f);
            if let Some(data) = bytes {
                let msg = (self.resp_de)(data)?;
                return Poll::Ready(Some(Ok(msg)));
            }
        }
    }

    // Cancel the call if we still have some messages or did not
    // receive status code.
    fn on_drop(&mut self) {
        if !self.read_done || !self.finished {
            self.cancel();
        }
    }

    async fn headers(&mut self) -> Result<&Metadata> {
        if let FutureOrValue::Future(f) = &mut self.headers_f {
            self.headers_f = FutureOrValue::Value(Pin::new(f).await?.initial_metadata);
        }
        match &self.headers_f {
            // We still have reference to call.
            FutureOrValue::Value(v) => Ok(unsafe { v.assume_valid() }),
            _ => unreachable!(),
        }
    }
}

/// A receiver for server streaming call.
#[must_use = "if unused the ClientSStreamReceiver may immediately cancel the RPC"]
pub struct ClientSStreamReceiver<Resp> {
    imp: ResponseStreamImpl<ShareCall, Resp>,
}

impl<Resp> ClientSStreamReceiver<Resp> {
    fn new(
        call: Call,
        finish_f: BatchFuture,
        de: DeserializeFn<Resp>,
        headers_f: BatchFuture,
    ) -> ClientSStreamReceiver<Resp> {
        let share_call = ShareCall::new(call, finish_f);
        ClientSStreamReceiver {
            imp: ResponseStreamImpl::new(share_call, de, headers_f),
        }
    }

    pub fn cancel(&mut self) {
        self.imp.cancel()
    }

    /// Get the initial metadata.
    #[inline]
    pub async fn headers(&mut self) -> Result<&Metadata> {
        self.imp.headers().await
    }
}

impl<Resp> Stream for ClientSStreamReceiver<Resp> {
    type Item = Result<Resp>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.imp).poll(cx)
    }
}

/// A response receiver for duplex call.
///
/// If the corresponding sink has dropped or cancelled, this will poll a
/// [`RpcFailure`] error with the [`Cancelled`] status.
///
/// [`RpcFailure`]: ./enum.Error.html#variant.RpcFailure
/// [`Cancelled`]: ./enum.RpcStatusCode.html#variant.Cancelled
#[must_use = "if unused the ClientDuplexReceiver may immediately cancel the RPC"]
pub struct ClientDuplexReceiver<Resp> {
    imp: ResponseStreamImpl<Arc<Mutex<ShareCall>>, Resp>,
}

impl<Resp> ClientDuplexReceiver<Resp> {
    fn new(
        call: Arc<Mutex<ShareCall>>,
        de: DeserializeFn<Resp>,
        headers_f: BatchFuture,
    ) -> ClientDuplexReceiver<Resp> {
        ClientDuplexReceiver {
            imp: ResponseStreamImpl::new(call, de, headers_f),
        }
    }

    pub fn cancel(&mut self) {
        self.imp.cancel()
    }

    /// Get the initial metadata.
    #[inline]
    pub async fn headers(&mut self) -> Result<&Metadata> {
        self.imp.headers().await
    }
}

impl<Resp> Drop for ClientDuplexReceiver<Resp> {
    /// The corresponding RPC will be canceled if the receiver did not
    /// finish before dropping.
    fn drop(&mut self) {
        self.imp.on_drop()
    }
}

impl<Resp> Stream for ClientDuplexReceiver<Resp> {
    type Item = Result<Resp>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.imp).poll(cx)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_change_flag() {
        let mut flag = 2 | 4;
        super::change_flag(&mut flag, 8, true);
        assert_eq!(flag, 2 | 4 | 8);
        super::change_flag(&mut flag, 4, false);
        assert_eq!(flag, 2 | 8);
    }
}
