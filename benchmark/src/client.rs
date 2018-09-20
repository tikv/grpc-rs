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

use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use futures::future::Loop;
use futures::sync::oneshot::{self, Receiver, Sender};
use futures::{future, Async, Future, Sink, Stream};
use grpc::{
    CallOption, Channel, ChannelBuilder, Client as GrpcClient, EnvBuilder, Environment, WriteFlags,
};
use grpc_proto::testing::control::{ClientConfig, ClientType, RpcType};
use grpc_proto::testing::messages::SimpleRequest;
use grpc_proto::testing::services_grpc::BenchmarkServiceClient;
use grpc_proto::testing::stats::ClientStats;
use grpc_proto::util as proto_util;
use rand::distributions::Exp;
use rand::distributions::Sample;
use rand::{self, SeedableRng, XorShiftRng};
use tokio_timer::{Sleep, Timer};

use bench;
use error::Error;
use util::{self, CpuRecorder, Histogram};

type BoxFuture<T, E> = Box<Future<Item = T, Error = E> + Send>;

fn gen_req(cfg: &ClientConfig) -> SimpleRequest {
    let mut req = SimpleRequest::new();
    let payload_config = cfg.get_payload_config();
    let simple_params = payload_config.get_simple_params();
    req.set_payload(proto_util::new_payload(
        simple_params.get_req_size() as usize
    ));
    req.set_response_size(simple_params.get_resp_size());
    req
}

trait Backoff {
    fn backoff_time(&mut self) -> Option<Duration>;
}

struct ClosedLoop;

impl Backoff for ClosedLoop {
    fn backoff_time(&mut self) -> Option<Duration> {
        None
    }
}

/// A timer that generates Poisson process load.
struct Poisson {
    exp: Exp,
    r: XorShiftRng,
    last_time: Instant,
}

impl Poisson {
    fn new(offered_load: f64) -> Poisson {
        Poisson {
            exp: Exp::new(offered_load),
            r: XorShiftRng::from_seed(rand::random()),
            last_time: Instant::now(),
        }
    }
}

impl Backoff for Poisson {
    fn backoff_time(&mut self) -> Option<Duration> {
        let backoff_time = self.exp.sample(&mut self.r);
        let sec = backoff_time as u64;
        let ns = (backoff_time.fract() * 1_000_000_000f64) as u32;
        self.last_time += Duration::new(sec, ns);
        let now = Instant::now();
        if self.last_time > now {
            Some(self.last_time - now)
        } else {
            None
        }
    }
}

struct ExecutorContext<B> {
    keep_running: Arc<AtomicBool>,
    histogram: Arc<Mutex<Histogram>>,
    backoff: B,
    timer: Timer,
    _trace: Sender<()>,
}

impl<B: Backoff> ExecutorContext<B> {
    fn new(
        histogram: Arc<Mutex<Histogram>>,
        keep_running: Arc<AtomicBool>,
        backoff: B,
        timer: Timer,
    ) -> (ExecutorContext<B>, Receiver<()>) {
        let (tx, rx) = oneshot::channel();
        (
            ExecutorContext {
                keep_running,
                histogram,
                backoff,
                timer,
                _trace: tx,
            },
            rx,
        )
    }

    fn observe_latency(&self, latency: Duration) {
        let f = util::dur_to_nanos(latency);
        let mut his = self.histogram.lock().unwrap();
        his.observe(f);
    }

    fn backoff_async(&mut self) -> Option<Sleep> {
        self.backoff.backoff_time().map(|dur| self.timer.sleep(dur))
    }

    fn backoff(&mut self) {
        if let Some(dur) = self.backoff.backoff_time() {
            thread::sleep(dur)
        }
    }

    fn keep_running(&self) -> bool {
        self.keep_running.load(Ordering::Relaxed)
    }
}

/// An executor that executes generic requests.
struct GenericExecutor<B> {
    ctx: ExecutorContext<B>,
    client: Arc<GrpcClient>,
    req: Vec<u8>,
}

