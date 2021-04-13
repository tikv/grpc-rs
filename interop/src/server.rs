// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::time::Duration;

use crate::grpc::{
    self, ClientStreamingSink, DuplexSink, RequestStream, RpcContext, RpcStatus,
    ServerStreamingSink, UnarySink, WriteFlags,
};
use futures::prelude::*;
use futures_timer::Delay;

use grpc_proto::testing::empty::Empty;
use grpc_proto::testing::messages::{
    SimpleRequest, SimpleResponse, StreamingInputCallRequest, StreamingInputCallResponse,
    StreamingOutputCallRequest, StreamingOutputCallResponse,
};
use grpc_proto::testing::test_grpc::TestService;
use grpc_proto::util;

#[derive(Clone)]
pub struct InteropTestService;

impl TestService for InteropTestService {
    fn empty_call(&mut self, ctx: RpcContext, _: Empty, resp: UnarySink<Empty>) {
        let res = Empty::default();
        let f = resp
            .success(res)
            .map_err(|e| panic!("failed to send response: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn unary_call(
        &mut self,
        ctx: RpcContext,
        mut req: SimpleRequest,
        sink: UnarySink<SimpleResponse>,
    ) {
        if req.has_response_status() {
            let code = req.get_response_status().get_code();
            let msg = req.take_response_status().take_message();
            let status = RpcStatus::with_message(code, msg);
            let f = sink
                .fail(status)
                .map_err(|e| panic!("failed to send response: {:?}", e))
                .map(|_| ());
            ctx.spawn(f);
            return;
        }
        let resp_size = req.get_response_size();
        let mut resp = SimpleResponse::default();
        resp.set_payload(util::new_payload(resp_size as usize));
        let f = sink
            .success(resp)
            .map_err(|e| panic!("failed to send response: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn cacheable_unary_call(
        &mut self,
        _: RpcContext,
        _: SimpleRequest,
        _: UnarySink<SimpleResponse>,
    ) {
        unimplemented!()
    }

    fn streaming_output_call(
        &mut self,
        ctx: RpcContext,
        mut req: StreamingOutputCallRequest,
        mut sink: ServerStreamingSink<StreamingOutputCallResponse>,
    ) {
        let f = async move {
            for param in req.take_response_parameters().into_iter() {
                let mut resp = StreamingOutputCallResponse::default();
                resp.set_payload(util::new_payload(param.get_size() as usize));
                sink.send((resp, WriteFlags::default())).await?;
            }
            sink.close().await?;
            Ok(())
        }
        .map_err(|e: grpcio::Error| panic!("failed to send response: {:?}", e))
        .map(|_| ());
        ctx.spawn(f)
    }

    fn streaming_input_call(
        &mut self,
        ctx: RpcContext,
        mut stream: RequestStream<StreamingInputCallRequest>,
        sink: ClientStreamingSink<StreamingInputCallResponse>,
    ) {
        let f = async move {
            let mut s = 0;
            while let Some(req) = stream.try_next().await? {
                s += req.get_payload().get_body().len();
            }

            let mut resp = StreamingInputCallResponse::default();
            resp.set_aggregated_payload_size(s as i32);
            sink.success(resp).await
        }
        .map_err(|e| match e {
            grpc::Error::RemoteStopped => {}
            e => error!("failed to send streaming input: {:?}", e),
        })
        .map(|_| ());
        ctx.spawn(f)
    }

    fn full_duplex_call(
        &mut self,
        ctx: RpcContext,
        mut stream: RequestStream<StreamingOutputCallRequest>,
        mut sink: DuplexSink<StreamingOutputCallResponse>,
    ) {
        let f = async move {
            while let Some(mut req) = stream.try_next().await? {
                if req.has_response_status() {
                    let code = req.get_response_status().get_code();
                    let msg = req.take_response_status().take_message();
                    let status = RpcStatus::with_message(code, msg);
                    sink.fail(status).await?;
                    return Ok(());
                }

                let mut resp = StreamingOutputCallResponse::default();
                if let Some(param) = req.get_response_parameters().get(0) {
                    resp.set_payload(util::new_payload(param.get_size() as usize));
                }
                // A workaround for timeout_on_sleeping_server test.
                // The request only has 27182 bytes of zeros in payload.
                //
                // Client timeout 1ms is too short for grpcio. The server
                // can response in 1ms. To make the test stable, the server
                // sleeps 1s explicitly.
                if req.get_payload().get_body().len() == 27182
                    && req.get_response_parameters().is_empty()
                    && !req.has_response_status()
                {
                    Delay::new(Duration::from_secs(1)).await;
                }
                sink.send((resp, WriteFlags::default())).await?;
            }
            sink.close().await?;
            Ok(())
        }
        .map_err(|e: grpc::Error| {
            if !matches!(e, grpc::Error::RemoteStopped) {
                error!("failed to handle duplex call: {:?}", e);
            }
        })
        .map(|_| ());
        ctx.spawn(f)
    }

    fn half_duplex_call(
        &mut self,
        _: RpcContext,
        _: RequestStream<StreamingOutputCallRequest>,
        _: DuplexSink<StreamingOutputCallResponse>,
    ) {
        unimplemented!()
    }
}
