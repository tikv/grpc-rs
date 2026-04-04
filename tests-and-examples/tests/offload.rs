// Copyright 2026 TiKV Project Authors. Licensed under Apache-2.0.

#![cfg(all(feature = "raw-codec", feature = "offload-codec"))]

use std::sync::Arc;

use futures_util::future::{FutureExt as _, TryFutureExt as _};
use grpcio::{ChannelBuilder, EnvBuilder, RpcContext, ServerBuilder, ServerCredentials, UnarySink};
use grpcio_proto::example::helloworld::{HelloReply, HelloRequest};
use grpcio_proto::example::helloworld_grpc::{
    create_greeter_offload, GreeterClient, GreeterOffload,
};
use grpcio_proto::offload::{decode, encode, Request, Response};

#[derive(Clone)]
struct OffloadGreeter;

impl GreeterOffload for OffloadGreeter {
    fn say_hello(
        &mut self,
        ctx: RpcContext<'_>,
        req: Request<HelloRequest>,
        sink: UnarySink<Response<HelloReply>>,
    ) {
        let req = decode(req).expect("offload request should decode");
        let mut resp = HelloReply::default();
        resp.message = format!("Hello {}", req.name);
        ctx.spawn(
            sink.success(encode(resp).expect("offload response should encode"))
                .map_err(|e| panic!("failed to reply {:?}", e))
                .map(|_| ()),
        );
    }
}

#[test]
fn test_offload_codec_roundtrip() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_greeter_offload(OffloadGreeter);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();

    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.name = "offload".to_owned();
    let reply = client.say_hello(&req).unwrap();
    assert_eq!(reply.message, "Hello offload");
}
