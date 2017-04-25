use std::ptr;

use grpc_sys::{self, GrpcCompletionQueue, GprClockType};

pub use grpc_sys::GrpcEvent as Event;
pub use grpc_sys::GrpcCompletionType as EventType;

pub struct CompletionQueue {
    cq: *mut GrpcCompletionQueue,
}

unsafe impl Sync for CompletionQueue {}
unsafe impl Send for CompletionQueue {}

impl CompletionQueue {
    pub fn new() -> CompletionQueue {
        CompletionQueue {
            cq: unsafe {
                grpc_sys::grpc_completion_queue_create(ptr::null_mut())
            },
        }
    }

    pub fn next(&self) -> Event {
        unsafe {
            let inf = grpc_sys::gpr_inf_future(GprClockType::Realtime);
            grpc_sys::grpc_completion_queue_next(self.cq, inf, ptr::null_mut())
        }
    }

    pub fn shutdown(&self) {
        unsafe {
            grpc_sys::grpc_completion_queue_shutdown(self.cq);
        }
    }

    pub fn as_ptr(&self) -> *mut GrpcCompletionQueue {
        self.cq
    }
}

impl Drop for CompletionQueue {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_completion_queue_destroy(self.cq);
        }
    }
}
