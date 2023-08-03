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

const METHOD_TEST_SERVICE_EMPTY_CALL: ::grpcio::Method<super::empty::Empty, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.TestService/EmptyCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TEST_SERVICE_UNARY_CALL: ::grpcio::Method<super::messages::SimpleRequest, super::messages::SimpleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.TestService/UnaryCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL: ::grpcio::Method<super::messages::SimpleRequest, super::messages::SimpleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.TestService/CacheableUnaryCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL: ::grpcio::Method<super::messages::StreamingOutputCallRequest, super::messages::StreamingOutputCallResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/grpc.testing.TestService/StreamingOutputCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TEST_SERVICE_STREAMING_INPUT_CALL: ::grpcio::Method<super::messages::StreamingInputCallRequest, super::messages::StreamingInputCallResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/grpc.testing.TestService/StreamingInputCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TEST_SERVICE_FULL_DUPLEX_CALL: ::grpcio::Method<super::messages::StreamingOutputCallRequest, super::messages::StreamingOutputCallResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.TestService/FullDuplexCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TEST_SERVICE_HALF_DUPLEX_CALL: ::grpcio::Method<super::messages::StreamingOutputCallRequest, super::messages::StreamingOutputCallResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.TestService/HalfDuplexCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL: ::grpcio::Method<super::empty::Empty, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.TestService/UnimplementedCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct TestServiceClient {
    pub client: ::grpcio::Client,
}

