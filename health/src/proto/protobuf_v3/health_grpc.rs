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

const METHOD_HEALTH_CHECK: ::grpcio::Method<
    super::health::HealthCheckRequest,
    super::health::HealthCheckResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.health.v1.Health/Check",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_HEALTH_WATCH: ::grpcio::Method<
    super::health::HealthCheckRequest,
    super::health::HealthCheckResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/grpc.health.v1.Health/Watch",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

#[derive(Clone)]
pub struct HealthClient {
    pub client: ::grpcio::Client,
}

impl HealthClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        HealthClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn check_opt(
        &self,
        req: &super::health::HealthCheckRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::health::HealthCheckResponse> {
        self.client.unary_call(&METHOD_HEALTH_CHECK, req, opt)
    }

    pub fn check(
        &self,
        req: &super::health::HealthCheckRequest,
    ) -> ::grpcio::Result<super::health::HealthCheckResponse> {
        self.check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_async_opt(
        &self,
        req: &super::health::HealthCheckRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::health::HealthCheckResponse>> {
        self.client.unary_call_async(&METHOD_HEALTH_CHECK, req, opt)
    }

    pub fn check_async(
        &self,
        req: &super::health::HealthCheckRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::health::HealthCheckResponse>> {
        self.check_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn watch_opt(
        &self,
        req: &super::health::HealthCheckRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::health::HealthCheckResponse>> {
        self.client.server_streaming(&METHOD_HEALTH_WATCH, req, opt)
    }

    pub fn watch(
        &self,
        req: &super::health::HealthCheckRequest,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::health::HealthCheckResponse>> {
        self.watch_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait Health {
    fn check(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::health::HealthCheckRequest,
        sink: ::grpcio::UnarySink<super::health::HealthCheckResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn watch(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::health::HealthCheckRequest,
        sink: ::grpcio::ServerStreamingSink<super::health::HealthCheckResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_health<S: Health + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_HEALTH_CHECK, move |ctx, req, resp| {
        instance.check(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_HEALTH_WATCH, move |ctx, req, resp| {
        instance.watch(ctx, req, resp)
    });
    builder.build()
}
