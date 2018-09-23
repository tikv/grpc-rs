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

use std::sync::{Arc, Mutex};

use futures::{future, stream as streams, Async, Future, Poll, Sink, Stream};
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
    fn get_feature(&mut self, _: RpcContext, _: Point, sink: UnarySink<Feature>) {
        // Drop the sink, client should receive Cancelled.
        drop(sink);
    }

    fn list_features(&mut self, _: RpcContext, _: Rectangle, sink: ServerStreamingSink<Feature>) {
        // Drop the sink, client should receive Cancelled.
        drop(sink);
    }

    fn record_route(
        &mut self,
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
        &mut self,
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
{
    match rx.into_future().wait() {
        Err((Error::RpcFailure(s), _)) | Err((Error::RpcFinished(Some(s)), _)) => {
            assert_eq!(s.status, RpcStatusCode::Cancelled)
        }
        Err((e, _)) => panic!("expected cancel, but got: {:?}", e),
        Ok(_) => panic!("expected error, but got: Ok(_)"),
    }
}

fn prepare_suite() -> (CancelService, RouteGuideClient, Server) {
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
    let (service, client, _server) = prepare_suite();

    // Client streaming.
    *service.record_route_handler.lock().unwrap() = Some(Box::new(|stream, sink| {
        // Start the call and keep the stream and the sink.
        let f = stream.for_each(|_| Ok(())).then(|_| {
            let _sink = sink;
            Ok(())
        });
        Box::new(f)
    }));
    let (tx, rx) = client.record_route().unwrap();
    drop(tx);
    check_cancel(rx.into_stream());

    let (tx, rx) = client.record_route().unwrap();
    drop(rx);
    check_cancel(tx.send(Default::default()).into_stream());

    // Duplex streaming.
    *service.record_route_handler.lock().unwrap() = Some(Box::new(|stream, sink| {
        // Start the call and keep the stream and the sink.
        let f = stream.for_each(|_| Ok(())).then(|_| {
            let _sink = sink;
            Ok(())
        });
        Box::new(f)
    }));
    let (tx, rx) = client.route_chat().unwrap();
    drop(tx);
    check_cancel(rx);

    let (tx, rx) = client.route_chat().unwrap();
    drop(rx);
    check_cancel(tx.send(Default::default()).into_stream());
}

#[test]
fn test_server_cancel_on_dropping() {
    let (service, client, _server) = prepare_suite();

    // Unary
    let rx = client.get_feature_async(&Default::default()).unwrap();
    check_cancel(rx.into_stream());

    // Server streaming
    let rx = client.list_features(&Default::default()).unwrap();
    check_cancel(rx);

    // Start the call, keep the stream and drop the sink.
    fn drop_sink<S, R, T>(stream: S, sink: T) -> BoxFuture
    where
        S: Stream<Item = R, Error = Error> + Send + 'static,
        R: Send + 'static,
        T: Send + 'static,
    {
        let f = stream
            .for_each(|_| Ok(()))
            .join(future::result(Ok(())).map(move |_| {
                drop(sink);
            }))
            .then(|_| Ok(()));
        Box::new(f)
    }

    // Start the call, drop the stream and keep the sink.
    fn drop_stream<S, R, T>(stream: S, sink: T) -> BoxFuture
    where
        S: Stream<Item = R, Error = Error> + Send + 'static,
        R: Send + 'static,
        T: Send + 'static,
    {
        let mut stream = Some(stream);
        let f = streams::poll_fn(move || -> Poll<Option<()>, ()> {
            if stream.is_some() {
                let s = stream.as_mut().unwrap();
                // start the call.
                let _ = s.poll();
            }
            // drop the stream.
            stream.take();
            Ok(Async::NotReady)
        });
        // It never resolves.
        let f = f.for_each(|_| Ok(())).then(move |_| {
            let _sink = sink;
            Ok(())
        });
        Box::new(f)
    }

    // Client streaming, drop sink.
    *service.record_route_handler.lock().unwrap() =
        Some(Box::new(|stream, sink| drop_sink(stream, sink)));
    let (_tx, rx) = client.record_route().unwrap();
    check_cancel(rx.into_stream());

    // Client streaming, drop stream.
    *service.record_route_handler.lock().unwrap() =
        Some(Box::new(|stream, sink| drop_stream(stream, sink)));
    let (_tx, rx) = client.record_route().unwrap();
    check_cancel(rx.into_stream());

    // Duplex streaming, drop sink.
    *service.route_chat_handler.lock().unwrap() =
        Some(Box::new(|stream, sink| drop_sink(stream, sink)));
    let (_tx, rx) = client.route_chat().unwrap();
    check_cancel(rx);

    // Duplex streaming, drop stream.
    *service.route_chat_handler.lock().unwrap() =
        Some(Box::new(|stream, sink| drop_stream(stream, sink)));
    let (_tx, rx) = client.route_chat().unwrap();
    check_cancel(rx);
}
