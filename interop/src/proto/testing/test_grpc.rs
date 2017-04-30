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

const METHOD_TEST_SERVICE_EMPTY_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/grpc.testing.TestService/EmptyCall",
};

const METHOD_TEST_SERVICE_UNARY_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/grpc.testing.TestService/UnaryCall",
};

const METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/grpc.testing.TestService/CacheableUnaryCall",
};

const METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::ServerStreaming,
    name: "/grpc.testing.TestService/StreamingOutputCall",
};

const METHOD_TEST_SERVICE_STREAMING_INPUT_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::ClientStreaming,
    name: "/grpc.testing.TestService/StreamingInputCall",
};

const METHOD_TEST_SERVICE_FULL_DUPLEX_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Dulex,
    name: "/grpc.testing.TestService/FullDuplexCall",
};

const METHOD_TEST_SERVICE_HALF_DUPLEX_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Dulex,
    name: "/grpc.testing.TestService/HalfDuplexCall",
};

const METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/grpc.testing.TestService/UnimplementedCall",
};

pub struct TestServiceClient {
    client: ::grpc::Client,
}

impl TestServiceClient {
    pub fn new(channel: ::grpc::Channel) -> Self {
        TestServiceClient {
            client: ::grpc::Client::new(channel),
        }
    }

