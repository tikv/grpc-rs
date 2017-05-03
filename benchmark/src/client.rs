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
use std::sync::Mutex;

use grpc::{Client as GrpcClient};
use grpc_proto::testing::control::{ClientConfig, ServerType};
use grpc_proto::testing::stats::ClientStats;
use grpc_proto::testing::services_grpc;
use grpc::ChannelBuilder;
use tokio_core::reactor::Remote;

use error::Result;
use util::CpuRecorder;
use histogram::Histogram;

pub struct Client {
    client: GrpcClient,
    recorder: CpuRecoder,
    histogram: Arc<Mutex<Histogram>>,
}

impl Client {
    pub fn new(env: Arc<Environment>, cfg: &ClientConfig) -> Client {
        if cfg.get_async_client_threads() > 0  {
            unimplemented!()
        }
        if cfg.get_core_limit() > 0 {
            unimplemented!()
        }
        if cfg.has_security_params() {
            unimplemented!()
        }
        let channels: Vec<_> = (0..cfg.get_client_channels()).zip(cfg.get_server_targets()).map(|(_, target)| {
            ChannelBuilder::new(env.clone()).connect(addr)
        }).collect();
        unimplemented!()
    }

    pub fn get_stats(&mut self, reset: bool) -> ClientStats {
        let mut stats = ClientStats::new();

        let (real_time, user_time, sys_time) = self.recorder.cpu_time(reset);
        stats.set_time_elapsed(real_time);
        stats.set_time_user(user_time);
        stats.set_time_system(sys_time);

        {
            let his = self.histogram.lock().unwrap();
            stats.set_latencies(his.report());
        }

        stats
    }
}
