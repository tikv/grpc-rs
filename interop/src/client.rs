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


use std::thread;
use std::time::Duration;

use grpc::{self, CallOption, Channel};
use grpc_sys::GrpcStatusCode;
use futures::{Future, Sink, Stream, future, stream};

use proto::testing::test_grpc::TestServiceClient;
use proto::testing::empty::Empty;
use proto::testing::messages::{EchoStatus, SimpleRequest, StreamingInputCallRequest,
                               StreamingOutputCallRequest};
use util;

pub struct Client {
    client: TestServiceClient,
}

impl Client {
    pub fn new(ch: Channel) -> Client {
        Client { client: TestServiceClient::new(ch) }
    }

    pub fn empty_unary(&self) {
        let req = Empty::new();
        let resp = self.client.empty_call(req.clone()).unwrap();
        assert_eq!(req, resp);
    }

    pub fn large_unary(&self) {
        let mut req = SimpleRequest::new();
        req.set_response_size(314159);
        req.set_payload(util::new_payload(271828));
        let resp = self.client.unary_call(req).unwrap();
        assert_eq!(314159, resp.get_payload().get_body().len());
    }

    pub fn client_streaming(&self) {
        let reqs: Vec<grpc::Result<_>> = vec![27182, 8, 1828, 45904]
            .into_iter()
            .map(|s| {
                     let mut req = StreamingInputCallRequest::new();
                     req.set_payload(util::new_payload(s));
                     Ok(req)
                 })
            .collect();
        let mut handler = self.client.streaming_input_call().unwrap();
        handler = handler.send_all(stream::iter(reqs)).wait().unwrap().0;
        let resp = handler.into_receiver().wait().unwrap();
        assert_eq!(74922, resp.get_aggregated_payload_size());
    }

    pub fn server_streaming(&self) {
        let mut req = StreamingOutputCallRequest::new();
        let sizes = vec![31415, 9, 2653, 58979];
        for size in &sizes {
            req.mut_response_parameters()
                .push(util::new_parameters(*size));
        }
        let resp = self.client.streaming_output_call(req).unwrap();
        let resp_sizes = resp.map(|r| r.get_payload().get_body().len() as i32)
            .collect()
            .wait()
            .unwrap();
        assert_eq!(resp_sizes, sizes);
    }

    pub fn ping_pong(&self) {
        let mut handler = self.client.full_duplex_call().unwrap();
        let mut receiver = handler.take_receiver().unwrap();
        let cases = vec![(31415, 27182), (9, 8), (2653, 1828), (58979, 45904)];
        for (resp_size, payload_size) in cases {
            let mut req = StreamingOutputCallRequest::new();
            req.mut_response_parameters()
                .push(util::new_parameters(resp_size));
            req.set_payload(util::new_payload(payload_size));
            let send = handler.send(req);
            handler = send.wait().unwrap();
            let resp = match receiver.into_future().wait() {
                Ok((resp, recv)) => {
                    receiver = recv;
                    resp.unwrap()
                }
                Err((e, _)) => panic!("{:?}", e),
            };
            assert_eq!(resp.get_payload().get_body().len(), resp_size as usize);
        }
    }

    pub fn empty_stream(&self) {
        let mut handler = self.client.full_duplex_call().unwrap();
        let receiver = handler.take_receiver().unwrap();
        future::poll_fn(|| handler.close()).wait().unwrap();
        let resps = receiver.collect().wait().unwrap();
        assert!(resps.is_empty());
    }

    pub fn cancel_after_begin(&self) {
        let handler = self.client.streaming_input_call().unwrap();
        // so request has been sent.
        thread::sleep(Duration::from_millis(10));
        let receiver = handler.cancel();
        match receiver.wait().unwrap_err() {
            grpc::Error::RpcFailure(s) => assert_eq!(s.status, GrpcStatusCode::Cancelled),
            e => panic!("expected cancel, but got: {:?}", e),
        }
    }

    pub fn cancel_after_first_response(&self) {
        let mut handler = self.client.full_duplex_call().unwrap();
        let mut receiver = handler.take_receiver().unwrap();
        let mut req = StreamingOutputCallRequest::new();
        req.mut_response_parameters()
            .push(util::new_parameters(31415));
        req.set_payload(util::new_payload(27182));
        handler = handler.send(req).wait().unwrap();
        let resp = match receiver.into_future().wait() {
            Ok((r, recv)) => {
                receiver = recv;
                r.unwrap()
            }
            Err((e, _)) => panic!("{:?}", e),
        };

        assert_eq!(resp.get_payload().get_body().len(), 31415);
        handler.cancel();
        match receiver.into_future().wait() {
            Err((grpc::Error::RpcFailure(s), _)) => assert_eq!(s.status, GrpcStatusCode::Cancelled),
            Err((e, _)) => panic!("expected cancel, but got: {:?}", e),
            Ok((r, _)) => panic!("expected error, but got: {:?}", r),
        }
    }

    pub fn timeout_on_sleeping_server(&self) {
        let opt = CallOption::default().with_timeout(Duration::new(0, 100_000));
        let mut handler = self.client.full_duplex_call_opt(opt).unwrap();
        let receiver = handler.take_receiver().unwrap();
        let mut req = StreamingOutputCallRequest::new();
        req.set_payload(util::new_payload(27182));
        let _ = handler.send(req).wait();
        match receiver.into_future().wait() {
            Err((grpc::Error::RpcFailure(s), _)) => {
                assert_eq!(s.status, GrpcStatusCode::DeadlineExceeded)
            }
            Err((e, _)) => panic!("expected timeout, but got: {:?}", e),
            _ => panic!("expected error"),
        }
    }

    pub fn status_code_and_message(&self) {
        let error_msg = "test status message";
        let mut status = EchoStatus::new();
        status.set_code(2);
        status.set_message(error_msg.to_owned());
        let mut req = SimpleRequest::new();
        req.set_response_status(status.clone());
        match self.client.unary_call(req).unwrap_err() {
            grpc::Error::RpcFailure(s) => {
                assert_eq!(s.status, GrpcStatusCode::Unknown);
                assert_eq!(s.details.as_ref().unwrap(), error_msg);
            }
            e => panic!("expected rpc failure: {:?}", e),
        }
        let mut req = StreamingOutputCallRequest::new();
        req.set_response_status(status);
        let mut handler = self.client.full_duplex_call().unwrap();
        let receiver = handler.take_receiver().unwrap();
        handler.send(req).wait().unwrap();
        match receiver.into_future().wait() {
            Err((grpc::Error::RpcFailure(s), _)) => {
                assert_eq!(s.status, GrpcStatusCode::Unknown);
                assert_eq!(s.details.as_ref().unwrap(), error_msg);
            }
            Err((e, _)) => panic!("expected rpc failure: {:?}", e),
            Ok((r, _)) => panic!("error expected, but got: {:?}", r),
        }
    }

    pub fn test_all(&self) {
        self.empty_unary();
        self.large_unary();
        self.client_streaming();
        self.server_streaming();
        self.ping_pong();
        self.empty_stream();
        self.cancel_after_begin();
        self.cancel_after_first_response();
        self.timeout_on_sleeping_server();
        self.status_code_and_message();
    }
}
