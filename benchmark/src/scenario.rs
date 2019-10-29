use grpcio_proto::testing::control::{
    ChannelArg, ChannelArg_oneof_value, ClientConfig, ClientType, RpcType, Scenario, ServerConfig,
    ServerType,
};
use grpcio_proto::testing::payloads::{ByteBufferParams, PayloadConfig, SimpleProtoParams};
use grpcio_proto::testing::stats::HistogramParams;
use protobuf::RepeatedField;

/**
 * Other API: <test name> <args>
 * Note: this is used to custom tests
 */

#[allow(dead_code)]
pub fn async_unary_1channel_4rpcs_io50ms() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 50".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::UNARY);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(1);
    client.set_outstanding_rpcs_per_channel(4);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
pub fn async_unary_1channel_4rpcs_io50ms_1mb() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 50".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::UNARY);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(1);
    client.set_outstanding_rpcs_per_channel(4);
    let config = create_simple_config(1024 * 1024, 1024 * 1024);
    client.set_payload_config(config);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
pub fn async_unary_1channel_4rpcs_1mb() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 0".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::UNARY);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(1);
    client.set_outstanding_rpcs_per_channel(4);
    let config = create_simple_config(1024 * 1024, 1024 * 1024);
    client.set_payload_config(config);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
pub fn async_unary_1channel_1000rpcs_1mb() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 0".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::UNARY);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(1);
    client.set_outstanding_rpcs_per_channel(1000);
    let config = create_simple_config(1024 * 1024, 1024 * 1024);
    client.set_payload_config(config);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
pub fn async_unary_1channel_1000rpcs_64() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 0".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::UNARY);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(1);
    client.set_outstanding_rpcs_per_channel(1000);
    let config = create_simple_config(64, 64);
    client.set_payload_config(config);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
pub fn async_unary_1channel_1000rpcs_io500ms_32kb() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 500".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::UNARY);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(1);
    client.set_outstanding_rpcs_per_channel(1000);
    let config = create_simple_config(32, 32);
    client.set_payload_config(config);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
pub fn async_stream_8channel_1000rpcs_1mb() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 0".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::STREAMING);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(8);
    client.set_outstanding_rpcs_per_channel(1000);
    let config = create_simple_config(1024 * 1024, 1024 * 1024);
    client.set_payload_config(config);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
pub fn async_stream_1channel_100rpcs_1mb() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 0".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::STREAMING);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(1);
    client.set_outstanding_rpcs_per_channel(100);
    let config = create_simple_config(1024 * 1024, 1024 * 1024);
    client.set_payload_config(config);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
pub fn async_stream_4channel_100rpcs_16kb() -> Scenario {
    let mut client = ClientConfig::new();
    let mut server = ServerConfig::new();
    // config server
    server.set_server_type(ServerType::OTHER_SERVER);
    server.set_other_server_api("sleep 0".to_string());
    // config client
    client.set_client_type(ClientType::ASYNC_CLIENT);
    client.set_rpc_type(RpcType::STREAMING);
    client.set_histogram_params(create_histogram_params(0.01, 60e9));
    client.set_other_client_api("custom".to_string());
    client.set_client_channels(4);
    client.set_outstanding_rpcs_per_channel(100);
    let config = create_simple_config(16, 16);
    client.set_payload_config(config);
    // generate scenario
    let mut s = Scenario::new();
    s.set_client_config(client);
    s.set_server_config(server);
    s.set_num_clients(1);
    s.set_num_servers(1);
    s.set_warmup_seconds(1);
    s.set_benchmark_seconds(600);
    s
}

#[allow(dead_code)]
fn create_bytebuf_config(req_size: i32, resp_size: i32) -> PayloadConfig {
    let mut p = ByteBufferParams::new();
    let mut config = PayloadConfig::new();
    p.set_req_size(req_size);
    p.set_resp_size(resp_size);
    config.set_bytebuf_params(p);
    config
}

#[allow(dead_code)]
fn create_simple_config(req_size: i32, resp_size: i32) -> PayloadConfig {
    let mut p = SimpleProtoParams::new();
    let mut config = PayloadConfig::new();
    p.set_req_size(req_size);
    p.set_resp_size(resp_size);
    config.set_simple_params(p);
    config
}

#[allow(dead_code)]
fn create_channel_arg(name: String, arg_oneof_val: ChannelArg_oneof_value) -> ChannelArg {
    let mut ret = ChannelArg::new();
    ret.set_name(name);
    match arg_oneof_val {
        ChannelArg_oneof_value::str_value(val) => ret.set_str_value(val),
        ChannelArg_oneof_value::int_value(val) => ret.set_int_value(val),
    }
    ret
}

#[allow(dead_code)]
fn create_channel_arg_list(vec: Vec<ChannelArg>) -> RepeatedField<ChannelArg> {
    RepeatedField::from_vec(vec)
}

#[allow(dead_code)]
fn create_histogram_params(d1: f64, d2: f64) -> HistogramParams {
    let mut params = HistogramParams::new();
    params.set_resolution(d1);
    params.set_max_possible(d2);
    params
}
