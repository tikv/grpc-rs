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
use std::sync::atomic::*;
use std::cell::UnsafeCell;
use std::thread::{self, JoinHandle};

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
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = GreeterClient::new(ch);

    let req = HelloRequest::new();
    let resp = client.say_hello(&req).unwrap();

    assert!(resp.get_message().contains("127.0.0.1"), "{:?}", resp);
}


struct Counter {
    global_counter: Arc<AtomicUsize>,
    local_counter: UnsafeCell<usize>,
}

impl Counter {
    fn incr(&self) {
        unsafe {
            let counter = self.local_counter.get();
            let c = &mut *counter;
            *c += 1;
        }
    }

    fn flush(&self) {
        unsafe {
            let counter = self.local_counter.get();
            let c = &mut *counter;
            self.global_counter.fetch_add(*c, Ordering::SeqCst);
            *c = 0;
        }
    }
}

impl Clone for Counter {
    fn clone(&self) -> Counter {
        Counter {
            global_counter: self.global_counter.clone(),
            local_counter: UnsafeCell::new(0),
        }
    }
}

#[test]
fn test_soundness() {
    #[derive(Clone)]
    struct CounterService {
        c: Counter,
    }

    impl Greeter for CounterService {
        fn say_hello(&self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
            self.c.incr();
            let name = req.get_name();
            if name == "flush" {
                self.c.flush();
            }
            let mut resp = HelloReply::new();
            resp.set_message(name.to_string());
            ctx.spawn(
                sink.success(resp)
                    .map_err(|e| panic!("failed to reply {:?}", e)),
            );
        }
    }

    let env = Arc::new(EnvBuilder::new().cq_count(4).build());
    let counter = Arc::new(AtomicUsize::new(0));
    let service = CounterService { c: Counter { global_counter: counter.clone(), local_counter: UnsafeCell::new(0) } };
    let mut server = ServerBuilder::new(env.clone())
        .register_service(create_greeter(service))
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs()[0].1;
    let mut reqs = Vec::with_capacity(3000);
    for i in 0..3000 {
        let mut req = HelloRequest::new();
        if i == 1000 || i > 2000 {
            req.set_name("flush".to_string());
        } else {
            req.set_name("test".to_string());
        }
        reqs.push(req);
    }

    let spanw_reqs = || -> JoinHandle<()> {
        let ch = ChannelBuilder::new(env.clone()).connect(&format!("127.0.0.1:{}", port));
        let client = GreeterClient::new(ch);
        let mut resps = Vec::with_capacity(reqs.len());
        let reqs1 = reqs.clone();
        thread::spawn(move || {
            for req in &reqs1 {
                resps.push(client.say_hello_async(req).unwrap());
            }
            future::join_all(resps).wait().unwrap();
        })
    };
    let j1 = spanw_reqs();
    let j2 = spanw_reqs();
    let j3 = spanw_reqs();
    j1.join().unwrap();
    j2.join().unwrap();
    j3.join().unwrap();
    assert_eq!(counter.load(Ordering::SeqCst), 9000);
}
