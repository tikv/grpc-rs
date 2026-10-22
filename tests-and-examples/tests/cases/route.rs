// Copyright 2026 TiKV Project Authors. Licensed under Apache-2.0.

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use futures_executor::block_on;
use futures_util::{future, SinkExt as _, TryStreamExt as _};
use grpcio::{
    CallOption, ChannelBuilder, Client, ClientStreamingSink, EnvBuilder, GrpcSlice, Marshaller,
    MessageReader, Method, MethodType, RequestStream, RpcContext, ServerBuilder,
    ServerStreamingSink, ServiceBuilder, UnarySink, WriteFlags,
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

/// Also check codec affinity: moving only the handler would leave native
/// receive completions and later stream polls on the wrong CQ.
#[derive(Default)]
struct CqMessage {
    payload: Vec<u8>,
    decoded_on: Option<String>,
}

fn serialize_message(message: &CqMessage, buffer: &mut GrpcSlice) -> grpcio::Result<()> {
    if let Some(expected) = &message.decoded_on {
        assert_eq!(thread::current().name().unwrap(), expected);
    }
    *buffer = GrpcSlice::from(message.payload.as_slice());
    Ok(())
}

fn deserialize_message(mut reader: MessageReader) -> grpcio::Result<CqMessage> {
    let mut payload = Vec::new();
    reader
        .read_to_end(&mut payload)
        .map_err(|error| grpcio::Error::Codec(Box::new(error)))?;
    Ok(CqMessage {
        payload,
        decoded_on: Some(thread::current().name().unwrap().to_owned()),
    })
}

const ECHO: Method<CqMessage, CqMessage> = Method {
    ty: MethodType::Duplex,
    name: "/test.Cq/Echo",
    req_mar: Marshaller {
        ser: serialize_message,
        de: deserialize_message,
    },
    resp_mar: Marshaller {
        ser: serialize_message,
        de: deserialize_message,
    },
};

const UNARY_ECHO: Method<CqMessage, CqMessage> = Method {
    ty: MethodType::Unary,
    name: "/test.Cq/UnaryEcho",
    req_mar: Marshaller {
        ser: serialize_message,
        de: deserialize_message,
    },
    resp_mar: Marshaller {
        ser: serialize_message,
        de: deserialize_message,
    },
};

#[test]
fn test_routed_streams_distribute_call_bindings_and_keep_codec_affinity() {
    for cq_count in 1..=3 {
        // One listening CQ makes every connection start Core's scan at the
        // same index. All routed accept slots used to bind calls to routed-0.
        let env = Arc::new(EnvBuilder::new().cq_count(1).name_prefix("main").build());
        let routed_env = Arc::new(
            EnvBuilder::new()
                .cq_count(cq_count)
                .non_listening(true)
                .name_prefix("routed")
                .build(),
        );
        let (tx, rx) = mpsc::channel();
        let unary_tx = tx.clone();
        // This handler is Send but not Sync. Its cloned per-CQ state must
        // never be accessed by another CQ, including on the initial handoff.
        let owner = Cell::new(None);
        let service = ServiceBuilder::new()
            .add_duplex_streaming_handler(&ECHO, move |ctx, mut stream, mut sink| {
                let current = thread::current();
                if let Some(id) = owner.get() {
                    assert_eq!(id, current.id());
                }
                owner.set(Some(current.id()));
                let name = current.name().unwrap().to_owned();
                tx.send((name.clone(), ctx.peer())).unwrap();
                ctx.spawn(async move {
                    while let Ok(Some(message)) = stream.try_next().await {
                        assert_eq!(thread::current().name().unwrap(), name);
                        assert_eq!(message.decoded_on.as_ref().unwrap(), &name);
                        if sink.send((message, WriteFlags::default())).await.is_err() {
                            return;
                        }
                    }
                    let _ = sink.close().await;
                });
            })
            .add_unary_handler(&UNARY_ECHO, move |ctx, message, sink| {
                let name = thread::current().name().unwrap().to_owned();
                assert_eq!(message.decoded_on.as_ref().unwrap(), &name);
                unary_tx.send((name, ctx.peer())).unwrap();
                ctx.spawn(async move {
                    sink.success(message).await.unwrap();
                });
            })
            .build();
        let mut server = ServerBuilder::new(env.clone())
            .requests_slot_per_cq(2)
            .register_service(service)
            .route_methods(routed_env, &[ECHO.name, UNARY_ECHO.name])
            .bind("127.0.0.1", 0)
            .build()
            .unwrap();
        server.start();
        let address = format!("127.0.0.1:{}", server.bind_addrs().next().unwrap().1);
        let clients: Vec<_> = (0..cq_count)
            .map(|_| {
                Client::new(
                    ChannelBuilder::new(env.clone())
                        .use_local_subchannel_pool(true)
                        .connect(&address),
                )
            })
            .collect();
        let call_option = CallOption::default().timeout(Duration::from_secs(10));
        let open_streams = || {
            clients
                .iter()
                .map(|client| client.duplex_streaming(&ECHO, call_option.clone()).unwrap())
                .collect::<Vec<_>>()
        };

        // Exceed the initial accept capacity to exercise slot replenishment.
        // Each stream remains open while the others start and exchange data.
        for _ in 0..4 {
            let mut streams = open_streams();
            let mut queues = HashSet::new();
            let mut peers = HashSet::new();
            for _ in &streams {
                let (queue, peer) = rx.recv_timeout(Duration::from_secs(10)).unwrap();
                queues.insert(queue);
                peers.insert(peer);
            }
            assert_eq!(queues.len(), cq_count, "{:?}", queues);
            assert_eq!(
                peers.len(),
                cq_count,
                "clients must use separate connections"
            );
            block_on(async {
                for value in 0..4 {
                    for (sink, receiver) in &mut streams {
                        let message = CqMessage {
                            payload: vec![value; 4096],
                            ..CqMessage::default()
                        };
                        sink.send((message, WriteFlags::default())).await.unwrap();
                        let response = receiver.try_next().await.unwrap().unwrap();
                        assert_eq!(response.payload, vec![value; 4096]);
                    }
                }
                for (mut sink, mut receiver) in streams {
                    sink.close().await.unwrap();
                    assert!(receiver.try_next().await.unwrap().is_none());
                }
            });
        }

        let mut unary_queues = HashSet::new();
        for client in &clients {
            client
                .unary_call(&UNARY_ECHO, &CqMessage::default(), call_option.clone())
                .unwrap();
            unary_queues.insert(rx.recv_timeout(Duration::from_secs(10)).unwrap().0);
        }
        assert_eq!(unary_queues.len(), cq_count, "{:?}", unary_queues);

        // Closing the server with active routed calls also drains unused
        // accept slots whose notification CQ differs from their bound CQ.
        let streams = open_streams();
        for _ in &streams {
            rx.recv_timeout(Duration::from_secs(10)).unwrap();
        }
        let shutdown = server.shutdown();
        server.cancel_all_calls();
        block_on(shutdown).unwrap();
        drop(streams);
    }
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
