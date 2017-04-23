use std::thread::{Builder, JoinHandle};
use std::sync::Arc;

use grpc_sys;
use cq::{CompletionQueue, EventType};
use call::BatchContext;


fn poll_queue(cq: Arc<CompletionQueue>) {
    loop {
        let e = cq.next();
        match e.event_type {
            EventType::QueueShutdown => break,
            EventType::QueueTimeout => continue,
            EventType::OpComplete => {}
        }
        
        let mut ctx = BatchContext::from_raw(e.tag as *mut _);
        if let Some(promise) = ctx.take_promise() {
            promise.on_ready(ctx, e.success != 0);
        }
    }
}

pub struct Environment {
    cq: Arc<CompletionQueue>,
    _handle: JoinHandle<()>,
}

impl Environment {
    pub fn new() -> Environment {
        unsafe {
            grpc_sys::grpc_init();
        }
        let cq = Arc::new(CompletionQueue::new());
        let cq2 = cq.clone();
        // TODO: support thread pool
        let handle = Builder::new().name("grpc poll thread".to_owned()).spawn(move || poll_queue(cq2)).unwrap();
        
        Environment {
            cq: cq,
            _handle: handle,
        }
    }

    pub fn completion_queue(&self) -> &CompletionQueue {
        self.cq.as_ref()
    }
}
