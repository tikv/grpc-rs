extern crate futures;
extern crate grpcio;
extern crate grpcio_proto;

use futures::*;
use futures::sync::oneshot::{self, Sender};
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
    fn say_hello(&self, ctx: RpcContext, mut req: HelloRequest, sink: UnarySink<HelloReply>) {
        let (tx, rx) = oneshot::channel();
        let tx_lock = self.tx.clone();
        let name = req.take_name();
        let f = rx.map_err(|_| panic!("should receive message"))
            .join(lazy(move || {
                *tx_lock.lock().unwrap() = Some(tx);
                Ok(())
            }))
            .and_then(move |(greet, _)| {
                let mut resp = HelloReply::new();
                resp.set_message(format!("{} {}", greet, name));
                sink.success(resp)
                    .map_err(|e| panic!("failed to reply {:?}", e))
            });
        ctx.spawn(f)
    }
}

#[test]
fn test_alarm_notify() {
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
    let mut req = HelloRequest::new();
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
}
