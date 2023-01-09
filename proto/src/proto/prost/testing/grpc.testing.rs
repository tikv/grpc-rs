#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByteBufferParams {
    #[prost(int32, tag = "1")]
    pub req_size: i32,
    #[prost(int32, tag = "2")]
    pub resp_size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleProtoParams {
    #[prost(int32, tag = "1")]
    pub req_size: i32,
    #[prost(int32, tag = "2")]
    pub resp_size: i32,
}
/// TODO (vpai): Fill this in once the details of complex, representative
///               protos are decided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplexProtoParams {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadConfig {
    #[prost(oneof = "payload_config::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<payload_config::Payload>,
}
/// Nested message and enum types in `PayloadConfig`.
pub mod payload_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        BytebufParams(super::ByteBufferParams),
        #[prost(message, tag = "2")]
        SimpleParams(super::SimpleProtoParams),
        #[prost(message, tag = "3")]
        ComplexParams(super::ComplexProtoParams),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStats {
    /// wall clock time change in seconds since last reset
    #[prost(double, tag = "1")]
    pub time_elapsed: f64,
    /// change in user time (in seconds) used by the server since last reset
    #[prost(double, tag = "2")]
    pub time_user: f64,
    /// change in server time (in seconds) used by the server process and all
    /// threads since last reset
    #[prost(double, tag = "3")]
    pub time_system: f64,
    /// change in total cpu time of the server (data from proc/stat)
    #[prost(uint64, tag = "4")]
    pub total_cpu_time: u64,
    /// change in idle time of the server (data from proc/stat)
    #[prost(uint64, tag = "5")]
    pub idle_cpu_time: u64,
    /// Number of polls called inside completion queue
    #[prost(uint64, tag = "6")]
    pub cq_poll_count: u64,
}
/// Histogram params based on grpc/support/histogram.c
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramParams {
    /// first bucket is [0, 1 + resolution)
    #[prost(double, tag = "1")]
    pub resolution: f64,
    /// use enough buckets to allow this value
    #[prost(double, tag = "2")]
    pub max_possible: f64,
}
/// Histogram data based on grpc/support/histogram.c
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramData {
    #[prost(uint32, repeated, tag = "1")]
    pub bucket: ::prost::alloc::vec::Vec<u32>,
    #[prost(double, tag = "2")]
    pub min_seen: f64,
    #[prost(double, tag = "3")]
    pub max_seen: f64,
    #[prost(double, tag = "4")]
    pub sum: f64,
    #[prost(double, tag = "5")]
    pub sum_of_squares: f64,
    #[prost(double, tag = "6")]
    pub count: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestResultCount {
    #[prost(int32, tag = "1")]
    pub status_code: i32,
    #[prost(int64, tag = "2")]
    pub count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientStats {
    /// Latency histogram. Data points are in nanoseconds.
    #[prost(message, optional, tag = "1")]
    pub latencies: ::core::option::Option<HistogramData>,
    /// See ServerStats for details.
    #[prost(double, tag = "2")]
    pub time_elapsed: f64,
    #[prost(double, tag = "3")]
    pub time_user: f64,
    #[prost(double, tag = "4")]
    pub time_system: f64,
    /// Number of failed requests (one row per status code seen)
    #[prost(message, repeated, tag = "5")]
    pub request_results: ::prost::alloc::vec::Vec<RequestResultCount>,
    /// Number of polls called inside completion queue
    #[prost(uint64, tag = "6")]
    pub cq_poll_count: u64,
}
/// Parameters of poisson process distribution, which is a good representation
/// of activity coming in from independent identical stationary sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoissonParams {
    /// The rate of arrivals (a.k.a. lambda parameter of the exp distribution).
    #[prost(double, tag = "1")]
    pub offered_load: f64,
}
/// Once an RPC finishes, immediately start a new one.
/// No configuration parameters needed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosedLoopParams {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadParams {
    #[prost(oneof = "load_params::Load", tags = "1, 2")]
    pub load: ::core::option::Option<load_params::Load>,
}
/// Nested message and enum types in `LoadParams`.
pub mod load_params {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Load {
        #[prost(message, tag = "1")]
        ClosedLoop(super::ClosedLoopParams),
        #[prost(message, tag = "2")]
        Poisson(super::PoissonParams),
    }
}
/// presence of SecurityParams implies use of TLS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityParams {
    #[prost(bool, tag = "1")]
    pub use_test_ca: bool,
    #[prost(string, tag = "2")]
    pub server_host_override: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cred_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelArg {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "channel_arg::Value", tags = "2, 3")]
    pub value: ::core::option::Option<channel_arg::Value>,
}
/// Nested message and enum types in `ChannelArg`.
pub mod channel_arg {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "2")]
        StrValue(::prost::alloc::string::String),
        #[prost(int32, tag = "3")]
        IntValue(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConfig {
    /// List of targets to connect to. At least one target needs to be specified.
    #[prost(string, repeated, tag = "1")]
    pub server_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "ClientType", tag = "2")]
    pub client_type: i32,
    #[prost(message, optional, tag = "3")]
    pub security_params: ::core::option::Option<SecurityParams>,
    /// How many concurrent RPCs to start for each channel.
    /// For synchronous client, use a separate thread for each outstanding RPC.
    #[prost(int32, tag = "4")]
    pub outstanding_rpcs_per_channel: i32,
    /// Number of independent client channels to create.
    /// i-th channel will connect to server_target[i % server_targets.size()]
    #[prost(int32, tag = "5")]
    pub client_channels: i32,
    /// Only for async client. Number of threads to use to start/manage RPCs.
    #[prost(int32, tag = "7")]
    pub async_client_threads: i32,
    #[prost(enumeration = "RpcType", tag = "8")]
    pub rpc_type: i32,
    /// The requested load for the entire client (aggregated over all the threads).
    #[prost(message, optional, tag = "10")]
    pub load_params: ::core::option::Option<LoadParams>,
    #[prost(message, optional, tag = "11")]
    pub payload_config: ::core::option::Option<PayloadConfig>,
    #[prost(message, optional, tag = "12")]
    pub histogram_params: ::core::option::Option<HistogramParams>,
    /// Specify the cores we should run the client on, if desired
    #[prost(int32, repeated, tag = "13")]
    pub core_list: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, tag = "14")]
    pub core_limit: i32,
    /// If we use an OTHER_CLIENT client_type, this string gives more detail
    #[prost(string, tag = "15")]
    pub other_client_api: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "16")]
    pub channel_args: ::prost::alloc::vec::Vec<ChannelArg>,
    /// Number of threads that share each completion queue
    #[prost(int32, tag = "17")]
    pub threads_per_cq: i32,
    /// Number of messages on a stream before it gets finished/restarted
    #[prost(int32, tag = "18")]
    pub messages_per_stream: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientStatus {
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<ClientStats>,
}
/// Request current stats
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mark {
    /// if true, the stats will be reset after taking their snapshot.
    #[prost(bool, tag = "1")]
    pub reset: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientArgs {
    #[prost(oneof = "client_args::Argtype", tags = "1, 2")]
    pub argtype: ::core::option::Option<client_args::Argtype>,
}
/// Nested message and enum types in `ClientArgs`.
pub mod client_args {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Argtype {
        #[prost(message, tag = "1")]
        Setup(super::ClientConfig),
        #[prost(message, tag = "2")]
        Mark(super::Mark),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerConfig {
    #[prost(enumeration = "ServerType", tag = "1")]
    pub server_type: i32,
    #[prost(message, optional, tag = "2")]
    pub security_params: ::core::option::Option<SecurityParams>,
    /// Port on which to listen. Zero means pick unused port.
    #[prost(int32, tag = "4")]
    pub port: i32,
    /// Only for async server. Number of threads used to serve the requests.
    #[prost(int32, tag = "7")]
    pub async_server_threads: i32,
    /// Specify the number of cores to limit server to, if desired
    #[prost(int32, tag = "8")]
    pub core_limit: i32,
    /// payload config, used in generic server.
    /// Note this must NOT be used in proto (non-generic) servers. For proto servers,
    /// 'response sizes' must be configured from the 'response_size' field of the
    /// 'SimpleRequest' objects in RPC requests.
    #[prost(message, optional, tag = "9")]
    pub payload_config: ::core::option::Option<PayloadConfig>,
    /// Specify the cores we should run the server on, if desired
    #[prost(int32, repeated, tag = "10")]
    pub core_list: ::prost::alloc::vec::Vec<i32>,
    /// If we use an OTHER_SERVER client_type, this string gives more detail
    #[prost(string, tag = "11")]
    pub other_server_api: ::prost::alloc::string::String,
    /// Number of threads that share each completion queue
    #[prost(int32, tag = "12")]
    pub threads_per_cq: i32,
    /// Buffer pool size (no buffer pool specified if unset)
    #[prost(int32, tag = "1001")]
    pub resource_quota_size: i32,
    #[prost(message, repeated, tag = "1002")]
    pub channel_args: ::prost::alloc::vec::Vec<ChannelArg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerArgs {
    #[prost(oneof = "server_args::Argtype", tags = "1, 2")]
    pub argtype: ::core::option::Option<server_args::Argtype>,
}
/// Nested message and enum types in `ServerArgs`.
pub mod server_args {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Argtype {
        #[prost(message, tag = "1")]
        Setup(super::ServerConfig),
        #[prost(message, tag = "2")]
        Mark(super::Mark),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStatus {
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<ServerStats>,
    /// the port bound by the server
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// Number of cores available to the server
    #[prost(int32, tag = "3")]
    pub cores: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoreRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoreResponse {
    /// Number of cores available on the server
    #[prost(int32, tag = "1")]
    pub cores: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Void {}
/// A single performance scenario: input to qps_json_driver
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scenario {
    /// Human readable name for this scenario
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Client configuration
    #[prost(message, optional, tag = "2")]
    pub client_config: ::core::option::Option<ClientConfig>,
    /// Number of clients to start for the test
    #[prost(int32, tag = "3")]
    pub num_clients: i32,
    /// Server configuration
    #[prost(message, optional, tag = "4")]
    pub server_config: ::core::option::Option<ServerConfig>,
    /// Number of servers to start for the test
    #[prost(int32, tag = "5")]
    pub num_servers: i32,
    /// Warmup period, in seconds
    #[prost(int32, tag = "6")]
    pub warmup_seconds: i32,
    /// Benchmark time, in seconds
    #[prost(int32, tag = "7")]
    pub benchmark_seconds: i32,
    /// Number of workers to spawn locally (usually zero)
    #[prost(int32, tag = "8")]
    pub spawn_local_worker_count: i32,
}
/// A set of scenarios to be run with qps_json_driver
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scenarios {
    #[prost(message, repeated, tag = "1")]
    pub scenarios: ::prost::alloc::vec::Vec<Scenario>,
}
/// Basic summary that can be computed from ClientStats and ServerStats
/// once the scenario has finished.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScenarioResultSummary {
    /// Total number of operations per second over all clients.
    #[prost(double, tag = "1")]
    pub qps: f64,
    /// QPS per one server core.
    #[prost(double, tag = "2")]
    pub qps_per_server_core: f64,
    /// server load based on system_time (0.85 => 85%)
    #[prost(double, tag = "3")]
    pub server_system_time: f64,
    /// server load based on user_time (0.85 => 85%)
    #[prost(double, tag = "4")]
    pub server_user_time: f64,
    /// client load based on system_time (0.85 => 85%)
    #[prost(double, tag = "5")]
    pub client_system_time: f64,
    /// client load based on user_time (0.85 => 85%)
    #[prost(double, tag = "6")]
    pub client_user_time: f64,
    /// X% latency percentiles (in nanoseconds)
    #[prost(double, tag = "7")]
    pub latency_50: f64,
    #[prost(double, tag = "8")]
    pub latency_90: f64,
    #[prost(double, tag = "9")]
    pub latency_95: f64,
    #[prost(double, tag = "10")]
    pub latency_99: f64,
    #[prost(double, tag = "11")]
    pub latency_999: f64,
    /// server cpu usage percentage
    #[prost(double, tag = "12")]
    pub server_cpu_usage: f64,
    /// Number of requests that succeeded/failed
    #[prost(double, tag = "13")]
    pub successful_requests_per_second: f64,
    #[prost(double, tag = "14")]
    pub failed_requests_per_second: f64,
    /// Number of polls called inside completion queue per request
    #[prost(double, tag = "15")]
    pub client_polls_per_request: f64,
    #[prost(double, tag = "16")]
    pub server_polls_per_request: f64,
    /// Queries per CPU-sec over all servers or clients
    #[prost(double, tag = "17")]
    pub server_queries_per_cpu_sec: f64,
    #[prost(double, tag = "18")]
    pub client_queries_per_cpu_sec: f64,
}
/// Results of a single benchmark scenario.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScenarioResult {
    /// Inputs used to run the scenario.
    #[prost(message, optional, tag = "1")]
    pub scenario: ::core::option::Option<Scenario>,
    /// Histograms from all clients merged into one histogram.
    #[prost(message, optional, tag = "2")]
    pub latencies: ::core::option::Option<HistogramData>,
    /// Client stats for each client
    #[prost(message, repeated, tag = "3")]
    pub client_stats: ::prost::alloc::vec::Vec<ClientStats>,
    /// Server stats for each server
    #[prost(message, repeated, tag = "4")]
    pub server_stats: ::prost::alloc::vec::Vec<ServerStats>,
    /// Number of cores available to each server
    #[prost(int32, repeated, tag = "5")]
    pub server_cores: ::prost::alloc::vec::Vec<i32>,
    /// An after-the-fact computed summary
    #[prost(message, optional, tag = "6")]
    pub summary: ::core::option::Option<ScenarioResultSummary>,
    /// Information on success or failure of each worker
    #[prost(bool, repeated, tag = "7")]
    pub client_success: ::prost::alloc::vec::Vec<bool>,
    #[prost(bool, repeated, tag = "8")]
    pub server_success: ::prost::alloc::vec::Vec<bool>,
    /// Number of failed requests (one row per status code seen)
    #[prost(message, repeated, tag = "9")]
    pub request_results: ::prost::alloc::vec::Vec<RequestResultCount>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientType {
    /// Many languages support a basic distinction between using
    /// sync or async client, and this allows the specification
    SyncClient = 0,
    AsyncClient = 1,
    /// used for some language-specific variants
    OtherClient = 2,
}
impl ClientType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientType::SyncClient => "SYNC_CLIENT",
            ClientType::AsyncClient => "ASYNC_CLIENT",
            ClientType::OtherClient => "OTHER_CLIENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SYNC_CLIENT" => Some(Self::SyncClient),
            "ASYNC_CLIENT" => Some(Self::AsyncClient),
            "OTHER_CLIENT" => Some(Self::OtherClient),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerType {
    SyncServer = 0,
    AsyncServer = 1,
    AsyncGenericServer = 2,
    /// used for some language-specific variants
    OtherServer = 3,
}
impl ServerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServerType::SyncServer => "SYNC_SERVER",
            ServerType::AsyncServer => "ASYNC_SERVER",
            ServerType::AsyncGenericServer => "ASYNC_GENERIC_SERVER",
            ServerType::OtherServer => "OTHER_SERVER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SYNC_SERVER" => Some(Self::SyncServer),
            "ASYNC_SERVER" => Some(Self::AsyncServer),
            "ASYNC_GENERIC_SERVER" => Some(Self::AsyncGenericServer),
            "OTHER_SERVER" => Some(Self::OtherServer),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RpcType {
    Unary = 0,
    Streaming = 1,
    StreamingFromClient = 2,
    StreamingFromServer = 3,
    StreamingBothWays = 4,
}
impl RpcType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RpcType::Unary => "UNARY",
            RpcType::Streaming => "STREAMING",
            RpcType::StreamingFromClient => "STREAMING_FROM_CLIENT",
            RpcType::StreamingFromServer => "STREAMING_FROM_SERVER",
            RpcType::StreamingBothWays => "STREAMING_BOTH_WAYS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNARY" => Some(Self::Unary),
            "STREAMING" => Some(Self::Streaming),
            "STREAMING_FROM_CLIENT" => Some(Self::StreamingFromClient),
            "STREAMING_FROM_SERVER" => Some(Self::StreamingFromServer),
            "STREAMING_BOTH_WAYS" => Some(Self::StreamingBothWays),
            _ => None,
        }
    }
}
/// An empty message that you can re-use to avoid defining duplicated empty
/// messages in your project. A typical example is to use it as argument or the
/// return value of a service API. For instance:
///
///    service Foo {
///      rpc Bar (grpc.testing.Empty) returns (grpc.testing.Empty) { };
///    };
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// TODO(dgq): Go back to using well-known types once
/// <https://github.com/grpc/grpc/issues/6980> has been fixed.
/// import "google/protobuf/wrappers.proto";
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolValue {
    /// The bool value.
    #[prost(bool, tag = "1")]
    pub value: bool,
}
/// A block of data, to simply increase gRPC message size.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    /// DEPRECATED, don't use. To be removed shortly.
    /// The type of data in body.
    #[prost(enumeration = "PayloadType", tag = "1")]
    pub r#type: i32,
    /// Primary contents of payload.
    #[prost(bytes = "vec", tag = "2")]
    pub body: ::prost::alloc::vec::Vec<u8>,
}
/// A protobuf representation for grpc status. This is used by test
/// clients to specify a status that the server should attempt to return.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoStatus {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Unary request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleRequest {
    /// DEPRECATED, don't use. To be removed shortly.
    /// Desired payload type in the response from the server.
    /// If response_type is RANDOM, server randomly chooses one from other formats.
    #[prost(enumeration = "PayloadType", tag = "1")]
    pub response_type: i32,
    /// Desired payload size in the response from the server.
    #[prost(int32, tag = "2")]
    pub response_size: i32,
    /// Optional input payload sent along with the request.
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<Payload>,
    /// Whether SimpleResponse should include username.
    #[prost(bool, tag = "4")]
    pub fill_username: bool,
    /// Whether SimpleResponse should include OAuth scope.
    #[prost(bool, tag = "5")]
    pub fill_oauth_scope: bool,
    /// Whether to request the server to compress the response. This field is
    /// "nullable" in order to interoperate seamlessly with clients not able to
    /// implement the full compression tests by introspecting the call to verify
    /// the response's compression status.
    #[prost(message, optional, tag = "6")]
    pub response_compressed: ::core::option::Option<BoolValue>,
    /// Whether server should return a given status
    #[prost(message, optional, tag = "7")]
    pub response_status: ::core::option::Option<EchoStatus>,
    /// Whether the server should expect this request to be compressed.
    #[prost(message, optional, tag = "8")]
    pub expect_compressed: ::core::option::Option<BoolValue>,
}
/// Unary response, as configured by the request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleResponse {
    /// Payload to increase message size.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<Payload>,
    /// The user the request came from, for verifying authentication was
    /// successful when the client expected it.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// OAuth scope.
    #[prost(string, tag = "3")]
    pub oauth_scope: ::prost::alloc::string::String,
}
/// Client-streaming request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingInputCallRequest {
    /// Optional input payload sent along with the request.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<Payload>,
    /// Whether the server should expect this request to be compressed. This field
    /// is "nullable" in order to interoperate seamlessly with servers not able to
    /// implement the full compression tests by introspecting the call to verify
    /// the request's compression status.
    #[prost(message, optional, tag = "2")]
    pub expect_compressed: ::core::option::Option<BoolValue>,
}
/// Client-streaming response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingInputCallResponse {
    /// Aggregated size of payloads received from the client.
    #[prost(int32, tag = "1")]
    pub aggregated_payload_size: i32,
}
/// Configuration for a particular response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseParameters {
    /// Desired payload sizes in responses from the server.
    #[prost(int32, tag = "1")]
    pub size: i32,
    /// Desired interval between consecutive responses in the response stream in
    /// microseconds.
    #[prost(int32, tag = "2")]
    pub interval_us: i32,
    /// Whether to request the server to compress the response. This field is
    /// "nullable" in order to interoperate seamlessly with clients not able to
    /// implement the full compression tests by introspecting the call to verify
    /// the response's compression status.
    #[prost(message, optional, tag = "3")]
    pub compressed: ::core::option::Option<BoolValue>,
}
/// Server-streaming request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingOutputCallRequest {
    /// DEPRECATED, don't use. To be removed shortly.
    /// Desired payload type in the response from the server.
    /// If response_type is RANDOM, the payload from each response in the stream
    /// might be of different types. This is to simulate a mixed type of payload
    /// stream.
    #[prost(enumeration = "PayloadType", tag = "1")]
    pub response_type: i32,
    /// Configuration for each expected response message.
    #[prost(message, repeated, tag = "2")]
    pub response_parameters: ::prost::alloc::vec::Vec<ResponseParameters>,
    /// Optional input payload sent along with the request.
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<Payload>,
    /// Whether server should return a given status
    #[prost(message, optional, tag = "7")]
    pub response_status: ::core::option::Option<EchoStatus>,
}
/// Server-streaming response, as configured by the request and parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingOutputCallResponse {
    /// Payload to increase response size.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<Payload>,
}
/// For reconnect interop test only.
/// Client tells server what reconnection parameters it used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconnectParams {
    #[prost(int32, tag = "1")]
    pub max_reconnect_backoff_ms: i32,
}
/// For reconnect interop test only.
/// Server tells client whether its reconnects are following the spec and the
/// reconnect backoffs it saw.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconnectInfo {
    #[prost(bool, tag = "1")]
    pub passed: bool,
    #[prost(int32, repeated, tag = "2")]
    pub backoff_ms: ::prost::alloc::vec::Vec<i32>,
}
/// DEPRECATED, don't use. To be removed shortly.
/// The type of payload that should be returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayloadType {
    /// Compressable text format.
    Compressable = 0,
}
impl PayloadType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PayloadType::Compressable => "COMPRESSABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPRESSABLE" => Some(Self::Compressable),
            _ => None,
        }
    }
}
const METHOD_BENCHMARK_SERVICE_UNARY_CALL: ::grpcio::Method<SimpleRequest, SimpleResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/grpc.testing.BenchmarkService/UnaryCall",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_BENCHMARK_SERVICE_STREAMING_CALL: ::grpcio::Method<SimpleRequest, SimpleResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Duplex,
        name: "/grpc.testing.BenchmarkService/StreamingCall",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_BENCHMARK_SERVICE_STREAMING_FROM_CLIENT: ::grpcio::Method<
    SimpleRequest,
    SimpleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/grpc.testing.BenchmarkService/StreamingFromClient",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_BENCHMARK_SERVICE_STREAMING_FROM_SERVER: ::grpcio::Method<
    SimpleRequest,
    SimpleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/grpc.testing.BenchmarkService/StreamingFromServer",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_BENCHMARK_SERVICE_STREAMING_BOTH_WAYS: ::grpcio::Method<
    SimpleRequest,
    SimpleResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.BenchmarkService/StreamingBothWays",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
