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
use std::thread::{self, ThreadId};
use std::ptr;

use futures::executor::{self, Notify, Spawn};
use futures::{Async, Future};
use grpc_sys::{self, GprTimespec, GrpcAlarm};

use cq::CompletionQueue;
use error::{Error, Result};
use super::lock::SpinLock;
use super::CallTag;

type BoxFuture<T, E> = Box<Future<Item = T, Error = E> + Send>;

struct Alarm {
    alarm: *mut GrpcAlarm,
}

impl Alarm {
    fn new(cq: &CompletionQueue, tag: Box<CallTag>) -> Result<Alarm> {
        let alarm = unsafe {
            let ptr = Box::into_raw(tag);
            let timeout = GprTimespec::inf_future();
            let cq_ref = cq.borrow()?;
            let alarm = grpc_sys::grpc_alarm_create(ptr::null_mut());
            grpc_sys::grpc_alarm_set(alarm, cq_ref.as_ptr(), timeout, ptr as _, ptr::null_mut());
            alarm
        };
        Ok(Alarm { alarm: alarm })
    }

    fn alarm(&mut self) {
        // hack: because grpc's alarm feels more like a timer,
        // but what we need here is a notification hook. Hence
        // use cancel to implement the alarm behaviour.
        unsafe { grpc_sys::grpc_alarm_cancel(self.alarm) }
    }
}

impl Drop for Alarm {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_alarm_destroy(self.alarm) }
    }
}

/// A handle to a `Spawn`.
pub struct SpawnHandle {
    f: Option<Spawn<BoxFuture<(), ()>>>,
    cq: CompletionQueue,
    alarm: Option<Alarm>,
    alarmed: bool,
}

impl SpawnHandle {
    /// Create a SpawnHandle.
    ///
    /// Inner future is expected to be polled in the same thread as cq.
    pub fn new(s: Spawn<BoxFuture<(), ()>>, cq: CompletionQueue) -> SpawnHandle {
        SpawnHandle {
            f: Some(s),
            cq: cq,
            alarm: None,
            alarmed: false,
        }
    }

    /// Notify the alarm.
    ///
    /// It only makes sence to call this function from the thread
    /// that cq is not run on.
    pub fn notify(&mut self, tag: Box<CallTag>) {
        self.alarm.take();
        let mut alarm = match Alarm::new(&self.cq, tag) {
            Ok(a) => a,
            Err(Error::QueueShutdown) => {
                // If the queue is shutdown, then the tag will be notified
                // eventually. So just skip here.
                return;
            }
            Err(e) => panic!("failed to create alarm: {:?}", e),
        };
        alarm.alarm();
        // We need to keep the alarm until tag is resolved.
        self.alarm = Some(alarm);
    }
}

/// A custom notify.
///
/// It will poll the inner future directly if it's notified on the
/// same thread as inner cq.
#[derive(Clone)]
pub struct SpawnNotify {
    handle: Arc<SpinLock<SpawnHandle>>,
    worker_id: ThreadId,
}

impl SpawnNotify {
    fn new(s: Spawn<BoxFuture<(), ()>>, cq: CompletionQueue) -> SpawnNotify {
        SpawnNotify {
            worker_id: cq.worker_id(),
            handle: Arc::new(SpinLock::new(SpawnHandle::new(s, cq))),
        }
    }

    pub fn resolve(self, success: bool) {
        // it should always be canceled for now.
        assert!(!success);
        poll(Arc::new(self.clone()), true);
    }
}

unsafe impl Send for SpawnNotify {}
unsafe impl Sync for SpawnNotify {}

impl Notify for SpawnNotify {
    fn notify(&self, _: usize) {
        if thread::current().id() == self.worker_id {
            poll(Arc::new(self.clone()), false)
        } else {
            let mut handle = self.handle.lock();
            if handle.alarmed {
                return;
            }
            handle.notify(Box::new(CallTag::Spawn(self.clone())));
            handle.alarmed = true;
        }
    }
}

/// Poll the future.
///
/// `woken` indicates that if the alarm is woken by a cancel action.
fn poll(notify: Arc<SpawnNotify>, woken: bool) {
    let mut handle = notify.handle.lock();
    if woken {
        handle.alarmed = false;
    }
    if handle.f.is_none() {
        // it's resolved, no need to poll again.
        return;
    }
    match handle.f.as_mut().unwrap().poll_future_notify(&notify, 0) {
        Err(_) | Ok(Async::Ready(_)) => {
            // Future stores notify, and notify contains future,
            // hence circular reference. Take the future to break it.
            handle.f.take();
            return;
        }
        _ => {}
    }
}

/// An executor that drives a future in the grpc poll thread, which
/// can reduce thread context switching.
pub struct Executor<'a> {
    cq: &'a CompletionQueue,
}

impl<'a> Executor<'a> {
    pub fn new(cq: &CompletionQueue) -> Executor {
        Executor { cq: cq }
    }

    pub(crate) fn cq(&self) -> &CompletionQueue {
        self.cq
    }

    /// Spawn the future into inner poll loop.
    ///
    /// If you want to trace the future, you may need to create a sender/receiver
    /// pair by yourself.
    pub fn spawn<F>(&self, f: F)
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        let s = executor::spawn(Box::new(f) as BoxFuture<_, _>);
        let notify = Arc::new(SpawnNotify::new(s, self.cq.clone()));
        poll(notify, false)
    }
}
