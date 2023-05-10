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

const METHOD_BENCHMARK_SERVICE_UNARY_CALL: ::grpcio::Method<
    super::messages::SimpleRequest,
    super::messages::SimpleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.BenchmarkService/UnaryCall",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_BENCHMARK_SERVICE_STREAMING_CALL: ::grpcio::Method<
    super::messages::SimpleRequest,
    super::messages::SimpleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.BenchmarkService/StreamingCall",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_BENCHMARK_SERVICE_STREAMING_FROM_CLIENT: ::grpcio::Method<
    super::messages::SimpleRequest,
    super::messages::SimpleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/grpc.testing.BenchmarkService/StreamingFromClient",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_BENCHMARK_SERVICE_STREAMING_FROM_SERVER: ::grpcio::Method<
    super::messages::SimpleRequest,
    super::messages::SimpleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/grpc.testing.BenchmarkService/StreamingFromServer",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_BENCHMARK_SERVICE_STREAMING_BOTH_WAYS: ::grpcio::Method<
    super::messages::SimpleRequest,
    super::messages::SimpleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.BenchmarkService/StreamingBothWays",
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
pub struct BenchmarkServiceClient {
    pub client: ::grpcio::Client,
}

impl BenchmarkServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        BenchmarkServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn unary_call_opt(
        &self,
        req: &super::messages::SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::messages::SimpleResponse> {
        self.client
            .unary_call(&METHOD_BENCHMARK_SERVICE_UNARY_CALL, req, opt)
    }

    pub fn unary_call(
        &self,
        req: &super::messages::SimpleRequest,
    ) -> ::grpcio::Result<super::messages::SimpleResponse> {
        self.unary_call_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unary_call_async_opt(
        &self,
        req: &super::messages::SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messages::SimpleResponse>> {
        self.client
            .unary_call_async(&METHOD_BENCHMARK_SERVICE_UNARY_CALL, req, opt)
    }

    pub fn unary_call_async(
        &self,
        req: &super::messages::SimpleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::messages::SimpleResponse>> {
        self.unary_call_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn streaming_call_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::messages::SimpleRequest>,
        ::grpcio::ClientDuplexReceiver<super::messages::SimpleResponse>,
    )> {
        self.client
            .duplex_streaming(&METHOD_BENCHMARK_SERVICE_STREAMING_CALL, opt)
    }

    pub fn streaming_call(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::messages::SimpleRequest>,
        ::grpcio::ClientDuplexReceiver<super::messages::SimpleResponse>,
    )> {
        self.streaming_call_opt(::grpcio::CallOption::default())
    }

    pub fn streaming_from_client_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<super::messages::SimpleRequest>,
        ::grpcio::ClientCStreamReceiver<super::messages::SimpleResponse>,
    )> {
        self.client
            .client_streaming(&METHOD_BENCHMARK_SERVICE_STREAMING_FROM_CLIENT, opt)
    }

    pub fn streaming_from_client(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<super::messages::SimpleRequest>,
        ::grpcio::ClientCStreamReceiver<super::messages::SimpleResponse>,
    )> {
        self.streaming_from_client_opt(::grpcio::CallOption::default())
    }

    pub fn streaming_from_server_opt(
        &self,
        req: &super::messages::SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::messages::SimpleResponse>> {
        self.client
            .server_streaming(&METHOD_BENCHMARK_SERVICE_STREAMING_FROM_SERVER, req, opt)
    }

    pub fn streaming_from_server(
        &self,
        req: &super::messages::SimpleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::messages::SimpleResponse>> {
        self.streaming_from_server_opt(req, ::grpcio::CallOption::default())
    }

    pub fn streaming_both_ways_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::messages::SimpleRequest>,
        ::grpcio::ClientDuplexReceiver<super::messages::SimpleResponse>,
    )> {
        self.client
            .duplex_streaming(&METHOD_BENCHMARK_SERVICE_STREAMING_BOTH_WAYS, opt)
    }

    pub fn streaming_both_ways(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::messages::SimpleRequest>,
        ::grpcio::ClientDuplexReceiver<super::messages::SimpleResponse>,
    )> {
        self.streaming_both_ways_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait BenchmarkService {
    fn unary_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::messages::SimpleRequest,
        sink: ::grpcio::UnarySink<super::messages::SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<super::messages::SimpleRequest>,
        sink: ::grpcio::DuplexSink<super::messages::SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_from_client(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<super::messages::SimpleRequest>,
        sink: ::grpcio::ClientStreamingSink<super::messages::SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_from_server(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::messages::SimpleRequest,
        sink: ::grpcio::ServerStreamingSink<super::messages::SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_both_ways(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<super::messages::SimpleRequest>,
        sink: ::grpcio::DuplexSink<super::messages::SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_benchmark_service<S: BenchmarkService + Send + Clone + 'static>(
    s: S,
) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_BENCHMARK_SERVICE_UNARY_CALL,
        move |ctx, req, resp| instance.unary_call(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(
        &METHOD_BENCHMARK_SERVICE_STREAMING_CALL,
        move |ctx, req, resp| instance.streaming_call(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(
        &METHOD_BENCHMARK_SERVICE_STREAMING_FROM_CLIENT,
        move |ctx, req, resp| instance.streaming_from_client(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(
        &METHOD_BENCHMARK_SERVICE_STREAMING_FROM_SERVER,
        move |ctx, req, resp| instance.streaming_from_server(ctx, req, resp),
    );
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(
        &METHOD_BENCHMARK_SERVICE_STREAMING_BOTH_WAYS,
        move |ctx, req, resp| instance.streaming_both_ways(ctx, req, resp),
    );
    builder.build()
}

const METHOD_WORKER_SERVICE_RUN_SERVER: ::grpcio::Method<
    super::control::ServerArgs,
    super::control::ServerStatus,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.WorkerService/RunServer",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_WORKER_SERVICE_RUN_CLIENT: ::grpcio::Method<
    super::control::ClientArgs,
    super::control::ClientStatus,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.WorkerService/RunClient",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_WORKER_SERVICE_CORE_COUNT: ::grpcio::Method<
    super::control::CoreRequest,
    super::control::CoreResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.WorkerService/CoreCount",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
};

const METHOD_WORKER_SERVICE_QUIT_WORKER: ::grpcio::Method<
    super::control::Void,
    super::control::Void,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.WorkerService/QuitWorker",
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
pub struct WorkerServiceClient {
    pub client: ::grpcio::Client,
}

impl WorkerServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        WorkerServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn run_server_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::control::ServerArgs>,
        ::grpcio::ClientDuplexReceiver<super::control::ServerStatus>,
    )> {
        self.client
            .duplex_streaming(&METHOD_WORKER_SERVICE_RUN_SERVER, opt)
    }

    pub fn run_server(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::control::ServerArgs>,
        ::grpcio::ClientDuplexReceiver<super::control::ServerStatus>,
    )> {
        self.run_server_opt(::grpcio::CallOption::default())
    }

    pub fn run_client_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::control::ClientArgs>,
        ::grpcio::ClientDuplexReceiver<super::control::ClientStatus>,
    )> {
        self.client
            .duplex_streaming(&METHOD_WORKER_SERVICE_RUN_CLIENT, opt)
    }

    pub fn run_client(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<super::control::ClientArgs>,
        ::grpcio::ClientDuplexReceiver<super::control::ClientStatus>,
    )> {
        self.run_client_opt(::grpcio::CallOption::default())
    }

    pub fn core_count_opt(
        &self,
        req: &super::control::CoreRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::control::CoreResponse> {
        self.client
            .unary_call(&METHOD_WORKER_SERVICE_CORE_COUNT, req, opt)
    }

    pub fn core_count(
        &self,
        req: &super::control::CoreRequest,
    ) -> ::grpcio::Result<super::control::CoreResponse> {
        self.core_count_opt(req, ::grpcio::CallOption::default())
    }

    pub fn core_count_async_opt(
        &self,
        req: &super::control::CoreRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::control::CoreResponse>> {
        self.client
            .unary_call_async(&METHOD_WORKER_SERVICE_CORE_COUNT, req, opt)
    }

    pub fn core_count_async(
        &self,
        req: &super::control::CoreRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::control::CoreResponse>> {
        self.core_count_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn quit_worker_opt(
        &self,
        req: &super::control::Void,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::control::Void> {
        self.client
            .unary_call(&METHOD_WORKER_SERVICE_QUIT_WORKER, req, opt)
    }

    pub fn quit_worker(
        &self,
        req: &super::control::Void,
    ) -> ::grpcio::Result<super::control::Void> {
        self.quit_worker_opt(req, ::grpcio::CallOption::default())
    }

    pub fn quit_worker_async_opt(
        &self,
        req: &super::control::Void,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::control::Void>> {
        self.client
            .unary_call_async(&METHOD_WORKER_SERVICE_QUIT_WORKER, req, opt)
    }

    pub fn quit_worker_async(
        &self,
        req: &super::control::Void,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::control::Void>> {
        self.quit_worker_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait WorkerService {
    fn run_server(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<super::control::ServerArgs>,
        sink: ::grpcio::DuplexSink<super::control::ServerStatus>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn run_client(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<super::control::ClientArgs>,
        sink: ::grpcio::DuplexSink<super::control::ClientStatus>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn core_count(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::control::CoreRequest,
        sink: ::grpcio::UnarySink<super::control::CoreResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn quit_worker(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::control::Void,
        sink: ::grpcio::UnarySink<super::control::Void>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_worker_service<S: WorkerService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder
        .add_duplex_streaming_handler(&METHOD_WORKER_SERVICE_RUN_SERVER, move |ctx, req, resp| {
            instance.run_server(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder
        .add_duplex_streaming_handler(&METHOD_WORKER_SERVICE_RUN_CLIENT, move |ctx, req, resp| {
            instance.run_client(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder
        .add_unary_handler(&METHOD_WORKER_SERVICE_CORE_COUNT, move |ctx, req, resp| {
            instance.core_count(ctx, req, resp)
        });
    let mut instance = s;
    builder = builder
        .add_unary_handler(&METHOD_WORKER_SERVICE_QUIT_WORKER, move |ctx, req, resp| {
            instance.quit_worker(ctx, req, resp)
        });
    builder.build()
}

const METHOD_REPORT_QPS_SCENARIO_SERVICE_REPORT_SCENARIO: ::grpcio::Method<
    super::control::ScenarioResult,
    super::control::Void,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.ReportQpsScenarioService/ReportScenario",
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
pub struct ReportQpsScenarioServiceClient {
    pub client: ::grpcio::Client,
}

impl ReportQpsScenarioServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ReportQpsScenarioServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn report_scenario_opt(
        &self,
        req: &super::control::ScenarioResult,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::control::Void> {
        self.client.unary_call(
            &METHOD_REPORT_QPS_SCENARIO_SERVICE_REPORT_SCENARIO,
            req,
            opt,
        )
    }

    pub fn report_scenario(
        &self,
        req: &super::control::ScenarioResult,
    ) -> ::grpcio::Result<super::control::Void> {
        self.report_scenario_opt(req, ::grpcio::CallOption::default())
    }

    pub fn report_scenario_async_opt(
        &self,
        req: &super::control::ScenarioResult,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::control::Void>> {
        self.client.unary_call_async(
            &METHOD_REPORT_QPS_SCENARIO_SERVICE_REPORT_SCENARIO,
            req,
            opt,
        )
    }

    pub fn report_scenario_async(
        &self,
        req: &super::control::ScenarioResult,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::control::Void>> {
        self.report_scenario_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait ReportQpsScenarioService {
    fn report_scenario(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: super::control::ScenarioResult,
        sink: ::grpcio::UnarySink<super::control::Void>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_report_qps_scenario_service<S: ReportQpsScenarioService + Send + Clone + 'static>(
    s: S,
) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(
        &METHOD_REPORT_QPS_SCENARIO_SERVICE_REPORT_SCENARIO,
        move |ctx, req, resp| instance.report_scenario(ctx, req, resp),
    );
    builder.build()
}