impl<B: Backoff + Send + 'static> GenericExecutor<B> {
    fn new(ctx: ExecutorContext<B>, channel: Channel, cfg: &ClientConfig) -> GenericExecutor<B> {
        let cap = cfg.get_payload_config().get_bytebuf_params().get_req_size();
        let req = vec![0; cap as usize];
        GenericExecutor {
            ctx,
            client: Arc::new(GrpcClient::new(channel)),
            req,
        }
    }

    fn execute_stream(self) {
        let client = self.client.clone();
        let keep_running = self.ctx.keep_running.clone();
        let (sender, receiver) = self
            .client
            .duplex_streaming(
                &bench::METHOD_BENCHMARK_SERVICE_GENERIC_CALL,
                CallOption::default(),
            ).unwrap();
        let f = future::loop_fn(
            (sender, self, receiver),
            move |(sender, mut executor, receiver)| {
                let latency_timer = Instant::now();
                let send = sender.send((executor.req.clone(), WriteFlags::default()));
                send.map_err(Error::from).and_then(move |sender| {
                    receiver
                        .into_future()
                        .map_err(|(e, _)| Error::from(e))
                        .and_then(move |(_, r)| {
                            executor.ctx.observe_latency(latency_timer.elapsed());
                            let mut time = executor.ctx.backoff_async();
                            let mut res = Some((sender, executor, r));
                            future::poll_fn(move || {
                                if let Some(ref mut t) = time {
                                    try_ready!(t.poll());
                                }
                                time.take();
                                let r = res.take().unwrap();
                                let l = if r.1.ctx.keep_running() {
                                    Loop::Continue(r)
                                } else {
                                    Loop::Break(r)
                                };
                                Ok(Async::Ready(l))
                            })
                        })
                })
            },
        ).and_then(|(mut s, e, r)| {
            future::poll_fn(move || s.close().map_err(Error::from)).map(|_| (e, r))
        }).and_then(|(e, r)| r.into_future().map(|_| e).map_err(|(e, _)| Error::from(e)));
        spawn!(client, keep_running, "streaming ping pong", f)
    }
}

/// An executor that executes protobuf requests.
struct RequestExecutor<B> {
    ctx: ExecutorContext<B>,
    client: Arc<BenchmarkServiceClient>,
    req: SimpleRequest,
}

impl<B: Backoff + Send + 'static> RequestExecutor<B> {
    fn new(ctx: ExecutorContext<B>, channel: Channel, cfg: &ClientConfig) -> RequestExecutor<B> {
        RequestExecutor {
            ctx,
            client: Arc::new(BenchmarkServiceClient::new(channel)),
            req: gen_req(cfg),
        }
    }

    fn execute_unary(mut self) {
        thread::spawn(move || loop {
            if !self.ctx.keep_running() {
                break;
            }
            let latency_timer = Instant::now();
            self.client.unary_call(&self.req).unwrap();
            let elapsed = latency_timer.elapsed();
            self.ctx.observe_latency(elapsed);
            self.ctx.backoff();
        });
    }

    fn execute_unary_async(self) {
        let client = self.client.clone();
        let keep_running = self.ctx.keep_running.clone();
        let f = future::loop_fn(self, move |mut executor| {
            let latency_timer = Instant::now();
            let handler = executor.client.unary_call_async(&executor.req).unwrap();

            handler.map_err(Error::from).and_then(move |_| {
                let elapsed = latency_timer.elapsed();
                executor.ctx.observe_latency(elapsed);
                let mut time = executor.ctx.backoff_async();
                let mut res = Some(executor);
                future::poll_fn(move || {
                    if let Some(ref mut t) = time {
                        try_ready!(t.poll());
                    }
                    time.take();
                    let executor = res.take().unwrap();
                    let l = if executor.ctx.keep_running() {
                        Loop::Continue(executor)
                    } else {
                        Loop::Break(())
                    };
                    Ok(Async::Ready(l))
                })
            })
        });
        spawn!(client, keep_running, "unary async", f)
    }

    fn execute_stream_ping_pong(self) {
        let client = self.client.clone();
        let keep_running = self.ctx.keep_running.clone();
        let (sender, receiver) = self.client.streaming_call().unwrap();
        let f = future::loop_fn(
            (sender, self, receiver),
            move |(sender, mut executor, receiver)| {
                let latency_timer = Instant::now();
                let send = sender.send((executor.req.clone(), WriteFlags::default()));
                send.map_err(Error::from).and_then(move |sender| {
                    receiver
                        .into_future()
                        .map_err(|(e, _)| Error::from(e))
                        .and_then(move |(_, r)| {
                            executor.ctx.observe_latency(latency_timer.elapsed());
                            let mut time = executor.ctx.backoff_async();
                            let mut res = Some((sender, executor, r));
                            future::poll_fn(move || {
                                if let Some(ref mut t) = time {
                                    try_ready!(t.poll());
                                }
                                time.take();
                                let r = res.take().unwrap();
                                let l = if r.1.ctx.keep_running() {
                                    Loop::Continue(r)
                                } else {
                                    Loop::Break(r)
                                };
                                Ok(Async::Ready(l))
                            })
                        })
                })
            },
        ).and_then(|(mut s, e, r)| {
            future::poll_fn(move || s.close().map_err(Error::from)).map(|_| (e, r))
        }).and_then(|(e, r)| r.into_future().map(|_| e).map_err(|(e, _)| Error::from(e)));
        spawn!(client, keep_running, "streaming ping pong", f);
    }
}

