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


extern crate clap;
extern crate grpc;
extern crate interop;
extern crate tokio_core;

use std::sync::Arc;

use clap::{App, Arg};
use grpc::{Environment, ServerBuilder};
use interop::InteropTestService;
use interop::proto::testing::test_grpc;
use tokio_core::reactor::Core;

fn main() {
    let matches = App::new("Interoperability Test Server")
        .about("ref https://github.com/grpc/grpc/blob/v1.2.x/doc/interop-test-descriptions.md")
        .arg(Arg::with_name("host")
                 .help("The server host to listen to. For example, \"localhost\" or \"127.0.0.1\"")
                 .takes_value(true))
        .arg(Arg::with_name("port")
                 .help("The port to listen on. For example, \"8080\"")
                 .takes_value(true))
        .get_matches();
    let host = matches.value_of("host").unwrap_or("127.0.0.1");
    let port: u32 = matches
        .value_of("port")
        .unwrap_or("8080")
        .parse()
        .unwrap();

    let mut core = Core::new().unwrap();
    let remote = core.remote();
    let env = Arc::new(Environment::new(2));
    let instance = InteropTestService::new(remote);
    let service = test_grpc::create_test_service(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind(host, port)
        .build();

    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    server.start();

    loop {
        core.turn(None)
    }
}
