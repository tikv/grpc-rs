#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckRequest {
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckResponse {
    #[prost(enumeration = "health_check_response::ServingStatus", tag = "1")]
    pub status: i32,
}
/// Nested message and enum types in `HealthCheckResponse`.
pub mod health_check_response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ServingStatus {
        Unknown = 0,
        Serving = 1,
        NotServing = 2,
        /// Used only by the Watch method.
        ServiceUnknown = 3,
    }
    impl ServingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServingStatus::Unknown => "UNKNOWN",
                ServingStatus::Serving => "SERVING",
                ServingStatus::NotServing => "NOT_SERVING",
                ServingStatus::ServiceUnknown => "SERVICE_UNKNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SERVING" => Some(Self::Serving),
                "NOT_SERVING" => Some(Self::NotServing),
                "SERVICE_UNKNOWN" => Some(Self::ServiceUnknown),
                _ => None,
            }
        }
    }
}
const METHOD_HEALTH_CHECK: ::grpcio::Method<HealthCheckRequest, HealthCheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.health.v1.Health/Check",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_HEALTH_WATCH: ::grpcio::Method<HealthCheckRequest, HealthCheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/grpc.health.v1.Health/Watch",
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
        req: &HealthCheckRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<HealthCheckResponse> {
        self.client.unary_call(&METHOD_HEALTH_CHECK, req, opt)
    }
    pub fn check(
        &self,
        req: &HealthCheckRequest,
    ) -> ::grpcio::Result<HealthCheckResponse> {
        self.check_opt(req, ::grpcio::CallOption::default())
    }
    pub fn check_async_opt(
        &self,
        req: &HealthCheckRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<HealthCheckResponse>> {
        self.client.unary_call_async(&METHOD_HEALTH_CHECK, req, opt)
    }
    pub fn check_async(
        &self,
        req: &HealthCheckRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<HealthCheckResponse>> {
        self.check_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn watch_opt(
        &self,
        req: &HealthCheckRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<HealthCheckResponse>> {
        self.client.server_streaming(&METHOD_HEALTH_WATCH, req, opt)
    }
    pub fn watch(
        &self,
        req: &HealthCheckRequest,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<HealthCheckResponse>> {
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
        _req: HealthCheckRequest,
        sink: ::grpcio::UnarySink<HealthCheckResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn watch(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: HealthCheckRequest,
        sink: ::grpcio::ServerStreamingSink<HealthCheckResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_health<S: Health + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder
        .add_unary_handler(
            &METHOD_HEALTH_CHECK,
            move |ctx, req, resp| instance.check(ctx, req, resp),
        );
    let mut instance = s;
    builder = builder
        .add_server_streaming_handler(
            &METHOD_HEALTH_WATCH,
            move |ctx, req, resp| instance.watch(ctx, req, resp),
        );
    builder.build()
}
