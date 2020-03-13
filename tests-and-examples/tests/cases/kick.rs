// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

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
    let port = server.bind_addrs().next().unwrap().1;
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

#[derive(Clone)]
pub struct DeadLockService {
    reporter: mpsc::Sender<()>,
}

impl Greeter for DeadLockService {
    fn say_hello(
        &mut self,
        ctx: RpcContext<'_>,
        mut req: HelloRequest,
        sink: UnarySink<HelloReply>,
    ) {
        let chan = Arc::default();
        let tx = NaiveSender { chan };
        let rx = NaiveReceiver {
            chan: tx.chan.clone(),
        };
        let name = req.take_name();
        let reporter = self.reporter.clone();
        ctx.spawn(
            rx.map_err(|_| panic!("should receive message"))
                .and_then(move |greet| {
                    let mut resp = HelloReply::default();
                    resp.set_message(format!("{} {}", greet, name));
                    sink.success(resp)
                        .map_err(|e| panic!("failed to reply {:?}", e))
                        .map(move |_| {
                            let _ = reporter.send(());
                        })
                }),
        );
        ctx.spawn(lazy(|| {
            tx.send("hello".to_owned())
                .map_err(|_| panic!("failed to send message"))
        }));
    }
}

#[derive(Default)]
struct NaiveChannel<T> {
    data: Option<T>,
    task: Option<task::Task>,
}

struct NaiveSender<T> {
    chan: Arc<Mutex<NaiveChannel<T>>>,
}

impl<T> NaiveSender<T> {
    fn send(self, t: T) -> impl Future<Item = (), Error = ()> {
        lazy(move || {
            let timer = Instant::now();
            while timer.elapsed() < Duration::from_secs(3) {
                let mut chan = match self.chan.try_lock() {
                    Ok(c) => c,
                    Err(_) => continue,
                };

                chan.data = Some(t);
                if let Some(t) = chan.task.take() {
                    t.notify();
                }
                return Ok(());
            }
            panic!("failed to acquire lock for sender after 3 seconds.");
        })
    }
}

struct NaiveReceiver<T> {
    chan: Arc<Mutex<NaiveChannel<T>>>,
}

impl<T> Future for NaiveReceiver<T> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<T, ()> {
        let timer = Instant::now();
        while timer.elapsed() < Duration::from_secs(3) {
            let mut chan = match self.chan.try_lock() {
                Ok(c) => c,
                Err(_) => continue,
            };
            if let Some(t) = chan.data.take() {
                return Ok(Async::Ready(t));
            }
            chan.task = Some(task::current());
            return Ok(Async::NotReady);
        }
        panic!("failed to acquire lock for receiver after 3 seconds.");
    }
}

/// Executor used to poll futures in place, which can cause deadlock.
/// Following is the timeline:
/// 1. receiver is polled and task is set.
/// 2. sender acquires lock
/// 3. sender sends data and notify task
/// 4. executor poll futures in place, so notify will become polling receiver
///    directly. So it acquires lock that is held by sender since 2, hence
///    deadlock.
#[test]
fn test_deadlock() {
    let env = Arc::new(EnvBuilder::new().build());
    let (tx, rx) = mpsc::channel();
    let service = create_greeter(DeadLockService { reporter: tx });
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
    req.set_name("world".to_owned());
    let f = client.say_hello_async(&req).unwrap();
    if let Err(e) = rx.recv_timeout(Duration::from_secs(5)) {
        // Panic will still calling drop method of server, which will wait for
        // deadlock forever.
        eprintln!("failed to wait for the case to finish: {:?}", e);
        std::process::exit(1);
    }
    let reply = f.wait().expect("rpc");
    assert_eq!(reply.get_message(), "hello world");
}
