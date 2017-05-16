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


use std::sync::{Arc, Mutex};

use futures_cpupool::CpuPool;
use grpc_proto::testing::services_grpc::WorkerService;
use grpc_proto::testing::control::{ClientArgs, ClientStatus, CoreRequest, CoreResponse,
                                   ServerArgs, ServerStatus, Void};
use grpc::{DuplexSink, Environment, RequestStream, RpcContext, UnarySink};
use error::Error;
use futures::{Future, Sink, Stream};
use futures::sync::oneshot::Sender;

use client::Client;
use util;
use server::Server;

#[derive(Clone)]
pub struct Worker {
    env: Arc<Environment>,
    pool: CpuPool,
    shutdown_notifier: Arc<Mutex<Option<Sender<()>>>>,
}

impl Worker {
    pub fn new(env: Arc<Environment>, pool: CpuPool, sender: Sender<()>) -> Worker {
        Worker {
            env: env,
            pool: pool,
            shutdown_notifier: Arc::new(Mutex::new(Some(sender))),
        }
    }
}

impl WorkerService for Worker {
    fn run_server(&self,
                  _: RpcContext,
                  stream: RequestStream<ServerArgs>,
                  sink: DuplexSink<ServerStatus>) {
        let mut server: Option<Server> = None;
        let pool = self.pool.clone();
        let env = self.env.clone();
        let f = sink.sink_map_err(Error::from)
            .send_all(stream
                          .map_err(Error::from)
                          .and_then(move |arg| if arg.has_setup() {
                                        let cfg = arg.get_setup();
                                        println!("receive server setup: {:?}", cfg);
                                        if let Some(mut server) = server.take() {
                                            server.shutdown();
                                            return Err(Error::ServerStarted);
                                        }
                                        let s = try!(Server::new(env.clone(), cfg, pool.clone()));
                                        let status = s.get_status();
                                        server = Some(s);
                                        Ok(status)
                                    } else {
                                        let mark = arg.get_mark();
                                        println!("receive server mark: {:?}", mark);
                                        let stats = match server {
                                            None => return Err(Error::ServerNotStarted),
                                            Some(ref mut s) => s.get_stats(mark.get_reset()),
                                        };
                                        let mut status = server.as_mut().unwrap().get_status();
                                        status.set_stats(stats);
                                        Ok(status)
                                    }))
            .map(|_| println!("server shutdown."))
            .map_err(|e| println!("run server failed: {:?}", e));
        self.pool.spawn(f).forget()
    }

    fn run_client(&self,
                  _: RpcContext,
                  stream: RequestStream<ClientArgs>,
                  sink: DuplexSink<ClientStatus>) {
        let env = self.env.clone();
        let mut client: Option<Client> = None;
        let f = sink.sink_map_err(Error::from)
            .send_all(stream
                          .map_err(Error::from)
                          .and_then(move |arg| if arg.has_setup() {
                                        let cfg = arg.get_setup();
                                        println!("receive client setup: {:?}", cfg);
                                        client.take();
                                        let c = Client::new(env.clone(), cfg);
                                        client = Some(c);
                                        Ok(ClientStatus::new())
                                    } else {
                                        let mark = arg.get_mark();
                                        println!("receive client mark: {:?}", mark);
                                        let stats = match client {
                                            None => return Err(Error::ClientNotStarted),
                                            Some(ref mut c) => c.get_stats(mark.get_reset()),
                                        };
                                        let mut status = ClientStatus::new();
                                        status.set_stats(stats);
                                        Ok(status)
                                    }))
            .map(|_| {})
            .map_err(|e| println!("run client failed: {:?}", e));
        self.pool.spawn(f).forget()
    }

    fn core_count(&self, _: RpcContext, _: CoreRequest, sink: UnarySink<CoreResponse>) {
        let cpu_count = util::cpu_num_cores();
        let mut resp = CoreResponse::new();
        resp.set_cores(cpu_count as i32);
        self.pool
            .spawn(sink.success(resp)
                       .map_err(|e| println!("failed to report cpu count: {:?}", e)))
            .forget()
    }

    fn quit_worker(&self, _: RpcContext, _: Void, sink: ::grpc::UnarySink<Void>) {
        let notifier = self.shutdown_notifier.lock().unwrap().take();
        self.pool
            .spawn(sink.success(Void::new())
                       .map_err(|e| println!("failed to report quick worker: {:?}", e)))
            .forget();
        if let Some(notifier) = notifier {
            let _ = notifier.send(());
        }
    }
}
