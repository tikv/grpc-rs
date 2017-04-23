use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::{Deref, DerefMut};

use futures::task::{self, Task};
use futures::{Async, Poll};
use protobuf::{self, MessageStatic};
use grpc_sys::GrpcStatusCode;

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

pub struct Promise {
    ty: PromiseType,
    handle: UnsafeCell<NotifyHandle>,
    lock: AtomicBool,
}

unsafe impl Sync for Promise {}

impl Promise {
    pub fn new(ty: PromiseType) -> Promise {
        Promise {
            handle: UnsafeCell::new(Default::default()),
            ty: ty,
            lock: AtomicBool::new(false),
        }
    }

    fn lock(&self) -> PromiseGuard {
        // TODO: what if poison?
        while self.lock.swap(true, Ordering::SeqCst) {}
        PromiseGuard {
            promise: self,
        }
    }

    pub fn on_ready(&self, ctx: BatchContext, success: bool) {
        let mut guard = self.lock();
        match self.ty {
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

    // only call this method in poll context.
    pub fn poll_raw_resp(&self) -> Poll<Result<Vec<u8>>, Error> {
        let mut guard = self.lock();
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

struct PromiseGuard<'a> {
    promise: &'a Promise,
}

impl<'a> PromiseGuard<'a> {
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

impl<'a> Deref for PromiseGuard<'a> {
    type Target = NotifyHandle;

    fn deref(&self) -> &NotifyHandle {
        unsafe { &*self.promise.handle.get() }
    }
}

impl<'a> DerefMut for PromiseGuard<'a> {
    fn deref_mut(&mut self) -> &mut NotifyHandle {
        unsafe { &mut *self.promise.handle.get() }
    }
}

impl<'a> Drop for PromiseGuard<'a> {
    fn drop(&mut self) {
        assert_eq!(true, self.promise.lock.swap(false, Ordering::SeqCst));
    }
}
