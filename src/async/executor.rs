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

#![allow(deprecated)]

use std::sync::Arc;

use futures::executor::{self, Spawn, Unpark};
use futures::future::BoxFuture;
use futures::{Async, Future};

use super::lock::{LockGuard, SpinLock};

struct FutureHolder {
    f: Option<Spawn<BoxFuture<(), ()>>>,
}

impl FutureHolder {
    pub fn new(s: Spawn<BoxFuture<(), ()>>) -> FutureHolder {
        FutureHolder { f: Some(s) }
    }
}

/// A custom unpark that poll the future immediately instead of notify any poller.
pub struct EagerUnpark {
    handle: Arc<SpinLock<FutureHolder>>,
}

impl EagerUnpark {
    fn new(s: Spawn<BoxFuture<(), ()>>) -> EagerUnpark {
        EagerUnpark { handle: Arc::new(SpinLock::new(FutureHolder::new(s))) }
    }
}

// BoxFuture is Send, hence EagerUnpark is Send and Sync.
unsafe impl Send for EagerUnpark {}
unsafe impl Sync for EagerUnpark {}

impl Unpark for EagerUnpark {
    fn unpark(&self) {
        let mut handle = self.handle.lock();
        poll(&mut handle,
             Arc::new(EagerUnpark { handle: self.handle.clone() }));
    }
}

#[inline]
fn poll<'a>(handle: &mut LockGuard<'a, FutureHolder>, unpark: Arc<EagerUnpark>) {
    if handle.f.is_none() {
        // the future is resolved, skip.
        return;
    }

    match handle.f.as_mut().unwrap().poll_future(unpark) {
        Err(_) |
        Ok(Async::Ready(_)) => {
            // Future stores unpark, and unpark contains future,
            // hence circular reference. Take the future to break it.
            handle.f.take();
        }
        _ => {}
    }
}

/// Spawn the future into inner poll loop.
///
/// If you want to trace the future, you may need to create a sender/receiver
/// pair by yourself.
pub fn spawn<F>(f: F)
    where F: Future<Item = (), Error = ()> + Send + 'static
{
    let s = executor::spawn(f.boxed());
    let unpark = Arc::new(EagerUnpark::new(s));
    let mut handle = unpark.handle.lock();
    poll(&mut handle, unpark.clone())
}
