// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::ffi::CString;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use futures_channel::oneshot::{self, Receiver, Sender};
use futures_util::stream;
use futures_util::{
    FutureExt as _, SinkExt as _, StreamExt as _, TryFutureExt as _, TryStreamExt as _,
};
use grpcio::{
    CallOption, Channel, ChannelBuilder, Client as GrpcClient, EnvBuilder, Environment, WriteFlags,
};
use grpcio_proto::testing::control::SecurityParams;
use grpcio_proto::testing::control::{ClientConfig, ClientType, RpcType};
use grpcio_proto::testing::messages::SimpleRequest;
use grpcio_proto::testing::payloads::PayloadConfig;
use grpcio_proto::testing::services_grpc::BenchmarkServiceClient;
use grpcio_proto::testing::stats::ClientStats;
use grpcio_proto::util as proto_util;
use rand::{self, SeedableRng};
use rand_distr::{Distribution, Exp};
use rand_xorshift::XorShiftRng;

use crate::bench;
use crate::util::{self, CpuRecorder, Histogram};

#[cfg(feature = "protobuf-codec")]
fn gen_req(cfg: &ClientConfig) -> SimpleRequest {
    let mut req = SimpleRequest::default();
    let payload_config = cfg.get_payload_config();
    let simple_params = payload_config.get_simple_params();
    req.payload = Some(proto_util::new_payload(simple_params.req_size as usize)).into();
    req.response_size = simple_params.resp_size;
    req
}

#[cfg(feature = "protobufv3-codec")]
fn gen_req(cfg: &ClientConfig) -> SimpleRequest {
    let mut req = SimpleRequest::default();
    let payload_config = &cfg.payload_config;
    let simple_params = payload_config.simple_params();
    req.payload = Some(proto_util::new_payload(simple_params.req_size as usize)).into();
    req.response_size = simple_params.resp_size;
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
    exp: Exp<f64>,
    r: XorShiftRng,
    last_time: Instant,
}

