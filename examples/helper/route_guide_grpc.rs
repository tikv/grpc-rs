#![allow(dead_code)]

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

pub trait RouteGuide {
    fn get_feature(&self, ctx: RpcContext, point: Point, resp: UnaryResponseSink<Feature>);
    fn list_features(&self, ctx: RpcContext, rect: Rectangle, resp: ResponseSink<Feature>);
    fn record_route(&self, ctx: RpcContext, point: RequestStream<Point>, resp: ClientStreamingResponseSink<RouteSummary>);
    fn route_chat(&self, ctx: RpcContext, note: RequestStream<RouteNote>, resp: ResponseSink<RouteNote>);
}

pub fn create_service<R: RouteGuide + Send + Clone + 'static>(service: R) -> Service {
    let mut builder = ServiceBuilder::new();
    let instance = service.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTE_GUIDE_GET_FEATURE, move |ctx, point, resp| {
        instance.get_feature(ctx, point, resp)
    });
    let instance = service.clone();
    builder = builder.add_server_streaming_handler(&METHOD_ROUTE_GUIDE_LIST_FEATURES, move |ctx, point, resp| {
        instance.list_features(ctx, point, resp)
    });
    let instance = service.clone();
    builder = builder.add_client_streaming_handler(&METHOD_ROUTE_GUIDE_RECORD_ROUTE, move |ctx, point, resp| {
        instance.record_route(ctx, point, resp)
    });
    builder.add_duplex_streaming_handler(&METHOD_ROUTE_GUIDE_ROUTE_CHAT, move |ctx, point, resp| {
        service.route_chat(ctx, point, resp)
    }).build()
}
