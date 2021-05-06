// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::time::Duration;

use crate::grpc::{self, CallOption, Channel, RpcStatusCode, WriteFlags};
use futures::prelude::*;

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

    pub async fn empty_unary(&self) -> grpcio::Result<()> {
        print!("testing empty unary ... ");
        let req = Empty::default();
        let resp = self.client.empty_call_async(&req)?.await?;
        assert_eq!(req, resp);
        println!("pass");
        Ok(())
    }

    pub async fn large_unary(&self) -> grpcio::Result<()> {
        print!("testing large unary ... ");
        let mut req = SimpleRequest::default();
        req.set_response_size(314_159);
        req.set_payload(util::new_payload(271_828));
        let resp = self.client.unary_call_async(&req)?.await?;
        assert_eq!(314_159, resp.get_payload().get_body().len());
        println!("pass");
        Ok(())
    }

    pub async fn client_streaming(&self) -> grpcio::Result<()> {
        print!("testing client streaming ... ");
        let payload_size = vec![27182usize, 8, 1828, 45904];
        let (mut sender, receiver) = self.client.streaming_input_call()?;
        for size in payload_size {
            let mut req = StreamingInputCallRequest::default();
            req.set_payload(util::new_payload(size));
            sender.send((req, WriteFlags::default())).await?;
        }
        sender.close().await?;
        let resp = receiver.await?;
        assert_eq!(74922, resp.get_aggregated_payload_size());
        println!("pass");
        Ok(())
    }

    pub async fn server_streaming(&self) -> grpcio::Result<()> {
        print!("testing server streaming ... ");
        let mut req = StreamingOutputCallRequest::default();
        let sizes = vec![31415, 9, 2653, 58979];
        for size in &sizes {
            req.mut_response_parameters()
                .push(util::new_parameters(*size));
        }
        let mut resp = self.client.streaming_output_call(&req)?;
        let mut i = 0;
        while let Some(r) = resp.try_next().await? {
            assert_eq!(r.get_payload().get_body().len(), sizes[i] as usize);
            i += 1;
        }
        assert_eq!(sizes.len(), i);
        println!("pass");
        Ok(())
    }

    pub async fn ping_pong(&self) -> grpcio::Result<()> {
        print!("testing ping pong ... ");
        let (mut sender, mut receiver) = self.client.full_duplex_call()?;
        let cases = vec![(31415, 27182), (9, 8), (2653, 1828), (58979, 45904)];
        for (resp_size, payload_size) in cases {
            let mut req = StreamingOutputCallRequest::default();
            req.mut_response_parameters()
                .push(util::new_parameters(resp_size));
            req.set_payload(util::new_payload(payload_size));
            sender.send((req, WriteFlags::default())).await?;
            let resp = receiver.try_next().await?.unwrap();
            assert_eq!(resp.get_payload().get_body().len(), resp_size as usize);
        }
        sender.close().await?;
        assert_eq!(receiver.try_next().await?, None);
        println!("pass");
        Ok(())
    }

    pub async fn empty_stream(&self) -> grpcio::Result<()> {
        print!("testing empty stream ... ");
        let (mut sender, mut receiver) = self.client.full_duplex_call()?;
        sender.close().await?;
        assert_eq!(receiver.try_next().await?, None);
        println!("pass");
        Ok(())
    }

    pub async fn cancel_after_begin(&self) -> grpcio::Result<()> {
        print!("testing cancel_after_begin ... ");
        let (mut sender, receiver) = self.client.streaming_input_call()?;
        // so request has been sent.
        futures_timer::Delay::new(Duration::from_millis(10)).await;
        sender.cancel();
        match receiver.await.unwrap_err() {
            grpc::Error::RpcFailure(s) => assert_eq!(s.code(), RpcStatusCode::CANCELLED),
            e => panic!("expected cancel, but got: {:?}", e),
        }
        println!("pass");
        Ok(())
    }

    pub async fn cancel_after_first_response(&self) -> grpcio::Result<()> {
        print!("testing cancel_after_first_response ... ");
        let (mut sender, mut receiver) = self.client.full_duplex_call()?;
        let mut req = StreamingOutputCallRequest::default();
        req.mut_response_parameters()
            .push(util::new_parameters(31415));
        req.set_payload(util::new_payload(27182));
        sender.send((req, WriteFlags::default())).await?;
        let resp = receiver.try_next().await?.unwrap();
        assert_eq!(resp.get_payload().get_body().len(), 31415);
        sender.cancel();
        match receiver.try_next().await {
            Err(grpc::Error::RpcFailure(s)) => assert_eq!(s.code(), RpcStatusCode::CANCELLED),
            Err(e) => panic!("expected cancel, but got: {:?}", e),
            Ok(r) => panic!("expected error, but got: {:?}", r),
        }
        println!("pass");
        Ok(())
    }

    pub async fn timeout_on_sleeping_server(&self) -> grpcio::Result<()> {
        print!("testing timeout_of_sleeping_server ... ");
        let opt = CallOption::default().timeout(Duration::from_millis(1));
        let (mut sender, mut receiver) = self.client.full_duplex_call_opt(opt)?;
        let mut req = StreamingOutputCallRequest::default();
        req.set_payload(util::new_payload(27182));
        let _ = sender.send((req, WriteFlags::default())).await;
        match receiver.try_next().await {
            Err(grpc::Error::RpcFailure(s)) => {
                assert_eq!(s.code(), RpcStatusCode::DEADLINE_EXCEEDED)
            }
            Err(e) => panic!("expected timeout, but got: {:?}", e),
            Ok(r) => panic!("expected error: {:?}", r),
        }
        println!("pass");
        Ok(())
    }

    pub async fn status_code_and_message(&self) -> grpcio::Result<()> {
        print!("testing status_code_and_message ... ");
        let error_msg = "test status message";
        let mut status = EchoStatus::default();
        status.set_code(2);
        status.set_message(error_msg.to_owned());
        let mut req = SimpleRequest::default();
        req.set_response_status(status.clone());
        match self.client.unary_call_async(&req)?.await.unwrap_err() {
            grpc::Error::RpcFailure(s) => {
                assert_eq!(s.code(), RpcStatusCode::UNKNOWN);
                assert_eq!(s.message(), error_msg);
            }
            e => panic!("expected rpc failure: {:?}", e),
        }
        let mut req = StreamingOutputCallRequest::default();
        req.set_response_status(status);
        let (mut sender, mut receiver) = self.client.full_duplex_call()?;
        let _ = sender.send((req, WriteFlags::default())).await;
        match receiver.try_next().await {
            Err(grpc::Error::RpcFailure(s)) => {
                assert_eq!(s.code(), RpcStatusCode::UNKNOWN);
                assert_eq!(s.message(), error_msg);
            }
            Err(e) => panic!("expected rpc failure: {:?}", e),
            Ok(r) => panic!("error expected, but got: {:?}", r),
        }
        println!("pass");
        Ok(())
    }

    pub async fn unimplemented_method(&self) -> grpcio::Result<()> {
        print!("testing unimplemented_method ... ");
        match self
            .client
            .unimplemented_call_async(&Empty::default())?
            .await
            .unwrap_err()
        {
            grpc::Error::RpcFailure(s) => assert_eq!(s.code(), RpcStatusCode::UNIMPLEMENTED),
            e => panic!("expected rpc failure: {:?}", e),
        }
        println!("pass");
        Ok(())
    }

    pub async fn unimplemented_service(&self) -> grpcio::Result<()> {
        print!("testing unimplemented_service ... ");
        let client = UnimplementedServiceClient::new(self.channel.clone());
        match client
            .unimplemented_call_async(&Empty::default())?
            .await
            .unwrap_err()
        {
            grpc::Error::RpcFailure(s) => assert_eq!(s.code(), RpcStatusCode::UNIMPLEMENTED),
            e => panic!("expected rpc failure: {:?}", e),
        }
        println!("pass");
        Ok(())
    }

    pub async fn test_all(&self) -> grpcio::Result<()> {
        self.empty_unary().await?;
        self.large_unary().await?;
        self.client_streaming().await?;
        self.server_streaming().await?;
        self.ping_pong().await?;
        self.empty_stream().await?;
        self.cancel_after_begin().await?;
        self.cancel_after_first_response().await?;
        self.timeout_on_sleeping_server().await?;
        self.status_code_and_message().await?;
        self.unimplemented_method().await?;
        self.unimplemented_service().await?;
        Ok(())
    }
}
