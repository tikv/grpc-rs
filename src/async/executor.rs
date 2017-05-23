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

use futures::executor::{self, Unpark, Spawn};
use futures::future::BoxFuture;
use futures::{Future, Async};
use grpc_sys::{self, GrpcAlarm, GprTimespec};

use cq::CompletionQueue;
use super::lock::SpinLock;
use super::CallTag;

pub struct AlarmHandle {
    f: Spawn<BoxFuture<(), ()>>,
    finished: bool,
    alarm: *mut GrpcAlarm,
}

impl AlarmHandle {
    pub fn new(s: Spawn<BoxFuture<(), ()>>) -> AlarmHandle {
        AlarmHandle {
            f: s,
            finished: false,
            alarm: ptr::null_mut(),
        }
    }

    pub fn cancel(&mut self) {
        self.finished = true;
        if self.alarm.is_null() {
            return;
        }
        unsafe {
            grpc_sys::grpc_alarm_cancel(self.alarm)
        }
    }
}

impl Drop for AlarmHandle {
    fn drop(&mut self) {
        if self.alarm.is_null() {
            return;
        }
        unsafe {
            grpc_sys::grpc_alarm_destroy(self.alarm)
        }
    }
}

pub struct AlarmUnpark {
    handle: SpinLock<AlarmHandle>,
}

impl AlarmUnpark {
    fn new(s: Spawn<BoxFuture<(), ()>>) -> AlarmUnpark {
        AlarmUnpark {
            handle: SpinLock::new(AlarmHandle::new(s)),
        }
    }
}

unsafe impl Send for AlarmUnpark {}
unsafe impl Sync for AlarmUnpark {}

impl Unpark for AlarmUnpark {
    fn unpark(&self) {
        let mut handle = self.handle.lock();
        handle.cancel()
    }
}

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

// TODO: support timeout and trace future.
fn spawn(cq: &CompletionQueue, unpark: Arc<AlarmUnpark>) {
    let mut handle = unpark.handle.lock();
    match handle.f.poll_future(unpark.clone()) {
        Err(_) | Ok(Async::Ready(_)) => return,
        _ => {}
    }
    
    let tag = Box::new(CallTag::Alarm(Alarm { unpark: unpark.clone() }));
    handle.alarm = unsafe {
        grpc_sys::grpc_alarm_create(cq.as_ptr(), GprTimespec::inf_future(), Box::into_raw(tag) as _)
    };
}

pub struct Executor<'a> {
    cq: &'a CompletionQueue,
}

impl<'a> Executor<'a> {
    pub fn new(cq: &CompletionQueue) -> Executor {
        Executor {
            cq: cq,
        }
    }

    pub fn spawn<F>(&self, f: F)
        where F: Future<Item=(), Error=()> + Send + 'static
    {
        let s = executor::spawn(f.boxed());
        let unpark = Arc::new(AlarmUnpark::new(s));
        spawn(self.cq, unpark)
    }
}