impl Poisson {
    fn new(offered_load: f64) -> Poisson {
        Poisson {
            exp: Exp::new(offered_load).unwrap(),
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
    _trace: Sender<()>,
}

impl<B: Backoff> ExecutorContext<B> {
    #[allow(clippy::new_ret_no_self)]
    fn new(
        histogram: Arc<Mutex<Histogram>>,
        keep_running: Arc<AtomicBool>,
        backoff: B,
    ) -> (ExecutorContext<B>, Receiver<()>) {
        let (tx, rx) = oneshot::channel();
        (
            ExecutorContext {
                keep_running,
                histogram,
                backoff,
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

    fn backoff_async(&mut self) -> Option<futures_timer::Delay> {
        self.backoff.backoff_time().map(futures_timer::Delay::new)
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
        #[cfg(feature = "protobuf-codec")]
        let cap = cfg.get_payload_config().get_bytebuf_params().get_req_size();
        #[cfg(feature = "protobufv3-codec")]
        let cap = cfg.payload_config.bytebuf_params().req_size;

        let req = vec![0; cap as usize];
        GenericExecutor {
            ctx,
            client: Arc::new(GrpcClient::new(channel)),
            req,
        }
    }

    fn execute_stream(mut self) {
        let client = self.client.clone();
        let keep_running = self.ctx.keep_running.clone();
        let (mut sender, mut receiver) = self
            .client
            .duplex_streaming(
                &bench::METHOD_BENCHMARK_SERVICE_GENERIC_CALL,
                CallOption::default(),
            )
            .unwrap();
        let f = async move {
            loop {
                let latency_timer = Instant::now();
                sender
                    .send((self.req.clone(), WriteFlags::default()))
                    .await?;
                receiver.try_next().await?;
                self.ctx.observe_latency(latency_timer.elapsed());
                let mut time = self.ctx.backoff_async();
                if let Some(t) = &mut time {
                    t.await;
                }
                if !self.ctx.keep_running() {
                    break;
                }
            }
            sender.close().await?;
            receiver.try_next().await?;
            Ok(())
        };
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

    fn execute_unary_async(mut self) {
        let client = self.client.clone();
        let keep_running = self.ctx.keep_running.clone();
        let f = async move {
            loop {
                let latency_timer = Instant::now();
                self.client.unary_call_async(&self.req)?.await?;
                let elapsed = latency_timer.elapsed();
                self.ctx.observe_latency(elapsed);
                let mut time = self.ctx.backoff_async();
                if let Some(t) = &mut time {
                    t.await;
                }
                if !self.ctx.keep_running() {
                    break;
                }
            }
            Ok(())
        };
        spawn!(client, keep_running, "unary async", f)
    }

    fn execute_stream_ping_pong(mut self) {
        let client = self.client.clone();
        let keep_running = self.ctx.keep_running.clone();
        let (mut sender, mut receiver) = self.client.streaming_call().unwrap();
        let f = async move {
            loop {
                let latency_timer = Instant::now();
                sender
                    .send((self.req.clone(), WriteFlags::default()))
                    .await?;
                receiver.try_next().await?;
                self.ctx.observe_latency(latency_timer.elapsed());
                let mut time = self.ctx.backoff_async();
                if let Some(t) = &mut time {
                    t.await;
                }
                if !self.ctx.keep_running() {
                    break;
                }
            }
            sender.close().await?;
            receiver.try_next().await?;
            Ok(())
        };
        spawn!(client, keep_running, "streaming ping pong", f);
    }

    fn execute_stream_from_client(self) {
        let client = self.client.clone();
        let keep_running = self.ctx.keep_running.clone();

        let f = async move {
            let (mut sender, receiver) = self.client.streaming_from_client().unwrap();

            let send_stream = Box::pin(stream::unfold(
                (self, true, Instant::now()),
                |(mut c, init, last_time)| async move {
                    if !init {
                        c.ctx.observe_latency(last_time.elapsed());
                        let mut time = c.ctx.backoff_async();
                        if let Some(t) = &mut time {
                            t.await;
                        }
                    }
                    if c.ctx.keep_running() {
                        Some((c.req.clone(), (c, false, Instant::now())))
                    } else {
                        None
                    }
                },
            ));

            sender.enhance_batch(true);
            sender
                .send_all(&mut send_stream.map(move |item| Ok((item, WriteFlags::default()))))
                .await?;

            sender.close().await?;
            receiver.await?;
            Ok(())
        };
        spawn!(client, keep_running, "streaming from client", f);
    }
}

#[cfg(feature = "protobuf-codec")]
fn get_payload_cfg(cfg: &ClientConfig) -> &PayloadConfig {
    cfg.get_payload_config()
}
#[cfg(feature = "protobufv3-codec")]
fn get_payload_cfg(cfg: &ClientConfig) -> &PayloadConfig {
    &cfg.payload_config
}
#[cfg(feature = "protobuf-codec")]
fn get_rpc_type(cfg: &ClientConfig) -> RpcType {
    cfg.get_rpc_type()
}
#[cfg(feature = "protobufv3-codec")]
fn get_rpc_type(cfg: &ClientConfig) -> RpcType {
    cfg.rpc_type.enum_value().unwrap()
}

fn execute<B: Backoff + Send + 'static>(
    ctx: ExecutorContext<B>,
    ch: Channel,
    client_type: ClientType,
    cfg: &ClientConfig,
) {
    match client_type {
        ClientType::SYNC_CLIENT => {
            if get_payload_cfg(cfg).has_bytebuf_params() {
                panic!("only async_client is supported for generic service.");
            }
            RequestExecutor::new(ctx, ch, cfg).execute_unary()
        }
        ClientType::ASYNC_CLIENT => match get_rpc_type(cfg) {
            RpcType::UNARY => {
                if get_payload_cfg(cfg).has_bytebuf_params() {
                    panic!("only ping pong streaming is supported for generic service.");
                }
                RequestExecutor::new(ctx, ch, cfg).execute_unary_async()
            }
            RpcType::STREAMING => {
                if get_payload_cfg(cfg).has_bytebuf_params() {
                    GenericExecutor::new(ctx, ch, cfg).execute_stream()
                } else {
                    RequestExecutor::new(ctx, ch, cfg).execute_stream_ping_pong()
                }
            }
            RpcType::STREAMING_FROM_CLIENT => {
                if get_payload_cfg(cfg).has_bytebuf_params() {
                    panic!("only ping pong streaming is supported for generic service.");
                }
                RequestExecutor::new(ctx, ch, cfg).execute_stream_from_client()
            }
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

#[cfg(feature = "protobuf-codec")]
fn get_security_params(cfg: &ClientConfig) -> Option<&SecurityParams> {
    match cfg.has_security_params() {
        true => Some(cfg.get_security_params()),
        false => None,
    }
}
#[cfg(feature = "protobufv3-codec")]
fn get_security_params(cfg: &ClientConfig) -> Option<&SecurityParams> {
    cfg.security_params.0.as_deref()
}

impl Client {
    pub fn new(cfg: &ClientConfig) -> Client {
        let mut builder = EnvBuilder::new();
        let thd_cnt = cfg.async_client_threads as usize;
        if thd_cnt != 0 {
            builder = builder.cq_count(thd_cnt);
        }
        let env = Arc::new(builder.build());
        if cfg.core_limit > 0 {
            error!("client config core limit is set but ignored");
        }

        let ch_env = env.clone();
        let channels = (0..cfg.client_channels)
            .zip(cfg.server_targets.iter().cycle())
            .map(|(_, addr)| {
                let mut builder = ChannelBuilder::new(ch_env.clone());
                for arg in &cfg.channel_args {
                    let key = CString::new(arg.name.clone()).unwrap();
                    #[cfg(feature = "protobuf-codec")]
                    if arg.has_str_value() {
                        builder =
                            builder.raw_cfg_string(key, CString::new(arg.get_str_value()).unwrap());
                    } else if arg.has_int_value() {
                        builder = builder.raw_cfg_int(key, arg.get_int_value());
                    }
                    #[cfg(feature = "protobufv3-codec")]
                    if arg.has_str_value() {
                        builder =
                            builder.raw_cfg_string(key, CString::new(arg.str_value()).unwrap());
                    } else if arg.has_int_value() {
                        builder = builder.raw_cfg_int(key, arg.int_value());
                    }
                }
                // Check https://github.com/grpc/grpc/issues/31465.
                builder = builder.enable_retry(false);
                if let Some(params) = get_security_params(cfg) {
                    if !params.server_host_override.is_empty() {
                        builder =
                            builder.override_ssl_target(params.server_host_override.to_owned());
                    }
                    builder =
                        builder.set_credentials(proto_util::create_test_channel_credentials());
                }
                builder.connect(addr)
            });

        #[cfg(feature = "protobuf-codec")]
        let client_type = cfg.client_type;
        #[cfg(feature = "protobuf-codec")]
        let his_param = cfg.get_histogram_params();
        #[cfg(feature = "protobuf-codec")]
        let his = Arc::new(Mutex::new(Histogram::new(
            his_param.get_resolution(),
            his_param.get_max_possible(),
        )));
        #[cfg(feature = "protobuf-codec")]
        let has_poisson = cfg.get_load_params().has_poisson();

        #[cfg(feature = "protobufv3-codec")]
        let client_type = cfg.client_type.enum_value().unwrap();
        #[cfg(feature = "protobufv3-codec")]
        let his_param = &cfg.histogram_params;
        #[cfg(feature = "protobufv3-codec")]
        let his = Arc::new(Mutex::new(Histogram::new(
            his_param.resolution,
            his_param.max_possible,
        )));
        #[cfg(feature = "protobufv3-codec")]
        let has_poisson = cfg.load_params.has_poisson();

        let client_channels = cfg.client_channels as usize;
        let outstanding_rpcs_per_channel = cfg.outstanding_rpcs_per_channel as usize;

        let recorder = CpuRecorder::new();
        let keep_running = Arc::new(AtomicBool::new(true));
        let mut running_reqs = Vec::with_capacity(client_channels * outstanding_rpcs_per_channel);

        for ch in channels {
            for _ in 0..cfg.outstanding_rpcs_per_channel {
                let his = his.clone();
                let ch = ch.clone();
                let rx = if has_poisson {
                    #[cfg(feature = "protobuf-codec")]
                    let offered_load = cfg.get_load_params().get_poisson().get_offered_load();
                    #[cfg(feature = "protobufv3-codec")]
                    let offered_load = cfg.load_params.poisson().offered_load;

                    let lambda =
                        offered_load / client_channels as f64 / outstanding_rpcs_per_channel as f64;
                    let poisson = Poisson::new(lambda);
                    let (ctx, rx) = ExecutorContext::new(his, keep_running.clone(), poisson);
                    execute(ctx, ch, client_type, cfg);
                    rx
                } else {
                    let (ctx, rx) = ExecutorContext::new(his, keep_running.clone(), ClosedLoop);
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
        let mut stats = ClientStats::default();

        let sample = self.recorder.cpu_time(reset);
        stats.time_elapsed = sample.real_time;
        stats.time_user = sample.user_time;
        stats.time_system = sample.sys_time;

        {
            let mut his = self.histogram.lock().unwrap();
            stats.latencies = Some(his.report(reset)).into();
        }

        stats
    }

    pub fn shutdown(&mut self) -> impl Future<Output = ()> + Send {
        self.keep_running.store(false, Ordering::Relaxed);
        let tasks = self.running_reqs.take().unwrap();
        futures_util::future::join_all(tasks).map(|_| ())
    }
}
