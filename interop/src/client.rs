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

use futures::{future, stream, Future, Sink, Stream};
use grpc::{self, CallOption, Channel, RpcStatusCode, WriteFlags};

use grpc_proto::testing::empty::Empty;
use grpc_proto::testing::messages::{
    EchoStatus, SimpleRequest, StreamingInputCallRequest, StreamingOutputCallRequest,
};
use grpc_proto::testing::test_grpc::{TestServiceClient, UnimplementedServiceClient};
use grpc_proto::util;

pub struct Client {
    channel: Channel,
    client: TestServiceClient,
}

impl Client {
    pub fn new(ch: Channel) -> Client {
        Client {
            channel: ch.clone(),
            client: TestServiceClient::new(ch),
        }
    }

    pub fn empty_unary(&self) {
        print!("testing empty unary ... ");
        let req = Empty::new();
        let resp = self.client.empty_call(&req).unwrap();
        assert_eq!(req, resp);
        println!("pass");
    }

    pub fn large_unary(&self) {
        print!("testing large unary ... ");
        let mut req = SimpleRequest::new();
        req.set_response_size(314_159);
        req.set_payload(util::new_payload(271_828));
        let resp = self.client.unary_call(&req).unwrap();
        assert_eq!(314_159, resp.get_payload().get_body().len());
        println!("pass");
    }

    pub fn client_streaming(&self) {
        print!("testing client streaming ... ");
        let reqs = vec![27182, 8, 1828, 45904].into_iter().map(|s| {
            let mut req = StreamingInputCallRequest::new();
            req.set_payload(util::new_payload(s));
            (req, WriteFlags::default())
        });
        let (sender, receiver) = self.client.streaming_input_call().unwrap();
        // Keep the sender, so that receiver will not receive Cancelled error.
        let _sender = sender
            .send_all(stream::iter_ok::<_, grpc::Error>(reqs))
            .wait()
            .unwrap();
        let resp = receiver.wait().unwrap();
        assert_eq!(74922, resp.get_aggregated_payload_size());
        println!("pass");
    }

    pub fn server_streaming(&self) {
        print!("testing server streaming ... ");
        let mut req = StreamingOutputCallRequest::new();
        let sizes = vec![31415, 9, 2653, 58979];
        for size in &sizes {
            req.mut_response_parameters()
                .push(util::new_parameters(*size));
        }
        let resp = self.client.streaming_output_call(&req).unwrap();
        let resp_sizes = resp
            .map(|r| r.get_payload().get_body().len() as i32)
            .collect()
            .wait()
            .unwrap();
        assert_eq!(resp_sizes, sizes);
        println!("pass");
    }

    pub fn ping_pong(&self) {
        print!("testing ping pong ... ");
        let (mut sender, mut receiver) = self.client.full_duplex_call().unwrap();
        let cases = vec![(31415, 27182), (9, 8), (2653, 1828), (58979, 45904)];
        for (resp_size, payload_size) in cases {
            let mut req = StreamingOutputCallRequest::new();
            req.mut_response_parameters()
                .push(util::new_parameters(resp_size));
            req.set_payload(util::new_payload(payload_size));
            sender = sender.send((req, WriteFlags::default())).wait().unwrap();
            let resp = match receiver.into_future().wait() {
                Ok((resp, recv)) => {
                    receiver = recv;
                    resp.unwrap()
                }
                Err((e, _)) => panic!("{:?}", e),
            };
            assert_eq!(resp.get_payload().get_body().len(), resp_size as usize);
        }
        future::poll_fn(|| sender.close()).wait().unwrap();
        match receiver.into_future().wait() {
            Ok((resp, _)) => assert!(resp.is_none()),
            Err((e, _)) => panic!("{:?}", e),
        }
        println!("pass");
    }

    pub fn empty_stream(&self) {
        print!("testing empty stream ... ");
        let (mut sender, receiver) = self.client.full_duplex_call().unwrap();
        future::poll_fn(|| sender.close()).wait().unwrap();
        let resps = receiver.collect().wait().unwrap();
        assert!(resps.is_empty());
        println!("pass");
    }

    pub fn cancel_after_begin(&self) {
        print!("testing cancel_after_begin ... ");
        let (mut sender, receiver) = self.client.streaming_input_call().unwrap();
        // so request has been sent.
        thread::sleep(Duration::from_millis(10));
        sender.cancel();
        match receiver.wait().unwrap_err() {
            grpc::Error::RpcFailure(s) => assert_eq!(s.status, RpcStatusCode::Cancelled),
            e => panic!("expected cancel, but got: {:?}", e),
        }
        println!("pass");
    }