    pub fn empty_call_opt(&self, req: super::empty::Empty, opt: ::grpc::CallOption) -> ::grpc::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_TEST_SERVICE_EMPTY_CALL, req, opt)
    }

    pub fn empty_call(&self, req: super::empty::Empty) -> ::grpc::Result<super::empty::Empty> {
        self.empty_call_opt(req, ::grpc::CallOption::default())
    }

    pub fn empty_call_async_opt(&self, req: super::empty::Empty, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_EMPTY_CALL, req, opt)
    }

    pub fn empty_call_async(&self, req: super::empty::Empty) -> ::grpc::Result<::grpc::UnaryCallHandler<super::empty::Empty>> {
        self.empty_call_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn unary_call_opt(&self, req: super::messages::SimpleRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::messages::SimpleResponse> {
        self.client.unary_call(&METHOD_TEST_SERVICE_UNARY_CALL, req, opt)
    }

    pub fn unary_call(&self, req: super::messages::SimpleRequest) -> ::grpc::Result<super::messages::SimpleResponse> {
        self.unary_call_opt(req, ::grpc::CallOption::default())
    }

    pub fn unary_call_async_opt(&self, req: super::messages::SimpleRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::messages::SimpleResponse>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_UNARY_CALL, req, opt)
    }

    pub fn unary_call_async(&self, req: super::messages::SimpleRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::messages::SimpleResponse>> {
        self.unary_call_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn cacheable_unary_call_opt(&self, req: super::messages::SimpleRequest, opt: ::grpc::CallOption) -> ::grpc::Result<super::messages::SimpleResponse> {
        self.client.unary_call(&METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL, req, opt)
    }

    pub fn cacheable_unary_call(&self, req: super::messages::SimpleRequest) -> ::grpc::Result<super::messages::SimpleResponse> {
        self.cacheable_unary_call_opt(req, ::grpc::CallOption::default())
    }

    pub fn cacheable_unary_call_async_opt(&self, req: super::messages::SimpleRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::messages::SimpleResponse>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL, req, opt)
    }

    pub fn cacheable_unary_call_async(&self, req: super::messages::SimpleRequest) -> ::grpc::Result<::grpc::UnaryCallHandler<super::messages::SimpleResponse>> {
        self.cacheable_unary_call_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn streaming_output_call_opt(&self, req: super::messages::StreamingOutputCallRequest, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::ServerStreamingCallHandler<super::messages::StreamingOutputCallResponse>> {
        self.client.server_streaming(&METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL, req, opt)
    }

    pub fn streaming_output_call(&self, req: super::messages::StreamingOutputCallRequest) -> ::grpc::Result<::grpc::ServerStreamingCallHandler<super::messages::StreamingOutputCallResponse>> {
        self.streaming_output_call_opt(req, ::grpc::CallOption::default())
    }

    pub fn streaming_input_call_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::ClientStreamingCallHandler<super::messages::StreamingInputCallRequest, super::messages::StreamingInputCallResponse>> {
        self.client.client_streaming(&METHOD_TEST_SERVICE_STREAMING_INPUT_CALL, opt)
    }

    pub fn streaming_input_call(&self) -> ::grpc::Result<::grpc::ClientStreamingCallHandler<super::messages::StreamingInputCallRequest, super::messages::StreamingInputCallResponse>> {
        self.streaming_input_call_opt(::grpc::CallOption::default())
    }

    pub fn full_duplex_call_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<super::messages::StreamingOutputCallRequest, super::messages::StreamingOutputCallResponse>> {
        self.client.duplex_streaming(&METHOD_TEST_SERVICE_FULL_DUPLEX_CALL, opt)
    }

    pub fn full_duplex_call(&self) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<super::messages::StreamingOutputCallRequest, super::messages::StreamingOutputCallResponse>> {
        self.full_duplex_call_opt(::grpc::CallOption::default())
    }

    pub fn half_duplex_call_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<super::messages::StreamingOutputCallRequest, super::messages::StreamingOutputCallResponse>> {
        self.client.duplex_streaming(&METHOD_TEST_SERVICE_HALF_DUPLEX_CALL, opt)
    }

    pub fn half_duplex_call(&self) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<super::messages::StreamingOutputCallRequest, super::messages::StreamingOutputCallResponse>> {
        self.half_duplex_call_opt(::grpc::CallOption::default())
    }

    pub fn unimplemented_call_opt(&self, req: super::empty::Empty, opt: ::grpc::CallOption) -> ::grpc::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }

    pub fn unimplemented_call(&self, req: super::empty::Empty) -> ::grpc::Result<super::empty::Empty> {
        self.unimplemented_call_opt(req, ::grpc::CallOption::default())
    }

    pub fn unimplemented_call_async_opt(&self, req: super::empty::Empty, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }

    pub fn unimplemented_call_async(&self, req: super::empty::Empty) -> ::grpc::Result<::grpc::UnaryCallHandler<super::empty::Empty>> {
        self.unimplemented_call_async_opt(req, ::grpc::CallOption::default())
    }
}

pub trait TestService {
    fn empty_call(&self, ctx: ::grpc::RpcContext, req: super::empty::Empty, resp: ::grpc::UnaryResponseSink<super::empty::Empty>);
    fn unary_call(&self, ctx: ::grpc::RpcContext, req: super::messages::SimpleRequest, resp: ::grpc::UnaryResponseSink<super::messages::SimpleResponse>);
    fn cacheable_unary_call(&self, ctx: ::grpc::RpcContext, req: super::messages::SimpleRequest, resp: ::grpc::UnaryResponseSink<super::messages::SimpleResponse>);
    fn streaming_output_call(&self, ctx: ::grpc::RpcContext, req: super::messages::StreamingOutputCallRequest, resp: ::grpc::ResponseSink<super::messages::StreamingOutputCallResponse>);
    fn streaming_input_call(&self, ctx: ::grpc::RpcContext, req: ::grpc::RequestStream<super::messages::StreamingInputCallRequest>, resp: ::grpc::ClientStreamingResponseSink<super::messages::StreamingInputCallResponse>);
    fn full_duplex_call(&self, ctx: ::grpc::RpcContext, req: ::grpc::RequestStream<super::messages::StreamingOutputCallRequest>, resp: ::grpc::ResponseSink<super::messages::StreamingOutputCallResponse>);
    fn half_duplex_call(&self, ctx: ::grpc::RpcContext, req: ::grpc::RequestStream<super::messages::StreamingOutputCallRequest>, resp: ::grpc::ResponseSink<super::messages::StreamingOutputCallResponse>);
    fn unimplemented_call(&self, ctx: ::grpc::RpcContext, req: super::empty::Empty, resp: ::grpc::UnaryResponseSink<super::empty::Empty>);
}

pub fn create_test_service<S: TestService + Send + Clone + 'static>(s: S) -> ::grpc::Service {
    let mut builder = ::grpc::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_EMPTY_CALL, move |ctx, req, resp| {
        instance.empty_call(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_UNARY_CALL, move |ctx, req, resp| {
        instance.unary_call(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL, move |ctx, req, resp| {
        instance.cacheable_unary_call(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL, move |ctx, req, resp| {
        instance.streaming_output_call(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_TEST_SERVICE_STREAMING_INPUT_CALL, move |ctx, req, resp| {
        instance.streaming_input_call(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_TEST_SERVICE_FULL_DUPLEX_CALL, move |ctx, req, resp| {
        instance.full_duplex_call(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_TEST_SERVICE_HALF_DUPLEX_CALL, move |ctx, req, resp| {
        instance.half_duplex_call(ctx, req, resp)
    });
    builder.add_unary_handler(&METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL, move |ctx, req, resp| {
        s.unimplemented_call(ctx, req, resp)
    }).build()
}

const METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/grpc.testing.UnimplementedService/UnimplementedCall",
};

pub struct UnimplementedServiceClient {
    client: ::grpc::Client,
}

impl UnimplementedServiceClient {
    pub fn new(channel: ::grpc::Channel) -> Self {
        UnimplementedServiceClient {
            client: ::grpc::Client::new(channel),
        }
    }

    pub fn unimplemented_call_opt(&self, req: super::empty::Empty, opt: ::grpc::CallOption) -> ::grpc::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }

    pub fn unimplemented_call(&self, req: super::empty::Empty) -> ::grpc::Result<super::empty::Empty> {
        self.unimplemented_call_opt(req, ::grpc::CallOption::default())
    }

    pub fn unimplemented_call_async_opt(&self, req: super::empty::Empty, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }

    pub fn unimplemented_call_async(&self, req: super::empty::Empty) -> ::grpc::Result<::grpc::UnaryCallHandler<super::empty::Empty>> {
        self.unimplemented_call_async_opt(req, ::grpc::CallOption::default())
    }
}

pub trait UnimplementedService {
    fn unimplemented_call(&self, ctx: ::grpc::RpcContext, req: super::empty::Empty, resp: ::grpc::UnaryResponseSink<super::empty::Empty>);
}

pub fn create_unimplemented_service<S: UnimplementedService + Send + Clone + 'static>(s: S) -> ::grpc::Service {
    ::grpc::ServiceBuilder::new()
           .add_unary_handler(&METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL, move |ctx, req, resp| {
               s.unimplemented_call(ctx, req, resp)
           }).build()
}

const METHOD_RECONECT_SERVICE_START: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/grpc.testing.ReconnectService/Start",
};

const METHOD_RECONECT_SERVICE_STOP: ::grpc::Method = ::grpc::Method {
    ty: ::grpc::MethodType::Unary,
    name: "/grpc.testing.ReconnectService/Stop",
};

pub struct ReconnectServiceClient {
    client: ::grpc::Client,
}

impl ReconnectServiceClient {
    pub fn new(channel: ::grpc::Channel) -> Self {
        ReconnectServiceClient {
            client: ::grpc::Client::new(channel),
        }
    }

    pub fn start_opt(&self, req: super::messages::ReconnectParams, opt: ::grpc::CallOption) -> ::grpc::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_RECONECT_SERVICE_START, req, opt)
    }

    pub fn start(&self, req: super::messages::ReconnectParams) -> ::grpc::Result<super::empty::Empty> {
        self.start_opt(req, ::grpc::CallOption::default())
    }

    pub fn start_async_opt(&self, req: super::messages::ReconnectParams, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_RECONECT_SERVICE_START, req, opt)
    }

    pub fn start_async(&self, req: super::messages::ReconnectParams) -> ::grpc::Result<::grpc::UnaryCallHandler<super::empty::Empty>> {
        self.start_async_opt(req, ::grpc::CallOption::default())
    }

    pub fn stop_opt(&self, req: super::empty::Empty, opt: ::grpc::CallOption) -> ::grpc::Result<super::messages::ReconnectInfo> {
        self.client.unary_call(&METHOD_RECONECT_SERVICE_STOP, req, opt)
    }

    pub fn stop(&self, req: super::empty::Empty) -> ::grpc::Result<super::messages::ReconnectInfo> {
        self.stop_opt(req, ::grpc::CallOption::default())
    }

    pub fn stop_async_opt(&self, req: super::empty::Empty, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<super::messages::ReconnectInfo>> {
        self.client.unary_call_async(&METHOD_RECONECT_SERVICE_STOP, req, opt)
    }

    pub fn stop_async(&self, req: super::empty::Empty) -> ::grpc::Result<::grpc::UnaryCallHandler<super::messages::ReconnectInfo>> {
        self.stop_async_opt(req, ::grpc::CallOption::default())
    }
}

pub trait ReconnectService {
    fn start(&self, ctx: ::grpc::RpcContext, req: super::messages::ReconnectParams, resp: ::grpc::UnaryResponseSink<super::empty::Empty>);
    fn stop(&self, ctx: ::grpc::RpcContext, req: super::empty::Empty, resp: ::grpc::UnaryResponseSink<super::messages::ReconnectInfo>);
}

pub fn create_reconnect_service<S: ReconnectService + Send + Clone + 'static>(s: S) -> ::grpc::Service {
    let mut builder = ::grpc::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RECONECT_SERVICE_START, move |ctx, req, resp| {
        instance.start(ctx, req, resp)
    });
    builder.add_unary_handler(&METHOD_RECONECT_SERVICE_STOP, move |ctx, req, resp| {
        s.stop(ctx, req, resp)
    }).build()
}
