// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::{Arc, Mutex};

use futures_channel::oneshot::Sender;
use futures_util::{FutureExt as _, SinkExt as _, TryFutureExt as _, TryStreamExt as _};
use grpc_proto::testing::control::{
    ClientArgs, ClientStatus, CoreRequest, CoreResponse, ServerArgs, ServerStatus, Void,
};
use grpc_proto::testing::services_grpc::WorkerService;
use grpcio::{DuplexSink, Error, RequestStream, RpcContext, UnarySink, WriteFlags};

use crate::client::Client;
use crate::server::Server;
use crate::util;

#[derive(Clone)]
pub struct Worker {
    shutdown_notifier: Arc<Mutex<Option<Sender<()>>>>,
}

impl Worker {
    pub fn new(sender: Sender<()>) -> Worker {
        Worker {
            shutdown_notifier: Arc::new(Mutex::new(Some(sender))),
        }
    }
}

impl WorkerService for Worker {
    fn run_server(
        &mut self,
        ctx: RpcContext,
        mut stream: RequestStream<ServerArgs>,
        mut sink: DuplexSink<ServerStatus>,
    ) {
        let f = async move {
            let arg = match stream.try_next().await? {
                None => return sink.close().await.map_err(Error::from),
                Some(arg) => arg,
            };
            #[cfg(feature = "protobuf-codec")]
            let cfg = arg.get_setup();
            #[cfg(feature = "protobufv3-codec")]
            let cfg = arg.setup();
            info!("receive server setup: {:?}", cfg);
            let mut server = Server::new(cfg)?;
            let status = server.get_status();
            sink.send((status, WriteFlags::default())).await?;
            while let Some(arg) = stream.try_next().await? {
                #[cfg(feature = "protobuf-codec")]
                let mark = arg.get_mark();
                #[cfg(feature = "protobufv3-codec")]
                let mark = arg.mark();

                info!("receive server mark: {:?}", mark);
                let stats = server.get_stats(mark.reset);
                let mut status = server.get_status();
                status.stats = Some(stats).into();
                sink.send((status, WriteFlags::default())).await?;
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
        mut stream: RequestStream<ClientArgs>,
        mut sink: DuplexSink<ClientStatus>,
    ) {
        let f = async move {
            let arg = match stream.try_next().await? {
                None => return sink.close().await,
                Some(arg) => arg,
            };
            #[cfg(feature = "protobuf-codec")]
            let cfg = arg.get_setup();
            #[cfg(feature = "protobufv3-codec")]
            let cfg = arg.setup();
            info!("receive client setup: {:?}", cfg);
            let mut client = Client::new(cfg);
            sink.send((ClientStatus::default(), WriteFlags::default()))
                .await?;
            while let Some(arg) = stream.try_next().await? {
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
                sink.send((status, WriteFlags::default())).await?;
            }
            client.shutdown().await;
            sink.close().await?;
            Ok(())
        }
        .map_err(|e| error!("run client failed: {:?}", e))
        .map(|_| info!("client shutdown."));
        ctx.spawn(f)
    }

    fn core_count(&mut self, ctx: RpcContext, _: CoreRequest, sink: UnarySink<CoreResponse>) {
        let cpu_count = util::cpu_num_cores();
        let resp = CoreResponse {
            cores: cpu_count as i32,
            ..CoreResponse::default()
        };
        ctx.spawn(
            sink.success(resp)
                .map_err(|e| error!("failed to report cpu count: {:?}", e))
                .map(|_| ()),
        )
    }

    fn quit_worker(&mut self, ctx: RpcContext, _: Void, sink: crate::grpc::UnarySink<Void>) {
        let notifier = self.shutdown_notifier.lock().unwrap().take();
        if let Some(notifier) = notifier {
            let _ = notifier.send(());
        }
        ctx.spawn(
            sink.success(Void::default())
                .map_err(|e| error!("failed to report quick worker: {:?}", e))
                .map(|_| ()),
        );
    }
}
