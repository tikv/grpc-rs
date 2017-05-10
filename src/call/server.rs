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


use std::slice;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use grpc_sys::{self, GprClockType, GprTimespec, GrpcRequestCallContext};
use protobuf::{self, Message, MessageStatic};

use async::BatchFuture;
use call::{Call, RpcStatusCode, SinkBase, StreamingBase};
use error::Error;
use super::{CallHolder, RpcStatus};

pub struct Deadline {
    spec: GprTimespec,
}

impl Deadline {
    fn new(spec: GprTimespec) -> Deadline {
        let realtime_spec =
            unsafe { grpc_sys::gpr_convert_clock_type(spec, GprClockType::Realtime) };

        Deadline { spec: realtime_spec }
    }

    pub fn exceeded(&self) -> bool {
        unsafe {
            let now = grpc_sys::gpr_now(GprClockType::Realtime);
            grpc_sys::gpr_time_cmp(now, self.spec) >= 0
        }
    }
}

/// Context for accepting a request.
pub struct RequestContext {
    ctx: *mut GrpcRequestCallContext,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        let ctx = unsafe { grpc_sys::grpcwrap_request_call_context_create() };

        RequestContext { ctx: ctx }
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
        let data = try_ready!(self.base.poll(&mut self.call, false));

        match data {
            None => Ok(Async::Ready(None)),
            Some(data) => {
                let msg = try!(protobuf::parse_from_bytes(&data));
                Ok(Async::Ready(Some(msg)))
            }
        }
    }
}

// A helper macro used to implement server side unary sink.
// Not using generic here because we don't need to expose
// `CallHolder` or `Call` to caller.
macro_rules! impl_unary_sink {
    ($t:ident, $rt:ident, $holder:ty) => (
        pub struct $rt {
            _call: $holder,
            close_f: BatchFuture,
            cq_f: BatchFuture,
            flushed: bool,
        }

        impl Future for $rt {
            type Item = ();
            type Error = Error;

            fn poll(&mut self) -> Poll<(), Error> {
                if !self.flushed {
                    try_ready!(self.cq_f.poll());
                    self.flushed = true;
                }

                try_ready!(self.close_f.poll());
                Ok(Async::Ready(()))
            }
        }

        pub struct $t<T> {
            call: $holder,
            close_f: BatchFuture,
            write_flags: u32,
            _resp: PhantomData<T>,
        }

        impl<T: Message> $t<T> {
            fn new(call: $holder, close_f: BatchFuture) -> $t<T> {
                $t {
                    call: call,
                    close_f: close_f,
                    write_flags: 0,
                    _resp: PhantomData,
                }
            }

            pub fn success(self, t: T) -> $rt {
                self.complete(RpcStatus::ok(), Some(t))
            }

            pub fn fail(self, status: RpcStatus) -> $rt {
                self.complete(status, None)
            }

            fn complete(mut self, status: RpcStatus, t: Option<T>) -> $rt {
                let data = match t {
                    Some(t) => Some(t.write_to_bytes().unwrap()),
                    None => None,
                };

                let write_flags = self.write_flags;
                let cq_f = self.call.call(|c| {
                    c.start_send_status_from_server(&status, true, data, write_flags)
                });

                $rt {
                    _call: self.call,
                    close_f: self.close_f,
                    cq_f: cq_f,
                    flushed: false,
                }
            }
        }
    );
}

impl_unary_sink!(UnarySink, UnarySinkResult, Call);
impl_unary_sink!(ClientStreamingSink, ClientStreamingSinkResult, Arc<Mutex<Call>>);

