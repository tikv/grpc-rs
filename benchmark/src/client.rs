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
use std::thread;
use std::time::{Instant, Duration};

use grpc::{Channel, ChannelBuilder, Environment};
use grpc_proto::testing::control::{ClientConfig, ClientType, RpcType};
use grpc_proto::testing::messages::SimpleRequest;
use grpc_proto::testing::services_grpc::BenchmarkServiceClient;
use grpc_proto::testing::stats::ClientStats;
use grpc_proto::util as proto_util;
use futures::{future, Stream, Sink, Future, Async};
use futures::future::Loop;
use rand::distributions::Exp;
use rand::distributions::Sample;
use rand::{self, XorShiftRng, SeedableRng};
use tokio_core::reactor::Remote;
use tokio_timer::{Timer, Sleep};

use error::Error;
use util::{self, CpuRecorder, Histogram};

fn gen_req(cfg: &ClientConfig) -> SimpleRequest {
    let mut req = SimpleRequest::new();
    let payload_config = cfg.get_payload_config();
    let simple_params = payload_config.get_simple_params();
    req.set_payload(proto_util::new_payload(simple_params.get_req_size() as usize));
    req.set_response_size(simple_params.get_resp_size());
    req
}

trait BackOff {
    fn back_off_time(&mut self) -> Option<Duration>;

    fn back_off(&mut self) {
        if let Some(dur) = self.back_off_time() {
            thread::sleep(dur)
        }
    }

    fn back_off_async(&mut self, timer: &Timer) -> Option<Sleep> {
        self.back_off_time().map(|dur| {
            timer.sleep(dur)
        })
    }
}

struct ClosedLoop;

impl BackOff for ClosedLoop {
    fn back_off_time(&mut self) -> Option<Duration> {
        None
    }
}


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

impl BackOff for Poisson {
    fn back_off_time(&mut self) -> Option<Duration> {
        let back_off_time = self.exp.sample(&mut self.r);
        let sec = back_off_time as u64;
        let ns = (back_off_time.fract() * 1_000_000_000f64) as u32;
        self.last_time = self.last_time + Duration::new(sec, ns);
        let now = Instant::now();
        if self.last_time > now {
            Some(self.last_time - now)
        } else {
            None
        }
    }
}

struct RequestExecutor<B> {
    client: BenchmarkServiceClient,
    req: SimpleRequest,
    histogram: Arc<Mutex<Histogram>>,
    back_off: B,
    timer: Timer,
}

impl<B: BackOff + Send + 'static> RequestExecutor<B> {
    fn new(channel: Channel, cfg: &ClientConfig, histogram: Arc<Mutex<Histogram>>, back_off: B, timer: Timer) ->  RequestExecutor<B> {
        RequestExecutor {
            client: BenchmarkServiceClient::new(channel),
            req: gen_req(cfg),
            histogram: histogram,
            back_off: back_off,
            timer: timer,
        }
    }

    fn observe_latency(&self, latency: Duration) {
        let f = util::dur_to_secs(latency);
        let mut his = self.histogram.lock().unwrap();
        his.observe(f);
    }

    fn execute_unary(mut self) {
        thread::spawn(move || {
            loop {
                let latency_timer = Instant::now();
                self.client.unary_call(self.req.clone()).unwrap();
                let elapsed = latency_timer.elapsed();
                self.observe_latency(elapsed);
                self.back_off.back_off();
            }
        });
    }

    fn execute_unary_async(self, remote: &Remote) {
        let f = future::loop_fn(self, move |mut executor| {
            let latency_timer = Instant::now();
            let handler = executor.client.unary_call_async(executor.req.clone());

            handler.map_err(Error::from).and_then(move |_| {
                let elapsed = latency_timer.elapsed();
                executor.observe_latency(elapsed);
                let mut time = executor.back_off.back_off_async(&executor.timer);
                let mut res = Some(executor);
                future::poll_fn(move || {
                    if let Some(ref mut t) = time {
                        try_ready!(t.poll());
                    }
                    time.take();
                    Ok(Async::Ready(Loop::Continue(res.take().unwrap())))
                })
            })
        }).map_err(|e| println!("failed to execute unary async: {:?}", e));
        remote.spawn(|_| f)
    }

    fn execute_stream_ping_pong(self, r: &Remote) {
        let mut handler = self.client.streaming_call();
        let receiver = handler.take_receiver().unwrap();
        let f = future::loop_fn((handler, self, receiver), move |(h, mut executor, receiver)| {
            let latency_timer = Instant::now();
            let send = h.send(executor.req.clone());
            send.map_err(Error::from).and_then(move |h| {
                receiver.into_future().map_err(|(e, _)| Error::from(e)).and_then(move |(_, r)| {
                    let elapsed = latency_timer.elapsed();
                    let f = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000f64;
                    {
                        let mut his = executor.histogram.lock().unwrap();
                        his.observe(f);
                    }
                    let mut time = executor.back_off.back_off_async(&executor.timer);
                    let mut res = Some((h, executor, r));
                    future::poll_fn(move || {
                        if let Some(ref mut t) = time {
                            try_ready!(t.poll());
                        }
                        time.take();
                        Ok(Async::Ready(Loop::Continue(res.take().unwrap())))
                    })
                })
            })
        }).map_err(|e| println!("failed to execute streaming ping pong: {:?}", e));
        r.spawn(|_| f)
    }
} 

