// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc as std_mpsc;
use std::sync::{Arc, Mutex};
use std::task::Poll;
use std::thread;
use std::time::Duration;

use futures_channel::mpsc;
use futures_executor::block_on;
use futures_util::{future, stream, Stream};
use futures_util::{FutureExt as _, SinkExt as _, StreamExt as _, TryStreamExt as _};
use grpcio::*;
use grpcio_proto::example::route_guide::*;

type Handler<T> = Arc<Mutex<Option<Box<T>>>>;
type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
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
        mut sink: ServerStreamingSink<Feature>,
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
            if tx.unbounded_send(1u64).is_ok() {
                thread::sleep(Duration::from_secs(5));
            } else {
                break;
            }
        });

        let f = async move {
            sink.send_all(&mut rx.map(|_| Ok((Feature::default(), WriteFlags::default()))))
                .await?;
            sink.close().await?;
            Ok(())
        }
        .map(move |_: Result<()>| {
            let _ = listener.send(());
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

fn check_cancel<S, T>(mut rx: S, sink: bool)
where
    S: Stream<Item = Result<T>> + Unpin,
{
    match block_on(rx.try_next()) {
        Err(Error::RpcFailure(s)) | Err(Error::RpcFinished(Some(s))) => {
            assert_eq!(s.code(), RpcStatusCode::CANCELLED)
        }
        Err(Error::RemoteStopped) if sink => (),
        Err(e) => panic!("expected cancel, but got: {:?}", e),
        Ok(_) => panic!("expected error, but got: Ok(_)"),
    }
}

fn prepare_suite() -> (CancelService, RouteGuideClient, Server) {
    let env = Arc::new(EnvBuilder::new().build());
    let service = CancelService::new();
    let mut server = ServerBuilder::new(env.clone())
        .register_service(create_route_guide(service.clone()))
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = RouteGuideClient::new(ch);
    (service, client, server)
}

#[test]
fn test_client_cancel_on_dropping() {
    let (service, client, _server) = prepare_suite();

    // Client streaming.
    *service.record_route_handler.lock().unwrap() = Some(Box::new(move |stream, sink| {
        // Start the call and keep the stream and the sink.
        let f = stream
            .try_for_each(move |_| future::ready(Ok(())))
            .then(|_| {
                let _sink = sink;
                future::ready(())
            });
        Box::pin(f)
    }));
    let (tx, rx) = client.record_route().unwrap();
    drop(tx);
    check_cancel(rx.into_stream(), false);

    let (mut tx, rx) = client.record_route().unwrap();
    drop(rx);
    check_cancel(tx.send(Default::default()).into_stream(), true);

    // Duplex streaming.
    *service.record_route_handler.lock().unwrap() = Some(Box::new(|stream, sink| {
        // Start the call and keep the stream and the sink.
        let f = stream.try_for_each(|_| future::ready(Ok(()))).then(|_| {
            let _sink = sink;
            future::ready(())
        });
        Box::pin(f)
    }));
    let (tx, rx) = client.route_chat().unwrap();
    drop(tx);
    check_cancel(rx, false);

    let (mut tx, rx) = client.route_chat().unwrap();
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
        S: Stream<Item = Result<R>> + Send + 'static,
        R: Send + 'static,
        T: Send + 'static,
    {
        let f = async move {
            futures_util::join!(
                stream.try_for_each(|_| future::ready(Ok(()))).map(|_| ()),
                async move {
                    drop(sink);
                }
            );
        };
        Box::pin(f)
    }

    // Start the call, drop the stream and keep the sink.
    fn drop_stream<S, R, T>(stream: S, sink: T) -> BoxFuture
    where
        S: Stream<Item = Result<R>> + Unpin + Send + 'static,
        R: Send + 'static,
        T: Send + 'static,
    {
        let mut stream = Some(stream);
        let f = stream::poll_fn(move |cx| -> Poll<Option<Result<()>>> {
            if stream.is_some() {
                let s = stream.as_mut().unwrap();
                // start the call.
                let _ = Pin::new(s).poll_next(cx);
            }
            // drop the stream.
            stream.take();
            Poll::Pending
        });
        // It never resolves.
        let f = f.try_for_each(|_| future::ready(Ok(()))).then(move |_| {
            let _sink = sink;
            future::ready(())
        });
        Box::pin(f)
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
    let mut l = client.list_features(&rect).unwrap();
    match block_on(l.try_next()) {
        Ok(Some(_)) => drop(l),
        Ok(None) => panic!("should have result"),
        Err(e) => panic!("unexpected error {:?}", e),
    };

    rx.recv_timeout(Duration::from_secs(1)).unwrap();
}
