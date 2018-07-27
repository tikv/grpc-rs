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
use std::sync::*;

#[test]
fn test_peer() {
    #[derive(Clone)]
    struct PeerService;

    impl Greeter for PeerService {
        fn say_hello(&self, ctx: RpcContext, _: HelloRequest, sink: UnarySink<HelloReply>) {
            let peer = ctx.peer();
            let mut resp = HelloReply::new();
            resp.set_message(peer);
            ctx.spawn(
                sink.success(resp)
                    .map_err(|e| panic!("failed to reply {:?}", e)),
            );
        }
    }

    let env = Arc::new(EnvBuilder::new().build());
    let service = create_greeter(PeerService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs()[0].1;
    let cb = ChannelBuilder::new(env);
    let cb = cb.load_balancing_policy(LbPolicy::RoundRobin);
    let ch = cb.connect(&format!("127.0.0.1:{}", port));
    let client = GreeterClient::new(ch);

    let req = HelloRequest::new();
    let resp = client.say_hello(&req).unwrap();

    assert!(resp.get_message().contains("127.0.0.1"), "{:?}", resp);
}
