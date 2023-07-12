// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate log;

#[path = "../log_util.rs"]
mod log_util;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use grpcio_proto::example::helloworld::HelloRequest;
use grpcio_proto::example::helloworld_grpc::GreeterClient;

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.name = "world".to_owned();
    let reply = client.say_hello(&req).expect("rpc");
    info!("Greeter received: {}", reply.message);
}
