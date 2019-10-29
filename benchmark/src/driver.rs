use crate::create_worker;
use crate::util::{average, sum, Histogram};
use futures::{future, Future, Sink, Stream};
use grpcio::{ChannelBuilder, Environment, WriteFlags};
use grpcio::{ClientDuplexReceiver, ClientDuplexSender};
use grpcio_proto::testing::control::{
    ClientArgs, ClientConfig, ClientStatus, Mark, Scenario, ScenarioResult, ServerArgs,
    ServerConfig, ServerStatus, Void,
};

use grpcio_proto::testing::services_grpc::WorkerServiceClient;
use grpcio_proto::testing::stats::RequestResultCount;
use protobuf::RepeatedField;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

/// Send control information to run the test, if there
/// are no workers available, start the workers locally.
#[allow(clippy::cognitive_complexity)]
pub fn run_worker(s: Scenario, mut worker_addrs: Vec<String>) -> ScenarioResult {
    // Parse the scenario
    let client_config = s.get_client_config();
    let server_config = s.get_server_config();
    let warmup_seconds = s.get_warmup_seconds() as u64;
    let benchmark_seconds = s.get_benchmark_seconds() as u64;
    let num_clients = s.get_num_clients() as usize;
    let num_servers = s.get_num_servers() as usize;
    let mut result = ScenarioResult::new();
    let mut merged_latencies = Histogram::new(0.01, 60e9);
    let mut merged_statuses = HashMap::new();
    // Create workers
    if worker_addrs.is_empty() {
        for i in 0..2 {
            worker_addrs.push(format!("0.0.0.0:{}", 8080 + i));
            thread::spawn(move || {
                create_worker(8080 + i);
            });
        }
    } else {
        assert_eq!(num_clients + num_servers, worker_addrs.len());
    }
    for addr in &worker_addrs {
        info!("Driver is ready to connect {}", addr);
    }
    // Connect to server workers
    let env = Arc::new(Environment::new(2));
    let mut servers = vec![];
    for addr in worker_addrs.iter() {
        let ch = ChannelBuilder::new(env.clone()).connect(addr.as_str());
        let client = WorkerServiceClient::new(ch);
        let (tx, rx) = client.run_server().unwrap();
        servers.push(ServerWorker {
            client,
            tx: Some(tx),
            rx: Some(rx),
        });
    }
    // Connect to client workers
    let mut clients = vec![];
    for i in 0..num_clients {
        let addr = worker_addrs[i + num_servers].clone();
        let ch = ChannelBuilder::new(env.clone()).connect(addr.as_str());
        let client = WorkerServiceClient::new(ch);
        let (tx, rx) = client.run_client().unwrap();
        clients.push(ClientWorker {
            client,
            tx: Some(tx),
            rx: Some(rx),
        });
    }
    // Configure port for servers
    // Run servers
    for (i, worker) in servers.iter_mut().enumerate() {
        let mut config = server_config.clone();
        config.set_port(50000 + i as i32);
        let args = create_server_args(config);
        let new_tx = worker
            .tx
            .take()
            .unwrap()
            .send((args, WriteFlags::default()))
            .wait()
            .unwrap();
        worker.tx.replace(new_tx);
        match worker.rx.take().unwrap().into_future().wait() {
            Ok((Some(status), new_rx)) => {
                info!("Run server on port {}", status.get_port());
                worker.rx.replace(new_rx);
            }
            Err((e, _)) => panic!("Get server status failed: {:?}", e),
            _ => unimplemented!(),
        }
    }
    // Configure server targets (Todo: support identify outer address)
    // Run clients
    for worker in clients.iter_mut() {
        let mut config = client_config.clone();
        let mut target_list = vec![];
        for index in 0..num_servers {
            target_list.push(format!("0.0.0.0:{}", 50000 + index as i32));
        }
        config.set_server_targets(create_server_targets(target_list.clone()));
        let args = create_client_args(config);
        let new_tx = worker
            .tx
            .take()
            .unwrap()
            .send((args, WriteFlags::default()))
            .wait()
            .unwrap();
        worker.tx.replace(new_tx);
        match worker.rx.take().unwrap().into_future().wait() {
            Ok((Some(_), new_rx)) => {
                info!("Run client to ports {:?}", target_list);
                worker.rx.replace(new_rx);
            }
            Err((e, _)) => panic!("Get client status failed: {:?}", e),
            _ => unimplemented!(),
        }
    }
    // Initialization
    let server_mark = create_server_args_mark(true);
    let client_mark = create_client_args_mark(true);
    for worker in clients.iter_mut() {
        let new_tx = worker
            .tx
            .take()
            .unwrap()
            .send((client_mark.clone(), WriteFlags::default()))
            .wait()
            .unwrap();
        worker.tx.replace(new_tx);
        match worker.rx.take().unwrap().into_future().wait() {
            Ok((Some(_), new_rx)) => {
                info!("Initialized");
                worker.rx.replace(new_rx);
            }
            Err((e, _)) => panic!("Get client status failed: {:?}", e),
            _ => unimplemented!(),
        }
    }
    // Let everything warmup
    info!("Warming up...");
    thread::sleep(std::time::Duration::from_secs(warmup_seconds));
    // Start a Run
    for worker in servers.iter_mut() {
        let new_tx = worker
            .tx
            .take()
            .unwrap()
            .send((server_mark.clone(), WriteFlags::default()))
            .wait()
            .unwrap();
        worker.tx.replace(new_tx);
    }
    for worker in clients.iter_mut() {
        let new_tx = worker
            .tx
            .take()
            .unwrap()
            .send((client_mark.clone(), WriteFlags::default()))
            .wait()
            .unwrap();
        worker.tx.replace(new_tx);
    }
    for worker in servers.iter_mut() {
        match worker.rx.take().unwrap().into_future().wait() {
            Ok((Some(_), new_rx)) => {
                worker.rx.replace(new_rx);
            }
            Err((e, _)) => panic!("Get server status failed: {:?}", e),
            _ => unimplemented!(),
        }
    }
    for worker in clients.iter_mut() {
        match worker.rx.take().unwrap().into_future().wait() {
            Ok((Some(_), new_rx)) => {
                worker.rx.replace(new_rx);
            }
            Err((e, _)) => panic!("Get client status failed: {:?}", e),
            _ => unimplemented!(),
        }
    }
    info!("Running...");
    thread::sleep(std::time::Duration::from_secs(benchmark_seconds));
    // Finish write client
    for worker in clients.iter_mut() {
        let new_tx = worker
            .tx
            .take()
            .unwrap()
            .send((client_mark.clone(), WriteFlags::default()))
            .wait()
            .unwrap();
        worker.tx.replace(new_tx);
        future::poll_fn(|| worker.tx.as_mut().unwrap().close())
            .wait()
            .unwrap();
    }
    // Read Client Stats
    for worker in clients.iter_mut() {
        match worker.rx.take().unwrap().into_future().wait() {
            Ok((Some(mut status), new_rx)) => {
                let stats = status.take_stats();
                merged_latencies.merge_proto(stats.get_latencies());
                for request_result in stats.get_request_results() {
                    let code = request_result.get_status_code();
                    let count = request_result.get_count();
                    if let Some(val) = merged_statuses.get_mut(&code) {
                        *val += count;
                    } else {
                        merged_statuses.insert(code, count);
                    }
                }
                result.mut_client_stats().push(stats);
                // hack code
                result.mut_client_success().push(true);
                if let Ok((None, new_rx)) = new_rx.into_future().wait() {
                    worker.rx.replace(new_rx);
                } else {
                    panic!("final status should be the last message on the client stream");
                }
            }
            Err((e, _)) => panic!("Get client status failed: {:?}", e),
            _ => unimplemented!(),
        }
    }
    // Fill proto and request results
    merged_latencies.fill_proto(result.mut_latencies());
    for (key, val) in merged_statuses.iter() {
        let mut result_count = RequestResultCount::new();
        result_count.set_count(val.clone());
        result_count.set_status_code(key.clone());
        result.mut_request_results().push(result_count);
    }
    // Finish Server
    for worker in servers.iter_mut() {
        let new_tx = worker
            .tx
            .take()
            .unwrap()
            .send((server_mark.clone(), WriteFlags::default()))
            .wait()
            .unwrap();
        worker.tx.replace(new_tx);
        future::poll_fn(|| worker.tx.as_mut().unwrap().close())
            .wait()
            .unwrap();
    }
    // Read Server Stats
    for worker in servers.iter_mut() {
        match worker.rx.take().unwrap().into_future().wait() {
            Ok((Some(mut status), new_rx)) => {
                let stats = status.take_stats();
                let cores = status.get_cores();
                result.mut_server_stats().push(stats);
                result.mut_server_cores().push(cores);
                // hack code
                result.mut_server_success().push(true);
                if let Ok((None, new_rx)) = new_rx.into_future().wait() {
                    worker.rx.replace(new_rx);
                } else {
                    panic!("final status should be the last message on the server stream");
                }
            }
            Err((e, _)) => panic!("Get server status failed: {:?}", e),
            _ => unimplemented!(),
        }
    }
    // Quit worker
    for worker in clients.iter_mut() {
        let _ = worker.client.quit_worker(&Void::new());
    }
    for worker in servers.iter_mut() {
        let _ = worker.client.quit_worker(&Void::new());
    }
    thread::sleep(std::time::Duration::from_secs(3));
    // Post handle
    postprocess_scenario_result(&mut result);
    info!("{:?}", result.get_summary());
    result
}

