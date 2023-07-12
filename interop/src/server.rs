// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::time::Duration;

use futures_timer::Delay;
use futures_util::{FutureExt as _, SinkExt as _, TryFutureExt as _, TryStreamExt as _};
use grpcio::{
    self, ClientStreamingSink, DuplexSink, Metadata, MetadataBuilder, RequestStream, RpcContext,
    RpcStatus, ServerStreamingSink, UnarySink, WriteFlags,
};

use grpc_proto::testing::empty::Empty;
use grpc_proto::testing::messages::{
    SimpleRequest, SimpleResponse, StreamingInputCallRequest, StreamingInputCallResponse,
    StreamingOutputCallRequest, StreamingOutputCallResponse,
};
use grpc_proto::testing::test_grpc::TestService;
use grpc_proto::util;

fn may_echo_metadata(ctx: &RpcContext) -> Metadata {
    let mut builder = MetadataBuilder::new();
    for (key, val) in ctx.request_headers().iter() {
        if key.starts_with("x-grpc-test-echo") {
            if key.ends_with("-bin") {
                builder.add_bytes(key, val).unwrap();
            } else {
                builder
                    .add_str(key, std::str::from_utf8(val).unwrap())
                    .unwrap();
            }
        }
    }
    builder.build()
}

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
        mut sink: UnarySink<SimpleResponse>,
    ) {
        let metadata = may_echo_metadata(&ctx);
        if !metadata.is_empty() {
            sink.set_headers(metadata);
        }

        #[cfg(feature = "protobuf-codec")]
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

        #[cfg(feature = "protobufv3-codec")]
        if let Some(response_status) = &req.response_status.0 {
            let code = response_status.code;
            let msg = &response_status.message;
            let status = RpcStatus::with_message(code, msg.to_string());
            let f = sink
                .fail(status)
                .map_err(|e| panic!("failed to send response: {:?}", e))
                .map(|_| ());
            ctx.spawn(f);
            return;
        }
        let resp_size = req.response_size;
        let resp = SimpleResponse {
            payload: Some(util::new_payload(resp_size as usize)).into(),
            ..SimpleResponse::default()
        };
        let f = sink
            .success(resp)
            .map_err(|e| panic!("failed to send response: {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn streaming_output_call(
        &mut self,
        ctx: RpcContext,
        req: StreamingOutputCallRequest,
        mut sink: ServerStreamingSink<StreamingOutputCallResponse>,
    ) {
        let f = async move {
            for param in req.response_parameters.into_iter() {
                let resp = StreamingOutputCallResponse {
                    payload: Some(util::new_payload(param.size as usize)).into(),
                    ..StreamingOutputCallResponse::default()
                };
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
            #[cfg(feature = "protobuf-codec")]
            while let Some(req) = stream.try_next().await? {
                s += req.get_payload().get_body().len();
            }
            #[cfg(feature = "protobufv3-codec")]
            while let Some(req) = stream.try_next().await? {
                s += req.payload.body.len();
            }

            let resp = StreamingInputCallResponse {
                aggregated_payload_size: s as i32,
                ..StreamingInputCallResponse::default()
            };
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
        let metadata = may_echo_metadata(&ctx);
        if !metadata.is_empty() {
            sink.set_headers(metadata);
        }
        let f = async move {
            while let Some(req) = stream.try_next().await? {
                if let Some(response_status) = &req.response_status.clone().into_option() {
                    let code = response_status.code;
                    let msg = String::from(&response_status.message);
                    let status = RpcStatus::with_message(code, msg);
                    sink.fail(status).await?;
                    return Ok(());
                }

                let mut resp = StreamingOutputCallResponse::default();
                if let Some(param) = req.response_parameters.get(0) {
                    resp.payload = Some(util::new_payload(param.size as usize)).into();
                }
                // A workaround for timeout_on_sleeping_server test.
                // The request only has 27182 bytes of zeros in payload.
                //
                // Client timeout 1ms is too short for grpcio. The server
                // can response in 1ms. To make the test stable, the server
                // sleeps 1s explicitly.

                if req.response_parameters.is_empty() && req.response_status.is_none() {
                    #[cfg(feature = "protobuf-codec")]
                    if req.get_payload().get_body().len() == 27182 {
                        Delay::new(Duration::from_secs(1)).await;
                    }
                    #[cfg(feature = "protobufv3-codec")]
                    if req.payload.body.len() == 27182 {
                        Delay::new(Duration::from_secs(1)).await;
                    }
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
}
