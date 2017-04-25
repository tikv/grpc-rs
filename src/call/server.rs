use std::{slice, mem};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use grpc_sys::{self, GrpcRequestCallContext, GprTimespec, GprClockType, GrpcStatusCode};
use futures::{Future, Stream, Async, Poll, Sink, AsyncSink, StartSend};
use protobuf::{self, Message, MessageStatic};

use cq::CompletionQueue;
use call::{Call, StreamingBase, SinkBase};
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
}

pub struct RequestContext {
    ctx: *mut GrpcRequestCallContext,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        let ctx = unsafe {
            grpc_sys::grpcwrap_request_call_context_create()
        };

        RequestContext {
            ctx: ctx,
        }
    }

    pub unsafe fn from_raw(ctx: *mut GrpcRequestCallContext) -> RequestContext {
        RequestContext {
            ctx: ctx
        }
    }

    pub fn into_raw(self) -> *mut GrpcRequestCallContext {
        let ptr = self.ctx;
        mem::forget(self);
        ptr
    }

    pub fn handle(&mut self, cq: &CompletionQueue) {
        unsafe {

        }
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

    fn method(&self) -> &[u8] {
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
        match try!(self.req_f.poll_raw_resp()) {
            Async::Ready(res) => {
                let data = try!(res);
                let msg = try!(protobuf::parse_from_bytes(&data));
                Ok(Async::Ready(msg))
            }
            Async::NotReady => Ok(Async::NotReady)
        }
    }
}

pub struct RequestStream<T> {
    call: Arc<Mutex<Call>>,
    base: StreamingBase,
    _req: PhantomData<T>,
}

impl<T> RequestStream<T> {
    fn new(call: Arc<Mutex<Call>>, close_f: CqFuture) -> RequestStream<T> {
        RequestStream {
            call: call,
            base: StreamingBase::new(close_f),
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
                let data = try!(data);
                let msg = try!(protobuf::parse_from_bytes(&data));
                Ok(Async::Ready(Some(msg)))
            }
        }
    }
}

pub struct UnaryResponseSink<T> {
    call: Call,
    write_flags: u32,
    _resp: PhantomData<T>,
}

impl<T: Message> UnaryResponseSink<T> {
    fn new(call: Call) -> UnaryResponseSink<T> {
        UnaryResponseSink {
            call: call,
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
        let mut msg_sent = false;
        let cq_f = match t {
            Some(t) => {
                let data = try!(t.write_to_bytes());
                try!(self.call.start_send_message(&data, self.write_flags, true))
            },
            None => {
                msg_sent = true;
                try!(self.call.start_send_status_from_server(&status, true, self.write_flags))
            }
        };
        
        Ok(UnarySinkResult {
            call: self.call,
            status: status,
            cq_f: cq_f,
            write_flags: self.write_flags,
            msg_sent: msg_sent,
        })
    }
}

pub struct UnarySinkResult {
    call: Call,
    status: RpcStatus,
    cq_f: CqFuture,
    write_flags: u32,
    msg_sent: bool,
}

impl Future for UnarySinkResult {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        try_ready!(self.cq_f.poll_raw_resp());

        if self.msg_sent {
            return Ok(Async::Ready(()));
        }

        self.msg_sent = true;
        self.cq_f = try!(self.call.start_send_status_from_server(&self.status, false, self.write_flags));
        try_ready!(self.cq_f.poll_raw_resp());
        Ok(Async::Ready(()))
    }
}

pub struct ClientStreamingResponseSink<T> {
    call: Arc<Mutex<Call>>,
    write_flags: u32,
    _resp: PhantomData<T>,
}

impl<T: Message> ClientStreamingResponseSink<T> {
    fn new(call: Arc<Mutex<Call>>) -> ClientStreamingResponseSink<T> {
        ClientStreamingResponseSink {
            call: call,
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
        let mut msg_sent = false;
        let cq_f = match t {
            Some(t) => {
                let data = try!(t.write_to_bytes());
                let mut call = self.call.lock().unwrap();
                try!(call.start_send_message(&data, self.write_flags, true))
            },
            None => {
                msg_sent = true;
                let mut call = self.call.lock().unwrap();
                try!(call.start_send_status_from_server(&status, true, self.write_flags))
            }
        };
        
        Ok(ClientStreamingSinkResult {
            call: self.call,
            status: status,
            cq_f: cq_f,
            write_flags: self.write_flags,
            msg_sent: msg_sent,
        })
    }
}

pub struct ClientStreamingSinkResult {
    call: Arc<Mutex<Call>>,
    status: RpcStatus,
    cq_f: CqFuture,
    write_flags: u32,
    msg_sent: bool,
}

impl Future for ClientStreamingSinkResult {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        try_ready!(self.cq_f.poll_raw_resp());

        if self.msg_sent {
            return Ok(Async::Ready(()));
        }

        self.msg_sent = true;
        let mut call = self.call.lock().unwrap();
        self.cq_f = try!(call.start_send_status_from_server(&self.status, false, self.write_flags));
        try_ready!(self.cq_f.poll_raw_resp());
        Ok(Async::Ready(()))
    }
}

pub struct ResponseSink<T> {
    call: Arc<Mutex<Call>>,
    base: SinkBase,
    status: RpcStatus,
    closing: bool,
    _resp: PhantomData<T>,
}

impl<T> ResponseSink<T> {
    fn new(call: Arc<Mutex<Call>>) -> ResponseSink<T> {
        ResponseSink {
            call: call,
            base: SinkBase::new(0, true),
            status: RpcStatus::new(GrpcStatusCode::Ok),
            closing: false,
            _resp: PhantomData,
        }
    }

    fn set_status(&mut self, status: RpcStatus) {
        assert!(!self.closing);
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
        if self.base.closed {
            return Err(Error::FutureStale);
        }

        if self.base.close_f.is_none() {
            if let Async::NotReady = try!(self.base.poll_complete()) {
                return Ok(Async::NotReady);
            }

            let mut call = self.call.lock().unwrap();
            let close_f = try!(call.start_send_status_from_server(&self.status, false, self.base.flags));
            self.base.close_f = Some(close_f);
        }

        self.base.close_f.as_ref().unwrap().poll_raw_resp().map(|res| {
            res.map(|_| {
                self.base.closed = true;
            })
        })
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
    let cq_f = match call.start_recv_message() {
        Ok(f) => f,
        // TODO: log?
        Err(_) => return,
    };
    let req_f = UnaryRequest::new(cq_f);
    let sink = UnaryResponseSink::new(call);
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

    let req_s = RequestStream::new(call.clone(), close_f);
    let sink = ClientStreamingResponseSink::new(call);
    f(ctx, req_s, sink)
}

pub fn execute_server_streaming<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, UnaryRequest<P>, ResponseSink<Q>) {
    let call = Arc::new(Mutex::new(ctx.ctx.take_call().unwrap()));
    let req_f = {
        // TODO: remove lock
        let mut call = call.lock().unwrap();
        match call.start_recv_message() {
            Ok(f) => f,
            Err(_) => return,
        }
    };

    let req_s = UnaryRequest::new(req_f);
    let sink = ResponseSink::new(call);
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

    let req_s = RequestStream::new(call.clone(), close_f);
    let sink = ResponseSink::new(call);
    f(ctx, req_s, sink)
}