pub fn create_server_args(config: ServerConfig) -> ServerArgs {
    let mut arg = ServerArgs::new();
    arg.set_setup(config);
    arg
}

pub fn create_server_args_mark(m: bool) -> ServerArgs {
    let mut arg = ServerArgs::new();
    let mut mark = Mark::new();
    mark.set_reset(m);
    arg.set_mark(mark);
    arg
}

pub fn create_client_args(config: ClientConfig) -> ClientArgs {
    let mut arg = ClientArgs::new();
    arg.set_setup(config);
    arg
}

pub fn create_client_args_mark(m: bool) -> ClientArgs {
    let mut arg = ClientArgs::new();
    let mut mark = Mark::new();
    mark.set_reset(m);
    arg.set_mark(mark);
    arg
}

fn create_server_targets(vec: Vec<String>) -> RepeatedField<String> {
    RepeatedField::from_vec(vec)
}

struct ServerWorker {
    client: WorkerServiceClient,
    tx: Option<ClientDuplexSender<ServerArgs>>,
    rx: Option<ClientDuplexReceiver<ServerStatus>>,
}

struct ClientWorker {
    client: WorkerServiceClient,
    tx: Option<ClientDuplexSender<ClientArgs>>,
    rx: Option<ClientDuplexReceiver<ClientStatus>>,
}

