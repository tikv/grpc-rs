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
use std::time::Instant;

use error::Result;
use grpc::{Server as GrpcServer, ServerBuilder, Environment, ShutdownFuture};
use grpc_proto::testing::control::{ServerConfig, ServerType};
use grpc_proto::testing::stats::ServerStats;
use grpc_proto::testing::services_grpc;
use tokio_core::reactor::Remote;
use libc::rusage;

use bench::Benchmark;
use util::CpuRecorder;

pub struct Server {
    server: GrpcServer,
    recoder: CpuRecoder,
}

impl Server {
    pub fn new(env: Arc<Environment>, cfg: &ServerConfig, remote: Remote) -> Result<Server> {
        if cfg.has_security_params() {
            unimplemented!()
        }
        if cfg.get_core_limit() > 0 {
            unimplemented!()
        }
        let service = match cfg.get_server_type() {
            ServerType::ASYNC_SERVER => {
                let b = Benchmark::new(remote);
                services_grpc::create_benchmark_service(b)
            },
            _ => unimplemented!()
        };
        let s = ServerBuilder::new(env)
                    .bind("localhost", cfg.get_port() as u32)
                    .register_service(service)
                    .build();
        Ok(Server {
            server: s,
            recorder: CpuRecorder::new(),
        })
    }

    pub fn get_stats(&mut self, reset: bool) -> ServerStats {
        let (real_time, user_time, sys_time) = self.recoder.cpu_time(reset);

        let mut stats = ServerStats::new();
        stats.set_time_elapsed(real_time);
        stats.set_time_user(user);
        stats.set_time_system(sys);
        stats
    }

    pub fn shutdown(&mut self) -> ShutdownFuture {
        self.server.shutdown()
    }
}
