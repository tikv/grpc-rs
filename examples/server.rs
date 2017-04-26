extern crate grpc;
extern crate protobuf;
extern crate futures;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod route_guide;
mod route_guide_grpc;
mod utils;

use std::sync::Arc;
use std::io::Read;
use std::time::Instant;
use std::{io, thread};

use utils::*;
use grpc::*;
use futures::*;
use futures::sync::oneshot;
use tokio_core::reactor::*;

use route_guide::*;
use route_guide_grpc::RouteGuide;


#[derive(Clone)]
struct RouteGuideService {
    remote: Remote,
    data: Arc<Vec<Feature>>
}

impl RouteGuide for RouteGuideService {
    fn get_feature(&self, _: RpcContext, point: UnaryRequest<Point>, resp: UnaryResponseSink<Feature>) {
        let data = self.data.clone();
        self.remote.spawn(|_| {
            point.and_then(move |point| {
                let f = data.iter().find(|f| {
                    same_point(f.get_location(), &point)
                }).map_or_else(Feature::new, ToOwned::to_owned);
                resp.succeess(f)
            }).flatten().map_err(|e| println!("failed to handle getfeature request: {:?}", e))
        });
    }

    fn list_features(&self, _: RpcContext, rect: UnaryRequest<Rectangle>, resp: ResponseSink<Feature>) {
        let data = self.data.clone();
        self.remote.spawn(move |_| {
            rect.and_then(move |rect| {
                let mut features: Vec<Result<_>> = vec![];
                for f in data.iter() {
                    if fit_in(f.get_location(), &rect) {
                        features.push(Ok(f.to_owned()));
                    }
                }
                resp.send_all(stream::iter(features)).map(|_| {})
            }).map_err(|e| println!("failed to handle listfeatures request: {:?}", e))
        })
    }

    fn record_route(&self, _: RpcContext, points: RequestStream<Point>, resp: ClientStreamingResponseSink<RouteSummary>) {
        let data = self.data.clone();
        self.remote.spawn(move |_| {
            let timer = Instant::now();
            points.fold((None, 0f64, RouteSummary::new()), move |(last, mut dis, mut summary), point| {
                let total_count = summary.get_point_count();
                summary.set_point_count(total_count + 1);
                if data.iter().any(|f| !f.get_name().is_empty() && same_point(f.get_location(), &point)) {
                    let feature_count = summary.get_feature_count();
                    summary.set_feature_count(feature_count + 1);
                }
                if let Some(last_point) = last {
                    dis += cal_distance(&last_point, &point);
                }
                Ok((Some(point), dis, summary)) as Result<_>
            }).and_then(move |(_, dis, mut s)| {
                s.set_distance(dis as i32);
                let dur = timer.elapsed();
                s.set_elapsed_time(dur.as_secs() as i32);
                resp.succeess(s)
            }).map_err(|e| println!("failed to record route: {:?}", e)).map(|_| {})
        })
    }

    fn route_chat(&self, _: RpcContext, notes: RequestStream<RouteNote>, resp: ResponseSink<RouteNote>) {
        let mut buffer: Vec<RouteNote> = Vec::new();
        self.remote.spawn(|_| {
            let to_send = notes.map(move |note| {
                let to_prints: Vec<Result<_>> = buffer.iter().filter_map(|n| {
                    if same_point(n.get_location(), note.get_location()) {
                        Some(Ok(n.to_owned()))
                    } else {
                        None
                    }
                }).collect();
                buffer.push(note);
                stream::iter(to_prints)
            }).flatten();
            resp.send_all(to_send).map(|_| {}).map_err(|e| println!("failed to route chat: {:?}", e))
        })
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let remote = core.remote();
    let env = Arc::new(Environment::new(2));
    let instance = RouteGuideService { remote: remote, data: Arc::new(load_db()) };
    let mut server = route_guide_grpc::bind_service(ServerBuilder::new(env), instance).bind("127.0.0.1", 50051).build();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    core.run(rx).unwrap();
    core.run(server.shutdown()).unwrap();
}