    pub fn cancel_after_first_response(&self) {
        print!("testing cancel_after_first_response ... ");
        let (mut sender, mut receiver) = self.client.full_duplex_call().unwrap();
        let mut req = StreamingOutputCallRequest::new();
        req.mut_response_parameters()
            .push(util::new_parameters(31415));
        req.set_payload(util::new_payload(27182));
        sender = sender.send((req, WriteFlags::default())).wait().unwrap();
        let resp = match receiver.into_future().wait() {
            Ok((r, recv)) => {
                receiver = recv;
                r.unwrap()
            }
            Err((e, _)) => panic!("{:?}", e),
        };

        assert_eq!(resp.get_payload().get_body().len(), 31415);
        sender.cancel();
        match receiver.into_future().wait() {
            Err((grpc::Error::RpcFailure(s), _)) => assert_eq!(s.status, RpcStatusCode::Cancelled),
            Err((e, _)) => panic!("expected cancel, but got: {:?}", e),
            Ok((r, _)) => panic!("expected error, but got: {:?}", r),
        }
        println!("pass");
    }

    pub fn timeout_on_sleeping_server(&self) {
        print!("testing timeout_of_sleeping_server ... ");
        let opt = CallOption::default().timeout(Duration::from_millis(1));
        let (sender, receiver) = self.client.full_duplex_call_opt(opt).unwrap();
        let mut req = StreamingOutputCallRequest::new();
        req.set_payload(util::new_payload(27182));
        // Keep the sender, so that receiver will not receive Cancelled error.
        let _sender = sender.send((req, WriteFlags::default())).wait();
        match receiver.into_future().wait() {
            Err((grpc::Error::RpcFailure(s), _)) => {
                assert_eq!(s.status, RpcStatusCode::DeadlineExceeded)
            }
            Err((e, _)) => panic!("expected timeout, but got: {:?}", e),
            Ok((r, _)) => panic!("expected error: {:?}", r),
        }
        println!("pass");
    }

    pub fn status_code_and_message(&self) {
        print!("testing status_code_and_message ... ");
        let error_msg = "test status message";
        let mut status = EchoStatus::new();
        status.set_code(2);
        status.set_message(error_msg.to_owned());
        let mut req = SimpleRequest::new();
        req.set_response_status(status.clone());
        match self.client.unary_call(&req).unwrap_err() {
            grpc::Error::RpcFailure(s) => {
                assert_eq!(s.status, RpcStatusCode::Unknown);
                assert_eq!(s.details.as_ref().unwrap(), error_msg);
            }
            e => panic!("expected rpc failure: {:?}", e),
        }
        let mut req = StreamingOutputCallRequest::new();
        req.set_response_status(status);
        let (sender, receiver) = self.client.full_duplex_call().unwrap();
        // Keep the sender, so that receiver will not receive Cancelled error.
        let _sender = sender.send((req, WriteFlags::default())).wait();
        match receiver.into_future().wait() {
            Err((grpc::Error::RpcFailure(s), _)) => {
                assert_eq!(s.status, RpcStatusCode::Unknown);
                assert_eq!(s.details.as_ref().unwrap(), error_msg);
            }
            Err((e, _)) => panic!("expected rpc failure: {:?}", e),
            Ok((r, _)) => panic!("error expected, but got: {:?}", r),
        }
        println!("pass");
    }

    pub fn unimplemented_method(&self) {
        print!("testing unimplemented_method ... ");
        match self.client.unimplemented_call(&Empty::new()).unwrap_err() {
            grpc::Error::RpcFailure(s) => assert_eq!(s.status, RpcStatusCode::Unimplemented),
            e => panic!("expected rpc failure: {:?}", e),
        }
        println!("pass");
    }

    pub fn unimplemented_service(&self) {
        print!("testing unimplemented_service ... ");
        let client = UnimplementedServiceClient::new(self.channel.clone());
        match client.unimplemented_call(&Empty::new()).unwrap_err() {
            grpc::Error::RpcFailure(s) => assert_eq!(s.status, RpcStatusCode::Unimplemented),
            e => panic!("expected rpc failure: {:?}", e),
        }
        println!("pass");
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
        self.unimplemented_method();
        self.unimplemented_service();
    }
}
