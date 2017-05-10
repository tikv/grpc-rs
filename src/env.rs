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
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{Builder as ThreadBuilder, JoinHandle};

use grpc_sys;

use async::CallTag;
use cq::{CompletionQueue, EventType};

// event loop
fn poll_queue(cq: Arc<CompletionQueue>) {
    loop {
        let e = cq.next();
        match e.event_type {
            EventType::QueueShutdown => break,
            // timeout should not happen in theory.
            EventType::QueueTimeout => continue,
            EventType::OpComplete => {}
        }

        let tag: Box<CallTag> = unsafe { Box::from_raw(e.tag as _) };

        tag.resolve(&cq, e.success != 0);
    }
}

pub struct EnvBuilder {
    cq_count: usize,
    name_prefix: Option<String>,
}

impl EnvBuilder {
    pub fn new() -> EnvBuilder {
        EnvBuilder {
            cq_count: unsafe { grpc_sys::gpr_cpu_num_cores() },
            name_prefix: None,
        }
    }

    pub fn cq_count(mut self, count: usize) -> EnvBuilder {
        assert!(count > 0);
        self.cq_count = count;
        self
    }

    pub fn name_prefix<S: Into<String>>(mut self, prefix: S) -> EnvBuilder {
        self.name_prefix = Some(prefix.into());
        self
    }

    pub fn build(self) -> Environment {
        unsafe {
            grpc_sys::grpc_init();
        }
        let mut cqs = Vec::with_capacity(self.cq_count);
        let mut handles = Vec::with_capacity(self.cq_count);
        for i in 0..self.cq_count {
            let cq = Arc::new(CompletionQueue::new());
            let cq_ = cq.clone();
            let mut builder = ThreadBuilder::new();
            if let Some(ref prefix) = self.name_prefix {
                builder = builder.name(format!("{}-{}", prefix, i));
            }
            let handle = builder.spawn(move || poll_queue(cq_)).unwrap();
            cqs.push(cq);
            handles.push(handle);
        }

        Environment {
            cqs: cqs,
            idx: AtomicUsize::new(0),
            _handles: handles,
        }
    }
}

/// An object that used to control concurrency and start event loop.
pub struct Environment {
    cqs: Vec<Arc<CompletionQueue>>,
    idx: AtomicUsize,
    _handles: Vec<JoinHandle<()>>,
}

impl Environment {
    /// Initialize grpc and create a threadpool to poll event loop.
    ///
    /// Each thread in threadpool will have one event loop.
    pub fn new(cq_count: usize) -> Environment {
        assert!(cq_count > 0);
        EnvBuilder::new().name_prefix("grpc-poll").cq_count(cq_count).build()
    }

    /// Get all the created completion queues.
    pub fn completion_queues(&self) -> &[Arc<CompletionQueue>] {
        self.cqs.as_slice()
    }

    /// Pick an arbitrary completion queue.
    pub fn pick_cq(&self) -> Arc<CompletionQueue> {
        let idx = self.idx.fetch_add(1, Ordering::Relaxed);
        self.cqs[idx % self.cqs.len()].clone()
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        for cq in self.completion_queues() {
            // it's safe to shutdown more than once.
            cq.shutdown()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_loop() {
        let mut env = Environment::new(2);

        let q1_ptr = env.pick_cq();
        let q2_ptr = env.pick_cq();
        let q3_ptr = env.pick_cq();
        assert_eq!(q1_ptr.as_ptr(), q3_ptr.as_ptr());
        assert_ne!(q1_ptr.as_ptr(), q2_ptr.as_ptr());

        assert_eq!(env.completion_queues().len(), 2);
        for cq in env.completion_queues() {
            cq.shutdown();
        }

        for handle in env._handles.drain(..) {
            handle.join().unwrap();
        }
    }
}
