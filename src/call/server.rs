
use async::{BatchFuture, Promise};
use call::{BatchContext, Call, MethodType, SinkBase, StreamingBase};
use cq::CompletionQueue;
use error::{Error, Result};
use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};

use grpc_sys::{self, GprClockType, GprTimespec, GrpcCallStatus, GrpcRequestCallContext};
use protobuf::{self, Message, MessageStatic};
use server::{CallBack, Inner};
use std::{result, slice};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use super::RpcStatus;

pub struct Deadline {
    spec: GprTimespec,
}

impl Deadline {
    fn new(spec: GprTimespec) -> Deadline {
        let realtime_spec =
            unsafe { grpc_sys::gpr_convert_clock_type(spec, GprClockType::Realtime) };

        Deadline { spec: realtime_spec }
    }

    pub fn passed(&self) -> bool {
        unsafe {
            let now = grpc_sys::gpr_now(GprClockType::Realtime);
            grpc_sys::gpr_time_cmp(now, self.spec) >= 0
        }
    }
}

pub struct RequestContext {
    ctx: *mut GrpcRequestCallContext,
    inner: Option<Arc<Inner>>,
}

impl RequestContext {
    pub fn new(inner: Arc<Inner>) -> RequestContext {
        let ctx = unsafe { grpc_sys::grpcwrap_request_call_context_create() };

        RequestContext {
            ctx: ctx,
            inner: Some(inner),
        }
    }

    pub fn handle_stream_req(self, inner: &Inner) -> result::Result<(), Self> {
        match inner.get_method(self.method()) {
            Some(handler) => {
                match handler.method_type() {
                    MethodType::Unary |
                    MethodType::ServerStreaming => Err(self),
                    _ => {
                        execute(self, &[], handler.cb());
                        Ok(())
                    }
                }
            }
            None => {
                // TODO: handle undefine method properly.
                Ok(())
            }
        }
    }

    pub fn handle_unary_req(self, inner: Arc<Inner>, cq: &CompletionQueue) {
        // fetch message before calling callback.
        let prom = Box::new(Promise::unary_request(self, inner));
        let batch_ctx = prom.batch_ctx().unwrap().as_ptr();
        let request_ctx = prom.request_ctx().unwrap().as_ptr();
        let tag = Box::into_raw(prom);
        unsafe {
            let call = grpc_sys::grpcwrap_request_call_context_get_call(request_ctx);
            let code = grpc_sys::grpcwrap_call_recv_message(call, batch_ctx, tag as _);
            if code != GrpcCallStatus::Ok {
                let prom = Box::from_raw(tag);
                // TODO: log
                prom.resolve(cq, false);
            }
        }
    }

    pub fn take_inner(&mut self) -> Option<Arc<Inner>> {
        self.inner.take()
    }

    pub fn as_ptr(&self) -> *mut GrpcRequestCallContext {
        self.ctx
    }

    fn take_call(&mut self) -> Option<Call> {
        unsafe {
            let call = grpc_sys::grpcwrap_request_call_context_take_call(self.ctx);
            if call.is_null() {
                return None;
            }

            Some(Call::from_raw(call))
        }
    }

    pub fn method(&self) -> &[u8] {
        let mut len = 0;
        let method = unsafe { grpc_sys::grpcwrap_request_call_context_method(self.ctx, &mut len) };

        unsafe { slice::from_raw_parts(method as _, len) }
    }

    fn host(&self) -> &[u8] {
        let mut len = 0;
        let host = unsafe { grpc_sys::grpcwrap_request_call_context_host(self.ctx, &mut len) };

        unsafe { slice::from_raw_parts(host as _, len) }
    }

    fn deadline(&self) -> Deadline {
        let t = unsafe { grpc_sys::grpcwrap_request_call_context_deadline(self.ctx) };

        Deadline::new(t)
    }
}

pub struct UnaryRequestContext {
    request: RequestContext,
    inner: Option<Arc<Inner>>,
    batch: BatchContext,
}

impl UnaryRequestContext {
    pub fn new(ctx: RequestContext, inner: Arc<Inner>) -> UnaryRequestContext {
        UnaryRequestContext {
            request: ctx,
            inner: Some(inner),
            batch: BatchContext::new(),
        }
    }

    pub fn batch_ctx(&self) -> &BatchContext {
        &self.batch
    }

    pub fn request_ctx(&self) -> &RequestContext {
        &self.request
    }

    pub fn take_inner(&mut self) -> Option<Arc<Inner>> {
        self.inner.take()
    }

    pub fn handle(self, inner: &Arc<Inner>, data: &[u8]) {
        let handler = inner.get_method(self.request.method()).unwrap();
        // TODO: debug assert
        execute(self.request, data, handler.cb())
    }
}