fn execute<B: Backoff + Send + 'static>(
    ctx: ExecutorContext<B>,
    ch: Channel,
    client_type: ClientType,
    cfg: &ClientConfig,
) {
    match client_type {
        ClientType::SYNC_CLIENT => {
            if cfg.get_payload_config().has_bytebuf_params() {
                panic!("only async_client is supported for generic service.");
            }
            RequestExecutor::new(ctx, ch, cfg).execute_unary()
        }
        ClientType::ASYNC_CLIENT => match cfg.get_rpc_type() {
            RpcType::UNARY => {
                if cfg.get_payload_config().has_bytebuf_params() {
                    panic!("only streaming is supported for generic service.");
                }
                RequestExecutor::new(ctx, ch, cfg).execute_unary_async()
            }
            RpcType::STREAMING => if cfg.get_payload_config().has_bytebuf_params() {
                GenericExecutor::new(ctx, ch, cfg).execute_stream()
            } else {
                RequestExecutor::new(ctx, ch, cfg).execute_stream_ping_pong()
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

pub struct Client {
    keep_running: Arc<AtomicBool>,
    recorder: CpuRecorder,
    histogram: Arc<Mutex<Histogram>>,
    _env: Arc<Environment>,
    running_reqs: Option<Vec<Receiver<()>>>,
}

impl Client {
    pub fn new(cfg: &ClientConfig) -> Client {
        let mut builder = EnvBuilder::new();
        let thd_cnt = cfg.get_async_client_threads() as usize;
        if thd_cnt != 0 {
            builder = builder.cq_count(thd_cnt);
        }
        let env = Arc::new(builder.build());
        if cfg.get_core_limit() > 0 {
            error!("client config core limit is set but ignored");
        }

        let ch_env = env.clone();
        let channels = (0..cfg.get_client_channels())
            .zip(cfg.get_server_targets().into_iter().cycle())
            .map(|(_, addr)| {
                let mut builder = ChannelBuilder::new(ch_env.clone());
                for arg in cfg.get_channel_args() {
                    let key = CString::new(arg.get_name()).unwrap();
                    if arg.has_str_value() {
                        builder =
                            builder.raw_cfg_string(key, CString::new(arg.get_str_value()).unwrap());
                    } else if arg.has_int_value() {
                        builder = builder.raw_cfg_int(key, arg.get_int_value() as i32);
                    }
                }
                if cfg.has_security_params() {
                    let params = cfg.get_security_params();
                    if !params.get_server_host_override().is_empty() {
                        builder = builder
                            .override_ssl_target(params.get_server_host_override().to_owned());
                    }
                    builder.secure_connect(addr, proto_util::create_test_channel_credentials())
                } else {
                    builder.connect(addr)
                }
            });

        let client_type = cfg.get_client_type();
        let load_params = cfg.get_load_params();
        let client_channels = cfg.get_client_channels() as usize;
        let outstanding_rpcs_per_channel = cfg.get_outstanding_rpcs_per_channel() as usize;

        let recorder = CpuRecorder::new();
        let his_param = cfg.get_histogram_params();
        let his = Arc::new(Mutex::new(Histogram::new(
            his_param.get_resolution(),
            his_param.get_max_possible(),
        )));
        let timer = Timer::default();
        let keep_running = Arc::new(AtomicBool::new(true));
        let mut running_reqs = Vec::with_capacity(client_channels * outstanding_rpcs_per_channel);

        for ch in channels {
            for _ in 0..cfg.get_outstanding_rpcs_per_channel() {
                let his = his.clone();
                let timer = timer.clone();
                let ch = ch.clone();
                let rx = if load_params.has_poisson() {
                    let lambda = load_params.get_poisson().get_offered_load()
                        / client_channels as f64
                        / outstanding_rpcs_per_channel as f64;
                    let poisson = Poisson::new(lambda);
                    let (ctx, rx) = ExecutorContext::new(his, keep_running.clone(), poisson, timer);
                    execute(ctx, ch, client_type, cfg);
                    rx
                } else {
                    let (ctx, rx) =
                        ExecutorContext::new(his, keep_running.clone(), ClosedLoop, timer);
                    execute(ctx, ch, client_type, cfg);
                    rx
                };
                running_reqs.push(rx);
            }
        }

        Client {
            keep_running,
            recorder,
            histogram: his,
            _env: env,
            running_reqs: Some(running_reqs),
        }
    }

    pub fn get_stats(&mut self, reset: bool) -> ClientStats {
        let mut stats = ClientStats::new();

        let sample = self.recorder.cpu_time(reset);
        stats.set_time_elapsed(sample.real_time);
        stats.set_time_user(sample.user_time);
        stats.set_time_system(sample.sys_time);

        {
            let mut his = self.histogram.lock().unwrap();
            stats.set_latencies(his.report(reset));
        }

        stats
    }

    pub fn shutdown(&mut self) -> BoxFuture<(), Error> {
        self.keep_running.store(false, Ordering::Relaxed);
        let mut tasks = self.running_reqs.take().unwrap();
        let mut idx = tasks.len();
        Box::new(future::poll_fn(move || {
            while idx > 0 {
                if let Ok(Async::NotReady) = tasks[idx - 1].poll() {
                    return Ok(Async::NotReady);
                }
                idx -= 1;
            }
            Ok(Async::Ready(()))
        }))
    }
}
