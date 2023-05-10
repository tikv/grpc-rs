// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate log;

#[path = "../log_util.rs"]
mod log_util;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures_channel::oneshot;
use futures_executor::block_on;
use futures_util::future::{FutureExt as _, TryFutureExt as _};
use grpcio::{
    ChannelBuilder, Environment, ResourceQuota, RpcContext, Server, ServerBuilder,
    ServerCredentials, UnarySink,
};

use grpcio_proto::example::helloworld::{HelloReply, HelloRequest};
use grpcio_proto::example::helloworld_grpc::{create_greeter, Greeter};

#[derive(Clone)]
struct GreeterService {
    name: String,
}

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let msg = format!("Hello {}, I'm {}", req.name, self.name);
        let mut resp = HelloReply::default();
        resp.message = msg;
        let f = sink
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

fn build_server(env: Arc<Environment>, mut port: u16) -> Server {
    let service = create_greeter(GreeterService {
        name: format!("{port}"),
    });
    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    port = server
        .add_listening_port(&format!("127.0.0.1:{port}"), ServerCredentials::insecure())
        .unwrap();
    server.start();
    info!("listening on 127.0.0.1:{port}");
    server
}

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(Environment::new(1));
    let mut server1 = build_server(env.clone(), 50_051);
    let mut server2 = build_server(env, 50_052);
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server1.shutdown());
    let _ = block_on(server2.shutdown());
}
