// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use futures_executor::block_on;
use futures_util::future::{FutureExt as _, TryFutureExt as _};
use futures_util::{SinkExt, TryStreamExt};
use grpcio::*;
use grpcio_proto::example::helloworld::*;

use grpcio_proto::example::route_guide::{Feature, Rectangle};
use grpcio_proto::example::route_guide_grpc::{create_route_guide, RouteGuide, RouteGuideClient};
use grpcio_proto::google::rpc::Status;
use std::convert::TryInto;
use std::sync::*;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(
        &mut self,
        ctx: RpcContext<'_>,
        req: HelloRequest,
        mut sink: UnarySink<HelloReply>,
    ) {
        let headers = ctx.request_headers().clone();
        sink.set_headers(headers);

        if req.name == "root" {
            let mut status = Status {
                code: RpcStatusCode::INVALID_ARGUMENT.into(),
                message: "name can't be root".to_owned(),
                ..Default::default()
            };
            #[cfg(feature = "protobuf-codec")]
            let any = protobuf::well_known_types::Any::pack(&req).unwrap();

            #[cfg(feature = "protobufv3-codec")]
            let any = protobufv3::well_known_types::any::Any::pack(&req).unwrap();
            status.details.push(any);
            ctx.spawn(
                sink.fail(status.try_into().unwrap())
                    .map_err(|e| panic!("failed to report error: {:?}", e))
                    .map(|_| ()),
            );
            return;
        }

        let mut resp = HelloReply::default();
        resp.message = format!("hello {}", req.name);
        ctx.spawn(
            sink.success(resp)
                .map_err(|e| panic!("failed to reply {:?}", e))
                .map(|_| ()),
        );
    }
}

impl RouteGuide for GreeterService {
    fn list_features(
        &mut self,
        ctx: RpcContext,
        _req: Rectangle,
        mut sink: ServerStreamingSink<Feature>,
    ) {
        let headers = ctx.request_headers().clone();
        sink.set_headers(headers);
        ctx.spawn(async move {
            let f = Feature {
                name: "hello world".to_owned(),
                ..Default::default()
            };
            sink.send((f, WriteFlags::default())).await.unwrap();
            sink.close().await.unwrap();
        });
    }
}

// TODO: test it in interop tests once trailer is supported.
#[test]
fn test_metadata() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_route_guide(GreeterService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = RouteGuideClient::new(ch);

    let mut builder = MetadataBuilder::with_capacity(3);
    builder
        .add_str("k1", "v1")
        .unwrap()
        .add_bytes("k1-bin", &[0x00, 0x01, 0x02])
        .unwrap();
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);

    let mut req = HelloRequest::default();
    req.name = "world".to_owned();
    let mut resp = client
        .list_features_opt(&Default::default(), call_opt)
        .unwrap();
    let headers = block_on(resp.headers()).unwrap();

    // assert_eq!(msg.get_message(), "hello world");
    let mut v: Vec<_> = headers.iter().collect();
    v.sort();
    assert_eq!(v[0], ("k1", b"v1" as &[u8]));
    assert_eq!(v[1], ("k1-bin", &[0x00u8, 0x01, 0x02] as &[u8]));
    let msg = block_on(resp.try_next()).unwrap();
    assert_eq!(
        msg.as_ref().map(|f| f.name.as_str()),
        Some("hello world"),
        "{msg:?}"
    );
    assert_eq!(block_on(resp.try_next()).unwrap(), None);
}

/// Tests rich error can be accessed correctly.
#[test]
fn test_rich_error() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_greeter(GreeterService);
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
    req.name = "root".to_owned();
    let s: Status = match client.say_hello(&req) {
        Err(grpcio::Error::RpcFailure(s)) => s.try_into().unwrap(),
        res => panic!("expected failure, got {:?}", res),
    };
    assert_eq!(s.code, RpcStatusCode::INVALID_ARGUMENT.into());
    assert_eq!(s.message, "name can't be root");
    let details: Option<HelloRequest> = s.details[0].unpack().unwrap();
    assert_eq!(Some(req), details);
}
