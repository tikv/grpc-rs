extern crate grpc;
extern crate grpc_proto;
extern crate protobuf;
extern crate futures;
extern crate futures_cpupool;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod helper;

use std::sync::Arc;
use std::io::Read;
use std::time::Instant;
use std::{io, thread};

use grpc::*;
use futures::*;
use futures::sync::oneshot;
use futures_cpupool::CpuPool;

use helper::utils::*;
use grpc_proto::example::route_guide::*;
use grpc_proto::example::route_guide_grpc::{self, RouteGuide};


#[derive(Clone)]
struct RouteGuideService {
    pool: CpuPool,
    data: Arc<Vec<Feature>>,
}

impl RouteGuide for RouteGuideService {
    fn get_feature(&self, _: RpcContext, point: Point, sink: UnarySink<Feature>) {
        let data = self.data.clone();
        let resp = data.iter()
            .find(|f| same_point(f.get_location(), &point))
            .map_or_else(Feature::new, ToOwned::to_owned);
        let f = sink.success(resp)
            .map_err(|e| println!("failed to handle getfeature request: {:?}", e));
        self.pool.spawn(f).forget()
    }

    fn list_features(&self, _: RpcContext, rect: Rectangle, resp: ServerStreamingSink<Feature>) {
        let data = self.data.clone();
        let features: Vec<Result<_>> = data.iter()
            .filter_map(|f| if fit_in(f.get_location(), &rect) {
                            Some(Ok(f.to_owned()))
                        } else {
                            None
                        })
            .collect();
        let f = resp.send_all(stream::iter(features))
            .map(|_| {})
            .map_err(|e| println!("failed to handle listfeatures request: {:?}", e));
        self.pool.spawn(f).forget()
    }

    fn record_route(&self,
                    _: RpcContext,
                    points: RequestStream<Point>,
                    resp: ClientStreamingSink<RouteSummary>) {
        let data = self.data.clone();
        let timer = Instant::now();
        let f = points
            .fold((None, 0f64, RouteSummary::new()),
                  move |(last, mut dis, mut summary), point| {
                let total_count = summary.get_point_count();
                summary.set_point_count(total_count + 1);
                if data.iter()
                       .any(|f| {
                                !f.get_name().is_empty() && same_point(f.get_location(), &point)
                            }) {
                    let feature_count = summary.get_feature_count();
                    summary.set_feature_count(feature_count + 1);
                }
                if let Some(last_point) = last {
                    dis += cal_distance(&last_point, &point);
                }
                Ok((Some(point), dis, summary)) as Result<_>
            })
            .and_then(move |(_, dis, mut s)| {
                s.set_distance(dis as i32);
                let dur = timer.elapsed();
                s.set_elapsed_time(dur.as_secs() as i32);
                resp.success(s)
            })
            .map_err(|e| println!("failed to record route: {:?}", e));
        self.pool.spawn(f).forget()
    }

    fn route_chat(&self,
                  _: RpcContext,
                  notes: RequestStream<RouteNote>,
                  resp: DuplexSink<RouteNote>) {
        let mut buffer: Vec<RouteNote> = Vec::new();
        let to_send = notes
            .map(move |note| {
                let to_prints: Vec<Result<_>> = buffer
                    .iter()
                    .filter_map(|n| if same_point(n.get_location(), note.get_location()) {
                                    Some(Ok(n.to_owned()))
                                } else {
                                    None
                                })
                    .collect();
                buffer.push(note);
                stream::iter(to_prints)
            })
            .flatten();
        let f = resp.send_all(to_send)
            .map(|_| {})
            .map_err(|e| println!("failed to route chat: {:?}", e));
        self.pool.spawn(f).forget()
    }
}

fn main() {
    let pool = CpuPool::new(1);
    let env = Arc::new(Environment::new(2));
    let instance = RouteGuideService {
        pool: pool.clone(),
        data: Arc::new(load_db()),
    };
    let service = route_guide_grpc::create_route_guide(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build();
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
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
