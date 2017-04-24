use grpc::*;

use route_guide::*;

const METHOD_ROUTE_GUIDE_GET_FEATURE: Method = Method {
    ty: MethodType::Dulex,
    name: "/routeguide.RouteGuide/GetFeature",
};

const METHOD_ROUTE_GUIDE_LIST_FEATURES: Method = Method {
    ty: MethodType::ServerStreaming,
    name: "/routeguide.RouteGuide/ListFeatures",
};

const METHOD_ROUTE_GUIDE_RECORD_ROUTE: Method = Method {
    ty: MethodType::ClientStreaming,
    name: "/routeguide.RouteGuide/RecordRoute",
};

const METHOD_ROUTE_GUIDE_ROUTE_CHAT: Method = Method {
    ty: MethodType::Dulex,
    name: "/routeguide.RouteGuide/RouteChat",
};

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
        self.client.unary_call(&METHOD_ROUTE_GUIDE_GET_FEATURE, point, opt)
    }

    pub fn get_feature_async(&self, point: Point) -> Result<UnaryCallHandler<Feature>> {
        self.get_feature_async_opt(point, CallOption::default())
    }

    pub fn get_feature_async_opt(&self, point: Point, opt: CallOption) -> Result<UnaryCallHandler<Feature>> {
        self.client.unary_call_async(&METHOD_ROUTE_GUIDE_GET_FEATURE, point, opt)
    }

    pub fn list_features(&self, rect: Rectangle) -> Result<ServerStreamingCallHandler<Feature>> {
        self.list_features_opt(rect, CallOption::default())
    }

    pub fn list_features_opt(&self, rect: Rectangle, opt: CallOption) -> Result<ServerStreamingCallHandler<Feature>> {
        self.client.server_streaming(&METHOD_ROUTE_GUIDE_LIST_FEATURES, rect, opt)
    }

    pub fn record_route(&self) -> Result<ClientStreamingCallHandler<Point, RouteSummary>> {
        self.record_route_opt(CallOption::default())
    }

    pub fn record_route_opt(&self, opt: CallOption) -> Result<ClientStreamingCallHandler<Point, RouteSummary>> {
        self.client.client_streaming(&METHOD_ROUTE_GUIDE_RECORD_ROUTE, opt)
    }

    pub fn route_chat(&self) -> Result<DuplexStreamingCallHandler<RouteNote, RouteNote>> {
        self.route_chat_opt(CallOption::default())
    }

    pub fn route_chat_opt(&self, opt: CallOption) -> Result<DuplexStreamingCallHandler<RouteNote, RouteNote>> {
        self.client.duplex_streaming(&METHOD_ROUTE_GUIDE_ROUTE_CHAT, opt)
    }
}
