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
use std::thread;

use futures::executor::{self, Notify, Spawn};
use futures::{Async, Future};

use super::lock::SpinLock;
use super::CallTag;
use crate::call::Call;
use crate::cq::{CompletionQueue, WorkerInfo};
use crate::error::{Error, Result};
use crate::grpc_sys::{self, grpc_call_error};

type BoxFuture<T, E> = Box<dyn Future<Item = T, Error = E> + Send>;

/// A handle to a `Spawn`.
/// Inner future is expected to be polled in the same thread as cq.
type SpawnHandle = Option<Spawn<BoxFuture<(), ()>>>;

pub(crate) struct Kicker {
    call: Call,
}

impl Kicker {
    pub fn from_call(call: Call) -> Kicker {
        Kicker { call }
    }

    /// Kick its completion queue.
    pub fn kick(&self, tag: Box<CallTag>) -> Result<()> {
        let _ref = self.call.cq.borrow()?;
        unsafe {
            let ptr = Box::into_raw(tag);
            let status = grpc_sys::grpcwrap_call_kick_completion_queue(self.call.call, ptr as _);
            if status == grpc_call_error::GRPC_CALL_OK {
                Ok(())
            } else {
                Err(Error::CallFailure(status))
            }
        }
    }
}

unsafe impl Sync for Kicker {}

impl Clone for Kicker {
    fn clone(&self) -> Kicker {
        // Bump call's reference count.
        let call = unsafe {
            grpc_sys::grpc_call_ref(self.call.call);
            self.call.call
        };
        let cq = self.call.cq.clone();
        Kicker {
            call: Call { call, cq },
        }
    }
}

struct NotifyContext {
    kicked: bool,
    kicker: Kicker,
}

impl NotifyContext {
    /// Notify the completion queue.
    ///
    /// It only makes sense to call this function from the thread
    /// that cq is not run on.
    fn notify(&mut self, tag: Box<CallTag>) {
        match self.kicker.kick(tag) {
            // If the queue is shutdown, then the tag will be notified
            // eventually. So just skip here.
            Err(Error::QueueShutdown) => (),
            Err(e) => panic!("unexpected error when canceling call: {:?}", e),
            _ => (),
        }
    }
}

/// A custom notify.
///
/// It will poll the inner future directly if it's notified on the
/// same thread as inner cq.
#[derive(Clone)]
pub struct SpawnNotify {
    ctx: Arc<SpinLock<NotifyContext>>,
    handle: Arc<SpinLock<SpawnHandle>>,
    worker: Arc<WorkerInfo>,
}

impl SpawnNotify {
    fn new(s: Spawn<BoxFuture<(), ()>>, kicker: Kicker, worker: Arc<WorkerInfo>) -> SpawnNotify {
        SpawnNotify {
            worker,
            handle: Arc::new(SpinLock::new(Some(s))),
            ctx: Arc::new(SpinLock::new(NotifyContext {
                kicked: false,
                kicker,
            })),
        }
    }

    pub fn resolve(self, success: bool) {
        // it should always be canceled for now.
        assert!(success);
        poll(&Arc::new(self.clone()), true);
    }
}

impl Notify for SpawnNotify {
    fn notify(&self, _: usize) {
        match self.worker.begin_poll(thread::current().id()) {
            Some(_lease) => poll(&Arc::new(self.clone()), false),
            None => {
                // TODO: it's more friendly to cache if poll it immediately
                // if the worker thread id is still equal to current. However
                // we need a way to prevent deadlocks and stack overflows.
                let mut ctx = self.ctx.lock();
                if ctx.kicked {
                    return;
                }
                ctx.notify(Box::new(CallTag::Spawn(self.clone())));
                ctx.kicked = true;
            }
        }
    }
}

/// Poll the future.
///
/// `woken` indicates that if the cq is kicked by itself.
fn poll(notify: &Arc<SpawnNotify>, woken: bool) {
    let mut handle = notify.handle.lock();
    if woken {
        notify.ctx.lock().kicked = false;
    }
    if handle.is_none() {
        // it's resolved, no need to poll again.
        return;
    }
    match handle.as_mut().unwrap().poll_future_notify(notify, 0) {
        Err(_) | Ok(Async::Ready(_)) => {
            // Future stores notify, and notify contains future,
            // hence circular reference. Take the future to break it.
            handle.take();
        }
        _ => {}
    }
}

/// An executor that drives a future in the gRPC poll thread, which
/// can reduce thread context switching.
pub(crate) struct Executor<'a> {
    cq: &'a CompletionQueue,
}

impl<'a> Executor<'a> {
    pub fn new(cq: &CompletionQueue) -> Executor<'_> {
        Executor { cq }
    }

    pub fn cq(&self) -> &CompletionQueue {
        self.cq
    }

    /// Spawn the future into inner poll loop.
    ///
    /// If you want to trace the future, you may need to create a sender/receiver
    /// pair by yourself.
    pub fn spawn<F>(&self, f: F, kicker: Kicker)
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        let s = executor::spawn(Box::new(f) as BoxFuture<_, _>);
        let notify = Arc::new(SpawnNotify::new(s, kicker, self.cq.worker_info()));
        poll(&notify, false)
    }
}
