use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::{ptr, mem, slice, usize, result};
use std::marker::PhantomData;

use futures::{Future, Poll, Async, Stream, AsyncSink, Sink, StartSend};
use grpc_sys::{self, GrpcCall, GrpcBatchContext, GrpcCallStatus, GrpcStatusCode};
use protobuf::{self, Message, MessageStatic};

use channel::Channel;
use promise::{Promise, PromiseType};
use error::{Result, Error};

#[derive(Clone, Copy)]
pub enum MethodType {
    Unary,
    ClientStreaming,
    ServerStreaming,
    Dulex
}

pub struct Method {
    ty: MethodType,
    name: &'static str,
}

impl Method {
    pub fn new(ty: MethodType, name: &'static str) -> Method {
        Method {
            ty: ty,
            name: name,
        }
    }

    pub fn ty(&self) -> MethodType {
        self.ty
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

fn change_flag(res: &mut u32, flag: u32, set: bool) {
    if set {
        *res = flag;
    } else {
        *res &= !flag;
    }
}

#[derive(Default)]
pub struct CallOption {
    timeout: Option<Duration>,
    write_flags: u32,
    call_flags: u32,
}

impl CallOption {
    pub fn with_idempotent(mut self, is_idempotent: bool) -> CallOption {
        change_flag(&mut self.call_flags, grpc_sys::GRPC_INITIAL_METADATA_IDEMPOTENT_REQUEST, is_idempotent);
        self
    }

    pub fn with_wait_for_ready(mut self, wait_for_ready: bool) -> CallOption {
        change_flag(&mut self.call_flags, grpc_sys::GRPC_INITIAL_METADATA_WAIT_FOR_READY, wait_for_ready);
        self
    }

    pub fn with_cacheable(mut self, cacheable: bool) -> CallOption {
        change_flag(&mut self.call_flags, grpc_sys::GRPC_INITIAL_METADATA_CACHEABLE_REQUEST, cacheable);
        self
    }

    pub fn with_buffer_hint(mut self, need_buffered: bool) -> CallOption {
        change_flag(&mut self.write_flags, grpc_sys::GRPC_WRITE_BUFFER_HINT, need_buffered);
        self
    }

    pub fn with_force_no_compress(mut self, no_compress: bool) -> CallOption {
        change_flag(&mut self.write_flags, grpc_sys::GRPC_WRITE_NO_COMPRESS, no_compress);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> CallOption {
        self.timeout = Some(timeout);
        self
    }

    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

#[derive(Debug)]
pub struct RpcStatus {
    pub status: GrpcStatusCode,
    pub details: String,
}

pub struct BatchContext {
    ctx: *mut GrpcBatchContext,
}

impl BatchContext {
    pub fn new() -> BatchContext {
        BatchContext {
            ctx: unsafe {
                grpc_sys::grpcwrap_batch_context_create()
            }
        }
    }
    pub fn from_raw(ctx: *mut GrpcBatchContext) -> BatchContext {
        BatchContext {
            ctx: ctx,
        }
    }

    pub fn into_raw(self) -> *mut GrpcBatchContext {
        let ctx = self.ctx;
        // So the drop method won't be called.
        mem::forget(self);
        ctx
    }

    pub fn rpc_status(&self) -> RpcStatus {
        let status = unsafe {
            grpc_sys::grpcwrap_batch_context_recv_status_on_client_status(self.ctx)
        };
        let details = if status == GrpcStatusCode::Ok {
            String::new()
        } else {
            unsafe {
                let mut details_len = 0;
                let details_ptr = grpc_sys::grpcwrap_batch_context_recv_status_on_client_details(self.ctx, &mut details_len);
                let details_slice = slice::from_raw_parts(details_ptr as *const _, details_len);
                String::from_utf8_lossy(details_slice).into_owned()
            }
        };
        
        RpcStatus {
            status: status,
            details: details,
        }
    }

    pub fn recv_message(&self) -> Vec<u8> {
        // TODO: avoid copy
        let len = unsafe {
            grpc_sys::grpcwrap_batch_context_recv_message_length(self.ctx)
        };
        if len == usize::MAX {
            return Vec::new();
        }
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            grpc_sys::grpcwrap_batch_context_recv_message_to_buffer(self.ctx, buffer.as_mut_ptr() as *mut _, len);
            buffer.set_len(len);
        }
        buffer
    }

    pub fn is_closed_on_server_canceled(&self) -> bool {
        unsafe {
            grpc_sys::grpcwrap_batch_context_recv_close_on_server_cancelled(self.ctx) != 0
        }
    }

    pub fn set_promise(&mut self, cb: Box<Arc<Promise>>) {
        unsafe {
            grpc_sys::grpcwrap_batch_context_set_tag(self.ctx, Box::into_raw(cb) as *mut _)
        }
    }

    pub fn take_promise(&mut self) -> Option<Box<Arc<Promise>>> {
        unsafe {
            let tag = grpc_sys::grpcwrap_batch_context_take_tag(self.ctx);
            if tag.is_null() {
                return None;
            }
            Some(Box::from_raw(tag as *mut _))
        }
    }
}

impl Drop for BatchContext {
    fn drop(&mut self) {
        self.take_promise();

        unsafe {
            grpc_sys::grpcwrap_batch_context_destroy(self.ctx)
        }
    }
}

pub struct Call {
    call: *mut GrpcCall,
}

unsafe impl Send for Call {}

impl Call {
    pub fn from_raw(call: *mut grpc_sys::GrpcCall) -> Call {
        assert!(!call.is_null());
        Call { call: call }
    }

    pub fn unary_async<P: Message, Q>(channel: &Channel, method: &Method, req: P, opt: CallOption) -> Result<UnaryCallHandler<Q>> {
        let call = channel.create_call(&method, &opt);
        let promise = Arc::new(Promise::new(PromiseType::FinishUnary));
        let mut ctx = BatchContext::new();
        ctx.set_promise(Box::new(promise.clone()));
        let payload = try!(req.write_to_bytes());
        let tag = ctx.into_raw();
        let retured = unsafe {
            grpc_sys::grpcwrap_call_start_unary(call.call, tag, payload.as_ptr() as *const _, payload.len(), opt.write_flags, ptr::null_mut(), opt.call_flags)
        };
        if retured != GrpcCallStatus::Ok {
            BatchContext::from_raw(tag);
            return Err(Error::CallFailure(retured));
        }
        Ok(UnaryCallHandler::new(call, promise))
    }

    pub fn client_streaming<P, Q>(channel: &Channel, method: &Method, opt: CallOption) -> Result<ClientStreamingCallHandler<P, Q>> {
        let call = channel.create_call(&method, &opt);
        let promise = Arc::new(Promise::new(PromiseType::FinishUnary));
        let mut ctx = BatchContext::new();
        ctx.set_promise(Box::new(promise.clone()));
        let tag = ctx.into_raw();
        let returned = unsafe {
            grpc_sys::grpcwrap_call_start_client_streaming(call.call, tag, ptr::null_mut(), opt.call_flags)
        };
        if returned != GrpcCallStatus::Ok {
            BatchContext::from_raw(tag);
            return Err(Error::CallFailure(returned));
        }
        Ok(ClientStreamingCallHandler::new(call, promise, opt.write_flags))
    }

    pub fn server_streaming<P: Message, Q>(channel: &Channel, method: &Method, req: P, opt: CallOption) -> Result<ServerStreamingCallHandler<Q>> {
        let call = channel.create_call(&method, &opt);
        let promise = Arc::new(Promise::new(PromiseType::Finish));
        let mut ctx = BatchContext::new();
        ctx.set_promise(Box::new(promise.clone()));
        let payload = try!(req.write_to_bytes());
        let tag = ctx.into_raw();
        let returned = unsafe {
            grpc_sys::grpcwrap_call_start_server_streaming(call.call, tag, payload.as_ptr() as _, payload.len(), opt.write_flags, ptr::null_mut(), opt.call_flags)
        };
        if returned != GrpcCallStatus::Ok {
            BatchContext::from_raw(tag);
            return Err(Error::CallFailure(returned));
        }
        let header_ctx = BatchContext::new();
        // ignore header for now.
        let header_tag = header_ctx.into_raw();
        let returned = unsafe {
            grpc_sys::grpcwrap_call_recv_initial_metadata(call.call, header_tag)
        };
        if returned != GrpcCallStatus::Ok {
            BatchContext::from_raw(header_tag);
            panic!("failed to start receiving headers: {:?}", returned);
        }
        Ok(ServerStreamingCallHandler::new(call, promise))
    }

    pub fn duplex_streaming<P, Q>(channel: &Channel, method: &Method, opt: CallOption) -> Result<DuplexStreamingCallHandler<P, Q>> {
        let call = channel.create_call(&method, &opt);
        let promise = Arc::new(Promise::new(PromiseType::Finish));
        let mut ctx = BatchContext::new();
        ctx.set_promise(Box::new(promise.clone()));
        let tag = ctx.into_raw();
        let returned = unsafe {
            grpc_sys::grpcwrap_call_start_duplex_streaming(call.call, tag, ptr::null_mut(), opt.call_flags)
        };
        if returned != GrpcCallStatus::Ok {
            BatchContext::from_raw(tag);
            return Err(Error::CallFailure(returned));
        }
        let header_ctx = BatchContext::new();
        // ignore header for now.
        let header_tag = header_ctx.into_raw();
        let returned = unsafe {
            grpc_sys::grpcwrap_call_recv_initial_metadata(call.call, header_tag)
        };
        if returned != GrpcCallStatus::Ok {
            BatchContext::from_raw(header_tag);
            panic!("failed to start receiving headers: {:?}", returned);
        }
        Ok(DuplexStreamingCallHandler::new(call, promise, opt.write_flags))
    }

    pub fn start_send_message(&mut self, msg: &[u8], write_flags: u32, initial_meta: bool) -> Result<Arc<Promise>> {
        let promise = Arc::new(Promise::new(PromiseType::Finish));
        let mut ctx = BatchContext::new();
        ctx.set_promise(Box::new(promise.clone()));
        let tag = ctx.into_raw();
        let i = if initial_meta { 1 } else { 0 };
        let returned = unsafe {
            grpc_sys::grpcwrap_call_send_message(self.call, tag, msg.as_ptr() as _, msg.len(), write_flags, i)
        };
        if returned != GrpcCallStatus::Ok {
            BatchContext::from_raw(tag);
            return Err(Error::CallFailure(returned));
        }
        Ok(promise)
    }

    pub fn start_send_close_client(&mut self) -> Result<Arc<Promise>> {
        let promise = Arc::new(Promise::new(PromiseType::Finish));
        let mut ctx = BatchContext::new();
        // TODO: reuse box.
        ctx.set_promise(Box::new(promise.clone()));
        let tag = ctx.into_raw();
        let returned = unsafe {
            grpc_sys::grpcwrap_call_send_close_from_client(self.call, tag)
        };
        if returned != GrpcCallStatus::Ok {
            BatchContext::from_raw(tag);
            return Err(Error::CallFailure(returned));
        }
        Ok(promise)
    }

    pub fn start_recv_message(&mut self) -> Result<Arc<Promise>> {
        let mut ctx = BatchContext::new();
        let promise = Arc::new(Promise::new(PromiseType::ReadOne));
        ctx.set_promise(Box::new(promise.clone()));
        let tag = ctx.into_raw();
        let returned = unsafe {
            grpc_sys::grpcwrap_call_recv_message(self.call, tag)
        };
        if returned != GrpcCallStatus::Ok {
            BatchContext::from_raw(tag);
            return Err(Error::CallFailure(returned));
        }
        Ok(promise)
    }

    fn cancel(&self) {
        unsafe {
            grpc_sys::grpc_call_cancel(self.call, ptr::null_mut())
        }
    }
}

impl Drop for Call {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_call_destroy(self.call)
        }
    }
}

pub struct UnaryCallHandler<T> {
    call: Call,
    promise: Arc<Promise>,
    _resp: PhantomData<T>,
}

impl<T> UnaryCallHandler<T> {
    fn new(call: Call, promise: Arc<Promise>) -> UnaryCallHandler<T> {
        UnaryCallHandler {
            call: call,
            promise: promise,
            _resp: PhantomData,
        }
    }

