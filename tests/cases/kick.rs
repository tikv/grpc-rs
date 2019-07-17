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

use futures::sync::oneshot::{self, Sender};
use futures::*;
use grpcio::*;
use grpcio_proto::example::helloworld::*;
use grpcio_proto::example::helloworld_grpc::*;
use std::sync::*;
use std::thread;
use std::time::*;

#[derive(Clone)]
struct GreeterService {
    tx: Arc<Mutex<Option<Sender<String>>>>,
}

impl Greeter for GreeterService {
    fn say_hello(
        &mut self,
        ctx: RpcContext<'_>,
        mut req: HelloRequest,
        sink: UnarySink<HelloReply>,
    ) {
        let (tx, rx) = oneshot::channel();
        let tx_lock = self.tx.clone();
        let name = req.take_name();
        let f = rx
            .map_err(|_| panic!("should receive message"))
            .join(lazy(move || {
                *tx_lock.lock().unwrap() = Some(tx);
                Ok(())
            }))
            .and_then(move |(greet, _)| {
                let mut resp = HelloReply::default();
                resp.set_message(format!("{} {}", greet, name));
                sink.success(resp)
                    .map_err(|e| panic!("failed to reply {:?}", e))
            });
        ctx.spawn(f)
    }
}

#[test]
fn test_kick() {
    let env = Arc::new(EnvBuilder::new().build());
    let tx = Arc::new(Mutex::new(None));
    let service = create_greeter(GreeterService { tx: tx.clone() });
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs()[0].1;
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = GreeterClient::new(ch);
    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let f = client.say_hello_async(&req).unwrap();
    loop {
        thread::sleep(Duration::from_millis(10));
        let mut tx = tx.lock().unwrap();
        if tx.is_none() {
            continue;
        }
        tx.take().unwrap().send("hello".to_owned()).unwrap();
        break;
    }
    let reply = f.wait().expect("rpc");
    assert_eq!(reply.get_message(), "hello world");

    // Spawn a future in the client.
    let (tx1, rx2) = spawn_chianed_channel(&client);
    thread::sleep(Duration::from_millis(10));
    let _ = tx1.send(77);
    assert_eq!(rx2.wait().unwrap(), 77);

    // Drop the client before a future is resolved.
    let (tx1, rx2) = spawn_chianed_channel(&client);
    drop(client);
    thread::sleep(Duration::from_millis(10));
    let _ = tx1.send(88);
    assert_eq!(rx2.wait().unwrap(), 88);
}

fn spawn_chianed_channel(
    client: &GreeterClient,
) -> (oneshot::Sender<usize>, oneshot::Receiver<usize>) {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    let f = rx1
        .map(|n| {
            let _ = tx2.send(n);
        })
        .map_err(|_| ());
    client.spawn(f);

    (tx1, rx2)
}
