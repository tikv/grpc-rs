// Copyright 2026 TiKV Project Authors. Licensed under Apache-2.0.

use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use futures_executor::block_on;
use futures_util::{future, SinkExt as _, TryStreamExt as _};
use grpcio::{
    CallOption, ChannelBuilder, CheckResult, ClientStreamingSink, DuplexSink, EnvBuilder,
    RequestStream, RpcContext, ServerBuilder, ServerChecker, ServerCredentials,
    ServerStreamingSink, UnarySink, WriteFlags,
};
#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
use grpcio_proto::example::route_guide::*;
#[cfg(feature = "prost-codec")]
use grpcio_proto::example::routeguide::*;

const GET_FEATURE: &str = "/routeguide.RouteGuide/GetFeature";
const LIST_FEATURES: &str = "/routeguide.RouteGuide/ListFeatures";
const RECORD_ROUTE: &str = "/routeguide.RouteGuide/RecordRoute";
const ROUTE_CHAT: &str = "/routeguide.RouteGuide/RouteChat";
const RPC_TIMEOUT: Duration = Duration::from_secs(10);

/// A service that reports the method and the thread of every call it handles.
#[derive(Clone)]
struct CallRecorder {
    tx: mpsc::Sender<(Vec<u8>, String)>,
}

impl CallRecorder {
    fn record(&self, ctx: &RpcContext<'_>) {
        if ctx.method() == LIST_FEATURES.as_bytes() {
            assert!(!ctx.host().is_empty());
        } else {
            assert!(ctx.host().is_empty());
        }
        let thread_name = thread::current().name().unwrap_or_default().to_owned();
        self.tx.send((ctx.method().to_vec(), thread_name)).unwrap();
    }
}

impl ServerChecker for CallRecorder {
    fn check(&mut self, ctx: &RpcContext<'_>) -> CheckResult {
        self.record(ctx);
        CheckResult::Continue
    }

    fn box_clone(&self) -> Box<dyn ServerChecker> {
        Box::new(self.clone())
    }
}

impl RouteGuide for CallRecorder {
    fn get_feature(&mut self, ctx: RpcContext<'_>, _: Point, sink: UnarySink<Feature>) {
        self.record(&ctx);
        ctx.spawn(async move {
            let _ = sink.success(Feature::default()).await;
        });
    }

    fn list_features(
        &mut self,
        ctx: RpcContext<'_>,
        _: Rectangle,
        mut sink: ServerStreamingSink<Feature>,
    ) {
        self.record(&ctx);
        ctx.spawn(async move {
            let _ = sink.close().await;
        });
    }

    fn record_route(
        &mut self,
        ctx: RpcContext<'_>,
        points: RequestStream<Point>,
        sink: ClientStreamingSink<RouteSummary>,
    ) {
        self.record(&ctx);
        ctx.spawn(async move {
            let _ = points.try_for_each(|_| future::ok(())).await;
            let _ = sink.success(RouteSummary::default()).await;
        });
    }
}

#[test]
fn test_route_methods_to_another_env() {
    let env = Arc::new(EnvBuilder::new().cq_count(2).name_prefix("main").build());
    let routed_env = Arc::new(
        EnvBuilder::new()
            .cq_count(2)
            .set_non_listening(true)
            .name_prefix("routed")
            .build(),
    );
    let (tx, rx) = mpsc::channel();
    let mut server = ServerBuilder::new(env)
        .requests_slot_per_cq(2)
        .register_service(create_route_guide(CallRecorder { tx: tx.clone() }))
        .add_checker(CallRecorder { tx })
        .route_methods(routed_env.clone(), &[GET_FEATURE])
        .route_methods(routed_env.clone(), &[RECORD_ROUTE])
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();
    // The non-listening environment can serve both client and server calls.
    let ch = ChannelBuilder::new(routed_env).connect(&format!("127.0.0.1:{port}"));
    let client = RouteGuideClient::new(ch);

    // Routed unary and client streaming methods.
    // More calls than the available slots also exercise slot replenishment.
    for _ in 0..6 {
        client
            .get_feature_opt(
                &Point::default(),
                CallOption::default().timeout(RPC_TIMEOUT),
            )
            .unwrap();
        let (mut sink, receiver) = client
            .record_route_opt(CallOption::default().timeout(RPC_TIMEOUT))
            .unwrap();
        block_on(async {
            sink.send((Point::default(), WriteFlags::default()))
                .await
                .unwrap();
            sink.close().await.unwrap();
            receiver.await.unwrap();
        });
    }
    // A server streaming method that is not routed.
    let mut features = client
        .list_features_opt(
            &Rectangle::default(),
            CallOption::default().timeout(RPC_TIMEOUT),
        )
        .unwrap();
    block_on(async { while features.try_next().await.unwrap().is_some() {} });

    let recorded: Vec<_> = rx.try_iter().collect();
    assert_eq!(recorded.len(), 26);
    let mut calls = HashMap::new();
    for (method, thread_name) in recorded {
        let prefix = if method == LIST_FEATURES.as_bytes() {
            "main"
        } else {
            "routed"
        };
        assert!(
            thread_name.starts_with(prefix),
            "method {:?} ran on {}",
            method,
            thread_name
        );
        *calls.entry(method).or_insert(0) += 1;
    }
    assert_eq!(calls.len(), 3, "{:?}", calls);
    assert_eq!(calls[GET_FEATURE.as_bytes()], 12);
    assert_eq!(calls[RECORD_ROUTE.as_bytes()], 12);
    assert_eq!(calls[LIST_FEATURES.as_bytes()], 2);
    block_on(server.shutdown()).unwrap();
}