    pub fn cancel(&self) {
        self.call.cancel()
    }
}

impl<T: MessageStatic> Future for UnaryCallHandler<T> {
    type Item = Result<T>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Result<T>, Error> {
        self.promise.poll_resp()
    }
}

pub struct UnaryResponseReceiver<T> {
    _call: Call,
    promise: Arc<Promise>,
    _resp: PhantomData<T>,
}

impl<T: MessageStatic> Future for UnaryResponseReceiver<T> {
    type Item = Result<T>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Result<T>, Error> {
        self.promise.poll_resp()
    }
}

struct SinkBase {
    req_promise: Option<Arc<Promise>>,
    close_promise: Option<Arc<Promise>>,
    buf: Vec<u8>,
    flags: u32,
    closed: bool,
}

impl SinkBase {
    fn new(flags: u32) -> SinkBase {
        SinkBase {
            req_promise: None,
            close_promise: None,
            buf: Vec::new(),
            flags: flags,
            closed: false,
        }
    }

    fn start_send<F, E>(&mut self, call: &mut Call, fill_buf: F) -> Result<bool>
         where F: FnOnce(&mut Vec<u8>) -> result::Result<(), E>,
               E: Into<Error> {
        if self.req_promise.is_some() {
            try!(self.poll_complete());
            if self.req_promise.is_some() {
                return Ok(false);
            }
        }

        self.buf.clear();
        if let Err(e) = fill_buf(&mut self.buf) {
            return Err(e.into());
        }
        let promise = try!(call.start_send_message(&self.buf, self.flags, false));
        self.req_promise = Some(promise);
        Ok(true)
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        if let Some(ref promise) = self.req_promise {
            match promise.poll_raw_resp() {
                Ok(Async::Ready(Ok(_))) => {}
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) | Ok(Async::Ready(Err(e))) => return Err(e),
            }
        }
        
