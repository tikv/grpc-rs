// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::Arc;

use futures::executor::block_on;
use futures::prelude::*;
use futures::sink::SinkExt;
use grpcio::{
    ChannelBuilder, ClientStreamingSink, DuplexSink, EnvBuilder, RequestStream, RpcContext,
    ServerBuilder, ServerStreamingSink, UnarySink, WriteFlags,
};
use grpcio_proto::example::route_guide::*;

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
                assert_eq!(
                    point.get_longitude(),
                    current_num,
                    "messages sequence is wrong"
                );
                current_num += 1;
                summary.point_count += 1;
            }
            resp.success(summary).await?;
            Ok(())
        }
        .map_err(|_: grpcio::Error| panic!("server got error"))
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

#[test]
fn test_client_send_all() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_route_guide(RouteGuideService {});
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;
    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = RouteGuideClient::new(ch);

    let exec_test_f = async move {
        // test for send all
        let (mut sink, receiver) = client.record_route().unwrap();
        let mut send_data = vec![];
        for i in 0..3000 {
            let mut p = Point::default();
            p.set_longitude(i);
            send_data.push(p);
        }
        let send_stream = futures::stream::iter(send_data);
        sink.send_all(&mut send_stream.map(move |item| Ok((item, WriteFlags::default()))))
            .await
            .unwrap();
        sink.close().await.unwrap();
        let summary = receiver.await.unwrap();
        assert_eq!(summary.get_point_count(), 3000);
        // test for send all enable batch
        let (mut sink, receiver) = client.record_route().unwrap();
        let mut send_data = vec![];
        for i in 0..3000 {
            let mut p = Point::default();
            p.set_longitude(i);
            send_data.push(p);
        }
        let send_stream = futures::stream::iter(send_data);
        sink.enhance_batch(true);
        sink.send_all(&mut send_stream.map(move |item| Ok((item, WriteFlags::default()))))
            .await
            .unwrap();
        sink.close().await.unwrap();
        let summary = receiver.await.unwrap();
        assert_eq!(summary.get_point_count(), 3000);
        // test for send all and buffer hint is true
        let (mut sink, receiver) = client.record_route().unwrap();
        let mut send_data = vec![];
        for i in 0..3000 {
            let mut p = Point::default();
            p.set_longitude(i);
            send_data.push(p);
        }
        let send_stream = futures::stream::iter(send_data);
        sink.enhance_batch(false);
        sink.send_all(
            &mut send_stream.map(move |item| Ok((item, WriteFlags::default().buffer_hint(true)))),
        )
        .await
        .unwrap();
        sink.close().await.unwrap();
        let summary = receiver.await.unwrap();
        assert_eq!(summary.get_point_count(), 3000);
    };
    block_on(exec_test_f);
}
