// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
#[macro_use]
extern crate log;

use std::sync::Arc;

use clap::Parser;
use futures_executor::block_on;
use grpc::{Environment, ServerBuilder, ServerCredentials};
use grpc_proto::testing::test_grpc::create_test_service;
use grpc_proto::util;
use interop::InteropTestService;

/// Interoperability Test Server
///
/// ref https://github.com/grpc/grpc/blob/v1.3.x/doc/interop-test-descriptions.md.
#[derive(Parser)]
struct ServerCli {
    /// The server host to listen to. For example, "localhost" or "127.0.0.1"
    #[arg(long)]
    host: Option<String>,
    /// The port to listen on. For example, 8080
    #[arg(long)]
    port: Option<u16>,
    /// Whether to use a plaintext or encrypted connection
    #[arg(long)]
    use_tls: Option<bool>,
}

fn main() {
    let cli = ServerCli::parse();
    let host = cli.host.as_deref().unwrap_or("127.0.0.1");
    let mut port: u16 = cli.port.unwrap_or(8080);
    let use_tls: bool = cli.use_tls.unwrap_or(false);

    let env = Arc::new(Environment::new(2));
    let service = create_test_service(InteropTestService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .build()
        .unwrap();
    let creds = if use_tls {
        util::create_test_server_credentials()
    } else {
        ServerCredentials::insecure()
    };
    port = server
        .add_listening_port(&format!("{host}:{port}"), creds)
        .unwrap();
    info!("listening on {}:{}", host, port);
    server.start();

    block_on(futures_util::future::pending::<()>());
}
