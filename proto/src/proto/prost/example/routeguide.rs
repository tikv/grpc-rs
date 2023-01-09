/// Points are represented as latitude-longitude pairs in the E7 representation
/// (degrees multiplied by 10**7 and rounded to the nearest integer).
/// Latitudes should be in the range +/- 90 degrees and longitude should be in
/// the range +/- 180 degrees (inclusive).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(int32, tag = "1")]
    pub latitude: i32,
    #[prost(int32, tag = "2")]
    pub longitude: i32,
}
/// A latitude-longitude rectangle, represented as two diagonally opposite
/// points "lo" and "hi".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rectangle {
    /// One corner of the rectangle.
    #[prost(message, optional, tag = "1")]
    pub lo: ::core::option::Option<Point>,
    /// The other corner of the rectangle.
    #[prost(message, optional, tag = "2")]
    pub hi: ::core::option::Option<Point>,
}
/// A feature names something at a given point.
///
/// If a feature could not be named, the name is empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    /// The name of the feature.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The point where the feature is detected.
    #[prost(message, optional, tag = "2")]
    pub location: ::core::option::Option<Point>,
}
/// A RouteNote is a message sent while at a given point.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteNote {
    /// The location from which the message is sent.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<Point>,
    /// The message to be sent.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// A RouteSummary is received in response to a RecordRoute rpc.
///
/// It contains the number of individual points received, the number of
/// detected features, and the total distance covered as the cumulative sum of
/// the distance between each point.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteSummary {
    /// The number of points received.
    #[prost(int32, tag = "1")]
    pub point_count: i32,
    /// The number of known features passed while traversing the route.
    #[prost(int32, tag = "2")]
    pub feature_count: i32,
    /// The distance covered in metres.
    #[prost(int32, tag = "3")]
    pub distance: i32,
    /// The duration of the traversal in seconds.
    #[prost(int32, tag = "4")]
    pub elapsed_time: i32,
}
const METHOD_ROUTE_GUIDE_GET_FEATURE: ::grpcio::Method<Point, Feature> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.RouteGuide/GetFeature",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_ROUTE_GUIDE_LIST_FEATURES: ::grpcio::Method<Rectangle, Feature> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/routeguide.RouteGuide/ListFeatures",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_ROUTE_GUIDE_RECORD_ROUTE: ::grpcio::Method<Point, RouteSummary> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/routeguide.RouteGuide/RecordRoute",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_ROUTE_GUIDE_ROUTE_CHAT: ::grpcio::Method<RouteNote, RouteNote> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/routeguide.RouteGuide/RouteChat",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
#[derive(Clone)]
pub struct RouteGuideClient {
    pub client: ::grpcio::Client,
}
impl RouteGuideClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RouteGuideClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn get_feature_opt(
        &self,
        req: &Point,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<Feature> {
        self.client
            .unary_call(&METHOD_ROUTE_GUIDE_GET_FEATURE, req, opt)
    }
    pub fn get_feature(&self, req: &Point) -> ::grpcio::Result<Feature> {
        self.get_feature_opt(req, ::grpcio::CallOption::default())
    }
    pub fn get_feature_async_opt(
        &self,
        req: &Point,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Feature>> {
        self.client
            .unary_call_async(&METHOD_ROUTE_GUIDE_GET_FEATURE, req, opt)
    }
    pub fn get_feature_async(
        &self,
        req: &Point,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Feature>> {
        self.get_feature_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn list_features_opt(
        &self,
        req: &Rectangle,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<Feature>> {
        self.client
            .server_streaming(&METHOD_ROUTE_GUIDE_LIST_FEATURES, req, opt)
    }
    pub fn list_features(
        &self,
        req: &Rectangle,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<Feature>> {
        self.list_features_opt(req, ::grpcio::CallOption::default())
    }
    pub fn record_route_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<Point>,
        ::grpcio::ClientCStreamReceiver<RouteSummary>,
    )> {
        self.client
            .client_streaming(&METHOD_ROUTE_GUIDE_RECORD_ROUTE, opt)
    }
    pub fn record_route(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<Point>,
        ::grpcio::ClientCStreamReceiver<RouteSummary>,
    )> {
        self.record_route_opt(::grpcio::CallOption::default())
    }
    pub fn route_chat_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<RouteNote>,
        ::grpcio::ClientDuplexReceiver<RouteNote>,
    )> {
        self.client
            .duplex_streaming(&METHOD_ROUTE_GUIDE_ROUTE_CHAT, opt)
    }
    pub fn route_chat(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<RouteNote>,
        ::grpcio::ClientDuplexReceiver<RouteNote>,
    )> {
        self.route_chat_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait RouteGuide {
    fn get_feature(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: Point,
        sink: ::grpcio::UnarySink<Feature>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_features(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: Rectangle,
        sink: ::grpcio::ServerStreamingSink<Feature>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn record_route(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<Point>,
        sink: ::grpcio::ClientStreamingSink<RouteSummary>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn route_chat(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<RouteNote>,
        sink: ::grpcio::DuplexSink<RouteNote>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_route_guide<S: RouteGuide + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTE_GUIDE_GET_FEATURE, move |ctx, req, resp| {
        instance.get_feature(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder
        .add_server_streaming_handler(&METHOD_ROUTE_GUIDE_LIST_FEATURES, move |ctx, req, resp| {
            instance.list_features(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder
        .add_client_streaming_handler(&METHOD_ROUTE_GUIDE_RECORD_ROUTE, move |ctx, req, resp| {
            instance.record_route(ctx, req, resp)
        });
    let mut instance = s;
    builder = builder
        .add_duplex_streaming_handler(&METHOD_ROUTE_GUIDE_ROUTE_CHAT, move |ctx, req, resp| {
            instance.route_chat(ctx, req, resp)
        });
    builder.build()
}