#[derive(Clone)]
struct BlockingService {
    blocked: mpsc::Sender<()>,
    resume: Arc<Mutex<mpsc::Receiver<()>>>,
}

impl RouteGuide for BlockingService {
    fn get_feature(&mut self, ctx: RpcContext<'_>, point: Point, sink: UnarySink<Feature>) {
        if point.longitude == -1 {
            self.blocked.send(()).unwrap();
            self.resume
                .lock()
                .unwrap()
                .recv_timeout(RPC_TIMEOUT * 2)
                .unwrap();
        }
        ctx.spawn(async move {
            let _ = sink.success(Feature::default()).await;
        });
    }

    fn route_chat(
        &mut self,
        ctx: RpcContext<'_>,
        mut notes: RequestStream<RouteNote>,
        mut sink: DuplexSink<RouteNote>,
    ) {
        assert_eq!(ctx.method(), ROUTE_CHAT.as_bytes());
        assert!(ctx.host().is_empty());
        assert!(thread::current().name().unwrap().starts_with("routed"));
        ctx.spawn(async move {
            while let Some(note) = notes.try_next().await.unwrap() {
                sink.send((note, WriteFlags::default())).await.unwrap();
            }
            sink.close().await.unwrap();
        });
    }
}

/// Releases the blocked CQ before server shutdown, including on test failure.
struct ResumeMainCq(mpsc::Sender<()>);

impl Drop for ResumeMainCq {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}

#[test]
fn test_routed_duplex_progresses_while_main_cq_is_blocked() {
    let env = Arc::new(EnvBuilder::new().cq_count(1).name_prefix("main").build());
    let routed_env = Arc::new(
        EnvBuilder::new()
            .cq_count(1)
            .set_non_listening(true)
            .name_prefix("routed")
            .build(),
    );
    let (blocked_tx, blocked_rx) = mpsc::channel();
    let (resume_tx, resume_rx) = mpsc::channel();
    let mut server = ServerBuilder::new(env)
        .requests_slot_per_cq(1)
        .register_service(create_route_guide(BlockingService {
            blocked: blocked_tx,
            resume: Arc::new(Mutex::new(resume_rx)),
        }))
        .route_methods(routed_env, &[ROUTE_CHAT])
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();
    let client_env = Arc::new(EnvBuilder::new().cq_count(1).build());
    let client = RouteGuideClient::new(
        ChannelBuilder::new(client_env).connect(&format!("127.0.0.1:{port}")),
    );

    // Establish the connection before blocking its ordinary server CQ.
    // This covers routed streams on that connection, not independent
    // acceptance of new connections.
    client
        .get_feature_opt(
            &Point::default(),
            CallOption::default().timeout(RPC_TIMEOUT),
        )
        .unwrap();
    let resume = ResumeMainCq(resume_tx);
    let point = Point {
        longitude: -1,
        ..Point::default()
    };
    let blocked_call = client
        .get_feature_async_opt(&point, CallOption::default().timeout(RPC_TIMEOUT * 2))
        .unwrap();
    blocked_rx.recv_timeout(RPC_TIMEOUT).unwrap();

    // Payloads, responses and successive streams must progress on the routed CQ.
    for call in 0..3 {
        let (mut sender, mut receiver) = client
            .route_chat_opt(CallOption::default().timeout(RPC_TIMEOUT))
            .unwrap();
        block_on(async {
            for message in 0..3 {
                let note = RouteNote {
                    message: format!("{call}:{message}"),
                    ..RouteNote::default()
                };
                let expected = note.message.clone();
                sender.send((note, WriteFlags::default())).await.unwrap();
                assert_eq!(
                    receiver.try_next().await.unwrap().unwrap().message,
                    expected
                );
            }
            sender.close().await.unwrap();
            assert!(receiver.try_next().await.unwrap().is_none());
        });
    }
    drop(resume);
    block_on(blocked_call).unwrap();
    block_on(server.shutdown()).unwrap();
}

#[derive(Clone)]
struct EmptyService;

impl RouteGuide for EmptyService {}

#[test]
#[should_panic(expected = "has no handler")]
fn test_route_method_without_handler() {
    let env = Arc::new(EnvBuilder::new().cq_count(1).build());
    let routed_env = Arc::new(
        EnvBuilder::new()
            .cq_count(1)
            .set_non_listening(true)
            .build(),
    );
    let _ = ServerBuilder::new(env)
        .register_service(create_route_guide(EmptyService))
        .route_methods(routed_env, &["/routeguide.RouteGuide/Unknown"])
        .build();
}

#[test]
#[should_panic(expected = "is routed more than once")]
fn test_route_method_more_than_once() {
    let env = Arc::new(EnvBuilder::new().cq_count(1).build());
    let routed_env = Arc::new(
        EnvBuilder::new()
            .cq_count(1)
            .set_non_listening(true)
            .build(),
    );
    let _ = ServerBuilder::new(env)
        .register_service(create_route_guide(EmptyService))
        .route_methods(routed_env, &[GET_FEATURE, GET_FEATURE])
        .build();
}