// A macro helper to implement server side streaming sink.
macro_rules! impl_stream_sink {
    ($t:ident, $ft:ident, $holder:ty) => (
        pub struct $t<T> {
            call: $holder,
            base: SinkBase,
            close_f: BatchFuture,
            status: RpcStatus,
            flushed: bool,
            _resp: PhantomData<T>,
        }

        impl<T> $t<T> {
            fn new(call: $holder, close_f: BatchFuture) -> $t<T> {
                $t {
                    call: call,
                    base: SinkBase::new(0, true),
                    close_f: close_f,
                    status: RpcStatus::ok(),
                    flushed: false,
                    _resp: PhantomData,
                }
            }

            pub fn set_status(&mut self, status: RpcStatus) {
                assert!(self.base.close_f.is_none());
                self.status = status;
            }

            pub fn fail(mut self, status: RpcStatus) -> $ft {
                assert!(self.base.close_f.is_none());
                let send_metadata = self.base.send_metadata;
                let flags = self.base.flags;
                let fail_f = self.call.call(|c| {
                    c.start_send_status_from_server(&status, send_metadata, None, flags)
                });

                $ft {
                    _call: self.call,
                    close_f: self.close_f,
                    fail_f: Some(fail_f),
                }
            }
        }

        impl<T: Message> Sink for $t<T> {
            type SinkItem = T;
            type SinkError = Error;

            fn start_send(&mut self, item: T) -> StartSend<T, Error> {
                self.base
                    .start_send(&mut self.call, |buf| item.write_to_vec(buf))
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
                    try_ready!(self.base.poll_complete());

                    let send_metadata = self.base.send_metadata;
                    let flags = self.base.flags;
                    let status = &self.status;
                    let close_f = self.call.call(|c| {
                        c.start_send_status_from_server(status, send_metadata, None, flags)
                    });
                    self.base.close_f = Some(close_f);
                }

                if !self.flushed {
                    try_ready!(self.base.close_f.as_mut().unwrap().poll());
                    self.flushed = true;
                }

                try_ready!(self.close_f.poll());
                Ok(Async::Ready(()))
            }
        }

        pub struct $ft {
            _call: $holder,
            close_f: BatchFuture,
            fail_f: Option<BatchFuture>,
        }

        impl Future for $ft {
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
    )
}

impl_stream_sink!(ServerStreamingSink, ServerStreamingSinkFailure, Call);
impl_stream_sink!(DuplexSink, DuplexSinkFailure, Arc<Mutex<Call>>);

/// A context for rpc handling.
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

// Following four helper functions are used to create a callback closure.

// Helper function to call a unary handler.
pub fn execute_unary<P, Q, F>(mut ctx: RpcContext, payload: &[u8], f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, P, UnarySink<Q>)
{
    let mut call = ctx.ctx.take_call().unwrap();
    let close_f = call.start_server_side();
    let request = match protobuf::parse_from_bytes(payload) {
        Ok(f) => f,
        Err(e) => {
            let status =
                RpcStatus::new(RpcStatusCode::Internal,
                               Some(format!("Failed to deserialize response message: {:?}", e)));
            call.abort(status);
            return;
        }
    };
    let sink = UnarySink::new(call, close_f);
    f(ctx, request, sink)
}

// Helper function to call client streaming handler.
pub fn execute_client_streaming<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, RequestStream<P>, ClientStreamingSink<Q>)
{
    let call = Arc::new(Mutex::new(ctx.ctx.take_call().unwrap()));
    let close_f = {
        let mut call = call.lock().unwrap();
        call.start_server_side()
    };

    let req_s = RequestStream::new(call.clone());
    let sink = ClientStreamingSink::new(call, close_f);
    f(ctx, req_s, sink)
}

// Helper function to call server streaming handler.
pub fn execute_server_streaming<P, Q, F>(mut ctx: RpcContext, payload: &[u8], f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, P, ServerStreamingSink<Q>)
{
    let mut call = ctx.ctx.take_call().unwrap();
    let close_f = call.start_server_side();

    let request = match protobuf::parse_from_bytes(payload) {
        Ok(t) => t,
        Err(e) => {
            let status =
                RpcStatus::new(RpcStatusCode::Internal,
                               Some(format!("Failed to deserialize response message: {:?}", e)));
            call.abort(status);
            return;
        }
    };

    let sink = ServerStreamingSink::new(call, close_f);
    f(ctx, request, sink)
}

// Helper function to call duplex streaming handler.
pub fn execute_duplex_streaming<P, Q, F>(mut ctx: RpcContext, f: &F)
    where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, RequestStream<P>, DuplexSink<Q>)
{
    let call = Arc::new(Mutex::new(ctx.ctx.take_call().unwrap()));
    let close_f = {
        let mut call = call.lock().unwrap();
        call.start_server_side()
    };

    let req_s = RequestStream::new(call.clone());
    let sink = DuplexSink::new(call, close_f);
    f(ctx, req_s, sink)
}

// A helper function used to handle all undefined rpc calls.
pub fn execute_unimplemented(mut ctx: RequestContext) {
    let mut call = ctx.take_call().unwrap();
    call.start_server_side();
    call.abort(RpcStatus::new(RpcStatusCode::Unimplemented, None))
}
