// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[path = "../log_util.rs"]
mod log_util;
mod util;

use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{io, thread};

use futures_channel::oneshot;
use futures_executor::block_on;
use futures_util::{FutureExt as _, SinkExt as _, TryFutureExt as _, TryStreamExt as _};
use grpcio::*;

use crate::util::*;
use grpcio_proto::example::route_guide::*;

#[derive(Clone)]
struct RouteGuideService {
    data: Arc<Vec<Feature>>,
    received_notes: Arc<Mutex<Vec<RouteNote>>>,
}

#[cfg(feature = "protobuf-codec")]
fn get_point<'a>(f: &'a Feature) -> &'a grpcio_proto::example::route_guide::Point {
    f.get_location()
}
#[cfg(feature = "protobufv3-codec")]
fn get_point<'a>(f: &'a Feature) -> &'a grpcio_proto::example::route_guide::Point {
    f.location.0.as_ref().unwrap()
}

impl RouteGuide for RouteGuideService {
    fn get_feature(&mut self, ctx: RpcContext<'_>, point: Point, sink: UnarySink<Feature>) {
        let data = self.data.clone();
        let resp = data
            .iter()
            .find(|f| same_point(get_point(f), &point))
            .map_or_else(Feature::default, ToOwned::to_owned);
        let f = sink
            .success(resp)
            .map_err(|e: grpcio::Error| error!("failed to handle getfeature request: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn list_features(
        &mut self,
        ctx: RpcContext<'_>,
        rect: Rectangle,
        mut resp: ServerStreamingSink<Feature>,
    ) {
        let data = self.data.clone();
        let features: Vec<_> = data
            .iter()
            .filter_map(move |f| {
                if fit_in(get_point(f), &rect) {
                    Some((f.to_owned(), WriteFlags::default()))
                } else {
                    None
                }
            })
            .collect();
        let f = async move {
            for feature in features {
                resp.send(feature).await?;
            }
            resp.close().await?;
            Ok(())
        }
        .map_err(|e: grpcio::Error| error!("failed to handle listfeatures request: {:?}", e))
        .map(|_| ());
        ctx.spawn(f)
    }

    fn record_route(
        &mut self,
        ctx: RpcContext<'_>,
        mut points: RequestStream<Point>,
        resp: ClientStreamingSink<RouteSummary>,
    ) {
        let data = self.data.clone();
        let timer = Instant::now();
        let f = async move {
            let mut summary = RouteSummary::default();
            let mut last = None;
            let mut dis = 0f64;
            while let Some(point) = points.try_next().await? {
                summary.point_count += 1;
                let valid_point = data
                    .iter()
                    .any(|f| !f.name.is_empty() && same_point(get_point(f), &point));
                if valid_point {
                    summary.feature_count += 1;
                }
                if let Some(last_point) = last {
                    dis += cal_distance(&last_point, &point);
                }
                last = Some(point);
            }
            summary.distance = dis as i32;
            let dur = timer.elapsed();
            summary.elapsed_time = dur.as_secs() as i32;
            resp.success(summary).await?;
            Ok(())
        }
        .map_err(|e: grpcio::Error| error!("failed to record route: {:?}", e))
        .map(|_| ());
        ctx.spawn(f)
    }

    fn route_chat(
        &mut self,
        ctx: RpcContext<'_>,
        mut notes: RequestStream<RouteNote>,
        mut resp: DuplexSink<RouteNote>,
    ) {
        let received_notes = self.received_notes.clone();
        let f = async move {
            while let Some(n) = notes.try_next().await? {
                let buffer = received_notes.lock().unwrap().clone();
                for note in buffer.iter() {
                    #[cfg(feature = "protobuf-codec")]
                    if same_point(n.get_location(), note.get_location()) {
                        resp.send((note.clone(), WriteFlags::default())).await?;
                    }
                    #[cfg(feature = "protobufv3-codec")]
                    if same_point(
                        n.location.0.as_ref().unwrap(),
                        note.location.0.as_ref().unwrap(),
                    ) {
                        resp.send((note.clone(), WriteFlags::default())).await?;
                    }
                }
                received_notes.lock().unwrap().push(n);
            }
            resp.close().await?;
            Ok(())
        }
        .map_err(|e: grpcio::Error| error!("failed to route chat: {:?}", e))
        .map(|_| ());
        ctx.spawn(f)
    }
}

fn main() {
    let addr = "127.0.0.1:50051";
    let _guard = log_util::init_log(None);
    let env = Arc::new(Environment::new(2));
    let instance = RouteGuideService {
        data: Arc::new(load_db()),
        received_notes: Arc::default(),
    };
    let service = create_route_guide(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .build()
        .unwrap();
    server
        .add_listening_port(addr, ServerCredentials::insecure())
        .unwrap();
    server.start();
    info!("listening on {addr}");
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    block_on(rx).unwrap();
    block_on(server.shutdown()).unwrap();
}
