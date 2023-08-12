// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

//! A simple example shows that load balancing is working on the client side.
//!
//! For the design of load balancing, see https://github.com/grpc/grpc/blob/master/doc/load-balancing.md.
//!
//! Load report has not been implemented yet.

#[macro_use]
extern crate log;

#[path = "../log_util.rs"]
mod log_util;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder, LbPolicy};
use grpcio_proto::example::helloworld::HelloRequest;
use grpcio_proto::example::helloworld_grpc::GreeterClient;

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env)
        .load_balancing_policy(LbPolicy::RoundRobin)
        .connect("ipv4:127.0.0.1:50051,127.0.0.1:50052");
    let client = GreeterClient::new(ch);

    for _ in 0..2 {
        let mut req = HelloRequest::default();
        req.name = "world".to_owned();
        let reply = client.say_hello(&req).expect("rpc");
        info!("Greeter received: {}", reply.message);
    }
}
