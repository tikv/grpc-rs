// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
#[macro_use]
extern crate log;

use std::env;
use std::sync::Arc;

use benchmark::{init_log, Worker};
use clap::Parser;
use futures_channel::oneshot;
use grpc::{Environment, ServerBuilder, ServerCredentials};
use grpc_proto::testing::services_grpc::create_worker_service;
use rand::Rng;

const LOG_FILE: &str = "GRPCIO_BENCHMARK_LOG_FILE";

/// Benchmark QpsWorker
///
/// ref http://www.grpc.io/docs/guides/benchmarking.html.
#[derive(Parser)]
#[group(required = true, multiple = false)]
struct WorkerCli {
    /// The port the worker should listen on. For example, 8080
    #[arg(id = "driver_port", long)]
    driver_port0: Option<u16>,
    /// The port the worker should listen on. For example, 8080
    #[arg(id = "driver-port", long)]
    driver_port1: Option<u16>,
}

fn main() {
    let cli = WorkerCli::parse();
    let port = cli.driver_port0.unwrap_or(cli.driver_port1.unwrap_or(8080));

    let _log_guard = init_log(
        env::var(LOG_FILE)
            .ok()
            .map(|lf| format!("{}.{}", lf, rand::thread_rng().gen::<u32>())),
    );
    let env = Arc::new(Environment::new(2));
    let (tx, rx) = oneshot::channel();
    let worker = Worker::new(tx);
    let service = create_worker_service(worker);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port(format!("[::]:{port}"), ServerCredentials::insecure())
        .unwrap();

    info!("listening on [::]:{}", port);

    server.start();

    let _ = futures_executor::block_on(rx);
    let _ = futures_executor::block_on(server.shutdown());
}
