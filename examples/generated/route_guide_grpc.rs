// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

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

const METHOD_GET_FEATURE: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/routeguide.RouteGuide/GetFeature",
};

const METHOD_LIST_FEATURES: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::ServerStreaming,
    name: "/routeguide.RouteGuide/ListFeatures",
};

const METHOD_RECORD_ROUTE: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::ClientStreaming,
    name: "/routeguide.RouteGuide/RecordRoute",
};

const METHOD_ROUTE_CHAT: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Dulex,
    name: "/routeguide.RouteGuide/RouteChat",
};

pub struct RouteGuideClient {
    client: ::grpc::Client,
}

impl RouteGuideClient {
    pub fn new(channel: ::grpc::Channel) -> Self {
        RouteGuideClient {
            client: ::grpc::Client::new(channel),
        }
    }

    pub fn get_feature_opt(&self, req: super::route_guide::Point, opt: ::grpc::CallOption) -> ::grpc::Result<super::route_guide::Feature> {
        self.client.unary_call(&METHOD_GET_FEATURE, req, opt)
    }

    pub fn get_feature(&self, req: super::route_guide::Point) -> ::grpc::Result<super::route_guide::Feature> {
        self.get_feature_opt(req, ::grpc::CallOption::default())
    }

    pub fn get_feature_async_opt(&self, req: super::route_guide::Point, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::route_guide::Feature>> {
        self.client.unary_call_async(&METHOD_GET_FEATURE, req, opt)
    }

    pub fn get_feature_async(&self, req: super::route_guide::Point) -> ::grpc::Result<::grpc::UnaryCallHandler<super::route_guide::Feature>> {
        self.get_feature_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn list_features_opt(&self, req: super::route_guide::Rectangle, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::ServerStreamingCallHandler<super::route_guide::Feature>> {
        self.client.server_streaming(&METHOD_LIST_FEATURES, req, opt)
    }

    pub fn list_features(&self, req: super::route_guide::Rectangle) -> ::grpc::Result<::grpc::ServerStreamingCallHandler<super::route_guide::Feature>> {
        self.list_features_opt(req, ::grpc::CallOption::default())
    }

    pub fn record_route_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::ClientStreamingCallHandler<super::route_guide::Point, super::route_guide::RouteSummary>> {
        self.client.client_streaming(&METHOD_RECORD_ROUTE, opt)
    }

    pub fn record_route(&self) -> ::grpc::Result<::grpc::ClientStreamingCallHandler<super::route_guide::Point, super::route_guide::RouteSummary>> {
        self.record_route_opt(::grpc::CallOption::default())
    }

    pub fn route_chat_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<super::route_guide::RouteNote, super::route_guide::RouteNote>> {
        self.client.duplex_streaming(&METHOD_ROUTE_CHAT, opt)
    }

    pub fn route_chat(&self) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<super::route_guide::RouteNote, super::route_guide::RouteNote>> {
        self.route_chat_opt(::grpc::CallOption::default())
    }
}

pub trait RouteGuide {
    fn get_feature(&self, ctx: ::grpc::RpcContext, req: ::grpc::UnaryRequest<super::route_guide::Point>, resp: ::grpc::UnaryResponseSink<super::route_guide::Feature>);
    fn list_features(&self, ctx: ::grpc::RpcContext, req: ::grpc::UnaryRequest<super::route_guide::Rectangle>, resp: ::grpc::ResponseSink<super::route_guide::Feature>);
    fn record_route(&self, ctx: ::grpc::RpcContext, req: ::grpc::RequestStream<super::route_guide::Point>, resp: ::grpc::ClientStreamingResponseSink<super::route_guide::RouteSummary>);
    fn route_chat(&self, ctx: ::grpc::RpcContext, req: ::grpc::RequestStream<super::route_guide::RouteNote>, resp: ::grpc::ResponseSink<super::route_guide::RouteNote>);
}

pub fn bind_route_guide<S: RouteGuide + Send + Sync + 'static>(mut builder: ::grpc::ServerBuilder, s: S) -> ::grpc::ServerBuilder {
    let service = ::std::sync::Arc::new(s);
    let instance = service.clone();
    builder = builder.add_unary_handler(&METHOD_GET_FEATURE, move |ctx, req, resp| {
        instance.get_feature(ctx, req, resp)
    });
    let instance = service.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIST_FEATURES, move |ctx, req, resp| {
        instance.list_features(ctx, req, resp)
    });
    let instance = service.clone();
    builder = builder.add_client_streaming_handler(&METHOD_RECORD_ROUTE, move |ctx, req, resp| {
        instance.record_route(ctx, req, resp)
    });
    let instance = service.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ROUTE_CHAT, move |ctx, req, resp| {
        instance.route_chat(ctx, req, resp)
    });
    builder
}
