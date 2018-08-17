// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ptr;
use std::sync::Arc;
use std::time::Duration;

use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use grpc_sys;

use super::{check_run, Call, CallHolder, Finish, FinishHolder, Method, SinkBase, WriteFlags};
use async::{BatchFuture, BatchMessage, BatchType, CqFuture, SpinLock};
use channel::Channel;
use codec::{DeserializeFn, SerializeFn};
use error::{Error, Result};
use metadata::Metadata;

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
    /// Signal that the call is idempotent.
    pub fn idempotent(mut self, is_idempotent: bool) -> CallOption {
        change_flag(
            &mut self.call_flags,
            grpc_sys::GRPC_INITIAL_METADATA_IDEMPOTENT_REQUEST,
            is_idempotent,
        );
        self
    }

    /// Signal that the call should not return UNAVAILABLE before it has started.
    pub fn wait_for_ready(mut self, wait_for_ready: bool) -> CallOption {
        change_flag(
            &mut self.call_flags,
            grpc_sys::GRPC_INITIAL_METADATA_WAIT_FOR_READY,
            wait_for_ready,
        );
        self
    }

    /// Signal that the call is cacheable. gRPC is free to use GET verb.
    pub fn cacheable(mut self, cacheable: bool) -> CallOption {
        change_flag(
            &mut self.call_flags,
            grpc_sys::GRPC_INITIAL_METADATA_CACHEABLE_REQUEST,
            cacheable,
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
        let mut call = channel.create_call(method, &opt)?;
        let mut payload = vec![];
        (method.req_ser())(req, &mut payload);
        let cq_f = check_run(BatchType::CheckRead, |ctx, tag| {
            call.call(|call| unsafe {
                grpc_sys::grpcwrap_call_start_unary(
                    call,
                    ctx,
                    payload.as_ptr() as *const _,
                    payload.len(),
                    opt.write_flags.flags,
                    opt.headers
                        .as_mut()
                        .map_or_else(ptr::null_mut, |c| c as *mut _ as _),
                    opt.call_flags,
                    tag,
                )
            })
        })?;
        Ok(ClientUnaryReceiver::new(call, cq_f, method.resp_de()))
    }

    pub fn client_streaming<Req, Resp>(
        channel: &Channel,
        method: &Method<Req, Resp>,
        mut opt: CallOption,
    ) -> Result<(ClientCStreamSender<Req>, ClientCStreamReceiver<Resp>)> {
        let mut call = channel.create_call(method, &opt)?;
        let cq_f = check_run(BatchType::CheckRead, |ctx, tag| {
            call.call(|call| unsafe {
                grpc_sys::grpcwrap_call_start_client_streaming(
                    call,
                    ctx,
                    opt.headers
                        .as_mut()
                        .map_or_else(ptr::null_mut, |c| c as *mut _ as _),
                    opt.call_flags,
                    tag,
                )
            })
        })?;

        let call = call.into_sync();
        let finish = Arc::new(SpinLock::new(Finish::new(cq_f)));
        let sink = ClientCStreamSender::new(call.clone(), finish.clone(), method.req_ser());
        let recv = ClientCStreamReceiver {
            call,
            finish,
            resp_de: method.resp_de(),
        };
        Ok((sink, recv))
    }

    pub fn server_streaming<Req, Resp>(
        channel: &Channel,
        method: &Method<Req, Resp>,
        req: &Req,
        mut opt: CallOption,
    ) -> Result<ClientSStreamReceiver<Resp>> {
        let mut call = channel.create_call(method, &opt)?;
        let mut payload = vec![];
        (method.req_ser())(req, &mut payload);
        let cq_f = check_run(BatchType::Finish, |ctx, tag| {
            call.call(|call| unsafe {
                grpc_sys::grpcwrap_call_start_server_streaming(
                    call,
                    ctx,
                    payload.as_ptr() as _,
                    payload.len(),
                    opt.write_flags.flags,
                    opt.headers
                        .as_mut()
                        .map_or_else(ptr::null_mut, |c| c as *mut _ as _),
                    opt.call_flags,
                    tag,
                )
            })
        })?;

        // TODO: handle header
        let _ = check_run(BatchType::Finish, |ctx, tag| {
            call.call(|call| unsafe {
                grpc_sys::grpcwrap_call_recv_initial_metadata(call, ctx, tag)
            })
        });

        Ok(ClientSStreamReceiver::new(call, cq_f, method.resp_de()))
    }

    pub fn duplex_streaming<Req, Resp>(
        channel: &Channel,
        method: &Method<Req, Resp>,
        mut opt: CallOption,
    ) -> Result<(ClientDuplexSender<Req>, ClientDuplexReceiver<Resp>)> {
        let mut call = channel.create_call(method, &opt)?;
        let cq_f = check_run(BatchType::Finish, |ctx, tag| {
            call.call(|call| unsafe {
                grpc_sys::grpcwrap_call_start_duplex_streaming(
                    call,
                    ctx,
                    opt.headers
                        .as_mut()
                        .map_or_else(ptr::null_mut, |c| c as *mut _ as _),
                    opt.call_flags,
                    tag,
                )
            })
        })?;

        // TODO: handle header.
        let _ = check_run(BatchType::Finish, |ctx, tag| {
            call.call(|call| unsafe {
                grpc_sys::grpcwrap_call_recv_initial_metadata(call, ctx, tag)
            })
        });

        let call = call.into_sync();
        let finish = Arc::new(SpinLock::new(Finish::new(cq_f)));
        let sink = ClientDuplexSender::new(call.clone(), finish.clone(), method.req_ser());
        let recv = ClientDuplexReceiver::new(call, finish, method.resp_de());
        Ok((sink, recv))
    }
}

