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

use std::cell::UnsafeCell;
use std::mem;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use futures::executor::{self, Notify, Spawn};
use futures::{Async, Future};

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

const NOTIFIED: u8 = 1;
const IDLE: u8 = 2;
const POLLING: u8 = 3;
const COMPLETED: u8 = 4;

/// A custom notify.
///
/// It will poll the inner future directly if it's notified on the
/// same thread as inner cq.
pub struct SpawnTask {
    handle: UnsafeCell<SpawnHandle>,
    state: AtomicU8,
    kicker: Kicker,
}

impl SpawnTask {
    fn new(s: Spawn<BoxFuture<(), ()>>, kicker: Kicker) -> SpawnTask {
        SpawnTask {
            handle: UnsafeCell::new(Some(s)),
            state: AtomicU8::new(IDLE),
            kicker,
        }
    }

    /// Notify the completion queue.
    ///
    /// It only makes sense to call this function from the thread
    /// that cq is not run on.
    fn mark_notified(&self) -> bool {
        loop {
            match self.state.compare_exchange_weak(
                IDLE,
                NOTIFIED,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(POLLING) => match self.state.compare_exchange_weak(
                    POLLING,
                    NOTIFIED,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Err(IDLE) | Err(POLLING) => continue,
                    _ => return false,
                },
                Err(IDLE) => continue,
                _ => return false,
            }
        }
    }
}

pub fn resolve(cq: &CompletionQueue, task: Arc<SpawnTask>, success: bool) {
    // it should always be canceled for now.
    assert!(success);
    poll(cq, task, true);
}

impl Notify for WorkerInfo {
    fn notify(&self, id: usize) {
        let task = unsafe { Arc::from_raw(id as *mut SpawnTask) };
        if !task.mark_notified() {
            mem::forget(task);
            return;
        }

        if let Some(UnfinishedWork(w)) = self.push_work(UnfinishedWork(task.clone())) {
            match task.kicker.kick(Box::new(CallTag::Spawn(w))) {
                // If the queue is shutdown, then the tag will be notified
                // eventually. So just skip here.
                Err(Error::QueueShutdown) => (),
                Err(e) => panic!("unexpected error when canceling call: {:?}", e),
                _ => (),
            }
        }
        mem::forget(task);
    }

    fn clone_id(&self, id: usize) -> usize {
        let task = unsafe { Arc::from_raw(id as *mut SpawnTask) };
        let t = task.clone();
        mem::forget(task);
        Arc::into_raw(t) as usize
    }

    fn drop_id(&self, id: usize) {
        unsafe { Arc::from_raw(id as *mut SpawnTask) };
    }
}

pub struct UnfinishedWork(Arc<SpawnTask>);

impl UnfinishedWork {
    pub fn finish(self, cq: &CompletionQueue) {
        resolve(cq, self.0, true);
    }
}

/// Poll the future.
///
/// `woken` indicates that if the cq is kicked by itself.
fn poll(cq: &CompletionQueue, task: Arc<SpawnTask>, woken: bool) {
    let mut init_state = if woken { NOTIFIED } else { IDLE };
    // TODO: maybe we need to break the loop to avoid hunger.
    loop {
        match task
            .state
            .compare_exchange(init_state, POLLING, Ordering::SeqCst, Ordering::Acquire)
        {
            Ok(_) => {}
            Err(COMPLETED) => return,
            Err(s) => panic!("unexpected state {}", s),
        }

        let id = &*task as *const SpawnTask as usize;

        match unsafe { &mut *task.handle.get() }
            .as_mut()
            .unwrap()
            .poll_future_notify(&cq.worker, id)
        {
            Err(_) | Ok(Async::Ready(_)) => {
                task.state.store(COMPLETED, Ordering::SeqCst);
                unsafe { &mut *task.handle.get() }.take();
            }
            _ => {
                match task.state.compare_exchange(
                    POLLING,
                    IDLE,
                    Ordering::SeqCst,
                    Ordering::Acquire,
                ) {
                    Ok(_) => return,
                    Err(NOTIFIED) => {
                        init_state = NOTIFIED;
                    }
                    Err(s) => panic!("unexpected state {}", s),
                }
            }
        }
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
        let notify = Arc::new(SpawnTask::new(s, kicker));
        poll(self.cq, notify, false)
    }
}
