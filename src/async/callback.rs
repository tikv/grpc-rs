
use call::BatchContext;
use call::server::{RequestContext, UnaryRequestContext};
use cq::CompletionQueue;
use error::{Error, Result};
use futures::{Async, Poll};

use futures::task::{self, Task};
use grpc_sys::GrpcStatusCode;
use protobuf::{self, MessageStatic};
use server::{self, Inner as ServerInner};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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

        match self.ctx.handle_stream_req(&inner) {
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
        self.ctx.handle(&inner, &data);
        server::request_call(inner, cq);
    }
}
