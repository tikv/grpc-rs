// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


use grpc_proto::testing::services_grpc::BenchmarkService;
use grpc_proto::testing::messages::{SimpleRequest, SimpleResponse};
use grpc_proto::util;
use grpc::{self, DuplexSink, Method, MethodType, RequestStream, RpcContext, ServiceBuilder,
           UnarySink};
use futures::{Future, Sink, Stream};
use futures_cpupool::CpuPool;

fn gen_resp(req: SimpleRequest) -> SimpleResponse {
    let payload = util::new_payload(req.get_response_size() as usize);
    let mut resp = SimpleResponse::new();
    resp.set_payload(payload);
    resp
}

#[derive(Clone)]
pub struct Benchmark {
    pool: CpuPool,
}

impl Benchmark {
    pub fn new(pool: CpuPool) -> Benchmark {
        Benchmark { pool: pool }
    }
}

impl BenchmarkService for Benchmark {
    fn unary_call(&self, _: RpcContext, req: SimpleRequest, sink: UnarySink<SimpleResponse>) {
        let resp = gen_resp(req);
        self.pool
            .spawn(sink.success(resp)
                       .map_err(|e| println!("failed to handle unary: {:?}", e)))
            .forget();
    }

    fn streaming_call(&self,
                      _: RpcContext,
                      stream: RequestStream<SimpleRequest>,
                      sink: DuplexSink<SimpleResponse>) {
        self.pool
            .spawn(sink.send_all(stream.map(gen_resp))
                       .map_err(|e| println!("failed to handle streaming: {:?}", e))
                       .map(|_| {}))
            .forget()
    }
}

#[derive(Clone)]
pub struct Generic {
    pool: CpuPool,
}

impl Generic {
    pub fn new(pool: CpuPool) -> Generic {
        Generic { pool: pool }
    }

    pub fn streaming_call(&self,
                          _: RpcContext,
                          stream: RequestStream<Vec<u8>>,
                          sink: DuplexSink<Vec<u8>>) {
        self.pool
            .spawn(sink.send_all(stream.map(|req| req))
                       .map_err(|e| println!("failed to handle streaming: {:?}", e))
                       .map(|_| {}))
            .forget()
    }
}

#[inline]
pub fn bin_ser(t: &Vec<u8>, buf: &mut Vec<u8>) {
    buf.extend_from_slice(t)
}

#[inline]
pub fn bin_de(buf: &[u8]) -> grpc::Result<Vec<u8>> {
    Ok(buf.to_vec())
}

pub const METHOD_BENCHMARK_SERVICE_GENERIC_CALL: Method<Vec<u8>, Vec<u8>> = Method {
    ty: MethodType::Duplex,
    name: "/grpc.testing.BenchmarkService/StreamingCall",
    req_mar: ::grpc::Marshaller {
        ser: bin_ser,
        de: bin_de,
    },
    resp_mar: ::grpc::Marshaller {
        ser: bin_ser,
        de: bin_de,
    },
};

pub fn create_generic_service(s: Generic) -> ::grpc::Service {
    ServiceBuilder::new()
        .add_duplex_streaming_handler(&METHOD_BENCHMARK_SERVICE_GENERIC_CALL,
                                      move |ctx, req, resp| s.streaming_call(ctx, req, resp))
        .build()
}