        self.req_promise.take();
        Ok(Async::Ready(()))
    }

    fn close(&mut self, call: &mut Call) -> Poll<(), Error> {
        if self.closed {
            return Err(Error::FutureStale);
        }

        if self.close_promise.is_none() {
            if let Async::NotReady = try!(self.poll_complete()) {
                return Ok(Async::NotReady);
            }

            let promise = try!(call.start_send_close_client());
            self.close_promise = Some(promise);
        }

        self.close_promise.as_ref().unwrap().poll_raw_resp().map(|res| {
            res.map(|_| {
                self.closed = true;
            })
        })
    }
}

pub struct ClientStreamingCallHandler<P, Q> {
    call: Call,
    promise: Arc<Promise>,
    sink_base: SinkBase,
    _req: PhantomData<P>,
    _resp: PhantomData<Q>,
}

impl<P, Q> ClientStreamingCallHandler<P, Q> {
    fn new(call: Call, promise: Arc<Promise>, flags: u32) -> ClientStreamingCallHandler<P, Q> {
        ClientStreamingCallHandler {
            call: call,
            promise: promise,
            sink_base: SinkBase::new(flags),
            _req: PhantomData,
            _resp: PhantomData,
        }
    }
}

impl<P: Message, Q> Sink for ClientStreamingCallHandler<P, Q> {
    type SinkItem = P;
    type SinkError = Error;

