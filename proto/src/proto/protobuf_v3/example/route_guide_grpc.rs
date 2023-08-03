// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_ROUTE_GUIDE_GET_FEATURE: ::grpcio::Method<super::route_guide::Point, super::route_guide::Feature> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routeguide.RouteGuide/GetFeature",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTE_GUIDE_LIST_FEATURES: ::grpcio::Method<super::route_guide::Rectangle, super::route_guide::Feature> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/routeguide.RouteGuide/ListFeatures",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTE_GUIDE_RECORD_ROUTE: ::grpcio::Method<super::route_guide::Point, super::route_guide::RouteSummary> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/routeguide.RouteGuide/RecordRoute",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTE_GUIDE_ROUTE_CHAT: ::grpcio::Method<super::route_guide::RouteNote, super::route_guide::RouteNote> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/routeguide.RouteGuide/RouteChat",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
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

    pub fn get_feature_opt(&self, req: &super::route_guide::Point, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::route_guide::Feature> {
        self.client.unary_call(&METHOD_ROUTE_GUIDE_GET_FEATURE, req, opt)
    }

    pub fn get_feature(&self, req: &super::route_guide::Point) -> ::grpcio::Result<super::route_guide::Feature> {
        self.get_feature_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_feature_async_opt(&self, req: &super::route_guide::Point, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::route_guide::Feature>> {
        self.client.unary_call_async(&METHOD_ROUTE_GUIDE_GET_FEATURE, req, opt)
    }

    pub fn get_feature_async(&self, req: &super::route_guide::Point) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::route_guide::Feature>> {
        self.get_feature_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_features_opt(&self, req: &super::route_guide::Rectangle, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::route_guide::Feature>> {
        self.client.server_streaming(&METHOD_ROUTE_GUIDE_LIST_FEATURES, req, opt)
    }

    pub fn list_features(&self, req: &super::route_guide::Rectangle) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::route_guide::Feature>> {
        self.list_features_opt(req, ::grpcio::CallOption::default())
    }

    pub fn record_route_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::route_guide::Point>, ::grpcio::ClientCStreamReceiver<super::route_guide::RouteSummary>)> {
        self.client.client_streaming(&METHOD_ROUTE_GUIDE_RECORD_ROUTE, opt)
    }

    pub fn record_route(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::route_guide::Point>, ::grpcio::ClientCStreamReceiver<super::route_guide::RouteSummary>)> {
        self.record_route_opt(::grpcio::CallOption::default())
    }

    pub fn route_chat_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::route_guide::RouteNote>, ::grpcio::ClientDuplexReceiver<super::route_guide::RouteNote>)> {
        self.client.duplex_streaming(&METHOD_ROUTE_GUIDE_ROUTE_CHAT, opt)
    }

    pub fn route_chat(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::route_guide::RouteNote>, ::grpcio::ClientDuplexReceiver<super::route_guide::RouteNote>)> {
        self.route_chat_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RouteGuide {
    fn get_feature(&mut self, ctx: ::grpcio::RpcContext, _req: super::route_guide::Point, sink: ::grpcio::UnarySink<super::route_guide::Feature>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_features(&mut self, ctx: ::grpcio::RpcContext, _req: super::route_guide::Rectangle, sink: ::grpcio::ServerStreamingSink<super::route_guide::Feature>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn record_route(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::route_guide::Point>, sink: ::grpcio::ClientStreamingSink<super::route_guide::RouteSummary>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn route_chat(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::route_guide::RouteNote>, sink: ::grpcio::DuplexSink<super::route_guide::RouteNote>) {
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
    builder = builder.add_server_streaming_handler(&METHOD_ROUTE_GUIDE_LIST_FEATURES, move |ctx, req, resp| {
        instance.list_features(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_ROUTE_GUIDE_RECORD_ROUTE, move |ctx, req, resp| {
        instance.record_route(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_ROUTE_GUIDE_ROUTE_CHAT, move |ctx, req, resp| {
        instance.route_chat(ctx, req, resp)
    });
    builder.build()
}
