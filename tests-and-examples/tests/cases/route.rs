// Copyright 2026 TiKV Project Authors. Licensed under Apache-2.0.

use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use std::thread;

use futures_executor::block_on;
use futures_util::{future, SinkExt as _, TryStreamExt as _};
use grpcio::{
    ChannelBuilder, ClientStreamingSink, EnvBuilder, RequestStream, RpcContext, ServerBuilder,
    ServerStreamingSink, UnarySink, WriteFlags,
};
use grpcio_proto::example::route_guide::*;

const GET_FEATURE: &str = "/routeguide.RouteGuide/GetFeature";
const LIST_FEATURES: &str = "/routeguide.RouteGuide/ListFeatures";
const RECORD_ROUTE: &str = "/routeguide.RouteGuide/RecordRoute";

/// A service that reports the method and the thread of every call it handles.
#[derive(Clone)]
struct CallRecorder {
    tx: mpsc::Sender<(Vec<u8>, String)>,
}

impl CallRecorder {
    fn record(&self, ctx: &RpcContext<'_>) {
        let thread_name = thread::current().name().unwrap_or_default().to_owned();
        self.tx.send((ctx.method().to_vec(), thread_name)).unwrap();
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
            .non_listening(true)
            .name_prefix("routed")
            .build(),
    );
    let (tx, rx) = mpsc::channel();
    let mut server = ServerBuilder::new(env.clone())
        .register_service(create_route_guide(CallRecorder { tx }))
        .route_methods(routed_env, &[GET_FEATURE, RECORD_ROUTE])
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = RouteGuideClient::new(ch);

    // Routed unary and client streaming methods.
    client.get_feature(&Point::default()).unwrap();
    let (mut sink, receiver) = client.record_route().unwrap();
    block_on(async {
        sink.send((Point::default(), WriteFlags::default()))
            .await
            .unwrap();
        sink.close().await.unwrap();
        receiver.await.unwrap();
    });
    // A server streaming method that is not routed.
    let mut features = client.list_features(&Rectangle::default()).unwrap();
    block_on(async { while features.try_next().await.unwrap().is_some() {} });

    let calls: HashMap<Vec<u8>, String> = rx.try_iter().collect();
    assert_eq!(calls.len(), 3, "{:?}", calls);
    let thread_of = |method: &str| calls[method.as_bytes()].as_str();
    assert!(thread_of(GET_FEATURE).starts_with("routed"), "{:?}", calls);
    assert!(thread_of(RECORD_ROUTE).starts_with("routed"), "{:?}", calls);
    assert!(thread_of(LIST_FEATURES).starts_with("main"), "{:?}", calls);
    block_on(server.shutdown()).unwrap();
}

#[derive(Clone)]
struct EmptyService;

impl RouteGuide for EmptyService {}

#[test]
#[should_panic(expected = "has no handler")]
fn test_route_method_without_handler() {
    let env = Arc::new(EnvBuilder::new().cq_count(1).build());
    let routed_env = Arc::new(EnvBuilder::new().cq_count(1).non_listening(true).build());
    let _ = ServerBuilder::new(env)
        .register_service(create_route_guide(EmptyService))
        .route_methods(routed_env, &["/routeguide.RouteGuide/Unknown"])
        .build();
}
