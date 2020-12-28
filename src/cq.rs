// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

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
        let mut cnt = self.ref_cnt.load(Ordering::SeqCst);
        loop {
            if cnt <= 0 {
                // `shutdown` has been called, reject any requests.
                return Err(Error::QueueShutdown);
            }
            let new_cnt = cnt + 1;
            match self.ref_cnt.compare_exchange_weak(
                cnt,
                new_cnt,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return Ok(()),
                Err(c) => cnt = c,
            }
        }
    }

    fn unref(&self) {
        let mut cnt = self.ref_cnt.load(Ordering::SeqCst);
        let shutdown = loop {
            // If `shutdown` is not called, `cnt` > 0, so minus 1 to unref.
            // If `shutdown` is called, `cnt` < 0, so plus 1 to unref.
            let new_cnt = cnt - cnt.signum();
            match self.ref_cnt.compare_exchange_weak(
                cnt,
                new_cnt,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break new_cnt == 0,
                Err(c) => cnt = c,
            }
        };
        if shutdown {
            unsafe {
                grpc_sys::grpc_completion_queue_shutdown(self.cq);
            }
        }
    }

    fn shutdown(&self) {
        let mut cnt = self.ref_cnt.load(Ordering::SeqCst);
        let shutdown = loop {
            if cnt <= 0 {
                // `shutdown` is called, skipped.
                return;
            }
            // Make cnt negative to indicate that `shutdown` has been called.
            // Because `cnt` is initialized to 1, so minus 1 to make it reach
            // toward 0. That is `new_cnt = -(cnt - 1) = -cnt + 1`.
            let new_cnt = -cnt + 1;
            match self.ref_cnt.compare_exchange_weak(
                cnt,
                new_cnt,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break new_cnt == 0,
                Err(c) => cnt = c,
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

/// `WorkQueue` stores the unfinished work of a completion queue.
///
/// Every completion queue has a work queue, and every work queue belongs
/// to exact one completion queue. `WorkQueue` is a short path for future
/// notifications. When a future is ready to be polled, there are two way
/// to notify it.
/// 1. If it's in the same thread where the future is spawned, the future
///    will be pushed into `WorkQueue` and be polled when current call tag
///    is handled;
/// 2. If not, the future will be wrapped as a call tag and pushed into
///    completion queue and finally popped at the call to `grpc_completion_queue_next`.
pub struct WorkQueue {
    id: ThreadId,
    pending_work: UnsafeCell<VecDeque<UnfinishedWork>>,
}

unsafe impl Sync for WorkQueue {}
unsafe impl Send for WorkQueue {}

const QUEUE_CAPACITY: usize = 4096;

impl WorkQueue {
    pub fn new() -> WorkQueue {
        WorkQueue {
            id: std::thread::current().id(),
            pending_work: UnsafeCell::new(VecDeque::with_capacity(QUEUE_CAPACITY)),
        }
    }

    /// Pushes an unfinished work into the inner queue.
    ///
    /// If the method is not called from the same thread where it's created,
    /// the work will returned and no work is pushed.
    pub fn push_work(&self, work: UnfinishedWork) -> Option<UnfinishedWork> {
        if self.id == thread::current().id() {
            unsafe { &mut *self.pending_work.get() }.push_back(work);
            None
        } else {
            Some(work)
        }
    }

    /// Pops one unfinished work.
    ///
    /// It should only be called from the same thread where the queue is created.
    /// Otherwise it leads to undefined behavior.
    pub unsafe fn pop_work(&self) -> Option<UnfinishedWork> {
        let queue = &mut *self.pending_work.get();
        if queue.capacity() > QUEUE_CAPACITY && queue.len() < queue.capacity() / 2 {
            queue.shrink_to_fit();
        }
        { &mut *self.pending_work.get() }.pop_back()
    }
}

#[derive(Clone)]
pub struct CompletionQueue {
    handle: Arc<CompletionQueueHandle>,
    pub(crate) worker: Arc<WorkQueue>,
}

impl CompletionQueue {
    pub fn new(handle: Arc<CompletionQueueHandle>, worker: Arc<WorkQueue>) -> CompletionQueue {
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