    fn start_send(&mut self, item: P) -> StartSend<P, Error> {
        self.sink_base.start_send(&mut self.call, |buf| item.write_to_vec(buf)).map(|s| {
            if s {
                AsyncSink::Ready
            } else {
                AsyncSink::NotReady(item)
            }
        })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        self.sink_base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        self.sink_base.close(&mut self.call)
    }
}

impl<P, Q: MessageStatic> ClientStreamingCallHandler<P, Q> {
    pub fn into_receiver(self) -> UnaryResponseReceiver<Q> {
        UnaryResponseReceiver {
            _call: self.call,
            promise: self.promise.clone(),
            _resp: PhantomData,
        }
    }
}

struct StreamingBase {
    promise: Arc<Promise>,
    resp_promise: Option<Arc<Promise>>,
    stale: bool,
}

impl StreamingBase {
    fn new(promise: Arc<Promise>) -> StreamingBase {
        StreamingBase {
            promise: promise,
            resp_promise: None,
            stale: false,
        }
    }
}

pub struct ServerStreamingCallHandler<Q> {
    call: Call,
    base: StreamingBase,
    _resp: PhantomData<Q>,
}

impl<Q> ServerStreamingCallHandler<Q> {
    fn new(call: Call, finish_promise: Arc<Promise>) -> ServerStreamingCallHandler<Q> {
        ServerStreamingCallHandler {
            call: call,
            base: StreamingBase::new(finish_promise),
            _resp: PhantomData,
        }
    }
}

