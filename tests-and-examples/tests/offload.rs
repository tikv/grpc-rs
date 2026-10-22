// Copyright 2026 TiKV Project Authors. Licensed under Apache-2.0.

#![cfg(all(feature = "raw-codec", feature = "offload-codec"))]

use std::sync::Arc;

use futures_executor::block_on;
use futures_util::future::{FutureExt as _, TryFutureExt as _};
use grpcio::{Environment, RpcContext, ServerBuilder, ServerCredentials, UnarySink};
#[cfg(feature = "prost-codec")]
use grpcio_proto::example::helloworld::{create_greeter, Greeter};
use grpcio_proto::example::helloworld::{HelloReply, HelloRequest};
#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
use grpcio_proto::example::helloworld_grpc::{create_greeter, Greeter};
use grpcio_proto::offload::{decode, encode, Request, Response};

#[derive(Clone)]
struct OffloadGreeter;

impl Greeter for OffloadGreeter {
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
fn test_offload_codec_service_builds() {
    let env = Arc::new(Environment::new(1));
    let service = create_greeter(OffloadGreeter);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    assert!(port > 0);
    server.start();
    block_on(server.shutdown()).unwrap();
}
