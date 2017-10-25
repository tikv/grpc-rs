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
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{ThreadId, self};

use grpc_sys::{self, GprClockType, GrpcCompletionQueue};
use futures::Async;
use futures::future::BoxFuture;
use futures::executor::{Notify, Spawn};
use mio::util::BoundedQueue;

use async::{SpinLock, Alarm, CallTag};

pub use grpc_sys::GrpcCompletionType as EventType;
pub use grpc_sys::GrpcEvent as Event;

/// `CompletionQueueHandle` enable notification of the completion of asynchronous actions.
pub struct CompletionQueueHandle {
    cq: *mut GrpcCompletionQueue,
}

unsafe impl Sync for CompletionQueueHandle {}
unsafe impl Send for CompletionQueueHandle {}

impl CompletionQueueHandle {
    pub fn new() -> CompletionQueueHandle {
        CompletionQueueHandle {
            cq: unsafe { grpc_sys::grpc_completion_queue_create_for_next(ptr::null_mut()) },
        }
    }
}

impl Drop for CompletionQueueHandle {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_completion_queue_destroy(self.cq) }
    }
}

#[derive(Clone)]
pub struct CompletionQueue {
    handle: Arc<CompletionQueueHandle>,
    id: ThreadId,
    fq: Arc<ReadyQueue>,
}

impl CompletionQueue {
    pub fn new(handle: Arc<CompletionQueueHandle>, id: ThreadId) -> CompletionQueue {
       let fq = ReadyQueue {
            queue: BoundedQueue::with_capacity(BATCH_SIZE * 2),
            pending: AtomicUsize::new(0),
            worker_id: id,
        };
        CompletionQueue {
            handle: handle,
            id: id,
            fq: Arc::new(fq),
        }
    }

    /// Blocks until an event is available, the completion queue is being shut down.
    pub fn next(&self) -> Event {
        unsafe {
            let inf = grpc_sys::gpr_inf_future(GprClockType::Realtime);
            grpc_sys::grpc_completion_queue_next(self.handle.cq, inf, ptr::null_mut())
        }
    }

    /// Begin destruction of a completion queue.
    ///
    /// Once all possible events are drained then `next()` will start to produce
    /// `Event::QueueShutdown` events only.
    pub fn shutdown(&self) {
        unsafe {
            grpc_sys::grpc_completion_queue_shutdown(self.handle.cq);
        }
    }

    pub fn as_ptr(&self) -> *mut GrpcCompletionQueue {
        self.handle.cq
    }

    pub fn worker_id(&self) -> ThreadId {
        self.id
    }

    fn push_and_notify(&self, f: Item) {
        self.fq.push_and_notify(f, self.clone())
    }

    fn pop_and_poll(&self, notify: QueueNotify) {
        self.fq.pop_and_poll(notify, self.clone());
    }
}

const BATCH_SIZE: usize = 1024;
type Item = Spawn<BoxFuture<(), ()>>;

struct ReadyQueue {
    // TODO: use std::sync::mpsc::Receiver instead.
    queue: BoundedQueue<Item>,
    pending: AtomicUsize,
    worker_id: ThreadId,
}

unsafe impl Send for ReadyQueue {}
unsafe impl Sync for ReadyQueue {}

impl ReadyQueue {
    fn push_and_notify(&self, mut f: Item, cq: CompletionQueue) {
        let notify = QueueNotify::new(cq.clone());

        if thread::current().id()  == self.worker_id {
            let notify = Arc::new(notify);
            poll(f, &notify);
        } else {
            loop {
                if let Err(out) = self.queue.push(f) {
                    f = out;
                } else {
                    break;
                }
            }
            let pending = self.pending.fetch_add(1, Ordering::SeqCst);
            if 0 == pending {
                let alarm = notify.alarm.clone();
                let tag = Box::new(CallTag::Queue(notify));
                let mut al = alarm.lock();
                // We need to keep the alarm until queue is empty.
                *al = Some(Alarm::new(&cq, tag));
                al.as_mut().unwrap().alarm();
            }
        }
    }

    fn pop_and_poll(&self, mut notify: QueueNotify, cq: CompletionQueue) {
        // Drop alarm without locking.
        notify.alarm = Arc::new(SpinLock::new(None));
        let mut notify = Arc::new(notify);

        let mut batch = Vec::with_capacity(BATCH_SIZE);
        let mut pending;
        loop {
            pending = self.pending.load(Ordering::SeqCst);
            if pending == 0 {
                // TODO: There must be at least one ready future in the queue.
                // To make sure that at the first loop, pending should be zero.
                break;
            }

            // Batch full.
            if BATCH_SIZE == batch.len() {
                break;
            }

            match self.queue.pop() {
                Some(f) => {
                    assert_ne!(self.pending.fetch_sub(1, Ordering::SeqCst), 0);
                    batch.push(f);
                }
                None => {
                    if !batch.is_empty() {
                        // Do not wait for more ready futures.
                        break;
                    }
                }
            }
        }

        let mut done = true;
        for f in batch {
            notify = if done {
                // Future has resloved, and the notify is empty, reuse it.
                notify
            } else {
                // Future is not complete yet. Other thread holds the notify,
                // create a new one for the next ready Future.
                Arc::new(QueueNotify::new(cq.clone()))
            };

            done = poll(f, &notify);
        }

        if done {
            notify.alarm.lock().take();
        }

        // There are still pending ready futures, poll them later.
        if pending != 0 {
            let notify = QueueNotify::new(cq.clone());
            let alarm = notify.alarm.clone();
            let tag = Box::new(CallTag::Queue(notify));

            let mut al = alarm.lock();
            // We need to keep the alarm until it arrives in the CQ.
            *al = Some(Alarm::new(&cq, tag));
            al.as_mut().unwrap().alarm();
        }
    }
}

fn poll(f: Item, notify: &Arc<QueueNotify>) -> bool {
    let mut option = notify.f.lock();
    *option = Some(f);
    match option.as_mut().unwrap().poll_future_notify(notify, 0) {
        Err(_) |
        Ok(Async::Ready(_)) => {
            // Future has resloved, empty the future so that we can
            // reuse the notify.
            option.take();
            true
        }
        Ok(Async::NotReady) => {
            // Future is not complete yet.
            false
        }
    }
}

#[derive(Clone)]
pub struct QueueNotify {
    cq: CompletionQueue,
    f: Arc<SpinLock<Option<Item>>>,
    alarm: Arc<SpinLock<Option<Alarm>>>,
}

unsafe impl Send for QueueNotify {}
unsafe impl Sync for QueueNotify {}

impl QueueNotify {
    pub fn new(cq: CompletionQueue) -> QueueNotify {
        QueueNotify {
            cq: cq,
            f: Arc::new(SpinLock::new(None)),
            alarm: Arc::new(SpinLock::new(None)),
        }
    }

    pub fn resolve(self, success: bool) {
        // it should always be canceled for now.
        assert!(!success);
        self.cq.clone().pop_and_poll(self);
    }

    pub fn push_and_notify(&self, f: Item) {
        self.cq.push_and_notify(f);
    }
}

impl Notify for QueueNotify {
    fn notify(&self, _: usize) {
        if let Some(f) = self.f.lock().take() {
            self.cq.push_and_notify(f);
        }
    }
}
