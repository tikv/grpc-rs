// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::mpsc as std_mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use futures::sync::mpsc;
use futures::{future, stream as streams, Async, Future, Poll, Sink, Stream};
use grpcio::*;
use grpcio_proto::example::route_guide::*;
use grpcio_proto::example::route_guide_grpc::*;

type Handler<T> = Arc<Mutex<Option<Box<T>>>>;
type BoxFuture = Box<dyn Future<Item = (), Error = ()> + Send + 'static>;
type RecordRouteHandler =
    Handler<dyn Fn(RequestStream<Point>, ClientStreamingSink<RouteSummary>) -> BoxFuture + Send>;
type RouteChatHandler =
    Handler<dyn Fn(RequestStream<RouteNote>, DuplexSink<RouteNote>) -> BoxFuture + Send>;

#[derive(Clone)]
struct CancelService {
    list_feature_listener: Arc<Mutex<Option<std_mpsc::Sender<()>>>>,
    record_route_handler: RecordRouteHandler,
    route_chat_handler: RouteChatHandler,
}

impl CancelService {
    fn new() -> CancelService {
        CancelService {
            list_feature_listener: Arc::default(),
            record_route_handler: Arc::new(Mutex::new(None)),
            route_chat_handler: Arc::new(Mutex::new(None)),
        }
    }
}

impl RouteGuide for CancelService {
    fn get_feature(&mut self, _: RpcContext<'_>, _: Point, sink: UnarySink<Feature>) {
        // Drop the sink, client should receive Cancelled.
        drop(sink);
    }

    fn list_features(
        &mut self,
        ctx: RpcContext<'_>,
        _: Rectangle,
        sink: ServerStreamingSink<Feature>,
    ) {
        // Drop the sink, client should receive Cancelled.
        let listener = match self.list_feature_listener.lock().unwrap().take() {
            Some(l) => l,
            None => {
                drop(sink);
                return;
            }
        };
        let (tx, rx) = mpsc::unbounded();

        thread::spawn(move || loop {
            tx.unbounded_send(1).unwrap();
            thread::sleep(Duration::from_secs(5));
        });

        let f = rx
            .map(|_| {
                let f = Feature::default();
                (f, WriteFlags::default())
            })
            .forward(sink.sink_map_err(|_| ()))
            .map(|_| ())
            .map_err(|_| ())
            .then(move |_| {
                let _ = listener.send(());
                Ok(())
            });

        ctx.spawn(f);
    }

    fn record_route(
        &mut self,
        ctx: RpcContext<'_>,
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
        ctx: RpcContext<'_>,
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

fn check_cancel<S, T>(rx: S, sink: bool)
where
    S: Stream<Item = T, Error = Error>,
{
    match rx.into_future().wait() {
        Err((Error::RpcFailure(s), _)) | Err((Error::RpcFinished(Some(s)), _)) => {
            assert_eq!(s.status, RpcStatusCode::CANCELLED)
        }
        Err((Error::RemoteStopped, _)) if sink => return,
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
    let port = server.bind_addrs().next().unwrap().1;
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
    check_cancel(rx.into_stream(), false);

    let (tx, rx) = client.record_route().unwrap();
    drop(rx);
    check_cancel(tx.send(Default::default()).into_stream(), true);

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
    check_cancel(rx, false);

    let (tx, rx) = client.route_chat().unwrap();
    drop(rx);
    check_cancel(tx.send(Default::default()).into_stream(), true);
}

#[test]
fn test_server_cancel_on_dropping() {
    let (service, client, _server) = prepare_suite();

    // Unary
    let rx = client.get_feature_async(&Default::default()).unwrap();
    check_cancel(rx.into_stream(), false);

    // Server streaming
    let rx = client.list_features(&Default::default()).unwrap();
    check_cancel(rx, false);

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
    check_cancel(rx.into_stream(), false);

    // Client streaming, drop stream.
    *service.record_route_handler.lock().unwrap() =
        Some(Box::new(|stream, sink| drop_stream(stream, sink)));
    let (_tx, rx) = client.record_route().unwrap();
    check_cancel(rx.into_stream(), false);

    // Duplex streaming, drop sink.
    *service.route_chat_handler.lock().unwrap() =
        Some(Box::new(|stream, sink| drop_sink(stream, sink)));
    let (_tx, rx) = client.route_chat().unwrap();
    check_cancel(rx, false);

    // Duplex streaming, drop stream.
    *service.route_chat_handler.lock().unwrap() =
        Some(Box::new(|stream, sink| drop_stream(stream, sink)));
    let (_tx, rx) = client.route_chat().unwrap();
    check_cancel(rx, false);
}

#[test]
fn test_early_exit() {
    let (service, client, _server) = prepare_suite();
    let (tx, rx) = std_mpsc::channel();
    *service.list_feature_listener.lock().unwrap() = Some(tx);

    let rect = Rectangle::default();
    let l = client.list_features(&rect).unwrap();
    let f = l.into_future();
    match f.wait() {
        Ok((Some(_), _)) => {}
        Ok((None, _)) => panic!("should have result"),
        Err((e, _)) => panic!("unexpected error {:?}", e),
    };

    rx.recv_timeout(Duration::from_secs(1)).unwrap();
}
