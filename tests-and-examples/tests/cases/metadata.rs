// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use futures::*;
use grpcio::*;
use grpcio_proto::example::helloworld::*;
use grpcio_proto::google::rpc::Status;
use std::convert::TryInto;
use std::sync::mpsc::{self, Sender};
use std::sync::*;
use std::time::*;

#[derive(Clone)]
struct GreeterService {
    tx: Sender<(String, Vec<u8>)>,
}

impl Greeter for GreeterService {
    fn say_hello(
        &mut self,
        ctx: RpcContext<'_>,
        mut req: HelloRequest,
        sink: UnarySink<HelloReply>,
    ) {
        for (key, value) in ctx.request_headers() {
            self.tx.send((key.to_owned(), value.to_owned())).unwrap();
        }

        if req.name == "root" {
            let mut status = Status::default();
            status.code = RpcStatusCode::INVALID_ARGUMENT.into();
            status.message = "name can't be root".to_owned();
            let any = protobuf::well_known_types::Any::pack(&req).unwrap();
            status.details.push(any);
            ctx.spawn(
                sink.fail(status.try_into().unwrap())
                    .map_err(|e| panic!("failed to report error: {:?}", e))
                    .map(|_| ()),
            );
            return;
        }

        let mut resp = HelloReply::default();
        resp.set_message(format!("hello {}", req.take_name()));
        ctx.spawn(
            sink.success(resp)
                .map_err(|e| panic!("failed to reply {:?}", e))
                .map(|_| ()),
        );
    }
}

// TODO: test it in interop tests once trailer is supported.
#[test]
fn test_metadata() {
    let env = Arc::new(EnvBuilder::new().build());
    let (tx, rx) = mpsc::channel();
    let service = create_greeter(GreeterService { tx: tx });
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = GreeterClient::new(ch);

    let mut builder = MetadataBuilder::with_capacity(3);
    builder
        .add_str("k1", "v1")
        .unwrap()
        .add_bytes("k1-bin", &[0x00, 0x01, 0x02])
        .unwrap();
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);

    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let resp = client.say_hello_opt(&req, call_opt).unwrap();

    assert_eq!(resp.get_message(), "hello world");
    let metadata = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(metadata, ("k1".to_owned(), b"v1".to_vec()));
    let metadata = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(metadata, ("k1-bin".to_owned(), vec![0x00, 0x01, 0x02]));
}

/// Tests rich error can be accessed correctly.
#[test]
fn test_rich_error() {
    let env = Arc::new(EnvBuilder::new().build());
    let (tx, _rx) = mpsc::channel();
    let service = create_greeter(GreeterService { tx });
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.set_name("root".to_owned());
    let s: Status = match client.say_hello(&req) {
        Err(grpcio::Error::RpcFailure(s)) => s.try_into().unwrap(),
        res => panic!("expected failure, got {:?}", res),
    };
    assert_eq!(s.code, RpcStatusCode::INVALID_ARGUMENT.into());
    assert_eq!(s.message, "name can't be root");
    let details: Option<HelloRequest> = s.details[0].unpack().unwrap();
    assert_eq!(Some(req), details);
}
