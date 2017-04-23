use grpc::*;

use route_guide::*;

const GET_FEATURE_PATH: &'static str = "/routeguide.RouteGuide/GetFeature";
const LIST_FEATURES_PATH: &'static str = "/routeguide.RouteGuide/ListFeatures";
const RECORD_ROUTE_PATH: &'static str = "/routeguide.RouteGuide/RecordRoute";
const ROUTE_CHAT_PATH: &'static str = "/routeguide.RouteGuide/RouteChat";

pub struct RouteGuideClient {
    client: Client,
}

impl RouteGuideClient {
    pub fn new(channel: Channel) -> RouteGuideClient {
        RouteGuideClient {
            client: Client::new(channel),
        }
    }

    pub fn get_feature(&self, point: Point) -> Result<Feature> {
        self.get_feature_opt(point, CallOption::default())
    }

    pub fn get_feature_opt(&self, point: Point, opt: CallOption) -> Result<Feature> {
        let m = Method::new(MethodType::Unary, GET_FEATURE_PATH);
        self.client.unary_call(&m, point, opt)
    }

    pub fn get_feature_async(&self, point: Point) -> Result<UnaryCallHandler<Feature>> {
        self.get_feature_async_opt(point, CallOption::default())
    }

    pub fn get_feature_async_opt(&self, point: Point, opt: CallOption) -> Result<UnaryCallHandler<Feature>> {
        let m = Method::new(MethodType::Unary, GET_FEATURE_PATH);
        self.client.unary_call_async(&m, point, opt)
    }

    pub fn list_features(&self, rect: Rectangle) -> Result<ServerStreamingCallHandler<Feature>> {
        self.list_features_opt(rect, CallOption::default())
    }

    pub fn list_features_opt(&self, rect: Rectangle, opt: CallOption) -> Result<ServerStreamingCallHandler<Feature>> {
        let m = Method::new(MethodType::ServerStreaming, LIST_FEATURES_PATH);
        self.client.server_streaming(&m, rect, opt)
    }

    pub fn record_route(&self) -> Result<ClientStreamingCallHandler<Point, RouteSummary>> {
        self.record_route_opt(CallOption::default())
    }

    pub fn record_route_opt(&self, opt: CallOption) -> Result<ClientStreamingCallHandler<Point, RouteSummary>> {
        let m = Method::new(MethodType::ClientStreaming, RECORD_ROUTE_PATH);
        self.client.client_streaming(&m, opt)
    }

    pub fn route_chat(&self) -> Result<DuplexStreamingCallHandler<RouteNote, RouteNote>> {
        self.route_chat_opt(CallOption::default())
    }

    pub fn route_chat_opt(&self, opt: CallOption) -> Result<DuplexStreamingCallHandler<RouteNote, RouteNote>> {
        let m = Method::new(MethodType::Dulex, ROUTE_CHAT_PATH);
        self.client.duplex_streaming(&m, opt)
    }
}