/// A receiver for unary request.
///
/// The future is resolved once response is received.
pub struct ClientUnaryReceiver<T> {
    call: Call,
    resp_f: CqFuture<BatchMessage>,
    resp_de: DeserializeFn<T>,
}

impl<T> ClientUnaryReceiver<T> {
    fn new(
        call: Call,
        resp_f: CqFuture<BatchMessage>,
        de: DeserializeFn<T>,
    ) -> ClientUnaryReceiver<T> {
        ClientUnaryReceiver {
            call,
            resp_f,
            resp_de: de,
        }
    }

    /// Cancel the call.
    pub fn cancel(&mut self) {
        self.call.cancel()
    }
}

impl<T> Future for ClientUnaryReceiver<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let data = try_ready!(self.resp_f.poll());
        let t = (self.resp_de)(&data.unwrap())?;
        Ok(Async::Ready(t))
    }
}

/// A receiver for client streaming call.
pub struct ClientCStreamReceiver<T> {
    call: Arc<SpinLock<Call>>,
    finish: Arc<SpinLock<Finish>>,
    resp_de: DeserializeFn<T>,
}

impl<T> ClientCStreamReceiver<T> {
    /// Cancel the call.
    pub fn cancel(&mut self) {
        self.call.cancel()
    }
}

impl<T> Future for ClientCStreamReceiver<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let data = {
            let mut finish = self.finish.lock();
            try_ready!(finish.poll_finish())
        };
        let t = (self.resp_de)(&data.unwrap())?;
        Ok(Async::Ready(t))
    }
}

/// A sink for client streaming call and duplex streaming call.
pub struct StreamingCallSink<Req> {
    call: Arc<SpinLock<Call>>,
    finish: Arc<SpinLock<Finish>>,
    sink_base: SinkBase,
    close_f: Option<BatchFuture>,
    req_ser: SerializeFn<Req>,
}

impl<Req> StreamingCallSink<Req> {
    fn new(
        call: Arc<SpinLock<Call>>,
        finish: Arc<SpinLock<Finish>>,
        ser: SerializeFn<Req>,
    ) -> StreamingCallSink<Req> {
        StreamingCallSink {
            call,
            finish,
            sink_base: SinkBase::new(false),
            close_f: None,
            req_ser: ser,
        }
    }

    pub fn cancel(&mut self) {
        self.call.cancel()
    }
}

impl<Req> Sink for StreamingCallSink<Req> {
    type SinkItem = (Req, WriteFlags);
    type SinkError = Error;