pub struct Client {
    recorder: CpuRecorder,
    histogram: Arc<Mutex<Histogram>>,
}

impl Client {
    pub fn new(env: Arc<Environment>, cfg: &ClientConfig, remote: Remote) -> Client {
        if cfg.get_async_client_threads() > 0  {
            println!("client config asyc_client_threads is set but ignored");
        }
        if cfg.get_core_limit() > 0 {
            println!("client config core limit is set but ignored");
        }
        if cfg.has_security_params() {
            println!("client config security params is set but ignored");
        }

        let channels = (0..cfg.get_client_channels()).zip(cfg.get_server_targets().into_iter().cycle()).map(|(_, addr)| {
            ChannelBuilder::new(env.clone()).connect(addr)
        });
        
        if cfg.get_payload_config().has_bytebuf_params() {
            unimplemented!()
        }

        let client_type = cfg.get_client_type();
        let load_params = cfg.get_load_params();
        let poisson_lamda = if load_params.has_poisson() {
            let poisson = load_params.get_poisson();
            Some(poisson.get_offered_load() / cfg.get_client_channels() as f64 / cfg.get_outstanding_rpcs_per_channel() as f64)
        } else {
            None
        };

        let recorder = CpuRecorder::new();
        let his_param = cfg.get_histogram_params();
        let his = Arc::new(Mutex::new(Histogram::new(his_param.get_resolution(), his_param.get_max_possible())));
        let timer = Timer::default();
        
        for ch in channels {
            for _ in 0..cfg.get_outstanding_rpcs_per_channel() {
                let his = his.clone();
                let t = timer.clone();
                let poisson = poisson_lamda.map(Poisson::new);
                
                match client_type {
                    ClientType::SYNC_CLIENT => {
                        if let Some(p) = poisson {
                            RequestExecutor::new(ch.clone(), cfg, his, p, t).execute_unary()
                        } else {
                            RequestExecutor::new(ch.clone(), cfg, his, ClosedLoop, t).execute_unary()
                        }
                    }
                    ClientType::ASYNC_CLIENT => {
                        match cfg.get_rpc_type() {
                            RpcType::UNARY => {
                                if let Some(p) = poisson {
                                    RequestExecutor::new(ch.clone(), cfg, his, p, t).execute_unary_async(&remote)
                                } else {
                                    RequestExecutor::new(ch.clone(), cfg, his, ClosedLoop, t).execute_unary_async(&remote)
                                }
                            }
                            RpcType::STREAMING => {
                                if let Some(p) = poisson {
                                    RequestExecutor::new(ch.clone(), cfg, his, p, t).execute_stream_ping_pong(&remote)
                                } else {
                                    RequestExecutor::new(ch.clone(), cfg, his, ClosedLoop, t).execute_stream_ping_pong(&remote)
                                }
                            }
                        }
                    }
                    _ => unimplemented!()
                }
            }
        }

        Client {
            recorder: recorder,
            histogram: his,
        }
    }

    pub fn get_stats(&mut self, reset: bool) -> ClientStats {
        let mut stats = ClientStats::new();

        let (real_time, user_time, sys_time) = self.recorder.cpu_time(reset);
        stats.set_time_elapsed(real_time);
        stats.set_time_user(user_time);
        stats.set_time_system(sys_time);

        {
            let mut his = self.histogram.lock().unwrap();
            stats.set_latencies(his.report(reset));
        }

        stats
    }
}
