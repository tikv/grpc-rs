// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

use std::io::Read;
use std::mem::MaybeUninit;
use std::sync::Arc;

use futures_channel::mpsc;
use futures_executor::block_on;
use futures_timer::Delay;
use futures_util::{join, stream};
use futures_util::{
    FutureExt as _, SinkExt as _, StreamExt as _, TryFutureExt as _, TryStreamExt as _,
};
use grpcio::{
    ChannelBuilder, Client, ClientStreamingSink, DuplexSink, EnvBuilder, GrpcSlice, Marshaller,
    MessageReader, Method, MethodType, RequestStream, RpcContext, ServerBuilder, ServerCredentials,
    ServerStreamingSink, UnarySink, WriteFlags,
};
use grpcio_proto::example::route_guide::*;

const MESSAGE_NUM: i32 = 2000;
const CARDINALITY_METHOD_NAME: &str = "/grpc.testing.Cardinality/EmptyResponse";

#[derive(Clone)]
struct RouteGuideService {}

impl RouteGuide for RouteGuideService {
    fn get_feature(&mut self, _: RpcContext<'_>, _: Point, _: UnarySink<Feature>) {
        unimplemented!()
    }
    fn list_features(&mut self, _: RpcContext<'_>, _: Rectangle, _: ServerStreamingSink<Feature>) {
        unimplemented!()
    }
    fn record_route(
        &mut self,
        ctx: RpcContext<'_>,
        mut points: RequestStream<Point>,
        resp: ClientStreamingSink<RouteSummary>,
    ) {
        let f = async move {
            let mut summary = RouteSummary::default();
            let mut current_num = 0;
            while let Some(point) = points.try_next().await? {
                assert_eq!(point.longitude, current_num, "messages sequence is wrong");
                current_num += 1;
                summary.point_count += 1;
                // Send a reply message after receiving a limited number of messages, which
                // can be used to test the correctness under different buffer strategies.
                if current_num >= MESSAGE_NUM {
                    break;
                }
            }
            resp.success(summary).await?;
            Ok(())
        }
        .map_err(|e: grpcio::Error| panic!("server got error: {:?}", e))
        .map(|_| ());
        ctx.spawn(f)
    }

    fn route_chat(
        &mut self,
        _: RpcContext<'_>,
        _: RequestStream<RouteNote>,
        _: DuplexSink<RouteNote>,
    ) {
        unimplemented!()
    }
}

macro_rules! assert_finish {
    ($res:expr) => {
        match $res {
            // RouteGuide returns early, so `RpcFinished` is possible returned.
            Ok(()) | Err(grpcio::Error::RpcFinished(None)) => (),
            Err(e) => panic!("unexpected error {:?}", e),
        }
    };
}

fn serialize_bytes(t: &Vec<u8>, buf: &mut GrpcSlice) -> grpcio::Result<()> {
    unsafe {
        let bytes = buf.realloc(t.len());
        let bytes = &mut *(bytes as *mut [MaybeUninit<u8>] as *mut [u8]);
        bytes.copy_from_slice(t);
    }
    Ok(())
}

fn deserialize_bytes(mut reader: MessageReader) -> grpcio::Result<Vec<u8>> {
    let mut data = vec![];
    reader.read_to_end(&mut data).unwrap();
    Ok(data)
}

fn bytes_marshaller() -> Marshaller<Vec<u8>> {
    Marshaller {
        ser: serialize_bytes,
        de: deserialize_bytes,
    }
}

fn cardinality_method(ty: MethodType) -> Method<Vec<u8>, Vec<u8>> {
    Method {
        ty,
        name: CARDINALITY_METHOD_NAME,
        req_mar: bytes_marshaller(),
        resp_mar: bytes_marshaller(),
    }
}

fn assert_rpc_failure_code(err: grpcio::Error, code: grpcio::RpcStatusCode) {
    match err {
        grpcio::Error::RpcFailure(status) => assert_eq!(status.code(), code),
        e => panic!("unexpected error: {e:?}"),
    }
}