#[derive(Clone)]
pub struct BenchmarkServiceClient {
    pub client: ::grpcio::Client,
}
impl BenchmarkServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        BenchmarkServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn unary_call_opt(
        &self,
        req: &SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<SimpleResponse> {
        self.client
            .unary_call(&METHOD_BENCHMARK_SERVICE_UNARY_CALL, req, opt)
    }
    pub fn unary_call(&self, req: &SimpleRequest) -> ::grpcio::Result<SimpleResponse> {
        self.unary_call_opt(req, ::grpcio::CallOption::default())
    }
    pub fn unary_call_async_opt(
        &self,
        req: &SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SimpleResponse>> {
        self.client
            .unary_call_async(&METHOD_BENCHMARK_SERVICE_UNARY_CALL, req, opt)
    }
    pub fn unary_call_async(
        &self,
        req: &SimpleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SimpleResponse>> {
        self.unary_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn streaming_call_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<SimpleRequest>,
        ::grpcio::ClientDuplexReceiver<SimpleResponse>,
    )> {
        self.client
            .duplex_streaming(&METHOD_BENCHMARK_SERVICE_STREAMING_CALL, opt)
    }
    pub fn streaming_call(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<SimpleRequest>,
        ::grpcio::ClientDuplexReceiver<SimpleResponse>,
    )> {
        self.streaming_call_opt(::grpcio::CallOption::default())
    }
    pub fn streaming_from_client_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<SimpleRequest>,
        ::grpcio::ClientCStreamReceiver<SimpleResponse>,
    )> {
        self.client
            .client_streaming(&METHOD_BENCHMARK_SERVICE_STREAMING_FROM_CLIENT, opt)
    }
    pub fn streaming_from_client(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<SimpleRequest>,
        ::grpcio::ClientCStreamReceiver<SimpleResponse>,
    )> {
        self.streaming_from_client_opt(::grpcio::CallOption::default())
    }
    pub fn streaming_from_server_opt(
        &self,
        req: &SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<SimpleResponse>> {
        self.client
            .server_streaming(&METHOD_BENCHMARK_SERVICE_STREAMING_FROM_SERVER, req, opt)
    }
    pub fn streaming_from_server(
        &self,
        req: &SimpleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<SimpleResponse>> {
        self.streaming_from_server_opt(req, ::grpcio::CallOption::default())
    }
    pub fn streaming_both_ways_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<SimpleRequest>,
        ::grpcio::ClientDuplexReceiver<SimpleResponse>,
    )> {
        self.client
            .duplex_streaming(&METHOD_BENCHMARK_SERVICE_STREAMING_BOTH_WAYS, opt)
    }
    pub fn streaming_both_ways(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<SimpleRequest>,
        ::grpcio::ClientDuplexReceiver<SimpleResponse>,
    )> {
        self.streaming_both_ways_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait BenchmarkService {
    fn unary_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: SimpleRequest,
        sink: ::grpcio::UnarySink<SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<SimpleRequest>,
        sink: ::grpcio::DuplexSink<SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_from_client(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<SimpleRequest>,
        sink: ::grpcio::ClientStreamingSink<SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_from_server(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: SimpleRequest,
        sink: ::grpcio::ServerStreamingSink<SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_both_ways(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<SimpleRequest>,
        sink: ::grpcio::DuplexSink<SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_benchmark_service<S: BenchmarkService + Send + Clone + 'static>(
    s: S,
) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_BENCHMARK_SERVICE_UNARY_CALL,
        move |ctx, req, resp| instance.unary_call(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(
        &METHOD_BENCHMARK_SERVICE_STREAMING_CALL,
        move |ctx, req, resp| instance.streaming_call(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(
        &METHOD_BENCHMARK_SERVICE_STREAMING_FROM_CLIENT,
        move |ctx, req, resp| instance.streaming_from_client(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(
        &METHOD_BENCHMARK_SERVICE_STREAMING_FROM_SERVER,
        move |ctx, req, resp| instance.streaming_from_server(ctx, req, resp),
    );
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(
        &METHOD_BENCHMARK_SERVICE_STREAMING_BOTH_WAYS,
        move |ctx, req, resp| instance.streaming_both_ways(ctx, req, resp),
    );
    builder.build()
}
const METHOD_WORKER_SERVICE_RUN_SERVER: ::grpcio::Method<ServerArgs, ServerStatus> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Duplex,
        name: "/grpc.testing.WorkerService/RunServer",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_WORKER_SERVICE_RUN_CLIENT: ::grpcio::Method<ClientArgs, ClientStatus> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Duplex,
        name: "/grpc.testing.WorkerService/RunClient",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_WORKER_SERVICE_CORE_COUNT: ::grpcio::Method<CoreRequest, CoreResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/grpc.testing.WorkerService/CoreCount",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_WORKER_SERVICE_QUIT_WORKER: ::grpcio::Method<Void, Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.WorkerService/QuitWorker",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
#[derive(Clone)]
pub struct WorkerServiceClient {
    pub client: ::grpcio::Client,
}
impl WorkerServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        WorkerServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn run_server_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<ServerArgs>,
        ::grpcio::ClientDuplexReceiver<ServerStatus>,
    )> {
        self.client
            .duplex_streaming(&METHOD_WORKER_SERVICE_RUN_SERVER, opt)
    }
    pub fn run_server(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<ServerArgs>,
        ::grpcio::ClientDuplexReceiver<ServerStatus>,
    )> {
        self.run_server_opt(::grpcio::CallOption::default())
    }
    pub fn run_client_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<ClientArgs>,
        ::grpcio::ClientDuplexReceiver<ClientStatus>,
    )> {
        self.client
            .duplex_streaming(&METHOD_WORKER_SERVICE_RUN_CLIENT, opt)
    }
    pub fn run_client(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<ClientArgs>,
        ::grpcio::ClientDuplexReceiver<ClientStatus>,
    )> {
        self.run_client_opt(::grpcio::CallOption::default())
    }
    pub fn core_count_opt(
        &self,
        req: &CoreRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<CoreResponse> {
        self.client
            .unary_call(&METHOD_WORKER_SERVICE_CORE_COUNT, req, opt)
    }
    pub fn core_count(&self, req: &CoreRequest) -> ::grpcio::Result<CoreResponse> {
        self.core_count_opt(req, ::grpcio::CallOption::default())
    }
    pub fn core_count_async_opt(
        &self,
        req: &CoreRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CoreResponse>> {
        self.client
            .unary_call_async(&METHOD_WORKER_SERVICE_CORE_COUNT, req, opt)
    }
    pub fn core_count_async(
        &self,
        req: &CoreRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CoreResponse>> {
        self.core_count_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn quit_worker_opt(&self, req: &Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<Void> {
        self.client
            .unary_call(&METHOD_WORKER_SERVICE_QUIT_WORKER, req, opt)
    }
    pub fn quit_worker(&self, req: &Void) -> ::grpcio::Result<Void> {
        self.quit_worker_opt(req, ::grpcio::CallOption::default())
    }
    pub fn quit_worker_async_opt(
        &self,
        req: &Void,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Void>> {
        self.client
            .unary_call_async(&METHOD_WORKER_SERVICE_QUIT_WORKER, req, opt)
    }
    pub fn quit_worker_async(
        &self,
        req: &Void,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Void>> {
        self.quit_worker_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait WorkerService {
    fn run_server(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<ServerArgs>,
        sink: ::grpcio::DuplexSink<ServerStatus>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn run_client(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<ClientArgs>,
        sink: ::grpcio::DuplexSink<ClientStatus>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn core_count(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: CoreRequest,
        sink: ::grpcio::UnarySink<CoreResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn quit_worker(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: Void,
        sink: ::grpcio::UnarySink<Void>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_worker_service<S: WorkerService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder
        .add_duplex_streaming_handler(&METHOD_WORKER_SERVICE_RUN_SERVER, move |ctx, req, resp| {
            instance.run_server(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder
        .add_duplex_streaming_handler(&METHOD_WORKER_SERVICE_RUN_CLIENT, move |ctx, req, resp| {
            instance.run_client(ctx, req, resp)
        });
    let mut instance = s.clone();
    builder = builder
        .add_unary_handler(&METHOD_WORKER_SERVICE_CORE_COUNT, move |ctx, req, resp| {
            instance.core_count(ctx, req, resp)
        });
    let mut instance = s;
    builder = builder
        .add_unary_handler(&METHOD_WORKER_SERVICE_QUIT_WORKER, move |ctx, req, resp| {
            instance.quit_worker(ctx, req, resp)
        });
    builder.build()
}
const METHOD_REPORT_QPS_SCENARIO_SERVICE_REPORT_SCENARIO: ::grpcio::Method<ScenarioResult, Void> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/grpc.testing.ReportQpsScenarioService/ReportScenario",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
#[derive(Clone)]
pub struct ReportQpsScenarioServiceClient {
    pub client: ::grpcio::Client,
}
impl ReportQpsScenarioServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ReportQpsScenarioServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn report_scenario_opt(
        &self,
        req: &ScenarioResult,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<Void> {
        self.client.unary_call(
            &METHOD_REPORT_QPS_SCENARIO_SERVICE_REPORT_SCENARIO,
            req,
            opt,
        )
    }
    pub fn report_scenario(&self, req: &ScenarioResult) -> ::grpcio::Result<Void> {
        self.report_scenario_opt(req, ::grpcio::CallOption::default())
    }
    pub fn report_scenario_async_opt(
        &self,
        req: &ScenarioResult,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Void>> {
        self.client.unary_call_async(
            &METHOD_REPORT_QPS_SCENARIO_SERVICE_REPORT_SCENARIO,
            req,
            opt,
        )
    }
    pub fn report_scenario_async(
        &self,
        req: &ScenarioResult,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Void>> {
        self.report_scenario_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait ReportQpsScenarioService {
    fn report_scenario(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: ScenarioResult,
        sink: ::grpcio::UnarySink<Void>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_report_qps_scenario_service<S: ReportQpsScenarioService + Send + Clone + 'static>(
    s: S,
) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(
        &METHOD_REPORT_QPS_SCENARIO_SERVICE_REPORT_SCENARIO,
        move |ctx, req, resp| instance.report_scenario(ctx, req, resp),
    );
    builder.build()
}
const METHOD_TEST_SERVICE_EMPTY_CALL: ::grpcio::Method<Empty, Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.TestService/EmptyCall",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_TEST_SERVICE_UNARY_CALL: ::grpcio::Method<SimpleRequest, SimpleResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/grpc.testing.TestService/UnaryCall",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL: ::grpcio::Method<SimpleRequest, SimpleResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/grpc.testing.TestService/CacheableUnaryCall",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
const METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL: ::grpcio::Method<
    StreamingOutputCallRequest,
    StreamingOutputCallResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/grpc.testing.TestService/StreamingOutputCall",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_TEST_SERVICE_STREAMING_INPUT_CALL: ::grpcio::Method<
    StreamingInputCallRequest,
    StreamingInputCallResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/grpc.testing.TestService/StreamingInputCall",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_TEST_SERVICE_FULL_DUPLEX_CALL: ::grpcio::Method<
    StreamingOutputCallRequest,
    StreamingOutputCallResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.TestService/FullDuplexCall",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_TEST_SERVICE_HALF_DUPLEX_CALL: ::grpcio::Method<
    StreamingOutputCallRequest,
    StreamingOutputCallResponse,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/grpc.testing.TestService/HalfDuplexCall",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL: ::grpcio::Method<Empty, Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.TestService/UnimplementedCall",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
#[derive(Clone)]
pub struct TestServiceClient {
    pub client: ::grpcio::Client,
}
impl TestServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TestServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn empty_call_opt(
        &self,
        req: &Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<Empty> {
        self.client
            .unary_call(&METHOD_TEST_SERVICE_EMPTY_CALL, req, opt)
    }
    pub fn empty_call(&self, req: &Empty) -> ::grpcio::Result<Empty> {
        self.empty_call_opt(req, ::grpcio::CallOption::default())
    }
    pub fn empty_call_async_opt(
        &self,
        req: &Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Empty>> {
        self.client
            .unary_call_async(&METHOD_TEST_SERVICE_EMPTY_CALL, req, opt)
    }
    pub fn empty_call_async(
        &self,
        req: &Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Empty>> {
        self.empty_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn unary_call_opt(
        &self,
        req: &SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<SimpleResponse> {
        self.client
            .unary_call(&METHOD_TEST_SERVICE_UNARY_CALL, req, opt)
    }
    pub fn unary_call(&self, req: &SimpleRequest) -> ::grpcio::Result<SimpleResponse> {
        self.unary_call_opt(req, ::grpcio::CallOption::default())
    }
    pub fn unary_call_async_opt(
        &self,
        req: &SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SimpleResponse>> {
        self.client
            .unary_call_async(&METHOD_TEST_SERVICE_UNARY_CALL, req, opt)
    }
    pub fn unary_call_async(
        &self,
        req: &SimpleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SimpleResponse>> {
        self.unary_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn cacheable_unary_call_opt(
        &self,
        req: &SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<SimpleResponse> {
        self.client
            .unary_call(&METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL, req, opt)
    }
    pub fn cacheable_unary_call(&self, req: &SimpleRequest) -> ::grpcio::Result<SimpleResponse> {
        self.cacheable_unary_call_opt(req, ::grpcio::CallOption::default())
    }
    pub fn cacheable_unary_call_async_opt(
        &self,
        req: &SimpleRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SimpleResponse>> {
        self.client
            .unary_call_async(&METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL, req, opt)
    }
    pub fn cacheable_unary_call_async(
        &self,
        req: &SimpleRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SimpleResponse>> {
        self.cacheable_unary_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn streaming_output_call_opt(
        &self,
        req: &StreamingOutputCallRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<StreamingOutputCallResponse>> {
        self.client
            .server_streaming(&METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL, req, opt)
    }
    pub fn streaming_output_call(
        &self,
        req: &StreamingOutputCallRequest,
    ) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<StreamingOutputCallResponse>> {
        self.streaming_output_call_opt(req, ::grpcio::CallOption::default())
    }
    pub fn streaming_input_call_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<StreamingInputCallRequest>,
        ::grpcio::ClientCStreamReceiver<StreamingInputCallResponse>,
    )> {
        self.client
            .client_streaming(&METHOD_TEST_SERVICE_STREAMING_INPUT_CALL, opt)
    }
    pub fn streaming_input_call(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientCStreamSender<StreamingInputCallRequest>,
        ::grpcio::ClientCStreamReceiver<StreamingInputCallResponse>,
    )> {
        self.streaming_input_call_opt(::grpcio::CallOption::default())
    }
    pub fn full_duplex_call_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<StreamingOutputCallRequest>,
        ::grpcio::ClientDuplexReceiver<StreamingOutputCallResponse>,
    )> {
        self.client
            .duplex_streaming(&METHOD_TEST_SERVICE_FULL_DUPLEX_CALL, opt)
    }
    pub fn full_duplex_call(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<StreamingOutputCallRequest>,
        ::grpcio::ClientDuplexReceiver<StreamingOutputCallResponse>,
    )> {
        self.full_duplex_call_opt(::grpcio::CallOption::default())
    }
    pub fn half_duplex_call_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<StreamingOutputCallRequest>,
        ::grpcio::ClientDuplexReceiver<StreamingOutputCallResponse>,
    )> {
        self.client
            .duplex_streaming(&METHOD_TEST_SERVICE_HALF_DUPLEX_CALL, opt)
    }
    pub fn half_duplex_call(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<StreamingOutputCallRequest>,
        ::grpcio::ClientDuplexReceiver<StreamingOutputCallResponse>,
    )> {
        self.half_duplex_call_opt(::grpcio::CallOption::default())
    }
    pub fn unimplemented_call_opt(
        &self,
        req: &Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<Empty> {
        self.client
            .unary_call(&METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }
    pub fn unimplemented_call(&self, req: &Empty) -> ::grpcio::Result<Empty> {
        self.unimplemented_call_opt(req, ::grpcio::CallOption::default())
    }
    pub fn unimplemented_call_async_opt(
        &self,
        req: &Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Empty>> {
        self.client
            .unary_call_async(&METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }
    pub fn unimplemented_call_async(
        &self,
        req: &Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Empty>> {
        self.unimplemented_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait TestService {
    fn empty_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: Empty,
        sink: ::grpcio::UnarySink<Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn unary_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: SimpleRequest,
        sink: ::grpcio::UnarySink<SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cacheable_unary_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: SimpleRequest,
        sink: ::grpcio::UnarySink<SimpleResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_output_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: StreamingOutputCallRequest,
        sink: ::grpcio::ServerStreamingSink<StreamingOutputCallResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_input_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<StreamingInputCallRequest>,
        sink: ::grpcio::ClientStreamingSink<StreamingInputCallResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn full_duplex_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<StreamingOutputCallRequest>,
        sink: ::grpcio::DuplexSink<StreamingOutputCallResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn half_duplex_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _stream: ::grpcio::RequestStream<StreamingOutputCallRequest>,
        sink: ::grpcio::DuplexSink<StreamingOutputCallResponse>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn unimplemented_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: Empty,
        sink: ::grpcio::UnarySink<Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_test_service<S: TestService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_EMPTY_CALL, move |ctx, req, resp| {
        instance.empty_call(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TEST_SERVICE_UNARY_CALL, move |ctx, req, resp| {
        instance.unary_call(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(
        &METHOD_TEST_SERVICE_CACHEABLE_UNARY_CALL,
        move |ctx, req, resp| instance.cacheable_unary_call(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(
        &METHOD_TEST_SERVICE_STREAMING_OUTPUT_CALL,
        move |ctx, req, resp| instance.streaming_output_call(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(
        &METHOD_TEST_SERVICE_STREAMING_INPUT_CALL,
        move |ctx, req, resp| instance.streaming_input_call(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(
        &METHOD_TEST_SERVICE_FULL_DUPLEX_CALL,
        move |ctx, req, resp| instance.full_duplex_call(ctx, req, resp),
    );
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(
        &METHOD_TEST_SERVICE_HALF_DUPLEX_CALL,
        move |ctx, req, resp| instance.half_duplex_call(ctx, req, resp),
    );
    let mut instance = s;
    builder = builder.add_unary_handler(
        &METHOD_TEST_SERVICE_UNIMPLEMENTED_CALL,
        move |ctx, req, resp| instance.unimplemented_call(ctx, req, resp),
    );
    builder.build()
}
const METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL: ::grpcio::Method<Empty, Empty> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/grpc.testing.UnimplementedService/UnimplementedCall",
        req_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
        resp_mar: ::grpcio::Marshaller {
            ser: ::grpcio::pr_ser,
            de: ::grpcio::pr_de,
        },
    };
#[derive(Clone)]
pub struct UnimplementedServiceClient {
    pub client: ::grpcio::Client,
}
impl UnimplementedServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        UnimplementedServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn unimplemented_call_opt(
        &self,
        req: &Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<Empty> {
        self.client
            .unary_call(&METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }
    pub fn unimplemented_call(&self, req: &Empty) -> ::grpcio::Result<Empty> {
        self.unimplemented_call_opt(req, ::grpcio::CallOption::default())
    }
    pub fn unimplemented_call_async_opt(
        &self,
        req: &Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Empty>> {
        self.client
            .unary_call_async(&METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL, req, opt)
    }
    pub fn unimplemented_call_async(
        &self,
        req: &Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Empty>> {
        self.unimplemented_call_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait UnimplementedService {
    fn unimplemented_call(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: Empty,
        sink: ::grpcio::UnarySink<Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_unimplemented_service<S: UnimplementedService + Send + Clone + 'static>(
    s: S,
) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(
        &METHOD_UNIMPLEMENTED_SERVICE_UNIMPLEMENTED_CALL,
        move |ctx, req, resp| instance.unimplemented_call(ctx, req, resp),
    );
    builder.build()
}
const METHOD_RECONNECT_SERVICE_START: ::grpcio::Method<ReconnectParams, Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.ReconnectService/Start",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_RECONNECT_SERVICE_STOP: ::grpcio::Method<Empty, ReconnectInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/grpc.testing.ReconnectService/Stop",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
#[derive(Clone)]
pub struct ReconnectServiceClient {
    pub client: ::grpcio::Client,
}
impl ReconnectServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ReconnectServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn start_opt(
        &self,
        req: &ReconnectParams,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<Empty> {
        self.client
            .unary_call(&METHOD_RECONNECT_SERVICE_START, req, opt)
    }
    pub fn start(&self, req: &ReconnectParams) -> ::grpcio::Result<Empty> {
        self.start_opt(req, ::grpcio::CallOption::default())
    }
    pub fn start_async_opt(
        &self,
        req: &ReconnectParams,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Empty>> {
        self.client
            .unary_call_async(&METHOD_RECONNECT_SERVICE_START, req, opt)
    }
    pub fn start_async(
        &self,
        req: &ReconnectParams,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Empty>> {
        self.start_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn stop_opt(
        &self,
        req: &Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<ReconnectInfo> {
        self.client
            .unary_call(&METHOD_RECONNECT_SERVICE_STOP, req, opt)
    }
    pub fn stop(&self, req: &Empty) -> ::grpcio::Result<ReconnectInfo> {
        self.stop_opt(req, ::grpcio::CallOption::default())
    }
    pub fn stop_async_opt(
        &self,
        req: &Empty,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ReconnectInfo>> {
        self.client
            .unary_call_async(&METHOD_RECONNECT_SERVICE_STOP, req, opt)
    }
    pub fn stop_async(
        &self,
        req: &Empty,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ReconnectInfo>> {
        self.stop_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait ReconnectService {
    fn start(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: ReconnectParams,
        sink: ::grpcio::UnarySink<Empty>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn stop(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: Empty,
        sink: ::grpcio::UnarySink<ReconnectInfo>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_reconnect_service<S: ReconnectService + Send + Clone + 'static>(
    s: S,
) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RECONNECT_SERVICE_START, move |ctx, req, resp| {
        instance.start(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_RECONNECT_SERVICE_STOP, move |ctx, req, resp| {
        instance.stop(ctx, req, resp)
    });
    builder.build()
}
