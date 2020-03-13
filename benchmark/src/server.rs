// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use grpc::{ChannelBuilder, EnvBuilder, Server as GrpcServer, ServerBuilder, ShutdownFuture};
use grpc_proto::testing::control::{ServerConfig, ServerStatus, ServerType};
use grpc_proto::testing::services_grpc::create_benchmark_service;
use grpc_proto::testing::stats::ServerStats;
use grpc_proto::util as proto_util;

use crate::bench::{self, Benchmark, Generic};
use crate::error::Result;
use crate::util::{self, CpuRecorder};

pub struct Server {
    server: GrpcServer,
    recorder: CpuRecorder,
    keep_running: Arc<AtomicBool>,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(cfg: &ServerConfig) -> Result<Server> {
        let mut builder = EnvBuilder::new();
        let thd_cnt = cfg.get_async_server_threads() as usize;
        if thd_cnt != 0 {
            builder = builder.cq_count(thd_cnt);
        }
        let env = Arc::new(builder.build());
        if cfg.get_core_limit() > 0 {
            warn!("server config core limit is set but ignored");
        }
        let keep_running = Arc::new(AtomicBool::new(true));
        let keep_running1 = keep_running.clone();
        let service = match cfg.get_server_type() {
            ServerType::ASYNC_SERVER => {
                let b = Benchmark { keep_running };
                create_benchmark_service(b)
            }
            ServerType::ASYNC_GENERIC_SERVER => {
                let g = Generic { keep_running };
                bench::create_generic_service(g)
            }
            _ => unimplemented!(),
        };
        let mut builder = ServerBuilder::new(env.clone()).register_service(service);
        if !cfg.get_channel_args().is_empty() {
            let mut ch_builder = ChannelBuilder::new(env);
            for arg in cfg.get_channel_args() {
                let key = CString::new(arg.get_name()).unwrap();
                if arg.has_str_value() {
                    ch_builder =
                        ch_builder.raw_cfg_string(key, CString::new(arg.get_str_value()).unwrap());
                } else if arg.has_int_value() {
                    ch_builder = ch_builder.raw_cfg_int(key, arg.get_int_value() as i32);
                }
            }
            builder = builder.channel_args(ch_builder.build_args());
        }
        builder = if cfg.has_security_params() {
            builder.bind_with_cred(
                "[::]",
                cfg.get_port() as u16,
                proto_util::create_test_server_credentials(),
            )
        } else {
            builder.bind("[::]", cfg.get_port() as u16)
        };

        let mut s = builder.build().unwrap();
        s.start();
        Ok(Server {
            server: s,
            recorder: CpuRecorder::new(),
            keep_running: keep_running1,
        })
    }

    pub fn get_stats(&mut self, reset: bool) -> ServerStats {
        let sample = self.recorder.cpu_time(reset);

        let mut stats = ServerStats::default();
        stats.set_time_elapsed(sample.real_time);
        stats.set_time_user(sample.user_time);
        stats.set_time_system(sample.sys_time);
        stats.set_total_cpu_time(sample.total_cpu);
        stats.set_idle_cpu_time(sample.idle_cpu);
        stats
    }

    pub fn shutdown(&mut self) -> ShutdownFuture {
        self.keep_running.store(false, Ordering::SeqCst);
        self.server.shutdown()
    }

    pub fn get_status(&self) -> ServerStatus {
        let mut status = ServerStatus::default();
        status.set_port(i32::from(self.server.bind_addrs().next().unwrap().1));
        status.set_cores(util::cpu_num_cores() as i32);
        status
    }
}
