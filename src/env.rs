// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::{Builder as ThreadBuilder, JoinHandle};

use crate::cq::{CompletionQueue, CompletionQueueHandle, EventType, WorkQueue};
use crate::grpc_sys;
use crate::task::CallTag;

#[cfg(feature = "prometheus")]
use {
    crate::metrics::{
        GRPC_POOL_CQ_NEXT_DURATION, GRPC_POOL_EVENT_COUNT_VEC, GRPC_POOL_EXECUTE_DURATION,
        GRPC_TASK_WAIT_DURATION,
    },
    crate::task::resolve,
    prometheus::{
        core::{AtomicU64, GenericCounter},
        Histogram,
    },
    std::time::Instant,
};

#[cfg(feature = "prometheus")]
pub struct GRPCRunner {
    cq_next_duration_his: Histogram,
    execute_duration_his: Histogram,
    wait_duration_his: Histogram,
    event_counter: [GenericCounter<AtomicU64>; 6],
}

#[cfg(feature = "prometheus")]
impl GRPCRunner {
    pub fn new(name: &String) -> GRPCRunner {
        let cq_next_duration_his = GRPC_POOL_CQ_NEXT_DURATION.with_label_values(&[name]);
        let execute_duration_his = GRPC_POOL_EXECUTE_DURATION.with_label_values(&[name]);
        let wait_duration_his = GRPC_TASK_WAIT_DURATION.with_label_values(&[name]);
        let event_counter = ["batch", "request", "unary", "abort", "action", "spawn"]
            .map(|event| GRPC_POOL_EVENT_COUNT_VEC.with_label_values(&[name, event]));
        GRPCRunner {
            cq_next_duration_his,
            execute_duration_his,
            wait_duration_his,
            event_counter,
        }
    }

    // event loop
    pub fn run(&self, tx: mpsc::Sender<CompletionQueue>) {
        let cq = Arc::new(CompletionQueueHandle::new());
        let worker_info = Arc::new(WorkQueue::new());
        let cq = CompletionQueue::new(cq, worker_info);
        tx.send(cq.clone()).expect("send back completion queue");
        loop {
            let now = Instant::now();
            let e = cq.next();
            self.cq_next_duration_his
                .observe(now.elapsed().as_secs_f64());
            let now = Instant::now();
            match e.type_ {
                EventType::GRPC_QUEUE_SHUTDOWN => break,
                // timeout should not happen in theory.
                EventType::GRPC_QUEUE_TIMEOUT => continue,
                EventType::GRPC_OP_COMPLETE => {}
            }

            let tag: Box<CallTag> = unsafe { Box::from_raw(e.tag as _) };
            self.resolve(tag, &cq, e.success != 0);
            while let Some(work) = unsafe { cq.worker.pop_work() } {
                work.finish();
            }
            self.execute_duration_his
                .observe(now.elapsed().as_secs_f64());
        }
    }

    fn resolve(&self, tag: Box<CallTag>, cq: &CompletionQueue, success: bool) {
        match *tag {
            CallTag::Batch(prom) => {
                self.event_counter[0].inc();
                prom.resolve(success)
            }
            CallTag::Request(cb) => {
                self.event_counter[1].inc();
                cb.resolve(cq, success)
            }
            CallTag::UnaryRequest(cb) => {
                self.event_counter[2].inc();
                cb.resolve(cq, success)
            }
            CallTag::Abort(_) => self.event_counter[3].inc(),
            CallTag::Action(prom) => {
                self.event_counter[4].inc();
                prom.resolve(success)
            }
            CallTag::Spawn(task) => {
                self.event_counter[5].inc();
                self.wait_duration_his
                    .observe(task.reset_push_time().elapsed().as_secs_f64());
                resolve(task, success)
            }
        }
    }
}

#[cfg(not(feature = "prometheus"))]
fn poll_queue(tx: mpsc::Sender<CompletionQueue>) {
    let cq = Arc::new(CompletionQueueHandle::new());
    let worker_info = Arc::new(WorkQueue::new());
    let cq = CompletionQueue::new(cq, worker_info);
    tx.send(cq.clone()).expect("send back completion queue");

    loop {
        let e = cq.next();
        match e.type_ {
            EventType::GRPC_QUEUE_SHUTDOWN => break,
            // timeout should not happen in theory.
            EventType::GRPC_QUEUE_TIMEOUT => continue,
            EventType::GRPC_OP_COMPLETE => {}
        }
        let tag: Box<CallTag> = unsafe { Box::from_raw(e.tag as _) };
        tag.resolve(&cq, e.success != 0);
        while let Some(work) = unsafe { cq.worker.pop_work() } {
            work.finish();
        }
    }
}

