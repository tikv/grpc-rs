// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use futures_executor::block_on;
use futures_timer::Delay;
use futures_util::future::{self, FutureExt as _, TryFutureExt as _};
use grpcio::*;
use grpcio_proto::example::helloworld::*;

use std::sync::atomic::*;
use std::sync::*;
use std::thread::{self, JoinHandle};
use std::time::*;

#[derive(Clone)]
struct PeerService;

impl Greeter for PeerService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, _: HelloRequest, sink: UnarySink<HelloReply>) {
        let peer = ctx.peer();
        let mut resp = HelloReply::default();
        resp.message = peer;
        ctx.spawn(
            sink.success(resp)
                .map_err(|e| panic!("failed to reply {:?}", e))
                .map(|_| ()),
        );
    }
}

#[derive(Clone)]
struct SleepService(bool);

impl Greeter for SleepService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, _: HelloRequest, sink: UnarySink<HelloReply>) {
        let need_delay = self.0;
        ctx.spawn(async move {
            if need_delay {
                Delay::new(Duration::from_secs(3)).await;
            }
            let resp = HelloReply::default();
            sink.success(resp)
                .map_err(|e| panic!("failed to reply {:?}", e))
                .await
                .unwrap();
        });
    }
}

#[test]
fn test_peer() {
    let counter_add = Arc::new(AtomicI32::new(0));
    let counter_collect = counter_add.clone();
    let env = Arc::new(
        EnvBuilder::new()
            .cq_count(2)
            .after_start(move || {
                counter_add.fetch_add(1, Ordering::Relaxed);
            })
            .build(),
    );
    let service = create_greeter(PeerService);
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

    let req = HelloRequest::default();
    let resp = client.say_hello(&req).unwrap();

    assert!(resp.message.contains("127.0.0.1"), "{:?}", resp);
    assert_eq!(counter_collect.load(Ordering::Relaxed), 2);
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
                    .map_err(|e| panic!("failed to reply {:?}", e))
                    .map(|_| ()),
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
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();

    let spawn_reqs = |env| -> JoinHandle<()> {
        let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
        let client = GreeterClient::new(ch);
        let mut resps = Vec::with_capacity(3000);
        thread::spawn(move || {
            for _ in 0..3000 {
                resps.push(client.say_hello_async(&HelloRequest::default()).unwrap());
            }
            block_on(future::try_join_all(resps)).unwrap();
        })
    };
    let j1 = spawn_reqs(env.clone());
    let j2 = spawn_reqs(env.clone());
    let j3 = spawn_reqs(env.clone());
    j1.join().unwrap();
    j2.join().unwrap();
    j3.join().unwrap();
    block_on(server.shutdown()).unwrap();
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

#[cfg(unix)]
mod unix_domain_socket {
    use super::*;

    fn test_socket(path: &str) {
        let env = Arc::new(EnvBuilder::new().build());
        let service = create_greeter(PeerService);

        let mut server = ServerBuilder::new(env.clone())
            .register_service(service)
            .build()
            .unwrap();
        server
            .add_listening_port(path, ServerCredentials::insecure())
            .unwrap();
        server.start();
        let ch = ChannelBuilder::new(env).connect(path);
        let client = GreeterClient::new(ch);

        let req = HelloRequest::default();
        let resp = client.say_hello(&req).unwrap();

        assert_eq!(resp.message, "unix:", "{resp:?}");
    }

    #[test]
    fn test_unix_domain_socket() {
        struct Defer(&'static str);

        impl Drop for Defer {
            fn drop(&mut self) {
                let _ = std::fs::remove_file(&self.0[5..]);
            }
        }
        let socket_path = Defer("unix:test_socket");
        test_socket(socket_path.0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_abstract_unix_domain_socket() {
        test_socket("unix-abstract:/test_socket");
    }
}

#[test]
fn test_shutdown_when_exists_grpc_call() {
    let env = Arc::new(Environment::new(2));
    // Start a server and delay the process of grpc server.
    let service = create_greeter(SleepService(true));
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

    let req = HelloRequest::default();
    let send_task = client.say_hello_async(&req).unwrap();
    drop(server);
    assert!(
        block_on(send_task).is_err(),
        "Send should get error because server is shutdown, so the grpc is cancelled."
    );
}

#[test]
fn test_custom_checker_server_side() {
    let flag = Arc::new(atomic::AtomicBool::new(false));
    let checker = FlagChecker { flag: flag.clone() };

    let env = Arc::new(Environment::new(2));
    // Start a server and delay the process of grpc server.
    let service = create_greeter(PeerService);
    let mut server = ServerBuilder::new(env.clone())
        .add_checker(checker)
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = GreeterClient::new(ch);
    let req = HelloRequest::default();

    let _ = client.say_hello(&req).unwrap();
    let _ = client.say_hello(&req).unwrap();

    flag.store(true, Ordering::SeqCst);
    assert_eq!(
        client.say_hello(&req).unwrap_err().to_string(),
        "RpcFailure: 15-DATA_LOSS".to_owned()
    );
}

#[derive(Clone)]
struct FlagChecker {
    flag: Arc<atomic::AtomicBool>,
}

impl ServerChecker for FlagChecker {
    fn check(&mut self, ctx: &RpcContext) -> CheckResult {
        let method = String::from_utf8(ctx.method().to_owned());
        assert_eq!(&method.unwrap(), "/helloworld.Greeter/SayHello");

        if self.flag.load(Ordering::SeqCst) {
            CheckResult::Abort(RpcStatus::new(RpcStatusCode::DATA_LOSS))
        } else {
            CheckResult::Continue
        }
    }

    fn box_clone(&self) -> Box<dyn ServerChecker> {
        Box::new(self.clone())
    }
}

/// Tests connectivity related API works as expected.
#[test]
fn test_connectivity() {
    let env = Arc::new(Environment::new(2));
    let service = create_greeter(PeerService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();
    let ch = ChannelBuilder::new(env.clone()).connect(&format!("127.0.0.1:{port}"));
    assert!(block_on(ch.wait_for_connected(Duration::from_secs(3))));
    assert_eq!(
        ch.check_connectivity_state(false),
        ConnectivityState::GRPC_CHANNEL_READY
    );

    // Shutdown server should make the connection transit to failure.
    block_on(server.shutdown()).unwrap();
    assert!(block_on(ch.wait_for_state_change(
        ConnectivityState::GRPC_CHANNEL_READY,
        Duration::from_secs(3)
    )));
    // Shutdown will send goaway, hence the state goes to idle.
    assert_eq!(
        ch.check_connectivity_state(false),
        ConnectivityState::GRPC_CHANNEL_IDLE
    );

    // There is no pending rpc, so grpc will not retry connecting.
    assert!(!block_on(ch.wait_for_state_change(
        ConnectivityState::GRPC_CHANNEL_IDLE,
        Duration::from_millis(100)
    )));

    // Ask it to reconnect explicitly.
    ch.check_connectivity_state(true);
    assert!(block_on(ch.wait_for_state_change(
        ConnectivityState::GRPC_CHANNEL_IDLE,
        Duration::from_secs(3)
    )));
    assert_ne!(
        ch.check_connectivity_state(false),
        ConnectivityState::GRPC_CHANNEL_IDLE
    );

    // It can't be ready since no server is running.
    assert!(!block_on(ch.wait_for_connected(Duration::from_millis(100))));
    assert!(block_on(ch.wait_for_state_change(
        ConnectivityState::GRPC_CHANNEL_CONNECTING,
        Duration::from_secs(3)
    )));
    assert_ne!(
        ch.check_connectivity_state(false),
        ConnectivityState::GRPC_CHANNEL_READY
    );
    assert_ne!(
        ch.check_connectivity_state(false),
        ConnectivityState::GRPC_CHANNEL_IDLE
    );

    // After server is restarted, client should be able to reconnect successfully.
    // A shutdown server should not be restarted again, using a different instance.
    let service = create_greeter(PeerService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .build()
        .unwrap();
    server
        .add_listening_port(&format!("localhost:{port}"), ServerCredentials::insecure())
        .unwrap();
    server.start();
    assert!(block_on(ch.wait_for_connected(Duration::from_secs(3))));
    assert_eq!(
        ch.check_connectivity_state(false),
        ConnectivityState::GRPC_CHANNEL_READY
    );
    let client = GreeterClient::new(ch.clone());
    let req = HelloRequest::default();
    let resp = client.say_hello(&req).unwrap();
    assert!(!resp.message.is_empty());
    client.spawn(async move {
        ch.wait_for_connected(Duration::from_secs(3)).await;
    });
}

/// Tests channelz related API works as expected.
#[test]
fn test_channelz() {
    let env = Arc::new(Environment::new(2));
    let service = create_greeter(PeerService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();
    let mut res = None;
    channelz::get_servers(0, |s| {
        res = Some(s.to_string());
    });
    // There should be at least one server.
    assert!(
        res.as_ref().map_or(false, |s| s.contains("serverId")),
        "{:?}",
        res
    );
    res = None;
    channelz::get_server(0, |s| {
        res = Some(s.to_string());
    });
    // 0 will never be used as id.
    assert_eq!(res, Some(String::new()));

    res = None;
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    assert!(block_on(ch.wait_for_connected(Duration::from_secs(3))));
    channelz::get_top_channels(0, |s| {
        res = Some(s.to_string());
    });
    // There should be at least one channel.
    assert!(
        res.as_ref().map_or(false, |s| s.contains("channelId")),
        "{:?}",
        res
    );
}
