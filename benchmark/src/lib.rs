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

#![allow(unknown_lints)]

#[macro_use]
extern crate futures;
extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
extern crate grpcio_sys as grpc_sys;
#[cfg(target_os = "linux")]
extern crate libc;
#[macro_use]
extern crate log;
extern crate rand;
extern crate tokio_timer;

// Since impl trait is not stable yet, implement this as a function is impossible without box.
macro_rules! spawn {
    ($exec:ident, $keep_running:expr, $tag:expr, $f:expr) => {
        $exec.spawn($f.map(|_| ()).map_err(move |e| {
            if $keep_running.load(Ordering::SeqCst) {
                error!("failed to execute {}: {:?}", $tag, e);
            }
        }))
    };
}

mod bench;
mod client;
pub mod driver;
mod error;
pub mod scenario;
mod server;
mod util;
mod worker;

pub use crate::util::log_util::init_log;
pub use crate::worker::Worker;

use futures::sync::oneshot;
use futures::Future;
use grpc::{Environment, ServerBuilder};
use grpc_proto::testing::services_grpc::create_worker_service;
use std::sync::Arc;

pub fn create_worker(port: u16) {
    let env = Arc::new(Environment::new(2));
    let (tx, rx) = oneshot::channel();
    let worker = Worker::new(tx);
    let service = create_worker_service(worker);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("[::]", port)
        .build()
        .unwrap();

    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    server.start();
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
