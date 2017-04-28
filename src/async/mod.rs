mod promise;
mod callback;
mod lock;

use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;
use futures::{Async, Future, Poll};
use futures::task::{self, Task};

use call::BatchContext;
use call::server::RequestContext;
use cq::CompletionQueue;
use error::{Error, Result};
use self::callback::{Request as RequestCallback, UnaryRequest as UnaryRequestCallback};
use self::lock::SpinLock;
use self::promise::{Batch as BatchPromise, Shutdown as ShutdownPromise};
use server::Inner as ServerInner;

pub use self::promise::BatchType;

/// A handle that is used to notify future that the task finishes.
pub struct NotifyHandle<T> {
    result: Option<Result<T>>,
    park: Option<Task>,
    stale: bool,
}

impl<T> NotifyHandle<T> {
    fn new() -> NotifyHandle<T> {
        NotifyHandle {
            result: None,
            park: None,
            stale: false,
        }
    }

    /// Set the result and notify future if necessary.
    fn result(&mut self, res: Result<T>) {
        self.result = Some(res);

        if let Some(ref t) = self.park {
            t.unpark();
        }
    }
}

type Inner<T> = SpinLock<NotifyHandle<T>>;

fn new_inner<T>() -> Arc<Inner<T>> {
    Arc::new(SpinLock::new(NotifyHandle::new()))
}

/// A future object for task that is scheduled to `CompletionQueue`.
pub struct CqFuture<T> {
    inner: Arc<Inner<T>>,
}

impl<T> CqFuture<T> {
    fn new(inner: Arc<Inner<T>>) -> CqFuture<T> {
        CqFuture { inner: inner }
    }
}

impl<T> Future for CqFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let mut guard = self.inner.lock();
        if guard.stale {
            return Err(Error::FutureStale);
        }

        if let Some(res) = guard.result.take() {
            guard.stale = true;
            return Ok(Async::Ready(try!(res)));
        }

        // So the task has not finished yet, add notification hook.
        if guard.park.is_none() {
            guard.park = Some(task::park());
        }

        Ok(Async::NotReady)
    }
}

/// Future object for batch jobs.
pub type BatchFuture = CqFuture<Vec<u8>>;

/// A result holder for asynchronous execution.
pub enum Promise {
    Batch(BatchPromise),
    Request(RequestCallback),
    UnaryRequest(UnaryRequestCallback),
    Shutdown(ShutdownPromise),
}

impl Promise {
    /// Generate a future/promise pair for batch jobs.
    pub fn batch_pair(ty: BatchType) -> (CqFuture<Vec<u8>>, Promise) {
        let inner = new_inner();
        let batch = BatchPromise::new(ty, inner.clone());
        (CqFuture::new(inner), Promise::Batch(batch))
    }

    /// Generate a promise for request job. We don't have a eventloop
    /// to pull the future, so just the promise is enough.
    pub fn request(inner: Arc<ServerInner>) -> Promise {
        Promise::Request(RequestCallback::new(inner))
    }

    /// Generate a future/promise pair for shutdown call.
    pub fn shutdown_pair() -> (CqFuture<()>, Promise) {
        let inner = new_inner();
        let shutdown = ShutdownPromise::new(inner.clone());
        (CqFuture::new(inner), Promise::Shutdown(shutdown))
    }

    /// Generate a promise for unary request job.
    pub fn unary_request(ctx: RequestContext, inner: Arc<ServerInner>) -> Promise {
        let cb = UnaryRequestCallback::new(ctx, inner);
        Promise::UnaryRequest(cb)
    }

    /// Get the batch context from result holder.
    pub fn batch_ctx(&self) -> Option<&BatchContext> {
        match *self {
            Promise::Batch(ref prom) => Some(prom.context()),
            Promise::UnaryRequest(ref cb) => Some(cb.batch_ctx()),
            _ => None,
        }
    }

    /// Get the request context from the result holder.
    pub fn request_ctx(&self) -> Option<&RequestContext> {
        match *self {
            Promise::Request(ref prom) => Some(prom.context()),
            Promise::UnaryRequest(ref cb) => Some(cb.request_ctx()),
            _ => None,
        }
    }

    /// Resolve the promise with given status.
    pub fn resolve(self, cq: &CompletionQueue, success: bool) {
        match self {
            Promise::Batch(prom) => prom.resolve(success),
            Promise::Request(cb) => cb.resolve(cq, success),
            Promise::UnaryRequest(cb) => cb.resolve(cq, success),
            Promise::Shutdown(prom) => prom.resolve(success),
        }
    }
}

impl Debug for Promise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Promise::Batch(_) => write!(f, "Context::Batch(..)"),
            Promise::Request(_) => write!(f, "Context::Request(..)"),
            Promise::UnaryRequest(_) => write!(f, "Context::UnaryRequest(..)"),
            Promise::Shutdown(_) => write!(f, "Context::Shutdown"),
        }
    }
}