impl Drop for RequestContext {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpcwrap_request_call_context_destroy(self.ctx) }
    }
}

pub struct RequestStream<T> {
    call: Arc<Mutex<Call>>,
    base: StreamingBase,
    _req: PhantomData<T>,
}

impl<T> RequestStream<T> {
    fn new(call: Arc<Mutex<Call>>) -> RequestStream<T> {
        RequestStream {
            call: call,
            base: StreamingBase::new(None),
            _req: PhantomData,
        }
    }
}

impl<T: MessageStatic> Stream for RequestStream<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<T>, Error> {
        let data = {
            let mut call = self.call.lock().unwrap();
            try_ready!(self.base.poll(&mut call, false))
        };

        match data {
            None => Ok(Async::Ready(None)),
            Some(data) => {
                let msg = try!(protobuf::parse_from_bytes(&data));
                Ok(Async::Ready(Some(msg)))
            }
        }
    }
}

pub struct UnaryResponseSink<T> {
    call: Call,
    close_f: BatchFuture,
    write_flags: u32,
    _resp: PhantomData<T>,
}

impl<T: Message> UnaryResponseSink<T> {
    fn new(call: Call, close_f: BatchFuture) -> UnaryResponseSink<T> {
        UnaryResponseSink {
            call: call,
            close_f: close_f,
            write_flags: 0,
            _resp: PhantomData,
        }
    }

    pub fn success(self, t: T) -> Result<UnarySinkResult> {
        self.complete(RpcStatus::ok(), Some(t))
    }

    pub fn fail(self, status: RpcStatus) -> UnarySinkResult {
        self.complete(status, None).unwrap()
    }

    fn complete(mut self, status: RpcStatus, t: Option<T>) -> Result<UnarySinkResult> {
        let data = match t {
            Some(t) => Some(try!(t.write_to_bytes())),
            None => None,
        };

        let cq_f =
            try!(self.call
                     .start_send_status_from_server(&status, true, data, self.write_flags));

        Ok(UnarySinkResult {
               _call: self.call,
               close_f: self.close_f,
               cq_f: cq_f,
           })
    }
}

pub struct UnarySinkResult {
    _call: Call,
    close_f: BatchFuture,
    cq_f: BatchFuture,
}

