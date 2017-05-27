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


use std::ptr;
use std::sync::Arc;
use std::thread::{self, ThreadId};

use futures::executor::{self, Spawn, Unpark};
use futures::future::BoxFuture;
use futures::{Async, Future};
use grpc_sys::{self, GprTimespec, GrpcAlarm};

use cq::CompletionQueue;
use super::lock::{LockGuard, SpinLock};
use super::CallTag;

/// A handle to an alarm.
///
/// Alarm acts as a notification hook that wakes up poll thread once
/// inner future is ready to make progress.
pub struct AlarmHandle {
    f: Option<Spawn<BoxFuture<(), ()>>>,
    worker_id: ThreadId,
    finished: bool,
    alarm: *mut GrpcAlarm,
}

impl AlarmHandle {
    /// Create an alarm for the future.
    ///
    /// `alarm` will be initialized lazily.
    pub fn new(s: Spawn<BoxFuture<(), ()>>, worker_id: ThreadId) -> AlarmHandle {
        AlarmHandle {
            f: Some(s),
            worker_id: worker_id,
            finished: false,
            alarm: ptr::null_mut(),
        }
    }

    /// Notify the alarm.
    pub fn alarm(&mut self) {
        self.finished = true;
        if self.alarm.is_null() {
            // So the handle is notified but not polled again (yet).
            return;
        }
        unsafe {
            // hack: because grpc's alarm feels more like a timer,
            // but what we need here is a notification hook. Hence
            // use cancel to implement the alarm behaviour.
            grpc_sys::grpc_alarm_cancel(self.alarm);
        }
    }
}

impl Drop for AlarmHandle {
    fn drop(&mut self) {
        if self.alarm.is_null() {
            return;
        }
        unsafe { grpc_sys::grpc_alarm_destroy(self.alarm) }
    }
}

/// A custom unpark implemented with Alarm.
pub struct AlarmUnpark {
    handle: Arc<SpinLock<AlarmHandle>>,
}

impl AlarmUnpark {
    fn new(s: Spawn<BoxFuture<(), ()>>, worker_id: ThreadId) -> AlarmUnpark {
        AlarmUnpark { handle: Arc::new(SpinLock::new(AlarmHandle::new(s, worker_id))) }
    }
}

unsafe impl Send for AlarmUnpark {}
unsafe impl Sync for AlarmUnpark {}

impl Unpark for AlarmUnpark {
    fn unpark(&self) {
        let mut handle = self.handle.lock();
        if handle.worker_id == thread::current().id() {
            // Holder thread is already woken up, so we can reuse the alarm in this case.
            poll(&mut handle,
                 Arc::new(AlarmUnpark { handle: self.handle.clone() }));
        } else {
            handle.alarm()
        }
    }
}

/// A call tag for custom asynchronious notification.
pub struct Alarm {
    unpark: Arc<AlarmUnpark>,
}

impl Alarm {
    pub fn resolve(self, cq: &CompletionQueue, success: bool) {
        // it should always be canceled for now.
        assert!(!success);
        spawn(cq, self.unpark);
    }
}

#[inline]
fn poll<'a>(handle: &mut LockGuard<'a, AlarmHandle>, unpark: Arc<AlarmUnpark>) -> bool {
    if handle.f.is_none() {
        // the future is resolved, skip.
        return true;
    }

    match handle.f.as_mut().unwrap().poll_future(unpark.clone()) {
        Err(_) |
        Ok(Async::Ready(_)) => {
            // Future stores unpark, and unpark contains future,
            // hence circular reference. Take the future to break it.
            handle.f.take();
            true
        }
        _ => false,
    }
}

// TODO: support timeout and trace future.
fn spawn(cq: &CompletionQueue, unpark: Arc<AlarmUnpark>) {
    let mut handle = unpark.handle.lock();

    if poll(&mut handle, unpark.clone()) {
        return;
    }

    // handle.f is not resolved yet, need to register another alarm for notification.
    let tag = Box::new(CallTag::Alarm(Alarm { unpark: unpark.clone() }));

    if !handle.alarm.is_null() {
        unsafe {
            grpc_sys::grpc_alarm_destroy(handle.alarm);
        }
    }
    handle.alarm = unsafe {
        grpc_sys::grpc_alarm_create(cq.as_ptr(),
                                    GprTimespec::inf_future(),
                                    Box::into_raw(tag) as _)
    };
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

    /// Spawn the future into inner poll loop.
    ///
    /// If you want to trace the future, you may need to create a sender/receiver
    /// pair by yourself.
    pub fn spawn<F>(&self, f: F)
        where F: Future<Item = (), Error = ()> + Send + 'static
    {
        let s = executor::spawn(f.boxed());
        let unpark = Arc::new(AlarmUnpark::new(s, self.cq.worker_id()));
        spawn(self.cq, unpark)
    }
}
