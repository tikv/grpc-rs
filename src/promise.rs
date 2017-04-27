

use call::BatchContext;
use call::server::RequestContext;
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


#[derive(Default)]
pub struct NotifyHandle {
    result: Option<Result<Vec<u8>>>,
    park: Option<Task>,
    stale: bool,
}

#[derive(PartialEq, Debug)]
pub enum PromiseType {
    Finish,
    ReadOne,
    FinishUnary,
    HandleRpc,
}

struct Inner {
    ty: PromiseType,
    handle: UnsafeCell<NotifyHandle>,
    lock: AtomicBool,
}

unsafe impl Sync for Inner {}

impl Inner {
    fn lock(&self) -> InnerGuard {
        // TODO: what if poison?
        while self.lock.swap(true, Ordering::SeqCst) {}
        InnerGuard { inner: self }
    }

    fn resolve_batch(&self, ctx: BatchContext, success: bool) {
        let mut guard = self.lock();
        match self.ty {
            PromiseType::FinishUnary => {
                assert!(success);
                guard.handle_unary_response(ctx);
            }
            PromiseType::Finish => {
                guard.finish_response(ctx, success);
            }
            PromiseType::ReadOne => {
                guard.read_one_msg(&ctx);
            }
            PromiseType::HandleRpc => unreachable!(),
        }
    }

    fn resolve_request(&self, mut ctx: RequestContext, cq: &CompletionQueue, success: bool) {
        if !success {
            let inner = ctx.take_inner().unwrap();
            server::request_call(inner, cq);
            return;
        }
        // don't need to lock here since we just
        assert_eq!(PromiseType::HandleRpc, self.ty);
        let inner = ctx.take_inner().unwrap();
        inner.handle(ctx);
        server::request_call(inner, cq);
    }

    fn resolve_shutdown(&self, _: bool) {
        let mut guard = self.lock();
        guard.result(Ok(Vec::new()))
    }
}

struct InnerGuard<'a> {
    inner: &'a Inner,
}

impl<'a> InnerGuard<'a> {
    fn read_one_msg(&mut self, ctx: &BatchContext) {
        self.result(Ok(ctx.recv_message()));
    }

    fn handle_unary_response(&mut self, ctx: BatchContext) {
        let status = ctx.rpc_status();
        if status.status != GrpcStatusCode::Ok {
            self.result(Err(Error::RpcFailure(status)));
            return;
        }

        self.result(Ok(ctx.recv_message()))
    }

    fn finish_response(&mut self, ctx: BatchContext, succeed: bool) {
        let status = ctx.rpc_status();
        if status.status != GrpcStatusCode::Ok || !succeed {
            self.result(Err(Error::RpcFailure(status)));
            return;
        }

        self.result(Ok(vec![]))
    }

    fn result(&mut self, res: Result<Vec<u8>>) {
        self.result = Some(res);

        if let Some(ref t) = self.park {
            t.unpark();
        }
    }
}

impl<'a> Deref for InnerGuard<'a> {
    type Target = NotifyHandle;

    fn deref(&self) -> &NotifyHandle {
        unsafe { &*self.inner.handle.get() }
    }
}

impl<'a> DerefMut for InnerGuard<'a> {
    fn deref_mut(&mut self) -> &mut NotifyHandle {
        unsafe { &mut *self.inner.handle.get() }
    }
}

impl<'a> Drop for InnerGuard<'a> {
    fn drop(&mut self) {
        assert_eq!(true, self.inner.lock.swap(false, Ordering::SeqCst));
    }
}

pub struct CqFuture {
    inner: Arc<Inner>,
}

impl CqFuture {
    // only call this method in poll context.
    pub fn poll_raw_resp(&self) -> Poll<Vec<u8>, Error> {
        let mut guard = self.inner.lock();
        if guard.stale {
            return Err(Error::FutureStale);
        }
        if let Some(res) = guard.result.take() {
            guard.stale = true;
            return Ok(Async::Ready(try!(res)));
        }
        if guard.park.is_none() {
            guard.park = Some(task::park());
        }
        Ok(Async::NotReady)
    }

    // only call this method in poll context.
    pub fn poll_resp<T: MessageStatic>(&self) -> Poll<T, Error> {
        let bytes = try_ready!(self.poll_raw_resp());
        let t = try!(protobuf::parse_from_bytes(&bytes));
        Ok(Async::Ready(t))
    }
}

enum Context {
    Batch(BatchContext),
    Request(RequestContext),
    Shutdown,
}

use std::fmt::{self, Debug, Formatter};

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Context::Batch(_) => write!(f, "Context::Batch(..)"),
            Context::Request(_) => write!(f, "Context::Request(..)"),
            Context::Shutdown => write!(f, "Context::Shutdown"),
        }
    }
}

pub struct Promise {
    ctx: Context,
    inner: Arc<Inner>,
}

impl Promise {
    pub fn batch_ctx(&self) -> Option<&BatchContext> {
        match self.ctx {
            Context::Batch(ref ctx) => Some(ctx),
            Context::Request(_) => None,
            Context::Shutdown => None,
        }
    }

    pub fn request_ctx(&self) -> Option<&RequestContext> {
        match self.ctx {
            Context::Request(ref ctx) => Some(ctx),
            Context::Batch(_) => None,
            Context::Shutdown => None,
        }
    }

    pub fn resolve(self, cq: &CompletionQueue, success: bool) {
        match self.ctx {
            Context::Batch(ctx) => self.inner.resolve_batch(ctx, success),
            Context::Request(ctx) => self.inner.resolve_request(ctx, cq, success),
            Context::Shutdown => self.inner.resolve_shutdown(success),
        }
    }
}

pub fn batch_pair(ty: PromiseType) -> (CqFuture, Promise) {
    let ctx = BatchContext::new();
    pair(Context::Batch(ctx), ty)
}

pub fn request_pair(inner: Arc<ServerInner>) -> (CqFuture, Promise) {
    let ctx = RequestContext::new(inner);
    pair(Context::Request(ctx), PromiseType::HandleRpc)
}

pub fn shutdown_pair() -> (CqFuture, Promise) {
    pair(Context::Shutdown, PromiseType::Finish)
}

fn pair(ctx: Context, ty: PromiseType) -> (CqFuture, Promise) {
    let inner = Arc::new(Inner {
        handle: UnsafeCell::new(Default::default()),
        ty: ty,
        lock: AtomicBool::new(false),
    });
    (CqFuture { inner: inner.clone() },
     Promise {
        ctx: ctx,
        inner: inner,
    })
}
