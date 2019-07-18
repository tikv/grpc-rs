// Copyright 2018 PingCAP, Inc.
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

use futures::*;
use grpcio::*;
use grpcio_proto::example::helloworld::*;
use grpcio_proto::example::helloworld_grpc::*;
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

        let mut resp = HelloReply::default();
        resp.set_message(format!("hello {}", req.take_name()));
        ctx.spawn(
            sink.success(resp)
                .map_err(|e| panic!("failed to reply {:?}", e)),
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
    let port = server.bind_addrs()[0].1;
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
