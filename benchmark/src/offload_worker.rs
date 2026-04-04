// Copyright 2026 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::{Arc, Mutex};

use futures_channel::oneshot::Sender;
use futures_util::{FutureExt as _, SinkExt as _, TryFutureExt as _, TryStreamExt as _};
use grpc_proto::offload::{decode, encode, Request, Response};
use grpc_proto::testing::control::{
    ClientArgs, ClientStatus, CoreRequest, CoreResponse, ServerArgs, ServerStatus, Void,
};
use grpc_proto::testing::services_grpc::WorkerServiceOffload;
use grpcio::{DuplexSink, RequestStream, RpcContext, UnarySink, WriteFlags};

use crate::client::Client;
use crate::offload_server::OffloadServer;
use crate::util;

#[derive(Clone)]
pub struct OffloadWorker {
    shutdown_notifier: Arc<Mutex<Option<Sender<()>>>>,
}

impl OffloadWorker {
    pub fn new(sender: Sender<()>) -> OffloadWorker {
        OffloadWorker {
            shutdown_notifier: Arc::new(Mutex::new(Some(sender))),
        }
    }
}

impl WorkerServiceOffload for OffloadWorker {
    fn run_server(
        &mut self,
        ctx: RpcContext,
        mut stream: RequestStream<Request<ServerArgs>>,
        mut sink: DuplexSink<Response<ServerStatus>>,
    ) {
        let f = async move {
            let arg = match stream.try_next().await? {
                None => return sink.close().await,
                Some(arg) => decode(arg)?,
            };
            #[cfg(feature = "protobuf-codec")]
            let cfg = arg.get_setup();
            #[cfg(feature = "protobufv3-codec")]
            let cfg = arg.setup();
            info!("receive server setup: {:?}", cfg);
            let mut server = OffloadServer::new(cfg)?;
            let status = server.get_status();
            sink.send((encode(status)?, WriteFlags::default())).await?;
            while let Some(arg) = stream.try_next().await? {
                let arg = decode(arg)?;
                #[cfg(feature = "protobuf-codec")]
                let mark = arg.get_mark();
                #[cfg(feature = "protobufv3-codec")]
                let mark = arg.mark();

                info!("receive server mark: {:?}", mark);
                let stats = server.get_stats(mark.reset);
                let mut status = server.get_status();
                status.stats = Some(stats).into();
                sink.send((encode(status)?, WriteFlags::default())).await?;
            }
            server.shutdown().await?;
            sink.close().await?;
            Ok(())
        }
        .map_err(|e| error!("run server failed: {:?}", e))
        .map(|_| info!("server shutdown."));
        ctx.spawn(f)
    }

    fn run_client(
        &mut self,
        ctx: RpcContext,
        mut stream: RequestStream<Request<ClientArgs>>,
        mut sink: DuplexSink<Response<ClientStatus>>,
    ) {
        let f = async move {
            let arg = match stream.try_next().await? {
                None => return sink.close().await,
                Some(arg) => decode(arg)?,
            };
            #[cfg(feature = "protobuf-codec")]
            let cfg = arg.get_setup();
            #[cfg(feature = "protobufv3-codec")]
            let cfg = arg.setup();
            info!("receive client setup: {:?}", cfg);
            let mut client = Client::new(cfg);
            sink.send((encode(ClientStatus::default())?, WriteFlags::default()))
                .await?;
            while let Some(arg) = stream.try_next().await? {
                let arg = decode(arg)?;
                #[cfg(feature = "protobuf-codec")]
                let mark = arg.get_mark();
                #[cfg(feature = "protobufv3-codec")]
                let mark = arg.mark();

                info!("receive client mark: {:?}", mark);
                let stats = client.get_stats(mark.reset);
                let status = ClientStatus {
                    stats: Some(stats).into(),
                    ..ClientStatus::default()
                };
                sink.send((encode(status)?, WriteFlags::default())).await?;
            }
            client.shutdown().await;
            sink.close().await?;
            Ok(())
        }
        .map_err(|e| error!("run client failed: {:?}", e))
        .map(|_| info!("client shutdown."));
        ctx.spawn(f)
    }

    fn core_count(
        &mut self,
        ctx: RpcContext,
        _: Request<CoreRequest>,
        sink: UnarySink<Response<CoreResponse>>,
    ) {
        let cpu_count = util::cpu_num_cores();
        let resp = CoreResponse {
            cores: cpu_count as i32,
            ..CoreResponse::default()
        };
        ctx.spawn(
            sink.success(encode(resp).expect("core_count response should encode"))
                .map_err(|e| error!("failed to report cpu count: {:?}", e))
                .map(|_| ()),
        )
    }

    fn quit_worker(
        &mut self,
        ctx: RpcContext,
        _: Request<Void>,
        sink: crate::grpc::UnarySink<Response<Void>>,
    ) {
        let notifier = self.shutdown_notifier.lock().unwrap().take();
        if let Some(notifier) = notifier {
            let _ = notifier.send(());
        }
        ctx.spawn(
            sink.success(encode(Void::default()).expect("quit_worker response should encode"))
                .map_err(|e| error!("failed to report quick worker: {:?}", e))
                .map(|_| ()),
        );
    }
}
