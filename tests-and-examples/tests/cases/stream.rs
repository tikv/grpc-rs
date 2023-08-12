// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::Arc;

use futures_channel::mpsc;
use futures_executor::block_on;
use futures_timer::Delay;
use futures_util::{join, stream};
use futures_util::{
    FutureExt as _, SinkExt as _, StreamExt as _, TryFutureExt as _, TryStreamExt as _,
};
use grpcio::{
    ChannelBuilder, ClientStreamingSink, DuplexSink, EnvBuilder, RequestStream, RpcContext,
    ServerBuilder, ServerCredentials, ServerStreamingSink, UnarySink, WriteFlags,
};
use grpcio_proto::example::route_guide::*;

const MESSAGE_NUM: i32 = 2000;

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
