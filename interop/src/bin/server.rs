// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate clap;
extern crate futures;
extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
extern crate interop;
#[macro_use]
extern crate log;

use std::sync::Arc;

use clap::{App, Arg};
use futures::{future, Future};
use grpc::{Environment, ServerBuilder};
use grpc_proto::testing::test_grpc::create_test_service;
use grpc_proto::util;
use interop::InteropTestService;

fn main() {
    let matches = App::new("Interoperability Test Server")
        .about("ref https://github.com/grpc/grpc/blob/v1.3.x/doc/interop-test-descriptions.md")
        .arg(
            Arg::with_name("host")
                .long("host")
                .help("The server host to listen to. For example, \"localhost\" or \"127.0.0.1\"")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .help("The port to listen on. For example, \"8080\"")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("use_tls")
                .long("use_tls")
                .help("Whether to use a plaintext or encrypted connection")
                .takes_value(true),
        )
        .get_matches();
    let host = matches.value_of("host").unwrap_or("127.0.0.1");
    let port: u16 = matches.value_of("port").unwrap_or("8080").parse().unwrap();
    let use_tls: bool = matches
        .value_of("use_tls")
        .unwrap_or("false")
        .parse()
        .unwrap();

    let env = Arc::new(Environment::new(2));
    let service = create_test_service(InteropTestService);
    let mut builder = ServerBuilder::new(env).register_service(service);

    builder = if use_tls {
        let creds = util::create_test_server_credentials();
        builder.bind_with_cred(host, port, creds)
    } else {
        builder.bind(host, port)
    };

    let mut server = builder.build().unwrap();
    for (host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    server.start();

    let _ = future::empty::<(), ()>().wait();
}
