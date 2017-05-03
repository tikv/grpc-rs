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
use tokio_core::reactor::Remote;
use grpc::{RpcContext, UnaryResponseSink, ResponseSink, RequestStream};
use futures::{future, Future, Sink, Stream};

fn gen_resp(req: SimpleRequest) -> SimpleResponse {
    let payload = util::new_payload(req.get_response_size() as usize);
    let mut resp = SimpleResponse::new();
    resp.set_payload(payload);
    resp
}

#[derive(Clone)]
pub struct Benchmark {
    remote: Remote,
}

impl Benchmark {
    pub fn new(remote: Remote) -> Benchmark {
        Benchmark {
            remote: remote,
        }
    }
}

impl BenchmarkService for Benchmark {
    fn unary_call(&self, _: RpcContext, req: SimpleRequest, sink: UnaryResponseSink<SimpleResponse>) {
        let resp = gen_resp(req);
        self.remote.spawn(|_| future::result(sink.success(resp)).flatten().map_err(|e| println!("failed to handle unary: {:?}", e)))
    }

    fn streaming_call(&self, _: RpcContext, stream: RequestStream<SimpleRequest>, sink: ResponseSink<SimpleResponse>) {
        self.remote.spawn(|_| {
            sink.send_all(stream.map(gen_resp)).map_err(|e| println!("failed to handle streaming: {:?}", e)).map(|_| {})
        })
    }
}
