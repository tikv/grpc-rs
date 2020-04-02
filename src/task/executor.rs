// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

//! gRPC C Core binds a call to a completion queue, all the related readiness
//! will be forwarded to the completion queue. This module utilizes the mechanism
//! and using `Kicker` to wake up completion queue.
//!
//! Apparently, to minimize context switch, it's better to bind the future to the
//! same completion queue as its inner call. Hence method `Executor::spawn` is provided.

use std::cell::UnsafeCell;
use std::pin::Pin;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use futures::future::Future;
use futures::task::{waker_ref, ArcWake, Context, Poll};

use super::CallTag;
use crate::call::Call;
use crate::cq::{CompletionQueue, WorkQueue};
use crate::error::{Error, Result};
use crate::grpc_sys::{self, grpc_call_error};

/// A handle to a `Spawn`.
/// Inner future is expected to be polled in the same thread as cq.
type SpawnHandle = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// `Kicker` wakes up the completion queue that the inner call binds to.
pub(crate) struct Kicker {
    call: Call,
}

impl Kicker {
    pub fn from_call(call: Call) -> Kicker {
        Kicker { call }
    }

    /// Wakes up its completion queue.
    ///
    /// `tag` will be popped by `grpc_completion_queue_next` in the future.
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

/// When a future is scheduled, it becomes IDLE. When it's ready to be polled,
/// it will be notified via task.wake(), and marked as NOTIFIED. When executor
/// begins to poll the future, it's marked as POLLING. When the executor finishes
/// polling, the future can either be ready or not ready. In the former case, it's
/// marked as COMPLETED. If it's latter, it's marked as IDLE again.
///
/// Note it's possible the future is notified during polling, in which case, executor
/// should polling it when last polling is finished unless it returns ready.
const NOTIFIED: u8 = 1;
const IDLE: u8 = 2;
const POLLING: u8 = 3;
const COMPLETED: u8 = 4;

/// Maintains the spawned future with state, so that it can be notified and polled efficiently.
pub struct SpawnTask {
    handle: UnsafeCell<Option<SpawnHandle>>,
    state: AtomicU8,
    kicker: Kicker,
    queue: Arc<WorkQueue>,
}

/// `SpawnTask` access is guarded by `state` field, which guarantees Sync.
///
/// Sync is required by `ArcWake`.
unsafe impl Sync for SpawnTask {}

impl SpawnTask {
    fn new(s: SpawnHandle, kicker: Kicker, queue: Arc<WorkQueue>) -> SpawnTask {
        SpawnTask {
            handle: UnsafeCell::new(Some(s)),
            state: AtomicU8::new(IDLE),
            kicker,
            queue,
        }
    }

    /// Marks the state of this task to NOTIFIED.
    ///
    /// Returns true means the task was IDLE, needs to be scheduled.
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
                    // If it succeeds, then executor will poll the future again;
                    // if it fails, then the future should be resolved. In both
                    // cases, no need to notify the future, hence return false.
                    _ => return false,
                },
                Err(IDLE) => continue,
                _ => return false,
            }
        }
    }
}

pub fn resolve(task: Arc<SpawnTask>, success: bool) {
    // it should always be canceled for now.
    assert!(success);
    poll(task, true);
}

/// A custom Waker.
///
/// It will push the inner future to work_queue if it's notified on the
/// same thread as inner cq.
impl ArcWake for SpawnTask {
    fn wake_by_ref(task: &Arc<Self>) {
        if !task.mark_notified() {
            return;
        }

        // It can lead to deadlock if poll the future immediately. So we need to
        // defer the work instead.
        if let Some(UnfinishedWork(w)) = task.queue.push_work(UnfinishedWork(task.clone())) {
            match task.kicker.kick(Box::new(CallTag::Spawn(w))) {
                // If the queue is shutdown, then the tag will be notified
                // eventually. So just skip here.
                Err(Error::QueueShutdown) => (),
                Err(e) => panic!("unexpected error when canceling call: {:?}", e),
                _ => (),
            }
        }
    }
}

/// Work that should be deferred to be handled.
///
/// Sometimes a work can't be done immediately as it might lead
/// to resource conflict, deadlock for example. So they will be
/// pushed into a queue and handled when current work is done.
pub struct UnfinishedWork(Arc<SpawnTask>);

impl UnfinishedWork {
    pub fn finish(self) {
        resolve(self.0, true);
    }
}

/// Poll the future.
///
/// `woken` indicates that if the cq is waken up by itself.
fn poll(task: Arc<SpawnTask>, woken: bool) {
    let mut init_state = if woken { NOTIFIED } else { IDLE };
    // TODO: maybe we need to break the loop to avoid hunger.
    loop {
        match task
            .state
            .compare_exchange(init_state, POLLING, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => {}
            Err(COMPLETED) => return,
            Err(s) => panic!("unexpected state {}", s),
        }

        let waker = waker_ref(&task);
        let mut cx = Context::from_waker(&waker);

        // L208 "lock"s state, hence it's safe to get a mutable reference.
        match unsafe { &mut *task.handle.get() }
            .as_mut()
            .unwrap()
            .as_mut()
            .poll(&mut cx)
        {
            Poll::Ready(()) => {
                task.state.store(COMPLETED, Ordering::Release);
                unsafe { &mut *task.handle.get() }.take();
            }
            _ => {
                match task.state.compare_exchange(
                    POLLING,
                    IDLE,
                    Ordering::AcqRel,
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
        F: Future<Output = ()> + Send + 'static,
    {
        let s = Box::pin(f);
        let notify = Arc::new(SpawnTask::new(s, kicker, self.cq.worker.clone()));
        poll(notify, false)
    }
}