impl Future for UnarySinkResult {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        match self.cq_f.poll() {
            Ok(Async::Ready(_)) |
            Err(Error::FutureStale) => {
                try_ready!(self.close_f.poll());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

pub struct ClientStreamingResponseSink<T> {
    call: Arc<Mutex<Call>>,
    close_f: BatchFuture,
    write_flags: u32,
    _resp: PhantomData<T>,
}

impl<T: Message> ClientStreamingResponseSink<T> {
    fn new(call: Arc<Mutex<Call>>, close_f: BatchFuture) -> ClientStreamingResponseSink<T> {
        ClientStreamingResponseSink {
            call: call,
            close_f: close_f,
            write_flags: 0,
            _resp: PhantomData,
        }
    }

    pub fn success(self, t: T) -> Result<ClientStreamingSinkResult> {
        self.complete(RpcStatus::ok(), Some(t))
    }

    pub fn fail(self, status: RpcStatus) -> ClientStreamingSinkResult {
        self.complete(status, None).unwrap()
    }

    fn complete(self, status: RpcStatus, t: Option<T>) -> Result<ClientStreamingSinkResult> {
        let data = match t {
            Some(t) => Some(try!(t.write_to_bytes())),
            None => None,
        };

        let cq_f = {
            let mut call = self.call.lock().unwrap();
            try!(call.start_send_status_from_server(&status, true, data, self.write_flags))
        };

        Ok(ClientStreamingSinkResult {
               _call: self.call,
               close_f: self.close_f,
               cq_f: cq_f,
           })
    }
}

pub struct ClientStreamingSinkResult {
    _call: Arc<Mutex<Call>>,
    close_f: BatchFuture,
    cq_f: BatchFuture,
}

impl Future for ClientStreamingSinkResult {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        match self.cq_f.poll() {
            Ok(Async::Ready(_)) |
            Err(Error::FutureStale) => {
                try_ready!(self.close_f.poll());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

pub struct ResponseSink<T> {
    call: Arc<Mutex<Call>>,
    base: SinkBase,
    close_f: BatchFuture,
    status: RpcStatus,
    _resp: PhantomData<T>,
}

impl<T> ResponseSink<T> {
    fn new(call: Arc<Mutex<Call>>, close_f: BatchFuture) -> ResponseSink<T> {
        ResponseSink {
            call: call,
            base: SinkBase::new(0, true),
            close_f: close_f,
            status: RpcStatus::ok(),
            _resp: PhantomData,
        }
    }

    pub fn set_status(&mut self, status: RpcStatus) {
        assert!(self.base.close_f.is_none());
        self.status = status;
    }

    pub fn fail(self, status: RpcStatus) -> Result<SinkFailure> {
        assert!(self.base.close_f.is_none());
        let fail_f = {
            let mut call = self.call.lock().unwrap();
            try!(call.start_send_status_from_server(&status,
                                                    self.base.send_metadata,
                                                    None,
                                                    self.base.flags))
        };

        Ok(SinkFailure {
               _call: self.call,
               close_f: self.close_f,
               fail_f: Some(fail_f),
           })
    }
}

impl<T: Message> Sink for ResponseSink<T> {
    type SinkItem = T;
    type SinkError = Error;

    fn start_send(&mut self, item: T) -> StartSend<T, Error> {
        let mut call = self.call.lock().unwrap();
        self.base
            .start_send(&mut call, |buf| item.write_to_vec(buf))
            .map(|s| if s {
                     AsyncSink::Ready
                 } else {
                     AsyncSink::NotReady(item)
                 })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        self.base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        if self.base.close_f.is_none() {
            if let Async::NotReady = try!(self.base.poll_complete()) {
                return Ok(Async::NotReady);
            }

            let mut call = self.call.lock().unwrap();
            let close_f = try!(call.start_send_status_from_server(&self.status,
                                                                  self.base.send_metadata,
                                                                  None,
                                                                  self.base.flags));
            self.base.close_f = Some(close_f);
        }

        match self.base.close_f.as_mut().unwrap().poll() {
            Ok(Async::Ready(_)) |
            Err(Error::FutureStale) => {
                try_ready!(self.close_f.poll());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

pub struct SinkFailure {
    _call: Arc<Mutex<Call>>,
    close_f: BatchFuture,
    fail_f: Option<BatchFuture>,
}

impl Future for SinkFailure {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        if let Some(ref mut f) = self.fail_f {
            try_ready!(f.poll());
        }

        self.fail_f.take();
        try_ready!(self.close_f.poll());
        Ok(Async::Ready(()))
    }
}

pub struct RpcContext {
    ctx: RequestContext,
    deadline: Deadline,
}

impl RpcContext {
    fn new(ctx: RequestContext) -> RpcContext {
        RpcContext {
            deadline: ctx.deadline(),
            ctx: ctx,
        }
    }

    pub fn method(&self) -> &[u8] {
        self.ctx.method()
    }

    pub fn host(&self) -> &[u8] {
        self.ctx.host()
    }

    pub fn deadline(&self) -> &Deadline {
        &self.deadline
    }
}

pub fn execute_unary<P, Q, F>(mut ctx: RpcContext, payload: &[u8], f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, P, UnaryResponseSink<Q>)
{
    let mut call = ctx.ctx.take_call().unwrap();
    let close_f = match call.start_server_side() {
        Ok(f) => f,
        Err(_) => return,
    };
    let request = match protobuf::parse_from_bytes(payload) {
        Ok(f) => f,
        // TODO: log?
        Err(_) => return,
    };
    let sink = UnaryResponseSink::new(call, close_f);
    f(ctx, request, sink)
}

pub fn execute_client_streaming<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, RequestStream<P>, ClientStreamingResponseSink<Q>)
{
    let call = Arc::new(Mutex::new(ctx.ctx.take_call().unwrap()));
    let close_f = {
        let mut call = call.lock().unwrap();
        match call.start_server_side() {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    let req_s = RequestStream::new(call.clone());
    let sink = ClientStreamingResponseSink::new(call, close_f);
    f(ctx, req_s, sink)
}

pub fn execute_server_streaming<P, Q, F>(mut ctx: RpcContext, payload: &[u8], f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, P, ResponseSink<Q>)
{
    // TODO: remove lock.
    let call = Arc::new(Mutex::new(ctx.ctx.take_call().unwrap()));
    let close_f = {
        let mut call = call.lock().unwrap();
        match call.start_server_side() {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    let request = match protobuf::parse_from_bytes(payload) {
        Ok(t) => t,
        Err(_) => return,
    };

    let sink = ResponseSink::new(call, close_f);
    f(ctx, request, sink)
}

pub fn execute_duplex_streaming<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, RequestStream<P>, ResponseSink<Q>)
{
    let call = Arc::new(Mutex::new(ctx.ctx.take_call().unwrap()));
    let close_f = {
        let mut call = call.lock().unwrap();
        match call.start_server_side() {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    let req_s = RequestStream::new(call.clone());
    let sink = ResponseSink::new(call, close_f);
    f(ctx, req_s, sink)
}

fn execute(ctx: RequestContext, payload: &[u8], f: &CallBack) {
    let rpc_ctx = RpcContext::new(ctx);
    f(rpc_ctx, payload)
}
