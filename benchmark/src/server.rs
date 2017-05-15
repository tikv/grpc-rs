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

use error::Result;
use grpc::{Environment, Server as GrpcServer, ServerBuilder, ShutdownFuture};
use grpc_proto::testing::control::{ServerConfig, ServerStatus, ServerType};
use grpc_proto::testing::stats::ServerStats;
use grpc_proto::testing::services_grpc;
use grpc_proto::util as proto_util;
use tokio_core::reactor::Remote;

use bench::{self, Benchmark, Generic};
use util::{self, CpuRecorder};

pub struct Server {
    server: GrpcServer,
    recorder: CpuRecorder,
}

impl Server {
    pub fn new(env: Arc<Environment>, cfg: &ServerConfig, remote: Remote) -> Result<Server> {
        if cfg.get_core_limit() > 0 {
            println!("server config core limit is set but ignored");
        }
        let service = match cfg.get_server_type() {
            ServerType::ASYNC_SERVER => {
                let b = Benchmark::new(remote);
                services_grpc::create_benchmark_service(b)
            }
            ServerType::ASYNC_GENERIC_SERVER => {
                let b = Generic::new(remote);
                bench::create_generic_service(b)
            }
            _ => unimplemented!(),
        };
        let mut builder = ServerBuilder::new(env).register_service(service);
        builder = if cfg.has_security_params() {
            builder.bind_secure("[::]",
                                cfg.get_port() as u32,
                                proto_util::create_test_server_credentials())
        } else {
            builder.bind("[::]", cfg.get_port() as u32)
        };

        let mut s = builder.build();
        s.start();
        Ok(Server {
               server: s,
               recorder: CpuRecorder::new(),
           })
    }

    pub fn get_stats(&mut self, reset: bool) -> ServerStats {
        let (real_time, user_time, sys_time) = self.recorder.cpu_time(reset);

        let mut stats = ServerStats::new();
        stats.set_time_elapsed(real_time);
        stats.set_time_user(user_time);
        stats.set_time_system(sys_time);
        stats
    }

    pub fn shutdown(&mut self) -> ShutdownFuture {
        self.server.shutdown()
    }

    pub fn get_status(&self) -> ServerStatus {
        let mut status = ServerStatus::new();
        status.set_port(self.server.bind_addrs()[0].1 as i32);
        status.set_cores(util::cpu_num_cores() as i32);
        status
    }
}