/// [`Environment`] factory in order to configure the properties.
pub struct EnvBuilder {
    cq_count: usize,
    name_prefix: Option<String>,
    after_start: Option<Arc<dyn Fn() + Send + Sync>>,
    before_stop: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl EnvBuilder {
    /// Initialize a new [`EnvBuilder`].
    pub fn new() -> EnvBuilder {
        EnvBuilder {
            cq_count: unsafe { grpc_sys::gpr_cpu_num_cores() as usize },
            name_prefix: None,
            after_start: None,
            before_stop: None,
        }
    }

    /// Set the number of completion queues and polling threads. Each thread polls
    /// one completion queue.
    ///
    /// # Panics
    ///
    /// This method will panic if `count` is 0.
    pub fn cq_count(mut self, count: usize) -> EnvBuilder {
        assert!(count > 0);
        self.cq_count = count;
        self
    }

    /// Set the thread name prefix of each polling thread.
    pub fn name_prefix<S: Into<String>>(mut self, prefix: S) -> EnvBuilder {
        self.name_prefix = Some(prefix.into());
        self
    }

    /// Execute function `f` after each thread is started but before it starts doing work.
    pub fn after_start<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> EnvBuilder {
        self.after_start = Some(Arc::new(f));
        self
    }

    /// Execute function `f` before each thread stops.
    pub fn before_stop<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> EnvBuilder {
        self.before_stop = Some(Arc::new(f));
        self
    }

    /// Finalize the [`EnvBuilder`], build the [`Environment`] and initialize the gRPC library.
    pub fn build(self) -> Environment {
        unsafe {
            grpc_sys::grpc_init();
        }
        let mut cqs = Vec::with_capacity(self.cq_count);
        let mut handles = Vec::with_capacity(self.cq_count);
        let (tx, rx) = mpsc::channel();
        for i in 0..self.cq_count {
            let tx_i = tx.clone();
            let mut builder = ThreadBuilder::new();
            let name = self
                .name_prefix
                .as_ref()
                .map_or(format!("grpc-pool-{i}"), |prefix| format!("{prefix}-{i}"));
            #[cfg(feature = "prometheus")]
            let runner = GRPCRunner::new(&name);
            builder = builder.name(name);
            let after_start = self.after_start.clone();
            let before_stop = self.before_stop.clone();
            let handle = builder
                .spawn(move || {
                    if let Some(f) = after_start {
                        f();
                    }
                    #[cfg(feature = "prometheus")]
                    runner.run(tx_i);
                    #[cfg(not(feature = "prometheus"))]
                    poll_queue(tx_i);
                    if let Some(f) = before_stop {
                        f();
                    }
                })
                .unwrap();
            handles.push(handle);
        }
        for _ in 0..self.cq_count {
            cqs.push(rx.recv().unwrap());
        }

        Environment {
            cqs,
            idx: AtomicUsize::new(0),
            _handles: handles,
        }
    }
}

/// An object that used to control concurrency and start gRPC event loop.
pub struct Environment {
    cqs: Vec<CompletionQueue>,
    idx: AtomicUsize,
    _handles: Vec<JoinHandle<()>>,
}

impl Environment {
    /// Initialize gRPC and create a thread pool to poll completion queue. The thread pool size
    /// and the number of completion queue is specified by `cq_count`. Each thread polls one
    /// completion queue.
    ///
    /// # Panics
    ///
    /// This method will panic if `cq_count` is 0.
    pub fn new(cq_count: usize) -> Environment {
        assert!(cq_count > 0);
        EnvBuilder::new()
            .name_prefix("grpc-poll")
            .cq_count(cq_count)
            .build()
    }

    /// Get all the created completion queues.
    pub fn completion_queues(&self) -> &[CompletionQueue] {
        self.cqs.as_slice()
    }

    /// Pick an arbitrary completion queue.
    pub fn pick_cq(&self) -> CompletionQueue {
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

        let q1 = env.pick_cq();
        let q2 = env.pick_cq();
        let q3 = env.pick_cq();
        let cases = vec![(&q1, &q3, true), (&q1, &q2, false)];
        for (lq, rq, is_eq) in cases {
            let lq_ref = lq.borrow().unwrap();
            let rq_ref = rq.borrow().unwrap();
            if is_eq {
                assert_eq!(lq_ref.as_ptr(), rq_ref.as_ptr());
            } else {
                assert_ne!(lq_ref.as_ptr(), rq_ref.as_ptr());
            }
        }

        assert_eq!(env.completion_queues().len(), 2);
        for cq in env.completion_queues() {
            cq.shutdown();
        }

        for handle in env._handles.drain(..) {
            handle.join().unwrap();
        }
    }
}