impl TestServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TestServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn empty_call_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_TEST_SERVICE_EMPTY_CALL, req, opt)
    }

    pub fn empty_call(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::empty::Empty> {
        self.empty_call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn empty_call_async_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_EMPTY_CALL, req, opt)
    }

    pub fn empty_call_async(&self, req: &super::empty::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.empty_call_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unary_call_opt(&self, req: &super::messages::SimpleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::messages::SimpleResponse> {
        self.client.unary_call(&METHOD_TEST_SERVICE_UNARY_CALL, req, opt)
    }

    pub fn unary_call(&self, req: &super::messages::SimpleRequest) -> ::grpcio::Result<super::messages::SimpleResponse> {
        self.unary_call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unary_call_async_opt(&self, req: &super::messages::SimpleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messages::SimpleResponse>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_UNARY_CALL, req, opt)
    }

    pub fn unary_call_async(&self, req: &super::messages::SimpleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messages::SimpleResponse>> {
        self.unary_call_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cacheable_unary_call_opt(&self, req: &super::messages::SimpleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::messages::SimpleResponse> {
        self.client.unary_call(&METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL, req, opt)
    }

    pub fn cacheable_unary_call(&self, req: &super::messages::SimpleRequest) -> ::grpcio::Result<super::messages::SimpleResponse> {
        self.cacheable_unary_call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cacheable_unary_call_async_opt(&self, req: &super::messages::SimpleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messages::SimpleResponse>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL, req, opt)
    }

    pub fn cacheable_unary_call_async(&self, req: &super::messages::SimpleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messages::SimpleResponse>> {
        self.cacheable_unary_call_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn streaming_output_call_opt(&self, req: &super::messages::StreamingOutputCallRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::messages::StreamingOutputCallResponse>> {
        self.client.server_streaming(&METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL, req, opt)
    }

    pub fn streaming_output_call(&self, req: &super::messages::StreamingOutputCallRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::messages::StreamingOutputCallResponse>> {
        self.streaming_output_call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn streaming_input_call_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::messages::StreamingInputCallRequest>, ::grpcio::ClientCStreamReceiver<super::messages::StreamingInputCallResponse>)> {
        self.client.client_streaming(&METHOD_TEST_SERVICE_STREAMING_INPUT_CALL, opt)
    }

    pub fn streaming_input_call(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::messages::StreamingInputCallRequest>, ::grpcio::ClientCStreamReceiver<super::messages::StreamingInputCallResponse>)> {
        self.streaming_input_call_opt(::grpcio::CallOption::default())
    }

    pub fn full_duplex_call_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::messages::StreamingOutputCallRequest>, ::grpcio::ClientDuplexReceiver<super::messages::StreamingOutputCallResponse>)> {
        self.client.duplex_streaming(&METHOD_TEST_SERVICE_FULL_DUPLEX_CALL, opt)
    }

    pub fn full_duplex_call(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::messages::StreamingOutputCallRequest>, ::grpcio::ClientDuplexReceiver<super::messages::StreamingOutputCallResponse>)> {
        self.full_duplex_call_opt(::grpcio::CallOption::default())
    }

    pub fn half_duplex_call_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::messages::StreamingOutputCallRequest>, ::grpcio::ClientDuplexReceiver<super::messages::StreamingOutputCallResponse>)> {
        self.client.duplex_streaming(&METHOD_TEST_SERVICE_HALF_DUPLEX_CALL, opt)
    }

    pub fn half_duplex_call(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::messages::StreamingOutputCallRequest>, ::grpcio::ClientDuplexReceiver<super::messages::StreamingOutputCallResponse>)> {
        self.half_duplex_call_opt(::grpcio::CallOption::default())
    }

    pub fn unimplemented_call_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }

    pub fn unimplemented_call(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::empty::Empty> {
        self.unimplemented_call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unimplemented_call_async_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }

    pub fn unimplemented_call_async(&self, req: &super::empty::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.unimplemented_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait TestService {
    fn empty_call(&mut self, ctx: ::grpcio::RpcContext, _req: super::empty::Empty, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn unary_call(&mut self, ctx: ::grpcio::RpcContext, _req: super::messages::SimpleRequest, sink: ::grpcio::UnarySink<super::messages::SimpleResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cacheable_unary_call(&mut self, ctx: ::grpcio::RpcContext, _req: super::messages::SimpleRequest, sink: ::grpcio::UnarySink<super::messages::SimpleResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_output_call(&mut self, ctx: ::grpcio::RpcContext, _req: super::messages::StreamingOutputCallRequest, sink: ::grpcio::ServerStreamingSink<super::messages::StreamingOutputCallResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_input_call(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::messages::StreamingInputCallRequest>, sink: ::grpcio::ClientStreamingSink<super::messages::StreamingInputCallResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn full_duplex_call(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::messages::StreamingOutputCallRequest>, sink: ::grpcio::DuplexSink<super::messages::StreamingOutputCallResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn half_duplex_call(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::messages::StreamingOutputCallRequest>, sink: ::grpcio::DuplexSink<super::messages::StreamingOutputCallResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn unimplemented_call(&mut self, ctx: ::grpcio::RpcContext, _req: super::empty::Empty, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_test_service<S: TestService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_EMPTY_CALL, move |ctx, req, resp| {
        instance.empty_call(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_UNARY_CALL, move |ctx, req, resp| {
        instance.unary_call(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL, move |ctx, req, resp| {
        instance.cacheable_unary_call(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL, move |ctx, req, resp| {
        instance.streaming_output_call(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_TEST_SERVICE_STREAMING_INPUT_CALL, move |ctx, req, resp| {
        instance.streaming_input_call(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_TEST_SERVICE_FULL_DUPLEX_CALL, move |ctx, req, resp| {
        instance.full_duplex_call(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_TEST_SERVICE_HALF_DUPLEX_CALL, move |ctx, req, resp| {
        instance.half_duplex_call(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL, move |ctx, req, resp| {
        instance.unimplemented_call(ctx, req, resp)
    });
    builder.build()
}

const METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL: ::grpcio::Method<super::empty::Empty, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.UnimplementedService/UnimplementedCall",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct UnimplementedServiceClient {
    pub client: ::grpcio::Client,
}

impl UnimplementedServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        UnimplementedServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn unimplemented_call_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }

    pub fn unimplemented_call(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::empty::Empty> {
        self.unimplemented_call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unimplemented_call_async_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }

    pub fn unimplemented_call_async(&self, req: &super::empty::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.unimplemented_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait UnimplementedService {
    fn unimplemented_call(&mut self, ctx: ::grpcio::RpcContext, _req: super::empty::Empty, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_unimplemented_service<S: UnimplementedService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL, move |ctx, req, resp| {
        instance.unimplemented_call(ctx, req, resp)
    });
    builder.build()
}

const METHOD_RECONNECT_SERVICE_START: ::grpcio::Method<super::messages::ReconnectParams, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.ReconnectService/Start",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RECONNECT_SERVICE_STOP: ::grpcio::Method<super::empty::Empty, super::messages::ReconnectInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.ReconnectService/Stop",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ReconnectServiceClient {
    pub client: ::grpcio::Client,
}

impl ReconnectServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ReconnectServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn start_opt(&self, req: &super::messages::ReconnectParams, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_RECONNECT_SERVICE_START, req, opt)
    }

    pub fn start(&self, req: &super::messages::ReconnectParams) -> ::grpcio::Result<super::empty::Empty> {
        self.start_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_async_opt(&self, req: &super::messages::ReconnectParams, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_RECONNECT_SERVICE_START, req, opt)
    }

    pub fn start_async(&self, req: &super::messages::ReconnectParams) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.start_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::messages::ReconnectInfo> {
        self.client.unary_call(&METHOD_RECONNECT_SERVICE_STOP, req, opt)
    }

    pub fn stop(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::messages::ReconnectInfo> {
        self.stop_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_async_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messages::ReconnectInfo>> {
        self.client.unary_call_async(&METHOD_RECONNECT_SERVICE_STOP, req, opt)
    }

    pub fn stop_async(&self, req: &super::empty::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messages::ReconnectInfo>> {
        self.stop_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ReconnectService {
    fn start(&mut self, ctx: ::grpcio::RpcContext, _req: super::messages::ReconnectParams, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn stop(&mut self, ctx: ::grpcio::RpcContext, _req: super::empty::Empty, sink: ::grpcio::UnarySink<super::messages::ReconnectInfo>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_reconnect_service<S: ReconnectService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RECONNECT_SERVICE_START, move |ctx, req, resp| {
        instance.start(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_RECONNECT_SERVICE_STOP, move |ctx, req, resp| {
        instance.stop(ctx, req, resp)
    });
    builder.build()
}
