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


use grpc::{self, ClientStreamingSink, DuplexSink, RequestStream, RpcContext, RpcStatus,
           RpcStatusCode, ServerStreamingSink, UnarySink};
use futures::{Async, Future, Poll, Sink, Stream, future, stream};
use futures_cpupool::CpuPool;

use grpc_proto::testing::test_grpc::TestService;
use grpc_proto::testing::empty::Empty;
use grpc_proto::testing::messages::{SimpleRequest, SimpleResponse, StreamingInputCallRequest,
                                    StreamingInputCallResponse, StreamingOutputCallRequest,
                                    StreamingOutputCallResponse};
use grpc_proto::util;

enum Error {
    Grpc(grpc::Error),
    Abort,
}

impl From<grpc::Error> for Error {
    fn from(error: grpc::Error) -> Error {
        Error::Grpc(error)
    }
}

#[derive(Clone)]
pub struct InteropTestService {
    pool: CpuPool,
}

impl InteropTestService {
    pub fn new(pool: CpuPool) -> InteropTestService {
        InteropTestService { pool: pool }
    }
}

impl TestService for InteropTestService {
    fn empty_call(&self, _: RpcContext, _: Empty, resp: UnarySink<Empty>) {
        let res = Empty::new();
        let f = resp.success(res)
            .map_err(|e| panic!("failed to send response: {:?}", e));
        self.pool.spawn(f).forget()
    }

    fn unary_call(&self, _: RpcContext, mut req: SimpleRequest, sink: UnarySink<SimpleResponse>) {
        if req.has_response_status() {
            let code = req.get_response_status().get_code();
            let msg = Some(req.take_response_status().take_message());
            let status = RpcStatus::new(code.into(), msg);
            let f = sink.fail(status)
                .map_err(|e| panic!("failed to send response: {:?}", e));
            self.pool.spawn(f).forget();
            return;
        }
        let resp_size = req.get_response_size();
        let mut resp = SimpleResponse::new();
        resp.set_payload(util::new_payload(resp_size as usize));
        let f = sink.success(resp)
            .map_err(|e| panic!("failed to send response: {:?}", e));
        self.pool.spawn(f).forget()
    }

    fn cacheable_unary_call(&self, _: RpcContext, _: SimpleRequest, _: UnarySink<SimpleResponse>) {
        unimplemented!()
    }

    fn streaming_output_call(&self,
                             _: RpcContext,
                             req: StreamingOutputCallRequest,
                             sink: ServerStreamingSink<StreamingOutputCallResponse>) {
        let resps: Vec<Result<_, grpc::Error>> = req.get_response_parameters()
            .into_iter()
            .map(|param| {
                let mut resp = StreamingOutputCallResponse::new();
                resp.set_payload(util::new_payload(param.get_size() as usize));
                Ok(resp)
            })
            .collect();
        let f = sink.send_all(stream::iter(resps))
            .map(|_| {})
            .map_err(|e| panic!("failed to send response: {:?}", e));
        self.pool.spawn(f).forget()
    }

    fn streaming_input_call(&self,
                            _: RpcContext,
                            stream: RequestStream<StreamingInputCallRequest>,
                            sink: ClientStreamingSink<StreamingInputCallResponse>) {
        let f = stream
            .fold(0,
                  |s, req| Ok(s + req.get_payload().get_body().len()) as grpc::Result<_>)
            .and_then(|s| {
                let mut resp = StreamingInputCallResponse::new();
                resp.set_aggregated_payload_size(s as i32);
                sink.success(resp)
            })
            .map_err(|e| match e {
                         grpc::Error::RemoteStopped => {}
                         e => println!("failed to send streaming inptu: {:?}", e),
                     });
        self.pool.spawn(f).forget()
    }

    fn full_duplex_call(&self,
                        _: RpcContext,
                        stream: RequestStream<StreamingOutputCallRequest>,
                        sink: DuplexSink<StreamingOutputCallResponse>) {
        let f = stream
            .map_err(Error::Grpc)
            .fold(sink, |sink, mut req| {
                let mut failure = None;
                let mut send = None;
                if req.has_response_status() {
                    let code = req.get_response_status().get_code();
                    let msg = Some(req.take_response_status().take_message());
                    let status = RpcStatus::new(code.into(), msg);
                    failure = Some(sink.fail(status));
                } else {
                    let mut resp = StreamingOutputCallResponse::new();
                    if let Some(param) = req.get_response_parameters().get(0) {
                        resp.set_payload(util::new_payload(param.get_size() as usize));
                    }
                    send = Some(sink.send(resp));
                }
                future::poll_fn(move || -> Poll<DuplexSink<StreamingOutputCallResponse>, Error> {
                    if let Some(ref mut send) = send {
                        let sink = try_ready!(send.poll());
                        Ok(Async::Ready(sink))
                    } else {
                        try_ready!(failure.as_mut().unwrap().poll());
                        Err(Error::Abort)
                    }
                })
            })
            .and_then(|mut sink| future::poll_fn(move || sink.close().map_err(Error::from)))
            .map_err(|e| match e {
                         Error::Grpc(grpc::Error::RemoteStopped) |
                         Error::Abort => {}
                         Error::Grpc(e) => println!("failed to handle duplex call: {:?}", e),
                     });
        self.pool.spawn(f).forget()
    }

    fn half_duplex_call(&self,
                        _: RpcContext,
                        _: RequestStream<StreamingOutputCallRequest>,
                        _: DuplexSink<StreamingOutputCallResponse>) {
        unimplemented!()
    }

    fn unimplemented_call(&self, _: RpcContext, _: Empty, sink: UnarySink<Empty>) {
        let f = sink.fail(RpcStatus::new(RpcStatusCode::Unimplemented, None))
            .map_err(|e| println!("failed to report unimplemented method: {:?}", e));
        self.pool.spawn(f).forget()
    }
}
