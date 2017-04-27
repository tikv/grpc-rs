extern crate grpc;
extern crate protobuf;
extern crate futures;

#[path="./generated/route_guide.rs"]
mod route_guide;
#[path="./generated/route_guide_grpc.rs"]
mod route_guide_grpc;

use std::sync::Arc;

use grpc::*;
use futures::{Future, Stream, stream, Sink};

use route_guide::*;
use route_guide_grpc::RouteGuide;

fn new_point(lat: i32, lon: i32) -> Point {
    let mut point = Point::new();
    point.set_latitude(lat);
    point.set_longitude(lon);
    point
}

fn new_rect(lat1: i32, lon1: i32, lat2: i32, lon2: i32) -> Rectangle {
    let mut rect = Rectangle::new();
    rect.set_hi(new_point(lat1, lon1));
    rect.set_lo(new_point(lat2, lon2));
    rect
}

fn new_note(lat: i32, lon: i32, msg: &str) -> RouteNote {
    let mut note = RouteNote::new();
    note.set_location(new_point(lat, lon));
    note.set_message(msg.to_owned());
    note
}

struct RouteGuideService;

impl RouteGuide for RouteGuideService {
    fn get_feature(&self, ctx: RpcContext, point: UnaryRequest<Point>, resp: UnaryResponseSink<Feature>) {
        unimplemented!()
    }

    fn list_features(&self, ctx: RpcContext, point: UnaryRequest<Rectangle>, resp: ResponseSink<Feature>) {
        unimplemented!()
    }

    fn record_route(&self, ctx: RpcContext, point: RequestStream<Point>, resp: ClientStreamingResponseSink<RouteSummary>) {
        unimplemented!()
    }

    fn route_chat(&self, ctx: RpcContext, note: RequestStream<RouteNote>, resp: ResponseSink<RouteNote>) {
        unimplemented!()
    }
}

fn main() {
    let env = Arc::new(Environment::new(2));
    let instance = RouteGuideService;
    let server = route_guide_grpc::bind_route_guide(ServerBuilder::new(env), instance).bind("127.0.0.1", 50051).build();
    server.start();
    
}