#[test]
fn test_client_send_all() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_route_guide(RouteGuideService {});
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = RouteGuideClient::new(ch);

    let exec_test_f = async move {
        // Test for send all disable batch
        let (mut sink, receiver) = client.record_route().unwrap();
        let mut send_data = vec![];
        for i in 0..MESSAGE_NUM {
            let mut p = Point::default();
            p.longitude = i;
            send_data.push(p);
        }
        let send_stream = stream::iter(send_data);
        assert_finish!(
            sink.send_all(&mut send_stream.map(move |item| Ok((item, WriteFlags::default()))))
                .await
        );
        let summary = receiver.await.unwrap();
        assert_eq!(summary.point_count, MESSAGE_NUM);

        // Test for send all enable batch
        let (mut sink, receiver) = client.record_route().unwrap();
        let mut send_data = vec![];
        for i in 0..MESSAGE_NUM {
            let mut p = Point::default();
            p.longitude = i;
            send_data.push(p);
        }
        let send_stream = stream::iter(send_data);
        sink.enhance_batch(true);
        assert_finish!(
            sink.send_all(&mut send_stream.map(move |item| Ok((item, WriteFlags::default()))))
                .await
        );
        let summary = receiver.await.unwrap();
        assert_eq!(summary.point_count, MESSAGE_NUM);

        // Test for send all and all buffer hints are true
        let (mut sink, receiver) = client.record_route().unwrap();
        let mut send_data = vec![];
        for i in 0..MESSAGE_NUM {
            let mut p = Point::default();
            p.longitude = i;
            send_data.push(p);
        }
        let send_stream = stream::iter(send_data);
        sink.enhance_batch(false);
        sink.send_all(
            &mut send_stream.map(move |item| Ok((item, WriteFlags::default().buffer_hint(true)))),
        )
        .await
        .unwrap();
        // The following code is to test that when all msgs are set to be buffered, the msgs
        // should be stored in the buffer until `sink.close()` is called.
        let (mut tx, mut rx) = mpsc::channel(1);
        let close_sink_task = async move {
            Delay::new(std::time::Duration::from_secs(1)).await;
            rx.try_next().unwrap_err();
            sink.close().await.unwrap();
            Delay::new(std::time::Duration::from_secs(1)).await;
            rx.try_next().unwrap();
        };
        let recv_msg_task = async move {
            let summary = receiver.await.unwrap();
            tx.send(()).await.unwrap();
            assert_eq!(summary.point_count, MESSAGE_NUM);
        };
        join!(recv_msg_task, close_sink_task);
    };
    block_on(exec_test_f);
}

#[test]
fn test_unary_empty_response_returns_unimplemented() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = grpcio::ServiceBuilder::new()
        .add_server_streaming_handler(
            &cardinality_method(MethodType::ServerStreaming),
            |ctx: RpcContext<'_>, _: Vec<u8>, mut sink: ServerStreamingSink<Vec<u8>>| {
                ctx.spawn(async move { sink.close().await.unwrap() });
            },
        )
        .build();
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();

    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = Client::new(ch);
    let err = client
        .unary_call(
            &cardinality_method(MethodType::Unary),
            &vec![1, 2, 3],
            grpcio::CallOption::default(),
        )
        .unwrap_err();

    assert_rpc_failure_code(err, grpcio::RpcStatusCode::UNIMPLEMENTED);
}

#[test]
fn test_client_streaming_empty_response_returns_unimplemented() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = grpcio::ServiceBuilder::new()
        .add_duplex_streaming_handler(
            &cardinality_method(MethodType::Duplex),
            |ctx: RpcContext<'_>,
             mut reqs: RequestStream<Vec<u8>>,
             mut sink: DuplexSink<Vec<u8>>| {
                ctx.spawn(async move {
                    while reqs.try_next().await.unwrap().is_some() {}
                    sink.close().await.unwrap();
                });
            },
        )
        .build();
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();

    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = Client::new(ch);
    let (mut sink, receiver) = client
        .client_streaming(
            &cardinality_method(MethodType::ClientStreaming),
            grpcio::CallOption::default(),
        )
        .unwrap();

    let err = block_on(async move {
        sink.send((vec![1, 2, 3], WriteFlags::default())).await?;
        sink.close().await?;
        receiver.await
    })
    .unwrap_err();

    assert_rpc_failure_code(err, grpcio::RpcStatusCode::UNIMPLEMENTED);
}
