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


use std::sync::Arc;

use grpc_proto::testing::services_grpc::WorkerService;
use grpc_proto::testing::control::{CoreRequest, ServerArgs, ClientArgs, Void, CoreResponse, ServerStatus, ClientStatus};
use grpc_proto::testing::messages::{SimpleRequest, SimpleResponse};
use grpc_proto::util;
use tokio_core::reactor::Remote;
use grpc::{RpcContext, UnaryResponseSink, ResponseSink, RequestStream, Environment};
use error::Error;
use futures::{future, Future, Sink, Stream};
use num_cpus;

use server::Server;

#[derive(Clone)]
pub struct Worker {
    env: Arc<Environment>,
    remote: Remote,
}

impl Worker {
    pub fn new(env: Arc<Environment>, remote: Remote) -> Worker {
        Worker {
            env: env,
            remote: remote,
        }
    }
}

impl WorkerService for Worker {
    fn run_server(&self, _: RpcContext, stream: RequestStream<ServerArgs>, sink: ResponseSink<ServerStatus>) {
        let env = self.env.clone();
        let remote = self.remote.clone();
        self.remote.spawn(move |_| {
            let mut server: Option<Server> = None;
            sink.sink_map_err(Error::from).send_all(stream.map_err(Error::from).and_then(move |arg| {
                if arg.has_setup() {
                    let cfg = arg.get_setup();
                    println!("receive server setup: {:?}", cfg);
                    if let Some(mut server) = server.take() {
                        server.shutdown();
                    }
                    let server = try!(Server::new(env.clone(), cfg, remote.clone()));
                    Ok(ServerStatus::new())
                } else {
                    let mark = arg.get_mark();
                    println!("receive server mark: {:?}", mark);
                    let _stats = match server {
                        None => return Err(Error::ServerNotStarted),
                        Some(ref mut s) => s.get_stats(mark.get_reset())
                    };
                    Ok(ServerStatus::new())
                }
            })).map(|_| {}).map_err(|e| println!("run server failed: {:?}", e))
        })
    }

    fn run_client(&self, _: RpcContext, stream: RequestStream<ClientArgs>, sink: ResponseSink<ClientStatus>) {
        self.remote.spawn(move |_| {
            let mut client: Option<Client> = None;
            sink.sink_map_err(Error::from).send_all(stream.map_err(Error::from).and_then(move |arg| {
                if arg.has_setup() {
                    let cfg = arg.get_setup();
                    println!("receive client setup: {:?}", cfg);
                    client.take();
                    let client = try!(Client::new(env.clone(), cfg, remote.clone()));
                    Ok(ClientStatus::new())
                } else {
                    let mark = arg.get_mark();
                    println!("receive client mark: {:?}", mark);
                    let _stats = match client {
                        None => return Err(Error::ClientNotStarted),
                        Some(ref mut c) => c.get_stats(mark.get_reset())
                    };
                    Ok(ClientStatus::new())
                }
            })).map(|_| {}).map_err(|e| println!("run client failed: {:?}", e))
        })
    }

    fn core_count(&self, _: RpcContext, _: CoreRequest, resp: UnaryResponseSink<CoreResponse>) {
        let cpu_count = num_cpus::get();
        let mut resp = CoreResponse::new();
        resp.set_cores(cpu_count as i32);
        self.remote.spawn(|_| future::result(resp.success(resp)).flattern().map_err(|e| println!("failed to report cpu count: {:?}", e)))
    }

    fn quit_worker(&self, _: RpcContext, req: Void, resp: ::grpc::UnaryResponseSink<Void>) {
        unimplemented!()
    }
}
