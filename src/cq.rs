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
use std::thread::ThreadId;

use grpc_sys::{self, GprClockType, GrpcCompletionQueue};

pub use grpc_sys::GrpcCompletionType as EventType;
pub use grpc_sys::GrpcEvent as Event;

/// `CompletionQueue` enable notification of the completion of asynchronous actions.
pub struct CompletionQueueHandle {
    cq: *mut GrpcCompletionQueue,
}

unsafe impl Sync for CompletionQueueHandle {}
unsafe impl Send for CompletionQueueHandle {}

impl CompletionQueueHandle {
    pub fn new() -> CompletionQueueHandle {
        CompletionQueueHandle {
            cq: unsafe { grpc_sys::grpc_completion_queue_create(ptr::null_mut()) },
        }
    }
}

impl Drop for CompletionQueueHandle {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_completion_queue_destroy(self.cq);
        }
    }
}

#[derive(Clone)]
pub struct CompletionQueue {
    handle: Arc<CompletionQueueHandle>,
    id: ThreadId,
}

impl CompletionQueue {
    pub fn new(handle: Arc<CompletionQueueHandle>, id: ThreadId) -> CompletionQueue {
        CompletionQueue {
            handle: handle,
            id: id,
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
}