    fn start_send(&mut self, (msg, flags): Self::SinkItem) -> StartSend<Self::SinkItem, Error> {
        {
            let mut finish = self.finish.lock();
            finish.check_alive()?;
        }
        self.sink_base
            .start_send(&mut self.call, &msg, flags, self.req_ser)
            .map(|s| {
                if s {
                    AsyncSink::Ready
                } else {
                    AsyncSink::NotReady((msg, flags))
                }
            })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        {
            let mut finish = self.finish.lock();
            finish.check_alive()?;
        }
        self.sink_base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        if self.close_f.is_none() {
            try_ready!(self.sink_base.poll_complete());

            let close_f = self.call.start_send_close_client()?;
            self.close_f = Some(close_f);
        }

        if let Async::NotReady = self.close_f.as_mut().unwrap().poll()? {
            // if call is finished, can return early here.
            let mut finish = self.finish.lock();
            finish.check_alive()?;
            return Ok(Async::NotReady);
        }
        Ok(Async::Ready(()))
    }
}

pub type ClientCStreamSender<T> = StreamingCallSink<T>;
pub type ClientDuplexSender<T> = StreamingCallSink<T>;

struct ResponseStreamImpl<C, H, T> {
    call: C,
    finish: H,
    msg_f: Option<BatchFuture>,
    read_done: bool,
    resp_de: DeserializeFn<T>,
}

impl<C: CallHolder, H: FinishHolder, T> ResponseStreamImpl<C, H, T> {
    fn new(call: C, finish: H, resp_de: DeserializeFn<T>) -> ResponseStreamImpl<C, H, T> {
        ResponseStreamImpl {
            call,
            finish,
            msg_f: None,
            read_done: false,
            resp_de,
        }
    }

    fn cancel(&mut self) {
        self.call.cancel()
    }

    fn poll(&mut self) -> Poll<Option<T>, Error> {
        let mut finished = false;
        self.finish.call(|c| {
            if c.finished {
                finished = true;
                return Ok(());
            }

            let res = c.poll_finish().map(|_| ());
            finished = c.finished;
            res
        })?;

        let mut bytes = None;
        loop {
            if !self.read_done {
                if let Some(ref mut msg_f) = self.msg_f {
                    bytes = try_ready!(msg_f.poll());
                    if bytes.is_none() {
                        self.read_done = true;
                    }
                }
            }

            if self.read_done {
                if finished {
                    return Ok(Async::Ready(None));
                }
                return Ok(Async::NotReady);
            }

            // so msg_f must be either stale or not initialised yet.
            self.msg_f.take();
            let msg_f = self.call.start_recv_message()?;
            self.msg_f = Some(msg_f);
            if let Some(ref data) = bytes {
                let msg = (self.resp_de)(data)?;
                return Ok(Async::Ready(Some(msg)));
            }
        }
    }
}

/// A receiver for server streaming call.
pub struct ClientSStreamReceiver<Resp> {
    imp: ResponseStreamImpl<Call, Finish, Resp>,
}

impl<Resp> ClientSStreamReceiver<Resp> {
    fn new(
        call: Call,
        finish_f: CqFuture<BatchMessage>,
        de: DeserializeFn<Resp>,
    ) -> ClientSStreamReceiver<Resp> {
        let finish = Finish::new(finish_f);
        ClientSStreamReceiver {
            imp: ResponseStreamImpl::new(call, finish, de),
        }
    }

    pub fn cancel(&mut self) {
        self.imp.cancel()
    }
}

impl<Resp> Stream for ClientSStreamReceiver<Resp> {
    type Item = Resp;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Resp>, Error> {
        self.imp.poll()
    }
}

/// A response receiver for duplex call.
pub struct ClientDuplexReceiver<Resp> {
    imp: ResponseStreamImpl<Arc<SpinLock<Call>>, Arc<SpinLock<Finish>>, Resp>,
}

impl<Resp> ClientDuplexReceiver<Resp> {
    fn new(
        call: Arc<SpinLock<Call>>,
        finish: Arc<SpinLock<Finish>>,
        de: DeserializeFn<Resp>,
    ) -> ClientDuplexReceiver<Resp> {
        ClientDuplexReceiver {
            imp: ResponseStreamImpl::new(call, finish, de),
        }
    }

    pub fn cancel(&mut self) {
        self.imp.cancel()
    }
}

impl<Resp> Stream for ClientDuplexReceiver<Resp> {
    type Item = Resp;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Resp>, Error> {
        self.imp.poll()
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