fn postprocess_scenario_result(result: &mut ScenarioResult) {
    let mut his = Histogram::new(0.01, 60e9);
    his.merge_proto(result.mut_latencies());

    let time_estimate = average(result.get_client_stats(), |s| s.get_time_elapsed());
    let qps = his.get_count() / time_estimate;
    let mut sum_qps = 0f64;
    for i in result.get_server_cores() {
        sum_qps += f64::from(*i);
    }
    let qps_per_server_core = qps / sum_qps;
    result.mut_summary().set_qps(qps);
    result
        .mut_summary()
        .set_qps_per_server_core(qps_per_server_core);
    let server_system_time = 100.0 * sum(result.get_server_stats(), |s| s.get_time_system())
        / sum(result.get_server_stats(), |s| s.get_time_elapsed());
    let server_user_time = 100.0 * sum(result.get_server_stats(), |s| s.get_time_user())
        / sum(result.get_server_stats(), |s| s.get_time_elapsed());
    let client_system_time = 100.0 * sum(result.get_client_stats(), |s| s.get_time_system())
        / sum(result.get_client_stats(), |s| s.get_time_elapsed());
    let client_user_time = 100.0 * sum(result.get_client_stats(), |s| s.get_time_user())
        / sum(result.get_client_stats(), |s| s.get_time_elapsed());

    result
        .mut_summary()
        .set_server_system_time(server_system_time);
    result.mut_summary().set_server_user_time(server_user_time);
    result
        .mut_summary()
        .set_client_system_time(client_system_time);
    result.mut_summary().set_client_user_time(client_user_time);

    if average(result.get_server_stats(), |s| s.get_total_cpu_time() as f64) == 0f64 {
        result.mut_summary().set_server_cpu_usage(0f64);
    } else {
        let server_cpu_usage = 100.0
            - 100.0 * average(result.get_server_stats(), |s| s.get_idle_cpu_time() as f64)
                / average(result.get_server_stats(), |s| s.get_total_cpu_time() as f64);
        result.mut_summary().set_server_cpu_usage(server_cpu_usage);
    }

    if !result.get_request_results().is_empty() {
        let mut successes = 0f64;
        let mut failures = 0f64;
        for i in result.get_request_results() {
            if i.get_status_code() == 0 {
                successes += i.get_count() as f64;
            } else {
                failures += i.get_count() as f64;
            }
        }
        result
            .mut_summary()
            .set_successful_requests_per_second(successes / time_estimate);
        result
            .mut_summary()
            .set_failed_requests_per_second(failures / time_estimate);
    }
    let client_polls_per_request =
        sum(result.get_client_stats(), |s| s.get_cq_poll_count() as f64) / his.get_count();
    result
        .mut_summary()
        .set_client_polls_per_request(client_polls_per_request);
    let server_polls_per_request =
        sum(result.get_server_stats(), |s| s.get_cq_poll_count() as f64) / his.get_count();
    result
        .mut_summary()
        .set_server_polls_per_request(server_polls_per_request);

    let server_queries_per_cpu_sec = his.get_count()
        / (sum(result.get_server_stats(), |s| s.get_time_system())
            + sum(result.get_server_stats(), |s| s.get_time_user()));

    let client_queries_per_cpu_sec = his.get_count()
        / (sum(result.get_client_stats(), |s| s.get_time_system())
            + sum(result.get_client_stats(), |s| s.get_time_user()));

    result
        .mut_summary()
        .set_server_queries_per_cpu_sec(server_queries_per_cpu_sec);
    result
        .mut_summary()
        .set_client_queries_per_cpu_sec(client_queries_per_cpu_sec);
}
