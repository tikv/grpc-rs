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


use std::sync::Arc;

use call::{BatchContext, Call};
use call::server::{RequestContext, UnaryRequestContext};
use cq::CompletionQueue;
use server::{self, Inner as ServerInner};

pub struct Request {
    ctx: RequestContext,
}

impl Request {
    pub fn new(inner: Arc<ServerInner>) -> Request {
        let ctx = RequestContext::new(inner);
        Request { ctx: ctx }
    }

    pub fn context(&self) -> &RequestContext {
        &self.ctx
    }

    pub fn resolve(mut self, cq: &CompletionQueue, success: bool) {
        let inner = self.ctx.take_inner().unwrap();
        if !success {
            server::request_call(inner, cq);
            return;
        }

        match self.ctx.handle_stream_req(cq, &inner) {
            Ok(_) => server::request_call(inner, cq),
            Err(ctx) => ctx.handle_unary_req(inner, cq),
        }
    }
}

pub struct UnaryRequest {
    ctx: UnaryRequestContext,
}

impl UnaryRequest {
    pub fn new(ctx: RequestContext, inner: Arc<ServerInner>) -> UnaryRequest {
        let ctx = UnaryRequestContext::new(ctx, inner);
        UnaryRequest { ctx: ctx }
    }

    pub fn batch_ctx(&self) -> &BatchContext {
        self.ctx.batch_ctx()
    }

    pub fn request_ctx(&self) -> &RequestContext {
        self.ctx.request_ctx()
    }

    pub fn resolve(mut self, cq: &CompletionQueue, success: bool) {
        let inner = self.ctx.take_inner().unwrap();
        if !success {
            server::request_call(inner, cq);
            return;
        }

        let data = self.ctx.batch_ctx().recv_message();
        self.ctx
            .handle(&inner, cq, data.as_ref().map(|v| v.as_slice()));
        server::request_call(inner, cq);
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
