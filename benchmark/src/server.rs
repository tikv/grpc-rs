// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use grpc::ServerCredentials;
use grpc_proto::testing::control::{ServerConfig, ServerStatus, ServerType};
use grpc_proto::testing::services_grpc::create_benchmark_service;
use grpc_proto::testing::stats::ServerStats;
use grpc_proto::util as proto_util;
use grpcio::{
    ChannelBuilder, EnvBuilder, Result, Server as GrpcServer, ServerBuilder, ShutdownFuture,
};

use crate::bench::{self, Benchmark, Generic};
use crate::util::{self, CpuRecorder};

pub struct Server {
    server: GrpcServer,
    port: u16,
    recorder: CpuRecorder,
    keep_running: Arc<AtomicBool>,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(cfg: &ServerConfig) -> Result<Server> {
        #[cfg(feature = "protobuf-codec")]
        let server_type = cfg.server_type;
        #[cfg(feature = "protobufv3-codec")]
        let server_type = cfg.server_type.enum_value().unwrap();

        let mut builder = EnvBuilder::new();
        let thd_cnt = cfg.async_server_threads as usize;
        if thd_cnt != 0 {
            builder = builder.cq_count(thd_cnt);
        }
        let env = Arc::new(builder.build());
        if cfg.core_limit > 0 {
            warn!("server config core limit is set but ignored");
        }
        let keep_running = Arc::new(AtomicBool::new(true));
        let keep_running1 = keep_running.clone();
        let service = match server_type {
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
        if !cfg.channel_args.is_empty() {
            let mut ch_builder = ChannelBuilder::new(env);
            for arg in &cfg.channel_args {
                let key = CString::new(arg.name.clone()).unwrap();
                if arg.has_str_value() {
                    #[cfg(feature = "protobuf-codec")]
                    let val = CString::new(arg.get_str_value()).unwrap();
                    #[cfg(feature = "protobufv3-codec")]
                    let val = CString::new(arg.str_value()).unwrap();
                    ch_builder = ch_builder.raw_cfg_string(key, val);
                } else if arg.has_int_value() {
                    #[cfg(feature = "protobuf-codec")]
                    let val = arg.get_int_value();
                    #[cfg(feature = "protobufv3-codec")]
                    let val = arg.int_value();
                    ch_builder = ch_builder.raw_cfg_int(key, val);
                }
            }
            builder = builder.channel_args(ch_builder.build_args());
        }
        let mut s = builder.build().unwrap();

        #[cfg(feature = "protobuf-codec")]
        let has_security_param = cfg.has_security_params();
        #[cfg(feature = "protobufv3-codec")]
        let has_security_param = cfg.security_params.0.is_some();
        let creds = if has_security_param {
            proto_util::create_test_server_credentials()
        } else {
            ServerCredentials::insecure()
        };
        let port = s
            .add_listening_port(&format!("[::]:{}", cfg.port), creds)
            .unwrap();
        s.start();
        Ok(Server {
            server: s,
            port,
            recorder: CpuRecorder::new(),
            keep_running: keep_running1,
        })
    }

    pub fn get_stats(&mut self, reset: bool) -> ServerStats {
        let sample = self.recorder.cpu_time(reset);
        ServerStats {
            time_elapsed: sample.real_time,
            time_user: sample.user_time,
            time_system: sample.sys_time,
            total_cpu_time: sample.total_cpu,
            idle_cpu_time: sample.idle_cpu,
            ..ServerStats::default()
        }
    }

    pub fn shutdown(&mut self) -> ShutdownFuture {
        self.keep_running.store(false, Ordering::SeqCst);
        self.server.shutdown()
    }

    pub fn get_status(&self) -> ServerStatus {
        ServerStatus {
            port: self.port as i32,
            cores: util::cpu_num_cores() as i32,
            ..ServerStatus::default()
        }
    }
}
