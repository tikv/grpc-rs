// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
#[macro_use]
extern crate log;

use std::env;
use std::sync::Arc;

use benchmark::{init_log, Worker};
use clap::{App, Arg};
use futures_channel::oneshot;
use grpc::{Environment, ServerBuilder, ServerCredentials};
use grpc_proto::testing::services_grpc::create_worker_service;
use rand::Rng;

const LOG_FILE: &str = "GRPCIO_BENCHMARK_LOG_FILE";

fn main() {
    let matches = App::new("Benchmark QpsWorker")
        .about("ref http://www.grpc.io/docs/guides/benchmarking.html")
        .arg(
            Arg::with_name("port")
                .long("driver_port")
                .help("The port the worker should listen on. For example, \"8080\"")
                .takes_value(true),
        )
        .get_matches();
    let port: u16 = matches.value_of("port").unwrap_or("8080").parse().unwrap();

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
        .add_listening_port(&format!("[::]:{port}"), ServerCredentials::insecure())
        .unwrap();

    info!("listening on [::]:{}", port);

    server.start();

    let _ = futures_executor::block_on(rx);
    let _ = futures_executor::block_on(server.shutdown());
}
