// Copyright 2017 PingCAP, Inc.
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

extern crate futures;
extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[path = "../log_util.rs"]
mod log_util;
mod util;

use std::io::Read;
use std::sync::Arc;
use std::time::Instant;
use std::{io, thread};

use futures::sync::oneshot;
use futures::*;
use grpcio::*;

use grpcio_proto::example::route_guide::*;
use grpcio_proto::example::route_guide_grpc::{self, RouteGuide};
use util::*;

#[derive(Clone)]
struct RouteGuideService {
    data: Arc<Vec<Feature>>,
}

impl RouteGuide for RouteGuideService {
    fn get_feature(&mut self, ctx: RpcContext, point: Point, sink: UnarySink<Feature>) {
        let data = self.data.clone();
        let resp = data
            .iter()
            .find(|f| same_point(f.get_location(), &point))
            .map_or_else(Feature::new, ToOwned::to_owned);
        let f = sink
            .success(resp)
            .map_err(|e| error!("failed to handle getfeature request: {:?}", e));
        ctx.spawn(f)
    }

    fn list_features(
        &mut self,
        ctx: RpcContext,
        rect: Rectangle,
        resp: ServerStreamingSink<Feature>,
    ) {
        let data = self.data.clone();
        let features: Vec<_> = data
            .iter()
            .filter_map(move |f| {
                if fit_in(f.get_location(), &rect) {
                    Some((f.to_owned(), WriteFlags::default()))
                } else {
                    None
                }
            }).collect();
        let f = resp
            .send_all(stream::iter_ok::<_, Error>(features))
            .map(|_| {})
            .map_err(|e| error!("failed to handle listfeatures request: {:?}", e));
        ctx.spawn(f)
    }

    fn record_route(
        &mut self,
        ctx: RpcContext,
        points: RequestStream<Point>,
        resp: ClientStreamingSink<RouteSummary>,
    ) {
        let data = self.data.clone();
        let timer = Instant::now();
        let f = points
            .fold(
                (None, 0f64, RouteSummary::new()),
                move |(last, mut dis, mut summary), point| {
                    let total_count = summary.get_point_count();
                    summary.set_point_count(total_count + 1);
                    let valid_point = data
                        .iter()
                        .any(|f| !f.get_name().is_empty() && same_point(f.get_location(), &point));
                    if valid_point {
                        let feature_count = summary.get_feature_count();
                        summary.set_feature_count(feature_count + 1);
                    }
                    if let Some(last_point) = last {
                        dis += cal_distance(&last_point, &point);
                    }
                    Ok((Some(point), dis, summary)) as Result<_>
                },
            ).and_then(move |(_, dis, mut s)| {
                s.set_distance(dis as i32);
                let dur = timer.elapsed();
                s.set_elapsed_time(dur.as_secs() as i32);
                resp.success(s)
            }).map_err(|e| error!("failed to record route: {:?}", e));
        ctx.spawn(f)
    }

    fn route_chat(
        &mut self,
        ctx: RpcContext,
        notes: RequestStream<RouteNote>,
        resp: DuplexSink<RouteNote>,
    ) {
        let mut buffer: Vec<RouteNote> = Vec::new();
        let to_send = notes
            .map(move |note| {
                let to_prints: Vec<_> = buffer
                    .iter()
                    .filter_map(|n| {
                        if same_point(n.get_location(), note.get_location()) {
                            Some((n.to_owned(), WriteFlags::default()))
                        } else {
                            None
                        }
                    }).collect();
                buffer.push(note);
                stream::iter_ok::<_, Error>(to_prints)
            }).flatten();
        let f = resp
            .send_all(to_send)
            .map(|_| {})
            .map_err(|e| error!("failed to route chat: {:?}", e));
        ctx.spawn(f)
    }
}

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(Environment::new(2));
    let instance = RouteGuideService {
        data: Arc::new(load_db()),
    };
    let service = route_guide_grpc::create_route_guide(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
