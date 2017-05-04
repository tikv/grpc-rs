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


use grpc::{self, ClientStreamingResponseSink, RequestStream, ResponseSink, RpcContext, RpcStatus,
           UnaryResponseSink};
use tokio_core::reactor::Remote;
use futures::{Async, Future, Poll, Sink, Stream, future, stream};

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
    remote: Remote,
}

impl InteropTestService {
    pub fn new(remote: Remote) -> InteropTestService {
        InteropTestService { remote: remote }
    }
}

impl TestService for InteropTestService {
    fn empty_call(&self, _: RpcContext, _: Empty, resp: UnaryResponseSink<Empty>) {
        self.remote
            .spawn(move |_| {
                       let res = Empty::new();
                       resp.success(res)
                           .map_err(|e| panic!("failed to send response: {:?}", e))
                   })
    }

    fn unary_call(&self,
                  _: RpcContext,
                  mut req: SimpleRequest,
                  sink: UnaryResponseSink<SimpleResponse>) {
        if req.has_response_status() {
            let code = req.get_response_status().get_code();
            let msg = Some(req.take_response_status().take_message());
            let status = RpcStatus::new(code.into(), msg);
            self.remote
                .spawn(|_| {
                           sink.fail(status)
                               .map_err(|e| panic!("failed to send response: {:?}", e))
                       });
            return;
        }
        let resp_size = req.get_response_size();
        let mut resp = SimpleResponse::new();
        resp.set_payload(util::new_payload(resp_size as usize));
        self.remote
            .spawn(|_| {
                       sink.success(resp)
                           .map_err(|e| panic!("failed to send response: {:?}", e))
                   })
    }

    fn cacheable_unary_call(&self,
                            _: RpcContext,
                            _: SimpleRequest,
                            _: UnaryResponseSink<SimpleResponse>) {
        unimplemented!()
    }

    fn streaming_output_call(&self,
                             _: RpcContext,
                             req: StreamingOutputCallRequest,
                             sink: ResponseSink<StreamingOutputCallResponse>) {
        let resps: Vec<Result<_, grpc::Error>> = req.get_response_parameters()
            .into_iter()
            .map(|param| {
                     let mut resp = StreamingOutputCallResponse::new();
                     resp.set_payload(util::new_payload(param.get_size() as usize));
                     Ok(resp)
                 })
            .collect();
        self.remote
            .spawn(|_| {
                       sink.send_all(stream::iter(resps))
                           .map(|_| {})
                           .map_err(|e| panic!("failed to send response: {:?}", e))
                   })
    }

    fn streaming_input_call(&self,
                            _: RpcContext,
                            stream: RequestStream<StreamingInputCallRequest>,
                            sink: ClientStreamingResponseSink<StreamingInputCallResponse>) {
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
        self.remote.spawn(|_| f)
    }

    fn full_duplex_call(&self,
                        _: RpcContext,
                        stream: RequestStream<StreamingOutputCallRequest>,
                        sink: ResponseSink<StreamingOutputCallResponse>) {
        self.remote
            .spawn(|_| {
                stream.map_err(Error::Grpc).fold(sink, |sink, mut req| {
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
                future::poll_fn(move || -> Poll<ResponseSink<StreamingOutputCallResponse>, Error> {
                    if let Some(ref mut send) = send {
                        let sink = try_ready!(send.poll());
                        Ok(Async::Ready(sink))
                    } else {
                        try_ready!(failure.as_mut().unwrap().poll());
                        Err(Error::Abort)
                    }
                })
            }).and_then(|mut sink| {
                future::poll_fn(move || sink.close().map_err(From::from))
            }).or_else(|e| {
                match e {
                    Error::Grpc(grpc::Error::RemoteStopped) | Error::Abort => {}
                    Error::Grpc(e) => println!("failed to handle duplex call: {:?}", e),
                }
                Ok(())
            })
            })
    }

    fn half_duplex_call(&self,
                        _: RpcContext,
                        _: RequestStream<StreamingOutputCallRequest>,
                        _: ResponseSink<StreamingOutputCallResponse>) {
        unimplemented!()
    }

    fn unimplemented_call(&self, _: RpcContext, _: Empty, _: UnaryResponseSink<Empty>) {
        unimplemented!()
    }
}
