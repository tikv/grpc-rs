// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[path = "../log_util.rs"]
mod log_util;
mod util;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use futures_util::{SinkExt as _, TryStreamExt as _};
use grpcio::*;
use grpcio_proto::example::route_guide::{Point, Rectangle, RouteNote};
use grpcio_proto::example::route_guide_grpc::RouteGuideClient;
use rand::{seq::SliceRandom, Rng};

fn new_point(lat: i32, lon: i32) -> Point {
    let mut point = Point::default();
    point.latitude = lat;
    point.longitude = lon;
    point
}

fn new_rect(lat1: i32, lon1: i32, lat2: i32, lon2: i32) -> Rectangle {
    let mut rect = Rectangle::default();
    rect.lo = Some(new_point(lat1, lon1)).into();
    rect.hi = Some(new_point(lat2, lon2)).into();
    rect
}

fn new_note(lat: i32, lon: i32, msg: &str) -> RouteNote {
    let mut note = RouteNote::default();
    note.location = Some(new_point(lat, lon)).into();
    note.message = msg.to_owned();
    note
}

async fn get_feature(client: &RouteGuideClient, point: &Point) -> Result<()> {
    let get_feature = client.get_feature_async(point)?;
    let f = get_feature.await?;
    if f.location.is_none() {
        warn!("Server returns incomplete feature.");
        return Ok(());
    }
    if f.name.is_empty() {
        warn!("Found no feature at {}", util::format_point(point));
        return Ok(());
    }
    info!(
        "Found feature called {} at {}",
        f.name,
        util::format_point(point)
    );
    Ok(())
}

async fn list_features(client: &RouteGuideClient) -> Result<()> {
    let rect = new_rect(400_000_000, -750_000_000, 420_000_000, -730_000_000);
    info!("Looking for features between 40, -75 and 42, -73");
    let mut list_features = client.list_features(&rect)?;
    while let Some(feature) = list_features.try_next().await? {
        #[cfg(feature = "protobuf-codec")]
        info!(
            "Found feature {} at {}",
            feature.name,
            util::format_point(feature.get_location())
        );
        #[cfg(feature = "protobufv3-codec")]
        info!(
            "Found feature {} at {}",
            feature.name,
            util::format_point(&feature.location.0.unwrap())
        );
    }
    info!("List feature rpc succeeded.");
    Ok(())
}

async fn record_route(client: &RouteGuideClient) -> Result<()> {
    let features = util::load_db();
    let mut rng = rand::thread_rng();
    let (mut sink, receiver) = client.record_route()?;
    for _ in 0..10usize {
        let f = features.choose(&mut rng).unwrap();
        #[cfg(feature = "protobuf-codec")]
        let point = f.get_location().to_owned();
        #[cfg(feature = "protobufv3-codec")]
        let point = f.location.0.clone().unwrap().as_ref().clone();
        info!("Visiting {}", util::format_point(&point));
        sink.send((point, WriteFlags::default())).await?;
        thread::sleep(Duration::from_millis(rng.gen_range(500, 1500)));
    }
    // flush
    sink.close().await?;
    let summary = receiver.await?;
    info!("Finished trip with {} points", summary.point_count);
    info!("Passed {} features", summary.feature_count);
    info!("Travelled {} meters", summary.distance);
    info!("It took {} seconds", summary.elapsed_time);
    Ok(())
}

async fn route_chat(client: &RouteGuideClient) -> Result<()> {
    let (mut sink, mut receiver) = client.route_chat()?;

    let send = async move {
        let notes = vec![
            ("First message", 0, 0),
            ("Second message", 0, 1),
            ("Third message", 1, 0),
            ("Fourth message", 0, 0),
        ];

        for (msg, lat, lon) in notes {
            let note = new_note(lat, lon, msg);
            info!("Sending message {} at {}, {}", msg, lat, lon);
            sink.send((note, WriteFlags::default())).await?;
        }
        sink.close().await?;
        Ok(()) as Result<_>
    };

    let receive = async move {
        while let Some(note) = receiver.try_next().await? {
            #[cfg(feature = "protobuf-codec")]
            info!(
                "Got message {} at {}, {}",
                note.get_message(),
                note.get_location().get_latitude(),
                note.get_location().get_longitude()
            );
            #[cfg(feature = "protobufv3-codec")]
            info!(
                "Got message {} at {}, {}",
                note.message, note.location.latitude, note.location.longitude
            );
        }
        Ok(()) as Result<_>
    };
    let (sr, rr) = futures_util::join!(send, receive);
    sr.and(rr)?;
    Ok(())
}

async fn async_main() -> Result<()> {
    let _guard = log_util::init_log(None);
    let env = Arc::new(Environment::new(2));
    let channel = ChannelBuilder::new(env).connect("127.0.0.1:50051");
    let client = RouteGuideClient::new(channel);

    info!("-------------- GetFeature --------------");
    get_feature(&client, &new_point(409_146_138, -746_188_906)).await?;
    get_feature(&client, &new_point(0, 0)).await?;

    info!("-------------- ListFeatures --------------");
    list_features(&client).await?;

    info!("-------------- RecordRoute --------------");
    record_route(&client).await?;

    info!("-------------- RouteChat --------------");
    route_chat(&client).await?;

    Ok(())
}

fn main() {
    futures_executor::block_on(async_main()).unwrap()
}
