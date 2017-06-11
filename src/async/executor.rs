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

use futures::executor::{self, Notify, Spawn};
use futures::future::BoxFuture;
use futures::{Async, Future};
use grpc_sys::{self, GprTimespec};

use cq::CompletionQueue;
use super::lock::SpinLock;
use super::CallTag;

/// A handle to an alarm.
///
/// Alarm acts as a notification hook that wakes up poll thread once
/// inner future is ready to make progress.
pub struct AlarmHandle {
    f: Option<Spawn<BoxFuture<(), ()>>>,
    cq: CompletionQueue,
    alarmed: bool,
}

impl AlarmHandle {
    /// Create an alarm for the future.
    ///
    /// `alarm` will be initialized lazily.
    pub fn new(s: Spawn<BoxFuture<(), ()>>, cq: CompletionQueue) -> AlarmHandle {
        AlarmHandle {
            f: Some(s),
            cq: cq,
            alarmed: false,
        }
    }

    /// Notify the alarm.
    pub fn alarm(&mut self, alarm: Box<CallTag>) {
        unsafe {
            let ptr = Box::into_raw(alarm);
            // hack: because grpc's alarm feels more like a timer,
            // but what we need here is a notification hook. Hence
            // use cancel to implement the alarm behaviour.
            let alarm =
                grpc_sys::grpc_alarm_create(self.cq.as_ptr(), GprTimespec::inf_future(), ptr as _);
            grpc_sys::grpc_alarm_destroy(alarm);
        }
    }
}

/// A custom notify implemented with Alarm.
#[derive(Clone)]
pub struct Alarm {
    handle: Arc<SpinLock<AlarmHandle>>,
    worker_id: ThreadId,
}

impl Alarm {
    fn new(s: Spawn<BoxFuture<(), ()>>, cq: CompletionQueue) -> Alarm {
        Alarm {
            worker_id: cq.worker_id(),
            handle: Arc::new(SpinLock::new(AlarmHandle::new(s, cq))),
        }
    }

    pub fn resolve(self, success: bool) {
        // it should always be canceled for now.
        assert!(!success);
        spawn(Arc::new(self.clone()));
    }
}

unsafe impl Send for Alarm {}
unsafe impl Sync for Alarm {}

impl Notify for Alarm {
    fn notify(&self, _: usize) {
        if thread::current().id() == self.worker_id {
            spawn(Arc::new(self.clone()))
        } else {
            let mut handle = self.handle.lock();
            if handle.alarmed {
                return;
            }
            handle.alarm(Box::new(CallTag::Alarm(self.clone())));
            handle.alarmed = true;
        }
    }
}

// TODO: support timeout and trace future.
fn spawn(notify: Arc<Alarm>) {
    let mut handle = notify.handle.lock();
    handle.alarmed = false;
    match handle.f.as_mut().unwrap().poll_future_notify(&notify, 0) {
        Err(_) |
        Ok(Async::Ready(_)) => {
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

    /// Spawn the future into inner poll loop.
    ///
    /// If you want to trace the future, you may need to create a sender/receiver
    /// pair by yourself.
    pub fn spawn<F>(&self, f: F)
        where F: Future<Item = (), Error = ()> + Send + 'static
    {
        let s = executor::spawn(f.boxed());
        let notify = Arc::new(Alarm::new(s, self.cq.clone()));
        spawn(notify)
    }
}
