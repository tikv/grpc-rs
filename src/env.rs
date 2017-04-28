
use cq::{CompletionQueue, EventType};

use grpc_sys;
use async::Promise;
use std::sync::Arc;
use std::thread::{Builder, JoinHandle};

fn poll_queue(cq: Arc<CompletionQueue>) {
    loop {
        let e = cq.next();
        match e.event_type {
            EventType::QueueShutdown => break,
            EventType::QueueTimeout => continue,
            EventType::OpComplete => {}
        }

        let ctx: Box<Promise> = unsafe { Box::from_raw(e.tag as _) };

        ctx.resolve(&cq, e.success != 0);
    }
}

pub struct Environment {
    cqs: Vec<Arc<CompletionQueue>>,
    _handles: Vec<JoinHandle<()>>,
}

impl Environment {
    pub fn new(cq_count: usize) -> Environment {
        assert!(cq_count > 0);
        unsafe {
            grpc_sys::grpc_init();
        }
        let mut cqs = Vec::with_capacity(cq_count);
        let mut handles = Vec::with_capacity(cq_count);
        for i in 0..cq_count {
            let cq = Arc::new(CompletionQueue::new());
            let cq_ = cq.clone();
            let handle = Builder::new()
                .name(format!("grpcpollthread-{}", i))
                .spawn(move || poll_queue(cq_))
                .unwrap();
            cqs.push(cq);
            handles.push(handle);
        }

        Environment {
            cqs: cqs,
            _handles: handles,
        }
    }

    pub fn completion_queues(&self) -> &[Arc<CompletionQueue>] {
        self.cqs.as_slice()
    }

    pub fn pick_a_cq(&self) -> Arc<CompletionQueue> {
        // TODO: randomly pick up
        self.cqs[0].clone()
    }
}
