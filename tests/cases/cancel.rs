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

use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use futures::*;
use grpcio::*;
use grpcio_proto::example::route_guide::*;
use grpcio_proto::example::route_guide_grpc::*;

type Handler<T> = Arc<Mutex<Option<Box<T>>>>;
type BoxFuture = Box<Future<Item = (), Error = ()> + Send + 'static>;
type RecordRouteHandler =
    Handler<Fn(RequestStream<Point>, ClientStreamingSink<RouteSummary>) -> BoxFuture + Send>;
type RouteChatHandler =
    Handler<Fn(RequestStream<RouteNote>, DuplexSink<RouteNote>) -> BoxFuture + Send>;

#[derive(Clone)]
struct CancelService {
    record_route_handler: RecordRouteHandler,
    route_chat_handler: RouteChatHandler,
}

impl CancelService {
    fn new() -> CancelService {
        CancelService {
            record_route_handler: Arc::new(Mutex::new(None)),
            route_chat_handler: Arc::new(Mutex::new(None)),
        }
    }
}

impl RouteGuide for CancelService {
    fn get_feature(&self, _: RpcContext, _: Point, sink: UnarySink<Feature>) {
        // Drop the sink, client should receive Cancelled.
        drop(sink);
    }

    fn list_features(&self, _: RpcContext, _: Rectangle, sink: ServerStreamingSink<Feature>) {
        // Drop the sink, client should receive Cancelled.
        drop(sink);
    }

    fn record_route(
        &self,
        ctx: RpcContext,
        stream: RequestStream<Point>,
        sink: ClientStreamingSink<RouteSummary>,
    ) {
        let handler = self.record_route_handler.lock().unwrap();
        if handler.is_some() {
            let f = (handler.as_ref().unwrap())(stream, sink);
            ctx.spawn(f);
        }
    }

    fn route_chat(
        &self,
        ctx: RpcContext,
        stream: RequestStream<RouteNote>,
        sink: DuplexSink<RouteNote>,
    ) {
        let handler = self.route_chat_handler.lock().unwrap();
        if handler.is_some() {
            let f = (handler.as_ref().unwrap())(stream, sink);
            ctx.spawn(f);
        }
    }
}

fn check_cancel<S, T>(rx: S)
where
    S: Stream<Item = T, Error = Error>,
    T: Debug,
{
    match rx.into_future().wait() {
        Err((Error::RpcFailure(s), _)) => assert_eq!(s.status, RpcStatusCode::Cancelled),
        Err((e, _)) => panic!("expected cancel, but got: {:?}", e),
        Ok((r, _)) => panic!("expected error, but got: {:?}", r),
    }
}

fn run_suite() -> (CancelService, RouteGuideClient, Server) {
    let env = Arc::new(EnvBuilder::new().build());
    let service = CancelService::new();
    let mut server = ServerBuilder::new(env.clone())
        .register_service(create_route_guide(service.clone()))
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs()[0].1;
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = RouteGuideClient::new(ch);
    (service, client, server)
}

#[test]
fn test_client_cancel_on_dropping() {
    let (service, client, _server) = run_suite();

    // Client streaming.
    {
        let mut record_route_handler = service.record_route_handler.lock().unwrap();
        *record_route_handler = Some(Box::new(|stream, sink| {
            // Start the call and keep the stream and the sink.
            let f = stream.for_each(|_| Ok(())).then(|_| {
                let _sink = sink;
                Ok(())
            });
            Box::new(f)
        }));
    }
    let (tx, rx) = client.record_route().unwrap();
    drop(tx);
    check_cancel(rx.into_stream());

    // Duplex streaming.
    {
        let mut record_route_handler = service.record_route_handler.lock().unwrap();
        *record_route_handler = Some(Box::new(|stream, sink| {
            // Start the call and keep the stream and the sink.
            let f = stream.for_each(|_| Ok(())).then(|_| {
                let _sink = sink;
                Ok(())
            });
            Box::new(f)
        }));
    }
    let (tx, rx) = client.route_chat().unwrap();
    drop(tx);
    check_cancel(rx);
}

#[test]
fn test_server_cancel_on_dropping() {
    let (service, client, _server) = run_suite();

    // Unary
    let rx = client.get_feature_async(&Default::default()).unwrap();
    check_cancel(rx.into_stream());

    // Client streaming
    {
        // let timer = service.timer.clone();
        let mut record_route_handler = service.record_route_handler.lock().unwrap();
        *record_route_handler = Some(Box::new(|stream, sink| {
            // let sleep = timer.sleep(Duration::from_millis(500));
            // Start the call, keep the stream and drop the sink.
            let f = stream
                .for_each(|_| Ok(()))
                .join(future::result(Ok(())).map(move |_| {
                    drop(sink);
                }))
                .then(|_| Ok(()));
            Box::new(f)
        }));
    }
    let (_tx, rx) = client.record_route().unwrap();
    check_cancel(rx.into_stream());

    // Server streaming
    let rx = client.list_features(&Default::default()).unwrap();
    check_cancel(rx);

    // Duplex streaming
    {
        let mut route_chat_handler = service.route_chat_handler.lock().unwrap();
        *route_chat_handler = Some(Box::new(|stream, sink| {
            // Start the call, keep the stream and drop the sink.
            let f = stream
                .for_each(|_| Ok(()))
                .join(future::result(Ok(())).map(move |_| {
                    drop(sink);
                }))
                .then(|_| Ok(()));
            Box::new(f)
        }));
    }
    let (_tx, rx) = client.route_chat().unwrap();
    check_cancel(rx);
}