fn poll_streaming<Q: MessageStatic>(call: &mut Call, base: &mut StreamingBase, skip_finish_check: bool) -> Poll<Option<Result<Q>>, Error> {
    if base.stale {
        return Err(Error::FutureStale);
    }
    let mut repoll_resp = base.resp_promise.is_none();
    if let Some(ref promise) = base.resp_promise {
        match promise.poll_raw_resp() {
            // maybe we can schedule next poll immediately?
            Ok(Async::Ready(Ok(bytes))) => {
                if bytes.is_empty() {
                    base.stale = true;
                    return Ok(Async::Ready(None));
                }
                
                let msg = protobuf::parse_from_bytes(&bytes).map_err(From::from);
                return Ok(Async::Ready(Some(msg)))
            }
            Ok(Async::Ready(Err(e))) => return Ok(Async::Ready(Some(Err(e)))),
            Err(Error::FutureStale) => repoll_resp = true,
            Ok(Async::NotReady) => {
                if skip_finish_check {
                    return Ok(Async::NotReady);
                }
            },
            e => panic!("unexpected result: {:?}", e),
        }
    }
    match base.promise.poll_raw_resp() {
        Ok(Async::Ready(Ok(_))) => {
            base.stale = true;
            return Ok(Async::Ready(None))
        },
        Ok(Async::Ready(Err(e))) | Err(e) => {
            base.stale = true;
            return Err(e);
        },
        Ok(Async::NotReady) => {},
    }
    if !repoll_resp {
        return Ok(Async::NotReady);
    }

    base.resp_promise.take();
    return match call.start_recv_message() {
        Err(e) => Err(e),
        Ok(promise) => {
            base.resp_promise = Some(promise);
            poll_streaming(call, base, true)
        },
    };
}

impl<Q: MessageStatic> Stream for ServerStreamingCallHandler<Q> {
    type Item = Result<Q>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Result<Q>>, Error> {
        poll_streaming(&mut self.call, &mut self.base, false)
    }
}

pub struct DuplexStreamingCallHandler<P, Q> {
    // start_batch needs to be synchronized;
    call: Arc<Mutex<Call>>,
    promise: Arc<Promise>,
    sink_base: SinkBase,
    receiver_taken: bool,
    _req: PhantomData<P>,
    _resp: PhantomData<Q>,
}

impl<P, Q> DuplexStreamingCallHandler<P, Q> {
    fn new(call: Call, promise: Arc<Promise>, write_flags: u32) -> DuplexStreamingCallHandler<P, Q> {
        DuplexStreamingCallHandler {
            call: Arc::new(Mutex::new(call)),
            promise: promise,
            sink_base: SinkBase::new(write_flags),
            receiver_taken: false,
            _req: PhantomData,
            _resp: PhantomData,
        }
    }
}

impl<P: Message, Q> Sink for DuplexStreamingCallHandler<P, Q> {
    type SinkItem = P;
    type SinkError = Error;

    fn start_send(&mut self, item: P) -> StartSend<P, Error> {
        let mut call = self.call.lock().unwrap();
        self.sink_base.start_send(&mut call, |buf| item.write_to_vec(buf)).map(|s| {
            if s {
                AsyncSink::Ready
            } else {
                AsyncSink::NotReady(item)
            }
        })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        self.sink_base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        let mut call = self.call.lock().unwrap();
        self.sink_base.close(&mut call)
    }
}

pub struct StreamingResponseReceiver<Q> {
    call: Arc<Mutex<Call>>,
    base: StreamingBase,
    _resp: PhantomData<Q>,
}

impl<P, Q: MessageStatic> DuplexStreamingCallHandler<P, Q> {
    pub fn take_receiver(&mut self) -> Option<StreamingResponseReceiver<Q>> {
        if self.receiver_taken {
            return None;
        }
        self.receiver_taken = true;
        Some(StreamingResponseReceiver {
            call: self.call.clone(),
            base: StreamingBase::new(self.promise.clone()),
            _resp: PhantomData,
        })
    }
}

impl<Q: MessageStatic> Stream for StreamingResponseReceiver<Q> {
    type Item = Result<Q>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Result<Q>>, Error> {
        let mut call = self.call.lock().unwrap();
        poll_streaming(&mut call, &mut self.base, false)
    }
}
