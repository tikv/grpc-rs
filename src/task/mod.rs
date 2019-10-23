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

mod callback;
mod executor;
mod lock;
mod promise;

use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use futures::task::{self, Task};
use futures::{Async, Future, Poll};

use self::callback::{Abort, Request as RequestCallback, UnaryRequest as UnaryRequestCallback};
use self::executor::SpawnTask;
use self::promise::{Batch as BatchPromise, Shutdown as ShutdownPromise};
use crate::call::server::RequestContext;
use crate::call::{BatchContext, Call, MessageReader};
use crate::cq::CompletionQueue;
use crate::error::{Error, Result};
use crate::server::RequestCallContext;

pub(crate) use self::executor::{Executor, Kicker, UnfinishedWork};
pub use self::lock::SpinLock;
pub use self::promise::BatchType;

/// A handle that is used to notify future that the task finishes.
pub struct NotifyHandle<T> {
    result: Option<Result<T>>,
    task: Option<Task>,
    stale: bool,
}

impl<T> NotifyHandle<T> {
    fn new() -> NotifyHandle<T> {
        NotifyHandle {
            result: None,
            task: None,
            stale: false,
        }
    }

    /// Set the result and notify future if necessary.
    fn set_result(&mut self, res: Result<T>) -> Option<Task> {
        self.result = Some(res);

        self.task.take()
    }
}

type Inner<T> = SpinLock<NotifyHandle<T>>;

fn new_inner<T>() -> Arc<Inner<T>> {
    Arc::new(SpinLock::new(NotifyHandle::new()))
}

/// Get the future status without the need to poll.
///
/// If the future is polled successfully, this function will return None.
/// Not implemented as method as it's only for internal usage.
pub fn check_alive<T>(f: &CqFuture<T>) -> Result<()> {
    let guard = f.inner.lock();
    match guard.result {
        None => Ok(()),
        Some(Err(Error::RpcFailure(ref status))) => {
            Err(Error::RpcFinished(Some(status.to_owned())))
        }
        Some(Ok(_)) | Some(Err(_)) => Err(Error::RpcFinished(None)),
    }
}

/// A future object for task that is scheduled to `CompletionQueue`.
pub struct CqFuture<T> {
    inner: Arc<Inner<T>>,
}

impl<T> CqFuture<T> {
    fn new(inner: Arc<Inner<T>>) -> CqFuture<T> {
        CqFuture { inner }
    }
}

impl<T> Future for CqFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let mut guard = self.inner.lock();
        if guard.stale {
            panic!("Resolved future is not supposed to be polled again.");
        }

        if let Some(res) = guard.result.take() {
            guard.stale = true;
            return Ok(Async::Ready(res?));
        }

        // So the task has not been finished yet, add notification hook.
        if guard.task.is_none() || !guard.task.as_ref().unwrap().will_notify_current() {
            guard.task = Some(task::current());
        }

        Ok(Async::NotReady)
    }
}

/// Future object for batch jobs.
pub type BatchFuture = CqFuture<Option<MessageReader>>;

/// A result holder for asynchronous execution.
// This enum is going to be passed to FFI, so don't use trait or generic here.
pub enum CallTag {
    Batch(BatchPromise),
    Request(RequestCallback),
    UnaryRequest(UnaryRequestCallback),
    Abort(Abort),
    Shutdown(ShutdownPromise),
    Spawn(Arc<SpawnTask>),
}

impl CallTag {
    /// Generate a Future/CallTag pair for batch jobs.
    pub fn batch_pair(ty: BatchType) -> (BatchFuture, CallTag) {
        let inner = new_inner();
        let batch = BatchPromise::new(ty, inner.clone());
        (CqFuture::new(inner), CallTag::Batch(batch))
    }

    /// Generate a CallTag for request job. We don't have an eventloop
    /// to pull the future, so just the tag is enough.
    pub fn request(ctx: RequestCallContext) -> CallTag {
        CallTag::Request(RequestCallback::new(ctx))
    }

    /// Generate a Future/CallTag pair for shutdown call.
    pub fn shutdown_pair() -> (CqFuture<()>, CallTag) {
        let inner = new_inner();
        let shutdown = ShutdownPromise::new(inner.clone());
        (CqFuture::new(inner), CallTag::Shutdown(shutdown))
    }

    /// Generate a CallTag for abort call before handler is called.
    pub fn abort(call: Call) -> CallTag {
        CallTag::Abort(Abort::new(call))
    }

    /// Generate a CallTag for unary request job.
    pub fn unary_request(ctx: RequestContext, rc: RequestCallContext) -> CallTag {
        let cb = UnaryRequestCallback::new(ctx, rc);
        CallTag::UnaryRequest(cb)
    }

    /// Get the batch context from result holder.
    pub fn batch_ctx(&self) -> Option<&BatchContext> {
        match *self {
            CallTag::Batch(ref prom) => Some(prom.context()),
            CallTag::UnaryRequest(ref cb) => Some(cb.batch_ctx()),
            CallTag::Abort(ref cb) => Some(cb.batch_ctx()),
            _ => None,
        }
    }

    /// Get the request context from the result holder.
    pub fn request_ctx(&self) -> Option<&RequestContext> {
        match *self {
            CallTag::Request(ref prom) => Some(prom.context()),
            CallTag::UnaryRequest(ref cb) => Some(cb.request_ctx()),
            _ => None,
        }
    }

    /// Resolve the CallTag with given status.
    pub fn resolve(self, cq: &CompletionQueue, success: bool) {
        match self {
            CallTag::Batch(prom) => prom.resolve(success),
            CallTag::Request(cb) => cb.resolve(cq, success),
            CallTag::UnaryRequest(cb) => cb.resolve(cq, success),
            CallTag::Abort(_) => {}
            CallTag::Shutdown(prom) => prom.resolve(success),
            CallTag::Spawn(notify) => self::executor::resolve(cq, notify, success),
        }
    }
}

impl Debug for CallTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            CallTag::Batch(ref ctx) => write!(f, "CallTag::Batch({:?})", ctx),
            CallTag::Request(_) => write!(f, "CallTag::Request(..)"),
            CallTag::UnaryRequest(_) => write!(f, "CallTag::UnaryRequest(..)"),
            CallTag::Abort(_) => write!(f, "CallTag::Abort(..)"),
            CallTag::Shutdown(_) => write!(f, "CallTag::Shutdown"),
            CallTag::Spawn(_) => write!(f, "CallTag::Spawn"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::*;
    use std::sync::*;
    use std::thread;

    use super::*;
    use crate::env::Environment;

    #[test]
    fn test_resolve() {
        let env = Environment::new(1);

        let (cq_f1, tag1) = CallTag::shutdown_pair();
        let (cq_f2, tag2) = CallTag::shutdown_pair();
        let (tx, rx) = mpsc::channel();

        let handler = thread::spawn(move || {
            tx.send(cq_f1.wait()).unwrap();
            tx.send(cq_f2.wait()).unwrap();
        });

        assert_eq!(rx.try_recv().unwrap_err(), TryRecvError::Empty);
        tag1.resolve(&env.pick_cq(), true);
        assert!(rx.recv().unwrap().is_ok());

        assert_eq!(rx.try_recv().unwrap_err(), TryRecvError::Empty);
        tag2.resolve(&env.pick_cq(), false);
        match rx.recv() {
            Ok(Err(Error::ShutdownFailed)) => {}
            res => panic!("expect shutdown failed, but got {:?}", res),
        }

        handler.join().unwrap();
    }
}
