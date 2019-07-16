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

#[macro_use]
extern crate log;
use rand;
#[macro_use]
extern crate serde_derive;

#[path = "../log_util.rs"]
mod log_util;
mod util;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use futures::{future, Future, Sink, Stream};
use grpcio::*;
use grpcio_proto::example::route_guide::RouteGuideClient;
use grpcio_proto::example::route_guide::{Point, Rectangle, RouteNote};
use rand::Rng;

fn new_point(lat: i32, lon: i32) -> Point {
    let mut point = Point::default();
    point.set_latitude(lat);
    point.set_longitude(lon);
    point
}

fn new_rect(lat1: i32, lon1: i32, lat2: i32, lon2: i32) -> Rectangle {
    let mut rect = Rectangle::default();
    rect.set_lo(new_point(lat1, lon1));
    rect.set_hi(new_point(lat2, lon2));
    rect
}

fn new_note(lat: i32, lon: i32, msg: &str) -> RouteNote {
    let mut note = RouteNote::default();
    note.set_location(new_point(lat, lon));
    note.set_message(msg.to_owned());
    note
}

fn get_feature(client: &RouteGuideClient, point: &Point) {
    let get_feature = client.get_feature_async(point).unwrap();
    match get_feature.wait() {
        Err(e) => panic!("RPC failed: {:?}", e),
        Ok(f) => {
            if !f.has_location() {
                warn!("Server returns incomplete feature.");
                return;
            }
            if f.get_name().is_empty() {
                warn!("Found no feature at {}", util::format_point(point));
                return;
            }
            info!(
                "Found feature called {} at {}",
                f.get_name(),
                util::format_point(point)
            );
        }
    }
}

fn list_features(client: &RouteGuideClient) {
    let rect = new_rect(400_000_000, -750_000_000, 420_000_000, -730_000_000);
    info!("Looking for features between 40, -75 and 42, -73");
    let mut list_features = client.list_features(&rect).unwrap();
    loop {
        let f = list_features.into_future();
        match f.wait() {
            Ok((Some(feature), s)) => {
                list_features = s;
                let loc = feature.get_location();
                info!(
                    "Found feature {} at {}",
                    feature.get_name(),
                    util::format_point(loc)
                );
            }
            Ok((None, _)) => break,
            Err((e, _)) => panic!("List features failed: {:?}", e),
        }
    }
    info!("List feature rpc succeeded.");
}

fn record_route(client: &RouteGuideClient) {
    let features = util::load_db();
    let mut rng = rand::thread_rng();
    let (mut sink, receiver) = client.record_route().unwrap();
    for _ in 0..10 {
        let f = rng.choose(&features).unwrap();
        let point = f.get_location();
        info!("Visiting {}", util::format_point(point));
        sink = sink
            .send((point.to_owned(), WriteFlags::default()))
            .wait()
            .unwrap();
        thread::sleep(Duration::from_millis(rng.gen_range(500, 1500)));
    }
    // flush
    future::poll_fn(|| sink.close()).wait().unwrap();
    let sumary = receiver.wait().unwrap();
    info!("Finished trip with {} points", sumary.get_point_count());
    info!("Passed {} features", sumary.get_feature_count());
    info!("Travelled {} meters", sumary.get_distance());
    info!("It took {} seconds", sumary.get_elapsed_time());
}

fn route_chat(client: &RouteGuideClient) {
    let (mut sink, mut receiver) = client.route_chat().unwrap();
    let h = thread::spawn(move || {
        let notes = vec![
            ("First message", 0, 0),
            ("Second message", 0, 1),
            ("Third message", 1, 0),
            ("Fourth message", 0, 0),
        ];

        for (msg, lat, lon) in notes {
            let note = new_note(lat, lon, msg);
            info!("Sending message {} at {}, {}", msg, lat, lon);
            sink = sink.send((note, WriteFlags::default())).wait().unwrap();
        }
        future::poll_fn(|| sink.close()).wait().unwrap();
    });

    loop {
        match receiver.into_future().wait() {
            Ok((Some(note), r)) => {
                let location = note.get_location();
                info!(
                    "Got message {} at {}, {}",
                    note.get_message(),
                    location.get_latitude(),
                    location.get_longitude()
                );
                receiver = r;
            }
            Ok((None, _)) => break,
            Err((e, _)) => panic!("RouteChat RPC failed: {:?}", e),
        }
    }

    h.join().unwrap();
}

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(Environment::new(2));
    let channel = ChannelBuilder::new(env).connect("127.0.0.1:50051");
    let client = RouteGuideClient::new(channel);

    info!("-------------- GetFeature --------------");
    get_feature(&client, &new_point(409_146_138, -746_188_906));
    get_feature(&client, &new_point(0, 0));

    info!("-------------- ListFeatures --------------");
    list_features(&client);

    info!("-------------- RecordRoute --------------");
    record_route(&client);

    info!("-------------- RouteChat --------------");
    route_chat(&client);
}
