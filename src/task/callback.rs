// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::call::server::{RequestContext, UnaryRequestContext};
use crate::call::{BatchContext, Call};
use crate::cq::CompletionQueue;
use crate::server::{self, RequestCallContext};

pub struct Request {
    ctx: RequestContext,
}

impl Request {
    pub fn new(rc: RequestCallContext) -> Request {
        let ctx = RequestContext::new(rc);
        Request { ctx }
    }

    pub fn context(&self) -> &RequestContext {
        &self.ctx
    }

    pub fn resolve(mut self, cq: &CompletionQueue, success: bool) {
        let mut rc = self.ctx.take_request_call_context().unwrap();
        if !success {
            server::request_call(rc, cq);
            return;
        }

        match self.ctx.handle_stream_req(cq, &mut rc) {
            Ok(_) => server::request_call(rc, cq),
            Err(ctx) => ctx.handle_unary_req(rc, cq),
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
