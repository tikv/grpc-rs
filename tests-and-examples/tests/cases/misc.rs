// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use futures::*;
use grpcio::*;
use grpcio_proto::example::helloworld::*;
use grpcio_proto::example::helloworld_grpc::*;
use std::sync::atomic::*;
use std::sync::*;
use std::thread::{self, JoinHandle};
use std::time::*;

#[test]
fn test_peer() {
    #[derive(Clone)]
    struct PeerService;

    impl Greeter for PeerService {
        fn say_hello(&mut self, ctx: RpcContext<'_>, _: HelloRequest, sink: UnarySink<HelloReply>) {
            let peer = ctx.peer();
            let mut resp = HelloReply::default();
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
    let port = server.bind_addrs().next().unwrap().1;
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = GreeterClient::new(ch);

    let req = HelloRequest::default();
    let resp = client.say_hello(&req).unwrap();

    assert!(resp.get_message().contains("127.0.0.1"), "{:?}", resp);
}

#[derive(Clone)]
struct Counter {
    global_counter: Arc<AtomicUsize>,
    local_counter: usize,
}

impl Counter {
    fn incr(&mut self) {
        self.local_counter += 1;
    }

    fn flush(&self) {
        self.global_counter
            .fetch_add(self.local_counter, Ordering::SeqCst);
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        self.flush();
    }
}

#[test]
fn test_soundness() {
    #[derive(Clone)]
    struct CounterService {
        c: Counter,
    }

    impl Greeter for CounterService {
        fn say_hello(&mut self, ctx: RpcContext<'_>, _: HelloRequest, sink: UnarySink<HelloReply>) {
            self.c.incr();
            let resp = HelloReply::default();
            ctx.spawn(
                sink.success(resp)
                    .map_err(|e| panic!("failed to reply {:?}", e)),
            );
        }
    }

    let env = Arc::new(EnvBuilder::new().cq_count(4).build());
    let counter = Arc::new(AtomicUsize::new(0));
    let service = CounterService {
        c: Counter {
            global_counter: counter.clone(),
            local_counter: 0,
        },
    };
    let mut server = ServerBuilder::new(env.clone())
        .register_service(create_greeter(service))
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;

    let spawn_reqs = |env| -> JoinHandle<()> {
        let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
        let client = GreeterClient::new(ch);
        let mut resps = Vec::with_capacity(3000);
        thread::spawn(move || {
            for _ in 0..3000 {
                resps.push(client.say_hello_async(&HelloRequest::default()).unwrap());
            }
            future::join_all(resps).wait().unwrap();
        })
    };
    let j1 = spawn_reqs(env.clone());
    let j2 = spawn_reqs(env.clone());
    let j3 = spawn_reqs(env.clone());
    j1.join().unwrap();
    j2.join().unwrap();
    j3.join().unwrap();
    server.shutdown().wait().unwrap();
    drop(server);
    drop(env);
    for _ in 0..100 {
        let cnt = counter.load(Ordering::SeqCst);
        if cnt == 9000 {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }
    assert_eq!(counter.load(Ordering::SeqCst), 9000);
}
