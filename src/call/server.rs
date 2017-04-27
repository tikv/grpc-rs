use std::slice;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use grpc_sys::{self, GrpcRequestCallContext, GprTimespec, GprClockType, GrpcStatusCode};
use futures::{Future, Stream, Async, Poll, Sink, AsyncSink, StartSend};
use protobuf::{self, Message, MessageStatic};

use call::{Call, StreamingBase, SinkBase};
use server::Inner;
use promise::CqFuture;
use error::{Result, Error};
use super::RpcStatus;

pub struct Deadline {
    spec: GprTimespec,
}

impl Deadline {
    fn new(spec: GprTimespec) -> Deadline {
        let realtime_spec = unsafe {
            grpc_sys::gpr_convert_clock_type(spec, GprClockType::Realtime)
        };

        Deadline {
            spec: realtime_spec,
        }
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
        let ctx = unsafe {
            grpc_sys::grpcwrap_request_call_context_create()
        };

        RequestContext {
            ctx: ctx,
            inner: Some(inner),
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
        let method = unsafe {
            grpc_sys::grpcwrap_request_call_context_method(self.ctx, &mut len)
        };

        unsafe {
            slice::from_raw_parts(method as _, len)
        }
    }

    fn host(&self) -> &[u8] {
        let mut len = 0;
        let host = unsafe {
            grpc_sys::grpcwrap_request_call_context_host(self.ctx, &mut len)
        };

        unsafe {
            slice::from_raw_parts(host as _, len)
        }
    }

    fn deadline(&self) -> Deadline {
        let t = unsafe {
            grpc_sys::grpcwrap_request_call_context_deadline(self.ctx)
        };

        Deadline::new(t)
    }
}

impl Drop for RequestContext {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpcwrap_request_call_context_destroy(self.ctx)
        }
    }
}

pub struct UnaryRequest<T> {
    req_f: CqFuture,
    _req: PhantomData<T>
}

impl<T> UnaryRequest<T> {
    fn new(req_f: CqFuture) -> UnaryRequest<T> {
        UnaryRequest {
            req_f: req_f,
            _req: PhantomData,
        }
    }
}

impl<T: MessageStatic> Future for UnaryRequest<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let data = try_ready!(self.req_f.poll_raw_resp());
        let msg = try!(protobuf::parse_from_bytes(&data));
        Ok(Async::Ready(msg))
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
    close_f: CqFuture,
    write_flags: u32,
    _resp: PhantomData<T>,
}

impl<T: Message> UnaryResponseSink<T> {
    fn new(call: Call, close_f: CqFuture) -> UnaryResponseSink<T> {
        UnaryResponseSink {
            call: call,
            close_f: close_f,
            write_flags: 0,
            _resp: PhantomData,
        }
    }

    pub fn succeess(self, t: T) -> Result<UnarySinkResult> {
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

        let cq_f = try!(self.call.start_send_status_from_server(&status, true, data, self.write_flags));
        
        Ok(UnarySinkResult {
            _call: self.call,
            close_f: self.close_f,
            cq_f: cq_f,
        })
    }
}

pub struct UnarySinkResult {
    _call: Call,
    close_f: CqFuture,
    cq_f: CqFuture,
}

impl Future for UnarySinkResult {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        match self.cq_f.poll_raw_resp() {
            Ok(Async::Ready(_)) | Err(Error::FutureStale) => {
                try_ready!(self.close_f.poll_raw_resp());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

pub struct ClientStreamingResponseSink<T> {
    call: Arc<Mutex<Call>>,
    close_f: CqFuture,
    write_flags: u32,
    _resp: PhantomData<T>,
}

impl<T: Message> ClientStreamingResponseSink<T> {
    fn new(call: Arc<Mutex<Call>>, close_f: CqFuture) -> ClientStreamingResponseSink<T> {
        ClientStreamingResponseSink {
            call: call,
            close_f: close_f,
            write_flags: 0,
            _resp: PhantomData,
        }
    }

    pub fn succeess(self, t: T) -> Result<ClientStreamingSinkResult> {
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
    close_f: CqFuture,
    cq_f: CqFuture,
}

impl Future for ClientStreamingSinkResult {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        match self.cq_f.poll_raw_resp() {
            Ok(Async::Ready(_)) | Err(Error::FutureStale) => {
                try_ready!(self.close_f.poll_raw_resp());
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
    close_f: CqFuture,
    status: RpcStatus,
    _resp: PhantomData<T>,
}

impl<T> ResponseSink<T> {
    fn new(call: Arc<Mutex<Call>>, close_f: CqFuture) -> ResponseSink<T> {
        ResponseSink {
            call: call,
            base: SinkBase::new(0, true),
            close_f: close_f,
            status: RpcStatus::new(GrpcStatusCode::Ok),
            _resp: PhantomData,
        }
    }

    pub fn set_status(&mut self, status: RpcStatus) {
        assert!(self.base.close_f.is_none());
        self.status = status;
    }
}

impl<T: Message> Sink for ResponseSink<T> {
    type SinkItem = T;
    type SinkError = Error;

    fn start_send(&mut self, item: T) -> StartSend<T, Error> {
        let mut call = self.call.lock().unwrap();
        self.base.start_send(&mut call, |buf| item.write_to_vec(buf)).map(|s| {
            if s {
                AsyncSink::Ready
            } else {
                AsyncSink::NotReady(item)
            }
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
            let close_f = try!(call.start_send_status_from_server(&self.status, self.base.send_metadata, None, self.base.flags));
            self.base.close_f = Some(close_f);
        }

        match self.base.close_f.as_ref().unwrap().poll_raw_resp() {
            Ok(Async::Ready(_)) | Err(Error::FutureStale) => {
                try_ready!(self.close_f.poll_raw_resp());
                Ok(Async::Ready(()))
            },
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
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

pub fn execute_unary<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, UnaryRequest<P>, UnaryResponseSink<Q>) {
    let mut call = ctx.ctx.take_call().unwrap();
    let close_f = match call.start_server_side() {
        Ok(f) => f,
        Err(_) => return,
    };
    let cq_f = match call.start_recv_message() {
        Ok(f) => f,
        // TODO: log?
        Err(_) => return,
    };
    let req_f = UnaryRequest::new(cq_f);
    let sink = UnaryResponseSink::new(call, close_f);
    f(ctx, req_f, sink)
}

pub fn execute_client_streaming<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, RequestStream<P>, ClientStreamingResponseSink<Q>) {
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

pub fn execute_server_streaming<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, UnaryRequest<P>, ResponseSink<Q>) {
    let call = Arc::new(Mutex::new(ctx.ctx.take_call().unwrap()));
    let close_f = {
        let mut call = call.lock().unwrap();
        match call.start_server_side() {
            Ok(f) => f,
            Err(_) => return,
        }
    };
    let req_f = {
        // TODO: remove lock
        let mut call = call.lock().unwrap();
        match call.start_recv_message() {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    let req_s = UnaryRequest::new(req_f);
    let sink = ResponseSink::new(call, close_f);
    f(ctx, req_s, sink)
}

pub fn execute_duplex_streaming<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, RequestStream<P>, ResponseSink<Q>) {
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

pub fn execute(ctx: RequestContext, f: &Box<Fn(RpcContext)>) {
    let rpc_ctx = RpcContext::new(ctx);
    f(rpc_ctx)
}
