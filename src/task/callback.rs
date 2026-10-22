// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::call::server::{RequestContext, UnaryRequestContext};
use crate::call::{BatchContext, Call};
use crate::cq::CompletionQueue;
use crate::grpc_sys::{self, grpc_call_error};
use crate::server::{self, RequestCallContext};
use crate::task::CallTag;

pub struct Request {
    ctx: RequestContext,
    /// The call's CQ, which may differ from its initial accept notification CQ.
    cq: CompletionQueue,
}

impl Request {
    pub fn new(rc: RequestCallContext, cq: CompletionQueue) -> Request {
        let ctx = RequestContext::new(rc);
        Request { ctx, cq }
    }

    pub fn context(&self) -> &RequestContext {
        &self.ctx
    }

    pub fn resolve(mut self, cq: &CompletionQueue, success: bool) {
        if success && cq.worker_id() != self.cq.worker_id() {
            self.forward_to_call_queue();
            return;
        }

        let mut rc = self.ctx.take_request_call_context().unwrap();
        if !success {
            server::request_call(rc, &self.cq);
            return;
        }

        match self.ctx.handle_stream_req(cq, &mut rc) {
            Ok(_) => server::request_call(rc, cq),
            Err(ctx) => ctx.handle_unary_req(rc, cq),
        }
    }

    /// Deliver acceptance on the call's bound CQ before touching its per-CQ
    /// handler registry or checkers. An empty batch only posts a notification;
    /// it does not consume a message or change the call's binding.
    fn forward_to_call_queue(self) {
        let cq = self.cq.clone();
        let _cq_ref = match cq.borrow() {
            Err(_) => return,
            Ok(c) => c,
        };
        unsafe {
            let call = grpc_sys::grpcwrap_request_call_context_get_call(self.ctx.as_ptr());
            let tag = Box::into_raw(Box::new(CallTag::Request(self)));
            let code = grpc_sys::grpcwrap_call_kick_completion_queue(call, tag as _);
            if code != grpc_call_error::GRPC_CALL_OK {
                drop(Box::from_raw(tag));
                panic!("failed to forward accepted call: {:?}", code);
            }
        }
    }
}

pub struct UnaryRequest {
    ctx: UnaryRequestContext,
}

impl UnaryRequest {
    pub fn new(ctx: RequestContext, rc: RequestCallContext) -> UnaryRequest {
        let ctx = UnaryRequestContext::new(ctx, rc);
        UnaryRequest { ctx }
    }

    pub fn batch_ctx(&self) -> &BatchContext {
        self.ctx.batch_ctx()
    }

    pub fn request_ctx(&self) -> &RequestContext {
        self.ctx.request_ctx()
    }

    pub fn resolve(mut self, cq: &CompletionQueue, success: bool) {
        let mut rc = self.ctx.take_request_call_context().unwrap();
        if !success {
            server::request_call(rc, cq);
            return;
        }

        let reader = self.ctx.batch_ctx_mut().recv_message();
        self.ctx.handle(&mut rc, cq, reader);
        server::request_call(rc, cq);
    }
}

/// A callback to wait for status for the aborted rpc call to be sent.
pub struct Abort {
    ctx: BatchContext,
    _call: Call,
}

impl Abort {
    pub fn new(call: Call) -> Abort {
        Abort {
            ctx: BatchContext::new(),
            _call: call,
        }
    }

    pub fn batch_ctx(&self) -> &BatchContext {
        &self.ctx
    }
}
