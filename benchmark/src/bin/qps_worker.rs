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
use std::env;

use benchmark::init_log;
use clap::{App, Arg};
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

    benchmark::create_worker(port);
}
