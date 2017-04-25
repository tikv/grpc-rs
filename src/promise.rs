use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use futures::task::{self, Task};
use futures::{Async, Poll};
use protobuf::{self, MessageStatic};
use grpc_sys::{GrpcStatusCode, GrpcBatchContext};

use call::BatchContext;
use error::{Result, Error};


#[derive(Default)]
pub struct NotifyHandle {
    result: Option<Result<Vec<u8>>>,
    park: Option<Task>,
    stale: bool,
}

pub enum PromiseType {
    Finish,
    ReadOne,
    FinishUnary,
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
        InnerGuard {
            inner: self,
        }
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

    fn finish_response(&mut self, ctx: BatchContext) {
        let status = ctx.rpc_status();
        if status.status != GrpcStatusCode::Ok {
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
    pub fn poll_raw_resp(&self) -> Poll<Result<Vec<u8>>, Error> {
        let mut guard = self.inner.lock();
        if guard.stale {
            return Err(Error::FutureStale);
        }
        if let Some(res) = guard.result.take() {
            guard.stale = true;
            return Ok(Async::Ready(res));
        }
        if guard.park.is_none() {
            guard.park = Some(task::park());
        }
        Ok(Async::NotReady)
    }

    // only call this method in poll context.
    pub fn poll_resp<T: MessageStatic>(&self) -> Poll<Result<T>, Error> {
        self.poll_raw_resp().map(|ready| {
            ready.map(|res| {
                res.and_then(|bytes| {
                    protobuf::parse_from_bytes(&bytes).map_err(From::from)
                })
            })
        })
    }
}

pub struct Promise {
    inner: Arc<Inner>,
}

impl Promise {
    pub fn resolve(self, ctx: BatchContext, success: bool) {
        let mut guard = self.inner.lock();
        match self.inner.ty {
            PromiseType::FinishUnary => {
                assert!(success);
                guard.handle_unary_response(ctx);
            }
            PromiseType::Finish => {
                assert!(success);
                guard.finish_response(ctx);
            }
            PromiseType::ReadOne => {
                guard.read_one_msg(&ctx);
            }
        }
    }
}

pub fn pair(ty: PromiseType) -> (CqFuture, Promise) {
    let inner = Arc::new(Inner {
        handle: UnsafeCell::new(Default::default()),
        ty: ty,
        lock: AtomicBool::new(false),
    });
    (CqFuture { inner: inner.clone() }, Promise { inner: inner })
}
