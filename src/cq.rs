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
use std::collections::VecDeque;
use std::ptr;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;
use std::thread::{self, ThreadId};

use crate::error::{Error, Result};
use crate::grpc_sys::{self, gpr_clock_type, grpc_completion_queue};
use crate::task::UnfinishedWork;

pub use crate::grpc_sys::grpc_completion_type as EventType;
pub use crate::grpc_sys::grpc_event as Event;

/// `CompletionQueueHandle` enable notification of the completion of asynchronous actions.
pub struct CompletionQueueHandle {
    cq: *mut grpc_completion_queue,
    // When `ref_cnt` < 0, a shutdown is pending, completion queue should not
    // accept requests anymore; when `ref_cnt` == 0, completion queue should
    // be shutdown; When `ref_cnt` > 0, completion queue can accept requests
    // and should not be shutdown.
    ref_cnt: AtomicIsize,
}

unsafe impl Sync for CompletionQueueHandle {}
unsafe impl Send for CompletionQueueHandle {}

impl CompletionQueueHandle {
    pub fn new() -> CompletionQueueHandle {
        CompletionQueueHandle {
            cq: unsafe { grpc_sys::grpc_completion_queue_create_for_next(ptr::null_mut()) },
            ref_cnt: AtomicIsize::new(1),
        }
    }

    fn add_ref(&self) -> Result<()> {
        loop {
            let cnt = self.ref_cnt.load(Ordering::SeqCst);
            if cnt <= 0 {
                // `shutdown` has been called, reject any requests.
                return Err(Error::QueueShutdown);
            }
            let new_cnt = cnt + 1;
            if cnt
                == self
                    .ref_cnt
                    .compare_and_swap(cnt, new_cnt, Ordering::SeqCst)
            {
                return Ok(());
            }
        }
    }

    fn unref(&self) {
        let shutdown = loop {
            let cnt = self.ref_cnt.load(Ordering::SeqCst);
            // If `shutdown` is not called, `cnt` > 0, so minus 1 to unref.
            // If `shutdown` is called, `cnt` < 0, so plus 1 to unref.
            let new_cnt = cnt - cnt.signum();
            if cnt
                == self
                    .ref_cnt
                    .compare_and_swap(cnt, new_cnt, Ordering::SeqCst)
            {
                break new_cnt == 0;
            }
        };
        if shutdown {
            unsafe {
                grpc_sys::grpc_completion_queue_shutdown(self.cq);
            }
        }
    }

    fn shutdown(&self) {
        let shutdown = loop {
            let cnt = self.ref_cnt.load(Ordering::SeqCst);
            if cnt <= 0 {
                // `shutdown` is called, skipped.
                return;
            }
            // Make cnt negative to indicate that `shutdown` has been called.
            // Because `cnt` is initialised to 1, so minus 1 to make it reach
            // toward 0. That is `new_cnt = -(cnt - 1) = -cnt + 1`.
            let new_cnt = -cnt + 1;
            if cnt
                == self
                    .ref_cnt
                    .compare_and_swap(cnt, new_cnt, Ordering::SeqCst)
            {
                break new_cnt == 0;
            }
        };
        if shutdown {
            unsafe {
                grpc_sys::grpc_completion_queue_shutdown(self.cq);
            }
        }
    }
}

impl Drop for CompletionQueueHandle {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_completion_queue_destroy(self.cq) }
    }
}

pub struct CompletionQueueRef<'a> {
    queue: &'a CompletionQueue,
}

impl<'a> CompletionQueueRef<'a> {
    pub fn as_ptr(&self) -> *mut grpc_completion_queue {
        self.queue.handle.cq
    }
}

impl<'a> Drop for CompletionQueueRef<'a> {
    fn drop(&mut self) {
        self.queue.handle.unref();
    }
}

pub struct WorkerInfo {
    id: ThreadId,
    pending_work: UnsafeCell<VecDeque<UnfinishedWork>>,
}

unsafe impl Sync for WorkerInfo {}
unsafe impl Send for WorkerInfo {}

impl WorkerInfo {
    pub fn new() -> WorkerInfo {
        WorkerInfo {
            id: std::thread::current().id(),
            pending_work: UnsafeCell::new(VecDeque::new()),
        }
    }

    pub fn push_work(&self, work: UnfinishedWork) -> Option<UnfinishedWork> {
        if self.id == thread::current().id() {
            unsafe { &mut *self.pending_work.get() }.push_back(work);
            None
        } else {
            Some(work)
        }
    }

    pub fn pop_work(&self) -> Option<UnfinishedWork> {
        unsafe { &mut *self.pending_work.get() }.pop_back()
    }
}

#[derive(Clone)]
pub struct CompletionQueue {
    handle: Arc<CompletionQueueHandle>,
    pub(crate) worker: Arc<WorkerInfo>,
}

impl CompletionQueue {
    pub fn new(handle: Arc<CompletionQueueHandle>, worker: Arc<WorkerInfo>) -> CompletionQueue {
        CompletionQueue { handle, worker }
    }

    /// Blocks until an event is available, the completion queue is being shut down.
    pub fn next(&self) -> Event {
        unsafe {
            let inf = grpc_sys::gpr_inf_future(gpr_clock_type::GPR_CLOCK_REALTIME);
            grpc_sys::grpc_completion_queue_next(self.handle.cq, inf, ptr::null_mut())
        }
    }

    pub fn borrow(&self) -> Result<CompletionQueueRef<'_>> {
        self.handle.add_ref()?;
        Ok(CompletionQueueRef { queue: self })
    }

    /// Begin destruction of a completion queue.
    ///
    /// Once all possible events are drained then `next()` will start to produce
    /// `Event::QueueShutdown` events only.
    pub fn shutdown(&self) {
        self.handle.shutdown()
    }

    pub fn worker_id(&self) -> ThreadId {
        self.worker.id
    }
}
