pub const GRPC_ARES: u32 = 1;
pub const GRPC_IF_NAMETOINDEX: u32 = 1;
pub const GRPC_ALLOW_EXCEPTIONS: u32 = 1;
pub const GRPC_COMPRESSION_REQUEST_ALGORITHM_MD_KEY: &[u8; 31usize] =
    b"grpc-internal-encoding-request\0";
pub const GRPC_COMPRESSION_CHANNEL_DEFAULT_ALGORITHM: &[u8; 35usize] =
    b"grpc.default_compression_algorithm\0";
pub const GRPC_COMPRESSION_CHANNEL_DEFAULT_LEVEL: &[u8; 31usize] =
    b"grpc.default_compression_level\0";
pub const GRPC_COMPRESSION_CHANNEL_ENABLED_ALGORITHMS_BITSET: &[u8; 43usize] =
    b"grpc.compression_enabled_algorithms_bitset\0";
pub const GRPC_SLICE_BUFFER_INLINE_ELEMENTS: u32 = 8;
pub const GRPC_ARG_ENABLE_CENSUS: &[u8; 12usize] = b"grpc.census\0";
pub const GRPC_ARG_ENABLE_LOAD_REPORTING: &[u8; 19usize] = b"grpc.loadreporting\0";
pub const GRPC_ARG_SERVER_CALL_METRIC_RECORDING: &[u8; 34usize] =
    b"grpc.server_call_metric_recording\0";
pub const GRPC_ARG_MINIMAL_STACK: &[u8; 19usize] = b"grpc.minimal_stack\0";
pub const GRPC_ARG_MAX_CONCURRENT_STREAMS: &[u8; 28usize] = b"grpc.max_concurrent_streams\0";
pub const GRPC_ARG_MAX_RECEIVE_MESSAGE_LENGTH: &[u8; 32usize] =
    b"grpc.max_receive_message_length\0";
pub const GRPC_ARG_MAX_MESSAGE_LENGTH: &[u8; 32usize] = b"grpc.max_receive_message_length\0";
pub const GRPC_ARG_MAX_SEND_MESSAGE_LENGTH: &[u8; 29usize] = b"grpc.max_send_message_length\0";
pub const GRPC_ARG_MAX_CONNECTION_IDLE_MS: &[u8; 28usize] = b"grpc.max_connection_idle_ms\0";
pub const GRPC_ARG_MAX_CONNECTION_AGE_MS: &[u8; 27usize] = b"grpc.max_connection_age_ms\0";
pub const GRPC_ARG_MAX_CONNECTION_AGE_GRACE_MS: &[u8; 33usize] =
    b"grpc.max_connection_age_grace_ms\0";
pub const GRPC_ARG_CLIENT_IDLE_TIMEOUT_MS: &[u8; 28usize] = b"grpc.client_idle_timeout_ms\0";
pub const GRPC_ARG_ENABLE_PER_MESSAGE_COMPRESSION: &[u8; 29usize] =
    b"grpc.per_message_compression\0";
pub const GRPC_ARG_ENABLE_PER_MESSAGE_DECOMPRESSION: &[u8; 31usize] =
    b"grpc.per_message_decompression\0";
pub const GRPC_ARG_ENABLE_DEADLINE_CHECKS: &[u8; 30usize] = b"grpc.enable_deadline_checking\0";
pub const GRPC_ARG_HTTP2_INITIAL_SEQUENCE_NUMBER: &[u8; 35usize] =
    b"grpc.http2.initial_sequence_number\0";
pub const GRPC_ARG_HTTP2_STREAM_LOOKAHEAD_BYTES: &[u8; 27usize] = b"grpc.http2.lookahead_bytes\0";
pub const GRPC_ARG_HTTP2_HPACK_TABLE_SIZE_DECODER: &[u8; 36usize] =
    b"grpc.http2.hpack_table_size.decoder\0";
pub const GRPC_ARG_HTTP2_HPACK_TABLE_SIZE_ENCODER: &[u8; 36usize] =
    b"grpc.http2.hpack_table_size.encoder\0";
pub const GRPC_ARG_HTTP2_MAX_FRAME_SIZE: &[u8; 26usize] = b"grpc.http2.max_frame_size\0";
pub const GRPC_ARG_HTTP2_BDP_PROBE: &[u8; 21usize] = b"grpc.http2.bdp_probe\0";
pub const GRPC_ARG_HTTP2_MIN_SENT_PING_INTERVAL_WITHOUT_DATA_MS: &[u8; 37usize] =
    b"grpc.http2.min_time_between_pings_ms\0";
pub const GRPC_ARG_HTTP2_MIN_RECV_PING_INTERVAL_WITHOUT_DATA_MS: &[u8; 45usize] =
    b"grpc.http2.min_ping_interval_without_data_ms\0";
pub const GRPC_ARG_HTTP2_SCHEME: &[u8; 18usize] = b"grpc.http2_scheme\0";
pub const GRPC_ARG_HTTP2_MAX_PINGS_WITHOUT_DATA: &[u8; 34usize] =
    b"grpc.http2.max_pings_without_data\0";
pub const GRPC_ARG_HTTP2_MAX_PING_STRIKES: &[u8; 28usize] = b"grpc.http2.max_ping_strikes\0";
pub const GRPC_ARG_HTTP2_WRITE_BUFFER_SIZE: &[u8; 29usize] = b"grpc.http2.write_buffer_size\0";
pub const GRPC_ARG_HTTP2_ENABLE_TRUE_BINARY: &[u8; 23usize] = b"grpc.http2.true_binary\0";
pub const GRPC_ARG_EXPERIMENTAL_HTTP2_PREFERRED_CRYPTO_FRAME_SIZE: &[u8; 52usize] =
    b"grpc.experimental.http2.enable_preferred_frame_size\0";
pub const GRPC_ARG_KEEPALIVE_TIME_MS: &[u8; 23usize] = b"grpc.keepalive_time_ms\0";
pub const GRPC_ARG_KEEPALIVE_TIMEOUT_MS: &[u8; 26usize] = b"grpc.keepalive_timeout_ms\0";
pub const GRPC_ARG_KEEPALIVE_PERMIT_WITHOUT_CALLS: &[u8; 36usize] =
    b"grpc.keepalive_permit_without_calls\0";
pub const GRPC_ARG_DEFAULT_AUTHORITY: &[u8; 23usize] = b"grpc.default_authority\0";
pub const GRPC_ARG_PRIMARY_USER_AGENT_STRING: &[u8; 24usize] = b"grpc.primary_user_agent\0";
pub const GRPC_ARG_SECONDARY_USER_AGENT_STRING: &[u8; 26usize] = b"grpc.secondary_user_agent\0";
pub const GRPC_ARG_MIN_RECONNECT_BACKOFF_MS: &[u8; 30usize] = b"grpc.min_reconnect_backoff_ms\0";
pub const GRPC_ARG_MAX_RECONNECT_BACKOFF_MS: &[u8; 30usize] = b"grpc.max_reconnect_backoff_ms\0";
pub const GRPC_ARG_INITIAL_RECONNECT_BACKOFF_MS: &[u8; 34usize] =
    b"grpc.initial_reconnect_backoff_ms\0";
pub const GRPC_ARG_DNS_MIN_TIME_BETWEEN_RESOLUTIONS_MS: &[u8; 41usize] =
    b"grpc.dns_min_time_between_resolutions_ms\0";
pub const GRPC_ARG_SERVER_HANDSHAKE_TIMEOUT_MS: &[u8; 33usize] =
    b"grpc.server_handshake_timeout_ms\0";
pub const GRPC_SSL_TARGET_NAME_OVERRIDE_ARG: &[u8; 30usize] = b"grpc.ssl_target_name_override\0";
pub const GRPC_SSL_SESSION_CACHE_ARG: &[u8; 23usize] = b"grpc.ssl_session_cache\0";
pub const GRPC_ARG_TSI_MAX_FRAME_SIZE: &[u8; 24usize] = b"grpc.tsi.max_frame_size\0";
pub const GRPC_ARG_MAX_METADATA_SIZE: &[u8; 23usize] = b"grpc.max_metadata_size\0";
pub const GRPC_ARG_ABSOLUTE_MAX_METADATA_SIZE: &[u8; 32usize] =
    b"grpc.absolute_max_metadata_size\0";
pub const GRPC_ARG_ALLOW_REUSEPORT: &[u8; 18usize] = b"grpc.so_reuseport\0";
pub const GRPC_ARG_RESOURCE_QUOTA: &[u8; 20usize] = b"grpc.resource_quota\0";
pub const GRPC_ARG_EXPAND_WILDCARD_ADDRS: &[u8; 27usize] = b"grpc.expand_wildcard_addrs\0";
pub const GRPC_ARG_SERVICE_CONFIG: &[u8; 20usize] = b"grpc.service_config\0";
pub const GRPC_ARG_SERVICE_CONFIG_DISABLE_RESOLUTION: &[u8; 39usize] =
    b"grpc.service_config_disable_resolution\0";
pub const GRPC_ARG_LB_POLICY_NAME: &[u8; 20usize] = b"grpc.lb_policy_name\0";
pub const GRPC_ARG_RING_HASH_LB_RING_SIZE_CAP: &[u8; 32usize] =
    b"grpc.lb.ring_hash.ring_size_cap\0";
pub const GRPC_ARG_SOCKET_MUTATOR: &[u8; 20usize] = b"grpc.socket_mutator\0";
pub const GRPC_ARG_SOCKET_FACTORY: &[u8; 20usize] = b"grpc.socket_factory\0";
pub const GRPC_ARG_GZIP_COMPRESSION_LEVEL: &[u8; 28usize] = b"grpc.gzip_compression_level\0";
pub const GRPC_ARG_MIN_MESSAGE_SIZE_TO_COMPRESS: &[u8; 39usize] =
    b"grpc.gprc_min_message_size_to_compress\0";
pub const GRPC_ARG_MAX_CHANNEL_TRACE_EVENT_MEMORY_PER_NODE: &[u8; 45usize] =
    b"grpc.max_channel_trace_event_memory_per_node\0";
pub const GRPC_ARG_ENABLE_CHANNELZ: &[u8; 21usize] = b"grpc.enable_channelz\0";
pub const GRPC_ARG_USE_CRONET_PACKET_COALESCING: &[u8; 34usize] =
    b"grpc.use_cronet_packet_coalescing\0";
pub const GRPC_ARG_TCP_READ_CHUNK_SIZE: &[u8; 38usize] = b"grpc.experimental.tcp_read_chunk_size\0";
pub const GRPC_TCP_DEFAULT_READ_SLICE_SIZE: u32 = 8192;
pub const GRPC_ARG_TCP_MIN_READ_CHUNK_SIZE: &[u8; 42usize] =
    b"grpc.experimental.tcp_min_read_chunk_size\0";
pub const GRPC_ARG_TCP_MAX_READ_CHUNK_SIZE: &[u8; 42usize] =
    b"grpc.experimental.tcp_max_read_chunk_size\0";
pub const GRPC_ARG_TCP_TX_ZEROCOPY_ENABLED: &[u8; 42usize] =
    b"grpc.experimental.tcp_tx_zerocopy_enabled\0";
pub const GRPC_ARG_TCP_TX_ZEROCOPY_SEND_BYTES_THRESHOLD: &[u8; 55usize] =
    b"grpc.experimental.tcp_tx_zerocopy_send_bytes_threshold\0";
pub const GRPC_ARG_TCP_TX_ZEROCOPY_MAX_SIMULT_SENDS: &[u8; 57usize] =
    b"grpc.experimental.tcp_tx_zerocopy_max_simultaneous_sends\0";
pub const GRPC_ARG_TCP_RECEIVE_BUFFER_SIZE: &[u8; 29usize] = b"grpc.tcp_receive_buffer_size\0";
pub const GRPC_ARG_GRPCLB_CALL_TIMEOUT_MS: &[u8; 28usize] = b"grpc.grpclb_call_timeout_ms\0";
pub const GRPC_ARG_TEST_ONLY_DO_NOT_USE_IN_PROD_XDS_BOOTSTRAP_CONFIG: &[u8; 55usize] =
    b"grpc.TEST_ONLY_DO_NOT_USE_IN_PROD.xds_bootstrap_config\0";
pub const GRPC_ARG_GRPCLB_FALLBACK_TIMEOUT_MS: &[u8; 32usize] =
    b"grpc.grpclb_fallback_timeout_ms\0";
pub const GRPC_ARG_EXPERIMENTAL_GRPCLB_CHANNEL_ARGS: &[u8; 38usize] =
    b"grpc.experimental.grpclb_channel_args\0";
pub const GRPC_ARG_PRIORITY_FAILOVER_TIMEOUT_MS: &[u8; 34usize] =
    b"grpc.priority_failover_timeout_ms\0";
pub const GRPC_ARG_WORKAROUND_CRONET_COMPRESSION: &[u8; 35usize] =
    b"grpc.workaround.cronet_compression\0";
pub const GRPC_ARG_OPTIMIZATION_TARGET: &[u8; 25usize] = b"grpc.optimization_target\0";
pub const GRPC_ARG_ENABLE_RETRIES: &[u8; 20usize] = b"grpc.enable_retries\0";
pub const GRPC_ARG_EXPERIMENTAL_ENABLE_HEDGING: &[u8; 33usize] =
    b"grpc.experimental.enable_hedging\0";
pub const GRPC_ARG_PER_RPC_RETRY_BUFFER_SIZE: &[u8; 31usize] = b"grpc.per_rpc_retry_buffer_size\0";
pub const GRPC_ARG_MOBILE_LOG_CONTEXT: &[u8; 24usize] = b"grpc.mobile_log_context\0";
pub const GRPC_ARG_DISABLE_CLIENT_AUTHORITY_FILTER: &[u8; 37usize] =
    b"grpc.disable_client_authority_filter\0";
pub const GRPC_ARG_ENABLE_HTTP_PROXY: &[u8; 23usize] = b"grpc.enable_http_proxy\0";
pub const GRPC_ARG_HTTP_PROXY: &[u8; 16usize] = b"grpc.http_proxy\0";
pub const GRPC_ARG_SURFACE_USER_AGENT: &[u8; 24usize] = b"grpc.surface_user_agent\0";
pub const GRPC_ARG_INHIBIT_HEALTH_CHECKING: &[u8; 29usize] = b"grpc.inhibit_health_checking\0";
pub const GRPC_ARG_DNS_ENABLE_SRV_QUERIES: &[u8; 28usize] = b"grpc.dns_enable_srv_queries\0";
pub const GRPC_ARG_DNS_ARES_QUERY_TIMEOUT_MS: &[u8; 28usize] = b"grpc.dns_ares_query_timeout\0";
pub const GRPC_ARG_USE_LOCAL_SUBCHANNEL_POOL: &[u8; 31usize] = b"grpc.use_local_subchannel_pool\0";
pub const GRPC_ARG_CHANNEL_POOL_DOMAIN: &[u8; 28usize] = b"grpc.channel_pooling_domain\0";
pub const GRPC_ARG_CHANNEL_ID: &[u8; 16usize] = b"grpc.channel_id\0";
pub const GRPC_ARG_AUTHORIZATION_POLICY_PROVIDER: &[u8; 35usize] =
    b"grpc.authorization_policy_provider\0";
pub const GRPC_ARG_SERVER_CONFIG_CHANGE_DRAIN_GRACE_TIME_MS: &[u8; 59usize] =
    b"grpc.experimental.server_config_change_drain_grace_time_ms\0";
pub const GRPC_DEFAULT_MAX_SEND_MESSAGE_LENGTH: i32 = -1;
pub const GRPC_DEFAULT_MAX_RECV_MESSAGE_LENGTH: u32 = 4194304;
pub const GRPC_WRITE_BUFFER_HINT: u32 = 1;
pub const GRPC_WRITE_NO_COMPRESS: u32 = 2;
pub const GRPC_WRITE_THROUGH: u32 = 4;
pub const GRPC_WRITE_USED_MASK: u32 = 7;
pub const GRPC_INITIAL_METADATA_WAIT_FOR_READY: u32 = 32;
pub const GRPC_INITIAL_METADATA_WAIT_FOR_READY_EXPLICITLY_SET: u32 = 128;
pub const GRPC_INITIAL_METADATA_USED_MASK: u32 = 164;
pub const GRPC_CQ_CURRENT_VERSION: u32 = 2;
pub const GRPC_CQ_VERSION_MINIMUM_FOR_CALLBACKABLE: u32 = 2;
pub const GRPC_MAX_COMPLETION_QUEUE_PLUCKERS: u32 = 6;
pub const GRPC_TRANSPORT_SECURITY_TYPE_PROPERTY_NAME: &[u8; 24usize] = b"transport_security_type\0";
pub const GRPC_SSL_TRANSPORT_SECURITY_TYPE: &[u8; 4usize] = b"ssl\0";
pub const GRPC_TLS_TRANSPORT_SECURITY_TYPE: &[u8; 4usize] = b"tls\0";
pub const GRPC_X509_CN_PROPERTY_NAME: &[u8; 17usize] = b"x509_common_name\0";
pub const GRPC_X509_SUBJECT_PROPERTY_NAME: &[u8; 13usize] = b"x509_subject\0";
pub const GRPC_X509_SAN_PROPERTY_NAME: &[u8; 30usize] = b"x509_subject_alternative_name\0";
pub const GRPC_X509_PEM_CERT_PROPERTY_NAME: &[u8; 14usize] = b"x509_pem_cert\0";
pub const GRPC_X509_PEM_CERT_CHAIN_PROPERTY_NAME: &[u8; 20usize] = b"x509_pem_cert_chain\0";
pub const GRPC_SSL_SESSION_REUSED_PROPERTY: &[u8; 19usize] = b"ssl_session_reused\0";
pub const GRPC_TRANSPORT_SECURITY_LEVEL_PROPERTY_NAME: &[u8; 15usize] = b"security_level\0";
pub const GRPC_PEER_DNS_PROPERTY_NAME: &[u8; 9usize] = b"peer_dns\0";
pub const GRPC_PEER_SPIFFE_ID_PROPERTY_NAME: &[u8; 15usize] = b"peer_spiffe_id\0";
pub const GRPC_PEER_URI_PROPERTY_NAME: &[u8; 9usize] = b"peer_uri\0";
pub const GRPC_PEER_EMAIL_PROPERTY_NAME: &[u8; 11usize] = b"peer_email\0";
pub const GRPC_PEER_IP_PROPERTY_NAME: &[u8; 8usize] = b"peer_ip\0";
pub const GRPC_DEFAULT_SSL_ROOTS_FILE_PATH_ENV_VAR: &[u8; 33usize] =
    b"GRPC_DEFAULT_SSL_ROOTS_FILE_PATH\0";
pub const GRPC_GOOGLE_CREDENTIALS_ENV_VAR: &[u8; 31usize] = b"GOOGLE_APPLICATION_CREDENTIALS\0";
pub const GRPC_METADATA_CREDENTIALS_PLUGIN_SYNC_MAX: u32 = 4;
extern "C" {
    pub fn gpr_unreachable_code(
        reason: *const ::std::os::raw::c_char,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    );
}
#[repr(u32)]
#[doc = " The various compression algorithms supported by gRPC (not sorted by"]
#[doc = " compression level)"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_compression_algorithm {
    GRPC_COMPRESS_NONE = 0,
    GRPC_COMPRESS_DEFLATE = 1,
    GRPC_COMPRESS_GZIP = 2,
    GRPC_COMPRESS_ALGORITHMS_COUNT = 3,
}
#[repr(u32)]
#[doc = " Compression levels allow a party with knowledge of its peer's accepted"]
#[doc = " encodings to request compression in an abstract way. The level-algorithm"]
#[doc = " mapping is performed internally and depends on the peer's supported"]
#[doc = " compression algorithms."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_compression_level {
    GRPC_COMPRESS_LEVEL_NONE = 0,
    GRPC_COMPRESS_LEVEL_LOW = 1,
    GRPC_COMPRESS_LEVEL_MED = 2,
    GRPC_COMPRESS_LEVEL_HIGH = 3,
    GRPC_COMPRESS_LEVEL_COUNT = 4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_compression_options {
    #[doc = " All algs are enabled by default. This option corresponds to the channel"]
    #[doc = " argument key behind \\a GRPC_COMPRESSION_CHANNEL_ENABLED_ALGORITHMS_BITSET"]
    pub enabled_algorithms_bitset: u32,
    pub default_level: grpc_compression_options_grpc_compression_options_default_level,
    pub default_algorithm: grpc_compression_options_grpc_compression_options_default_algorithm,
}
#[doc = " The default compression level. It'll be used in the absence of call"]
#[doc = " specific settings. This option corresponds to the channel"]
#[doc = " argument key behind \\a GRPC_COMPRESSION_CHANNEL_DEFAULT_LEVEL. If present,"]
#[doc = " takes precedence over \\a default_algorithm."]
#[doc = " TODO(dgq): currently only available for server channels."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_compression_options_grpc_compression_options_default_level {
    pub is_set: ::std::os::raw::c_int,
    pub level: grpc_compression_level,
}
#[doc = " The default message compression algorithm. It'll be used in the absence of"]
#[doc = " call specific settings. This option corresponds to the channel argument key"]
#[doc = " behind \\a GRPC_COMPRESSION_CHANNEL_DEFAULT_ALGORITHM."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_compression_options_grpc_compression_options_default_algorithm {
    pub is_set: ::std::os::raw::c_int,
    pub algorithm: grpc_compression_algorithm,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_slice_refcount {
    _unused: [u8; 0],
}
#[doc = " A grpc_slice s, if initialized, represents the byte range"]
#[doc = "s.bytes[0..s.length-1]."]
#[doc = ""]
#[doc = "It can have an associated ref count which has a destruction routine to be run"]
#[doc = "when the ref count reaches zero (see grpc_slice_new() and grp_slice_unref())."]
#[doc = "Multiple grpc_slice values may share a ref count."]
#[doc = ""]
#[doc = "If the slice does not have a refcount, it represents an inlined small piece"]
#[doc = "of data that is copied by value."]
#[doc = ""]
#[doc = "As a special case, a slice can be given refcount == uintptr_t(1), meaning"]
#[doc = "that the slice represents external data that is not refcounted."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_slice {
    pub refcount: *mut grpc_slice_refcount,
    pub data: grpc_slice_grpc_slice_data,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union grpc_slice_grpc_slice_data {
    pub refcounted: grpc_slice_grpc_slice_data_grpc_slice_refcounted,
    pub inlined: grpc_slice_grpc_slice_data_grpc_slice_inlined,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_slice_grpc_slice_data_grpc_slice_refcounted {
    pub length: usize,
    pub bytes: *mut u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_slice_grpc_slice_data_grpc_slice_inlined {
    pub length: u8,
    pub bytes: [u8; 23usize],
}
impl ::std::fmt::Debug for grpc_slice_grpc_slice_data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "grpc_slice_grpc_slice_data {{ union }}")
    }
}
impl ::std::fmt::Debug for grpc_slice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_slice {{ refcount: {:?}, data: {:?} }}",
            self.refcount, self.data
        )
    }
}
#[doc = " Represents an expandable array of slices, to be interpreted as a"]
#[doc = "single item."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_slice_buffer {
    #[doc = " This is for internal use only. External users (i.e any code outside grpc"]
    #[doc = " core) MUST NOT use this field"]
    pub base_slices: *mut grpc_slice,
    #[doc = " slices in the array (Points to the first valid grpc_slice in the array)"]
    pub slices: *mut grpc_slice,
    #[doc = " the number of slices in the array"]
    pub count: usize,
    #[doc = " the number of slices allocated in the array. External users (i.e any code"]
    #[doc = " outside grpc core) MUST NOT use this field"]
    pub capacity: usize,
    #[doc = " the combined length of all slices in the array"]
    pub length: usize,
    #[doc = " inlined elements to avoid allocations"]
    pub inlined: [grpc_slice; 8usize],
}
impl ::std::fmt::Debug for grpc_slice_buffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_slice_buffer {{ base_slices: {:?}, slices: {:?}, inlined: {:?} }}",
            self.base_slices, self.slices, self.inlined
        )
    }
}
#[repr(u32)]
#[doc = " The clocks we support."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum gpr_clock_type {
    #[doc = " Monotonic clock. Epoch undefined. Always moves forwards."]
    GPR_CLOCK_MONOTONIC = 0,
    #[doc = " Realtime clock. May jump forwards or backwards. Settable by"]
    #[doc = "the system administrator. Has its epoch at 0:00:00 UTC 1 Jan 1970."]
    GPR_CLOCK_REALTIME = 1,
    #[doc = " CPU cycle time obtained by rdtsc instruction on x86 platforms. Epoch"]
    #[doc = "undefined. Degrades to GPR_CLOCK_REALTIME on other platforms."]
    GPR_CLOCK_PRECISE = 2,
    #[doc = " Unmeasurable clock type: no base, created by taking the difference"]
    #[doc = "between two times"]
    GPR_TIMESPAN = 3,
}
#[doc = " Analogous to struct timespec. On some machines, absolute times may be in"]
#[doc = " local time."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_timespec {
    pub tv_sec: i64,
    pub tv_nsec: i32,
    #[doc = " Against which clock was this time measured? (or GPR_TIMESPAN if"]
    #[doc = "this is a relative time measure)"]
    pub clock_type: gpr_clock_type,
}
extern "C" {
    #[doc = " Time constants. */"]
    pub fn gpr_time_0(type_: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    #[doc = " The far future"]
    pub fn gpr_inf_future(type_: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    #[doc = " The far past."]
    pub fn gpr_inf_past(type_: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    #[doc = " initialize time subsystem"]
    pub fn gpr_time_init();
}
extern "C" {
    #[doc = " Return the current time measured from the given clocks epoch."]
    pub fn gpr_now(clock: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    #[doc = " Convert a timespec from one clock to another"]
    pub fn gpr_convert_clock_type(t: gpr_timespec, clock_type: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    #[doc = " Return -ve, 0, or +ve according to whether a < b, a == b, or a > b"]
    #[doc = "respectively."]
    pub fn gpr_time_cmp(a: gpr_timespec, b: gpr_timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gpr_time_max(a: gpr_timespec, b: gpr_timespec) -> gpr_timespec;
}
extern "C" {
    pub fn gpr_time_min(a: gpr_timespec, b: gpr_timespec) -> gpr_timespec;
}
extern "C" {
    #[doc = " Add and subtract times.  Calculations saturate at infinities."]
    pub fn gpr_time_add(a: gpr_timespec, b: gpr_timespec) -> gpr_timespec;
}
extern "C" {
    pub fn gpr_time_sub(a: gpr_timespec, b: gpr_timespec) -> gpr_timespec;
}
extern "C" {
    #[doc = " Return a timespec representing a given number of time units. INT64_MIN is"]
    #[doc = "interpreted as gpr_inf_past, and INT64_MAX as gpr_inf_future."]
    pub fn gpr_time_from_micros(us: i64, clock_type: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    pub fn gpr_time_from_nanos(ns: i64, clock_type: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    pub fn gpr_time_from_millis(ms: i64, clock_type: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    pub fn gpr_time_from_seconds(s: i64, clock_type: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    pub fn gpr_time_from_minutes(m: i64, clock_type: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    pub fn gpr_time_from_hours(h: i64, clock_type: gpr_clock_type) -> gpr_timespec;
}
extern "C" {
    pub fn gpr_time_to_millis(timespec: gpr_timespec) -> i32;
}
extern "C" {
    #[doc = " Return 1 if two times are equal or within threshold of each other,"]
    #[doc = "0 otherwise"]
    pub fn gpr_time_similar(
        a: gpr_timespec,
        b: gpr_timespec,
        threshold: gpr_timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Sleep until at least 'until' - an absolute timeout"]
    pub fn gpr_sleep_until(until: gpr_timespec);
}
extern "C" {
    pub fn gpr_timespec_to_micros(t: gpr_timespec) -> f64;
}
pub type gpr_atm = isize;
extern "C" {
    #[doc = " Adds \\a delta to \\a *value, clamping the result to the range specified"]
    #[doc = "by \\a min and \\a max.  Returns the new value."]
    pub fn gpr_atm_no_barrier_clamped_add(
        value: *mut gpr_atm,
        delta: gpr_atm,
        min: gpr_atm,
        max: gpr_atm,
    ) -> gpr_atm;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_event {
    pub state: gpr_atm,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_refcount {
    pub count: gpr_atm,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_stats_counter {
    pub value: gpr_atm,
}
extern "C" {
    #[doc = " Initialize *ev."]
    pub fn gpr_event_init(ev: *mut gpr_event);
}
extern "C" {
    #[doc = " Set *ev so that gpr_event_get() and gpr_event_wait() will return value."]
    #[doc = "Requires:  *ev initialized; value != NULL; no prior or concurrent calls to"]
    #[doc = "gpr_event_set(ev, ...) since initialization."]
    pub fn gpr_event_set(ev: *mut gpr_event, value: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = " Return the value set by gpr_event_set(ev, ...), or NULL if no such call has"]
    #[doc = "completed.  If the result is non-NULL, all operations that occurred prior to"]
    #[doc = "the gpr_event_set(ev, ...) set will be visible after this call returns."]
    #[doc = "Requires:  *ev initialized.  This operation is faster than acquiring a mutex"]
    #[doc = "on most platforms."]
    pub fn gpr_event_get(ev: *mut gpr_event) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Wait until *ev is set by gpr_event_set(ev, ...), or abs_deadline is"]
    #[doc = "exceeded, then return gpr_event_get(ev).  Requires:  *ev initialized.  Use"]
    #[doc = "abs_deadline==gpr_inf_future for no deadline.  When the event has been"]
    #[doc = "signalled before the call, this operation is faster than acquiring a mutex"]
    #[doc = "on most platforms."]
    pub fn gpr_event_wait(
        ev: *mut gpr_event,
        abs_deadline: gpr_timespec,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Initialize *r to value n."]
    pub fn gpr_ref_init(r: *mut gpr_refcount, n: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Increment the reference count *r.  Requires *r initialized."]
    pub fn gpr_ref(r: *mut gpr_refcount);
}
extern "C" {
    #[doc = " Increment the reference count *r.  Requires *r initialized."]
    #[doc = "Crashes if refcount is zero"]
    pub fn gpr_ref_non_zero(r: *mut gpr_refcount);
}
extern "C" {
    #[doc = " Increment the reference count *r by n.  Requires *r initialized, n > 0."]
    pub fn gpr_refn(r: *mut gpr_refcount, n: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Decrement the reference count *r and return non-zero iff it has reached"]
    #[doc = "zero. .  Requires *r initialized."]
    pub fn gpr_unref(r: *mut gpr_refcount) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Return non-zero iff the reference count of *r is one, and thus is owned"]
    #[doc = "by exactly one object."]
    pub fn gpr_ref_is_unique(r: *mut gpr_refcount) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Initialize *c to the value n."]
    pub fn gpr_stats_init(c: *mut gpr_stats_counter, n: isize);
}
extern "C" {
    #[doc = " *c += inc.  Requires: *c initialized."]
    pub fn gpr_stats_inc(c: *mut gpr_stats_counter, inc: isize);
}
extern "C" {
    #[doc = " Return *c.  Requires: *c initialized."]
    pub fn gpr_stats_read(c: *const gpr_stats_counter) -> isize;
}
extern "C" {
    #[doc = " Increment the refcount of s. Requires slice is initialized."]
    #[doc = "Returns s."]
    pub fn grpc_slice_ref(s: grpc_slice) -> grpc_slice;
}
extern "C" {
    #[doc = " Decrement the ref count of s.  If the ref count of s reaches zero, all"]
    #[doc = "slices sharing the ref count are destroyed, and considered no longer"]
    #[doc = "initialized.  If s is ultimately derived from a call to grpc_slice_new(start,"]
    #[doc = "len, dest) where dest!=NULL , then (*dest)(start) is called, else if s is"]
    #[doc = "ultimately derived from a call to grpc_slice_new_with_len(start, len, dest)"]
    #[doc = "where dest!=NULL , then (*dest)(start, len).  Requires s initialized."]
    pub fn grpc_slice_unref(s: grpc_slice);
}
extern "C" {
    #[doc = " Copy slice - create a new slice that contains the same data as s"]
    pub fn grpc_slice_copy(s: grpc_slice) -> grpc_slice;
}
extern "C" {
    #[doc = " Create a slice pointing at some data. Calls malloc to allocate a refcount"]
    #[doc = "for the object, and arranges that destroy will be called with the pointer"]
    #[doc = "passed in at destruction."]
    pub fn grpc_slice_new(
        p: *mut ::std::os::raw::c_void,
        len: usize,
        destroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) -> grpc_slice;
}
extern "C" {
    #[doc = " Equivalent to grpc_slice_new, but with a separate pointer that is"]
    #[doc = "passed to the destroy function.  This function can be useful when"]
    #[doc = "the data is part of a larger structure that must be destroyed when"]
    #[doc = "the data is no longer needed."]
    pub fn grpc_slice_new_with_user_data(
        p: *mut ::std::os::raw::c_void,
        len: usize,
        destroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        user_data: *mut ::std::os::raw::c_void,
    ) -> grpc_slice;
}
extern "C" {
    #[doc = " Equivalent to grpc_slice_new, but with a two argument destroy function that"]
    #[doc = "also takes the slice length."]
    pub fn grpc_slice_new_with_len(
        p: *mut ::std::os::raw::c_void,
        len: usize,
        destroy: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: usize),
        >,
    ) -> grpc_slice;
}
extern "C" {
    #[doc = " Equivalent to grpc_slice_new(malloc(len), len, free), but saves one malloc()"]
    #[doc = "call."]
    #[doc = "Aborts if malloc() fails."]
    pub fn grpc_slice_malloc(length: usize) -> grpc_slice;
}
extern "C" {
    pub fn grpc_slice_malloc_large(length: usize) -> grpc_slice;
}
extern "C" {
    #[doc = " Create a slice by copying a string."]
    #[doc = "Does not preserve null terminators."]
    #[doc = "Equivalent to:"]
    #[doc = "size_t len = strlen(source);"]
    #[doc = "grpc_slice slice = grpc_slice_malloc(len);"]
    #[doc = "memcpy(slice->data, source, len);"]
    pub fn grpc_slice_from_copied_string(source: *const ::std::os::raw::c_char) -> grpc_slice;
}
extern "C" {
    #[doc = " Create a slice by copying a buffer."]
    #[doc = "Equivalent to:"]
    #[doc = "grpc_slice slice = grpc_slice_malloc(len);"]
    #[doc = "memcpy(slice->data, source, len);"]
    pub fn grpc_slice_from_copied_buffer(
        source: *const ::std::os::raw::c_char,
        len: usize,
    ) -> grpc_slice;
}
extern "C" {
    #[doc = " Create a slice pointing to constant memory"]
    pub fn grpc_slice_from_static_string(source: *const ::std::os::raw::c_char) -> grpc_slice;
}
extern "C" {
    #[doc = " Create a slice pointing to constant memory"]
    pub fn grpc_slice_from_static_buffer(
        source: *const ::std::os::raw::c_void,
        len: usize,
    ) -> grpc_slice;
}
extern "C" {
    #[doc = " Return a result slice derived from s, which shares a ref count with \\a s,"]
    #[doc = "where result.data==s.data+begin, and result.length==end-begin. The ref count"]
    #[doc = "of \\a s is increased by one. Do not assign result back to \\a s."]
    #[doc = "Requires s initialized, begin <= end, begin <= s.length, and"]
    #[doc = "end <= source->length."]
    pub fn grpc_slice_sub(s: grpc_slice, begin: usize, end: usize) -> grpc_slice;
}
extern "C" {
    #[doc = " The same as grpc_slice_sub, but without altering the ref count"]
    pub fn grpc_slice_sub_no_ref(s: grpc_slice, begin: usize, end: usize) -> grpc_slice;
}
extern "C" {
    #[doc = " Splits s into two: modifies s to be s[0:split], and returns a new slice,"]
    #[doc = "sharing a refcount with s, that contains s[split:s.length]."]
    #[doc = "Requires s initialized, split <= s.length"]
    pub fn grpc_slice_split_tail(s: *mut grpc_slice, split: usize) -> grpc_slice;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_slice_ref_whom {
    GRPC_SLICE_REF_TAIL = 1,
    GRPC_SLICE_REF_HEAD = 2,
    GRPC_SLICE_REF_BOTH = 3,
}
extern "C" {
    #[doc = " The same as grpc_slice_split_tail, but with an option to skip altering"]
    #[doc = " refcounts (grpc_slice_split_tail_maybe_ref(..., true) is equivalent to"]
    #[doc = " grpc_slice_split_tail(...))"]
    pub fn grpc_slice_split_tail_maybe_ref(
        s: *mut grpc_slice,
        split: usize,
        ref_whom: grpc_slice_ref_whom,
    ) -> grpc_slice;
}
extern "C" {
    #[doc = " Splits s into two: modifies s to be s[split:s.length], and returns a new"]
    #[doc = "slice, sharing a refcount with s, that contains s[0:split]."]
    #[doc = "Requires s initialized, split <= s.length"]
    pub fn grpc_slice_split_head(s: *mut grpc_slice, split: usize) -> grpc_slice;
}
extern "C" {
    pub fn grpc_empty_slice() -> grpc_slice;
}
extern "C" {
    pub fn grpc_slice_eq(a: grpc_slice, b: grpc_slice) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns <0 if a < b, ==0 if a == b, >0 if a > b"]
    #[doc = "The order is arbitrary, and is not guaranteed to be stable across different"]
    #[doc = "versions of the API."]
    pub fn grpc_slice_cmp(a: grpc_slice, b: grpc_slice) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grpc_slice_str_cmp(
        a: grpc_slice,
        b: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " return non-zero if the first blen bytes of a are equal to b"]
    pub fn grpc_slice_buf_start_eq(
        a: grpc_slice,
        b: *const ::std::os::raw::c_void,
        blen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " return the index of the last instance of \\a c in \\a s, or -1 if not found"]
    pub fn grpc_slice_rchr(s: grpc_slice, c: ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grpc_slice_chr(s: grpc_slice, c: ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " return the index of the first occurrence of \\a needle in \\a haystack, or -1"]
    #[doc = "if it's not found"]
    pub fn grpc_slice_slice(haystack: grpc_slice, needle: grpc_slice) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Do two slices point at the same memory, with the same length"]
    #[doc = "If a or b is inlined, actually compares data"]
    pub fn grpc_slice_is_equivalent(a: grpc_slice, b: grpc_slice) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Return a slice pointing to newly allocated memory that has the same contents"]
    #[doc = " as \\a s"]
    pub fn grpc_slice_dup(a: grpc_slice) -> grpc_slice;
}
extern "C" {
    #[doc = " Return a copy of slice as a C string. Offers no protection against embedded"]
    #[doc = "NULL's. Returned string must be freed with gpr_free."]
    pub fn grpc_slice_to_c_string(s: grpc_slice) -> *mut ::std::os::raw::c_char;
}
pub mod grpc_status_code {
    pub type Type = ::std::os::raw::c_int;
    #[doc = " Not an error; returned on success"]
    pub const GRPC_STATUS_OK: Type = 0;
    #[doc = " The operation was cancelled (typically by the caller)."]
    pub const GRPC_STATUS_CANCELLED: Type = 1;
    #[doc = " Unknown error.  An example of where this error may be returned is"]
    #[doc = "if a Status value received from another address space belongs to"]
    #[doc = "an error-space that is not known in this address space.  Also"]
    #[doc = "errors raised by APIs that do not return enough error information"]
    #[doc = "may be converted to this error."]
    pub const GRPC_STATUS_UNKNOWN: Type = 2;
    #[doc = " Client specified an invalid argument.  Note that this differs"]
    #[doc = "from FAILED_PRECONDITION.  INVALID_ARGUMENT indicates arguments"]
    #[doc = "that are problematic regardless of the state of the system"]
    #[doc = "(e.g., a malformed file name)."]
    pub const GRPC_STATUS_INVALID_ARGUMENT: Type = 3;
    #[doc = " Deadline expired before operation could complete.  For operations"]
    #[doc = "that change the state of the system, this error may be returned"]
    #[doc = "even if the operation has completed successfully.  For example, a"]
    #[doc = "successful response from a server could have been delayed long"]
    #[doc = "enough for the deadline to expire."]
    pub const GRPC_STATUS_DEADLINE_EXCEEDED: Type = 4;
    #[doc = " Some requested entity (e.g., file or directory) was not found."]
    pub const GRPC_STATUS_NOT_FOUND: Type = 5;
    #[doc = " Some entity that we attempted to create (e.g., file or directory)"]
    #[doc = "already exists."]
    pub const GRPC_STATUS_ALREADY_EXISTS: Type = 6;
    #[doc = " The caller does not have permission to execute the specified"]
    #[doc = "operation.  PERMISSION_DENIED must not be used for rejections"]
    #[doc = "caused by exhausting some resource (use RESOURCE_EXHAUSTED"]
    #[doc = "instead for those errors).  PERMISSION_DENIED must not be"]
    #[doc = "used if the caller can not be identified (use UNAUTHENTICATED"]
    #[doc = "instead for those errors)."]
    pub const GRPC_STATUS_PERMISSION_DENIED: Type = 7;
    #[doc = " The request does not have valid authentication credentials for the"]
    #[doc = "operation."]
    pub const GRPC_STATUS_UNAUTHENTICATED: Type = 16;
    #[doc = " Some resource has been exhausted, perhaps a per-user quota, or"]
    #[doc = "perhaps the entire file system is out of space."]
    pub const GRPC_STATUS_RESOURCE_EXHAUSTED: Type = 8;
    #[doc = " Operation was rejected because the system is not in a state"]
    #[doc = "required for the operation's execution.  For example, directory"]
    #[doc = "to be deleted may be non-empty, an rmdir operation is applied to"]
    #[doc = "a non-directory, etc."]
    #[doc = ""]
    #[doc = "A litmus test that may help a service implementor in deciding"]
    #[doc = "between FAILED_PRECONDITION, ABORTED, and UNAVAILABLE:"]
    #[doc = "(a) Use UNAVAILABLE if the client can retry just the failing call."]
    #[doc = "(b) Use ABORTED if the client should retry at a higher-level"]
    #[doc = "(e.g., restarting a read-modify-write sequence)."]
    #[doc = "(c) Use FAILED_PRECONDITION if the client should not retry until"]
    #[doc = "the system state has been explicitly fixed.  E.g., if an \"rmdir\""]
    #[doc = "fails because the directory is non-empty, FAILED_PRECONDITION"]
    #[doc = "should be returned since the client should not retry unless"]
    #[doc = "they have first fixed up the directory by deleting files from it."]
    #[doc = "(d) Use FAILED_PRECONDITION if the client performs conditional"]
    #[doc = "REST Get/Update/Delete on a resource and the resource on the"]
    #[doc = "server does not match the condition. E.g., conflicting"]
    #[doc = "read-modify-write on the same resource."]
    pub const GRPC_STATUS_FAILED_PRECONDITION: Type = 9;
    #[doc = " The operation was aborted, typically due to a concurrency issue"]
    #[doc = "like sequencer check failures, transaction aborts, etc."]
    #[doc = ""]
    #[doc = "See litmus test above for deciding between FAILED_PRECONDITION,"]
    #[doc = "ABORTED, and UNAVAILABLE."]
    pub const GRPC_STATUS_ABORTED: Type = 10;
    #[doc = " Operation was attempted past the valid range.  E.g., seeking or"]
    #[doc = "reading past end of file."]
    #[doc = ""]
    #[doc = "Unlike INVALID_ARGUMENT, this error indicates a problem that may"]
    #[doc = "be fixed if the system state changes. For example, a 32-bit file"]
    #[doc = "system will generate INVALID_ARGUMENT if asked to read at an"]
    #[doc = "offset that is not in the range [0,2^32-1], but it will generate"]
    #[doc = "OUT_OF_RANGE if asked to read from an offset past the current"]
    #[doc = "file size."]
    #[doc = ""]
    #[doc = "There is a fair bit of overlap between FAILED_PRECONDITION and"]
    #[doc = "OUT_OF_RANGE.  We recommend using OUT_OF_RANGE (the more specific"]
    #[doc = "error) when it applies so that callers who are iterating through"]
    #[doc = "a space can easily look for an OUT_OF_RANGE error to detect when"]
    #[doc = "they are done."]
    pub const GRPC_STATUS_OUT_OF_RANGE: Type = 11;
    #[doc = " Operation is not implemented or not supported/enabled in this service."]
    pub const GRPC_STATUS_UNIMPLEMENTED: Type = 12;
    #[doc = " Internal errors.  Means some invariants expected by underlying"]
    #[doc = "system has been broken.  If you see one of these errors,"]
    #[doc = "something is very broken."]
    pub const GRPC_STATUS_INTERNAL: Type = 13;
    #[doc = " The service is currently unavailable.  This is a most likely a"]
    #[doc = "transient condition and may be corrected by retrying with"]
    #[doc = "a backoff. Note that it is not always safe to retry non-idempotent"]
    #[doc = "operations."]
    #[doc = ""]
    #[doc = "WARNING: Although data MIGHT not have been transmitted when this"]
    #[doc = "status occurs, there is NOT A GUARANTEE that the server has not seen"]
    #[doc = "anything. So in general it is unsafe to retry on this status code"]
    #[doc = "if the call is non-idempotent."]
    #[doc = ""]
    #[doc = "See litmus test above for deciding between FAILED_PRECONDITION,"]
    #[doc = "ABORTED, and UNAVAILABLE."]
    pub const GRPC_STATUS_UNAVAILABLE: Type = 14;
    #[doc = " Unrecoverable data loss or corruption."]
    pub const GRPC_STATUS_DATA_LOSS: Type = 15;
    #[doc = " Force users to include a default branch:"]
    pub const GRPC_STATUS__DO_NOT_USE: Type = -1;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_byte_buffer_type {
    GRPC_BB_RAW = 0,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_byte_buffer {
    pub reserved: *mut ::std::os::raw::c_void,
    pub type_: grpc_byte_buffer_type,
    pub data: grpc_byte_buffer_grpc_byte_buffer_data,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union grpc_byte_buffer_grpc_byte_buffer_data {
    pub reserved: grpc_byte_buffer_grpc_byte_buffer_data__bindgen_ty_1,
    pub raw: grpc_byte_buffer_grpc_byte_buffer_data_grpc_compressed_buffer,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_byte_buffer_grpc_byte_buffer_data__bindgen_ty_1 {
    pub reserved: [*mut ::std::os::raw::c_void; 8usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_byte_buffer_grpc_byte_buffer_data_grpc_compressed_buffer {
    pub compression: grpc_compression_algorithm,
    pub slice_buffer: grpc_slice_buffer,
}
impl ::std::fmt::Debug for grpc_byte_buffer_grpc_byte_buffer_data_grpc_compressed_buffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "grpc_byte_buffer_grpc_byte_buffer_data_grpc_compressed_buffer {{ compression: {:?}, slice_buffer: {:?} }}" , self . compression , self . slice_buffer)
    }
}
impl ::std::fmt::Debug for grpc_byte_buffer_grpc_byte_buffer_data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "grpc_byte_buffer_grpc_byte_buffer_data {{ union }}")
    }
}
impl ::std::fmt::Debug for grpc_byte_buffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_byte_buffer {{ reserved: {:?}, type: {:?}, data: {:?} }}",
            self.reserved, self.type_, self.data
        )
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_completion_queue {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_channel {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_server {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_call {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_socket_mutator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_socket_factory {
    _unused: [u8; 0],
}
#[repr(u32)]
#[doc = " Type specifier for grpc_arg"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_arg_type {
    GRPC_ARG_STRING = 0,
    GRPC_ARG_INTEGER = 1,
    GRPC_ARG_POINTER = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_arg_pointer_vtable {
    pub copy: ::std::option::Option<
        unsafe extern "C" fn(p: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(p: *mut ::std::os::raw::c_void)>,
    pub cmp: ::std::option::Option<
        unsafe extern "C" fn(
            p: *mut ::std::os::raw::c_void,
            q: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
}
#[doc = " A single argument... each argument has a key and a value"]
#[doc = ""]
#[doc = "A note on naming keys:"]
#[doc = "Keys are namespaced into groups, usually grouped by library, and are"]
#[doc = "keys for module XYZ are named XYZ.key1, XYZ.key2, etc. Module names must"]
#[doc = "be restricted to the regex [A-Za-z][_A-Za-z0-9]{,15}."]
#[doc = "Key names must be restricted to the regex [A-Za-z][_A-Za-z0-9]{,47}."]
#[doc = ""]
#[doc = "GRPC core library keys are prefixed by grpc."]
#[doc = ""]
#[doc = "Library authors are strongly encouraged to \\#define symbolic constants for"]
#[doc = "their keys so that it's possible to change them in the future."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_arg {
    pub type_: grpc_arg_type,
    pub key: *mut ::std::os::raw::c_char,
    pub value: grpc_arg_grpc_arg_value,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union grpc_arg_grpc_arg_value {
    pub string: *mut ::std::os::raw::c_char,
    pub integer: ::std::os::raw::c_int,
    pub pointer: grpc_arg_grpc_arg_value_grpc_arg_pointer,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_arg_grpc_arg_value_grpc_arg_pointer {
    pub p: *mut ::std::os::raw::c_void,
    pub vtable: *const grpc_arg_pointer_vtable,
}
impl ::std::fmt::Debug for grpc_arg_grpc_arg_value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "grpc_arg_grpc_arg_value {{ union }}")
    }
}
impl ::std::fmt::Debug for grpc_arg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_arg {{ type: {:?}, key: {:?}, value: {:?} }}",
            self.type_, self.key, self.value
        )
    }
}
#[doc = " An array of arguments that can be passed around."]
#[doc = ""]
#[doc = "Used to set optional channel-level configuration."]
#[doc = "These configuration options are modelled as key-value pairs as defined"]
#[doc = "by grpc_arg; keys are strings to allow easy backwards-compatible extension"]
#[doc = "by arbitrary parties. All evaluation is performed at channel creation"]
#[doc = "time (i.e. the keys and values in this structure need only live through the"]
#[doc = "creation invocation)."]
#[doc = ""]
#[doc = "However, if one of the args has grpc_arg_type==GRPC_ARG_POINTER, then the"]
#[doc = "grpc_arg_pointer_vtable must live until the channel args are done being"]
#[doc = "used by core (i.e. when the object for use with which they were passed"]
#[doc = "is destroyed)."]
#[doc = ""]
#[doc = "See the description of the \\ref grpc_arg_keys \"available args\" for more"]
#[doc = "details."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_channel_args {
    pub num_args: usize,
    pub args: *mut grpc_arg,
}
#[repr(u32)]
#[doc = " Result of a grpc call. If the caller satisfies the prerequisites of a"]
#[doc = "particular operation, the grpc_call_error returned will be GRPC_CALL_OK."]
#[doc = "Receiving any other value listed here is an indication of a bug in the"]
#[doc = "caller."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_call_error {
    #[doc = " everything went ok"]
    GRPC_CALL_OK = 0,
    #[doc = " something failed, we don't know what"]
    GRPC_CALL_ERROR = 1,
    #[doc = " this method is not available on the server"]
    GRPC_CALL_ERROR_NOT_ON_SERVER = 2,
    #[doc = " this method is not available on the client"]
    GRPC_CALL_ERROR_NOT_ON_CLIENT = 3,
    #[doc = " this method must be called before server_accept"]
    GRPC_CALL_ERROR_ALREADY_ACCEPTED = 4,
    #[doc = " this method must be called before invoke"]
    GRPC_CALL_ERROR_ALREADY_INVOKED = 5,
    #[doc = " this method must be called after invoke"]
    GRPC_CALL_ERROR_NOT_INVOKED = 6,
    #[doc = " this call is already finished"]
    #[doc = "(writes_done or write_status has already been called)"]
    GRPC_CALL_ERROR_ALREADY_FINISHED = 7,
    #[doc = " there is already an outstanding read/write operation on the call"]
    GRPC_CALL_ERROR_TOO_MANY_OPERATIONS = 8,
    #[doc = " the flags value was illegal for this call"]
    GRPC_CALL_ERROR_INVALID_FLAGS = 9,
    #[doc = " invalid metadata was passed to this call"]
    GRPC_CALL_ERROR_INVALID_METADATA = 10,
    #[doc = " invalid message was passed to this call"]
    GRPC_CALL_ERROR_INVALID_MESSAGE = 11,
    #[doc = " completion queue for notification has not been registered"]
    #[doc = " with the server"]
    GRPC_CALL_ERROR_NOT_SERVER_COMPLETION_QUEUE = 12,
    #[doc = " this batch of operations leads to more operations than allowed"]
    GRPC_CALL_ERROR_BATCH_TOO_BIG = 13,
    #[doc = " payload type requested is not the type registered"]
    GRPC_CALL_ERROR_PAYLOAD_TYPE_MISMATCH = 14,
    #[doc = " completion queue has been shutdown"]
    GRPC_CALL_ERROR_COMPLETION_QUEUE_SHUTDOWN = 15,
}
#[doc = " A single metadata element"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_metadata {
    #[doc = " the key, value values are expected to line up with grpc_mdelem: if"]
    #[doc = "changing them, update metadata.h at the same time."]
    pub key: grpc_slice,
    pub value: grpc_slice,
    pub internal_data: grpc_metadata__bindgen_ty_1,
}
#[doc = " The following fields are reserved for grpc internal use."]
#[doc = "There is no need to initialize them, and they will be set to garbage"]
#[doc = "during calls to grpc."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_metadata__bindgen_ty_1 {
    pub obfuscated: [*mut ::std::os::raw::c_void; 4usize],
}
impl ::std::fmt::Debug for grpc_metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_metadata {{ key: {:?}, value: {:?}, internal_data: {:?} }}",
            self.key, self.value, self.internal_data
        )
    }
}
#[repr(u32)]
#[doc = " The type of completion (for grpc_event)"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_completion_type {
    #[doc = " Shutting down"]
    GRPC_QUEUE_SHUTDOWN = 0,
    #[doc = " No event before timeout"]
    GRPC_QUEUE_TIMEOUT = 1,
    #[doc = " Operation completion"]
    GRPC_OP_COMPLETE = 2,
}
#[doc = " The result of an operation."]
#[doc = ""]
#[doc = "Returned by a completion queue when the operation started with tag."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_event {
    #[doc = " The type of the completion."]
    pub type_: grpc_completion_type,
    #[doc = " If the grpc_completion_type is GRPC_OP_COMPLETE, this field indicates"]
    #[doc = "whether the operation was successful or not; 0 in case of failure and"]
    #[doc = "non-zero in case of success."]
    #[doc = "If grpc_completion_type is GRPC_QUEUE_SHUTDOWN or GRPC_QUEUE_TIMEOUT, this"]
    #[doc = "field is guaranteed to be 0"]
    pub success: ::std::os::raw::c_int,
    #[doc = " The tag passed to grpc_call_start_batch etc to start this operation."]
    #[doc = "Only* GRPC_OP_COMPLETE has a tag. For all other grpc_completion_type"]
    #[doc = "values, tag is uninitialized."]
    pub tag: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_metadata_array {
    pub count: usize,
    pub capacity: usize,
    pub metadata: *mut grpc_metadata,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_call_details {
    pub method: grpc_slice,
    pub host: grpc_slice,
    pub deadline: gpr_timespec,
}
impl ::std::fmt::Debug for grpc_call_details {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_call_details {{ method: {:?}, host: {:?}, deadline: {:?} }}",
            self.method, self.host, self.deadline
        )
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_op_type {
    #[doc = " Send initial metadata: one and only one instance MUST be sent for each"]
    #[doc = "call, unless the call was cancelled - in which case this can be skipped."]
    #[doc = "This op completes after all bytes of metadata have been accepted by"]
    #[doc = "outgoing flow control."]
    GRPC_OP_SEND_INITIAL_METADATA = 0,
    #[doc = " Send a message: 0 or more of these operations can occur for each call."]
    #[doc = "This op completes after all bytes for the message have been accepted by"]
    #[doc = "outgoing flow control."]
    GRPC_OP_SEND_MESSAGE = 1,
    #[doc = " Send a close from the client: one and only one instance MUST be sent from"]
    #[doc = "the client, unless the call was cancelled - in which case this can be"]
    #[doc = "skipped. This op completes after all bytes for the call"]
    #[doc = "(including the close) have passed outgoing flow control."]
    GRPC_OP_SEND_CLOSE_FROM_CLIENT = 2,
    #[doc = " Send status from the server: one and only one instance MUST be sent from"]
    #[doc = "the server unless the call was cancelled - in which case this can be"]
    #[doc = "skipped. This op completes after all bytes for the call"]
    #[doc = "(including the status) have passed outgoing flow control."]
    GRPC_OP_SEND_STATUS_FROM_SERVER = 3,
    #[doc = " Receive initial metadata: one and only one MUST be made on the client,"]
    #[doc = "must not be made on the server."]
    #[doc = "This op completes after all initial metadata has been read from the"]
    #[doc = "peer."]
    GRPC_OP_RECV_INITIAL_METADATA = 4,
    #[doc = " Receive a message: 0 or more of these operations can occur for each call."]
    #[doc = "This op completes after all bytes of the received message have been"]
    #[doc = "read, or after a half-close has been received on this call."]
    GRPC_OP_RECV_MESSAGE = 5,
    #[doc = " Receive status on the client: one and only one must be made on the client."]
    #[doc = "This operation always succeeds, meaning ops paired with this operation"]
    #[doc = "will also appear to succeed, even though they may not have. In that case"]
    #[doc = "the status will indicate some failure."]
    #[doc = "This op completes after all activity on the call has completed."]
    GRPC_OP_RECV_STATUS_ON_CLIENT = 6,
    #[doc = " Receive close on the server: one and only one must be made on the"]
    #[doc = "server. This op completes after the close has been received by the"]
    #[doc = "server. This operation always succeeds, meaning ops paired with"]
    #[doc = "this operation will also appear to succeed, even though they may not"]
    #[doc = "have."]
    GRPC_OP_RECV_CLOSE_ON_SERVER = 7,
}
#[doc = " Operation data: one field for each op type (except SEND_CLOSE_FROM_CLIENT"]
#[doc = "which has no arguments)"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_op {
    #[doc = " Operation type, as defined by grpc_op_type"]
    pub op: grpc_op_type,
    #[doc = " Write flags bitset for grpc_begin_messages"]
    pub flags: u32,
    #[doc = " Reserved for future usage"]
    pub reserved: *mut ::std::os::raw::c_void,
    pub data: grpc_op_grpc_op_data,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union grpc_op_grpc_op_data {
    pub reserved: grpc_op_grpc_op_data__bindgen_ty_1,
    pub send_initial_metadata: grpc_op_grpc_op_data_grpc_op_send_initial_metadata,
    pub send_message: grpc_op_grpc_op_data_grpc_op_send_message,
    pub send_status_from_server: grpc_op_grpc_op_data_grpc_op_send_status_from_server,
    pub recv_initial_metadata: grpc_op_grpc_op_data_grpc_op_recv_initial_metadata,
    pub recv_message: grpc_op_grpc_op_data_grpc_op_recv_message,
    pub recv_status_on_client: grpc_op_grpc_op_data_grpc_op_recv_status_on_client,
    pub recv_close_on_server: grpc_op_grpc_op_data_grpc_op_recv_close_on_server,
}
#[doc = " Reserved for future usage"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data__bindgen_ty_1 {
    pub reserved: [*mut ::std::os::raw::c_void; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data_grpc_op_send_initial_metadata { pub count : usize , pub metadata : * mut grpc_metadata , pub maybe_compression_level : grpc_op_grpc_op_data_grpc_op_send_initial_metadata_grpc_op_send_initial_metadata_maybe_compression_level , }
#[doc = " If \\a is_set, \\a compression_level will be used for the call."]
#[doc = " Otherwise, \\a compression_level won't be considered"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data_grpc_op_send_initial_metadata_grpc_op_send_initial_metadata_maybe_compression_level
{
    pub is_set: u8,
    pub level: grpc_compression_level,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data_grpc_op_send_message {
    #[doc = " This op takes ownership of the slices in send_message.  After"]
    #[doc = " a call completes, the contents of send_message are not guaranteed"]
    #[doc = " and likely empty.  The original owner should still call"]
    #[doc = " grpc_byte_buffer_destroy() on this object however."]
    pub send_message: *mut grpc_byte_buffer,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data_grpc_op_send_status_from_server {
    pub trailing_metadata_count: usize,
    pub trailing_metadata: *mut grpc_metadata,
    pub status: grpc_status_code::Type,
    #[doc = " optional: set to NULL if no details need sending, non-NULL if they do"]
    #[doc = " pointer will not be retained past the start_batch call"]
    pub status_details: *mut grpc_slice,
}
#[doc = " ownership of the array is with the caller, but ownership of the elements"]
#[doc = "stays with the call object (ie key, value members are owned by the call"]
#[doc = "object, recv_initial_metadata->array is owned by the caller)."]
#[doc = "After the operation completes, call grpc_metadata_array_destroy on this"]
#[doc = "value, or reuse it in a future op."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data_grpc_op_recv_initial_metadata {
    pub recv_initial_metadata: *mut grpc_metadata_array,
}
#[doc = " ownership of the byte buffer is moved to the caller; the caller must"]
#[doc = "call grpc_byte_buffer_destroy on this value, or reuse it in a future op."]
#[doc = "The returned byte buffer will be NULL if trailing metadata was"]
#[doc = "received instead of a message."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data_grpc_op_recv_message {
    pub recv_message: *mut *mut grpc_byte_buffer,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data_grpc_op_recv_status_on_client {
    #[doc = " ownership of the array is with the caller, but ownership of the"]
    #[doc = "elements stays with the call object (ie key, value members are owned"]
    #[doc = "by the call object, trailing_metadata->array is owned by the caller)."]
    #[doc = "After the operation completes, call grpc_metadata_array_destroy on"]
    #[doc = "this value, or reuse it in a future op."]
    pub trailing_metadata: *mut grpc_metadata_array,
    pub status: *mut grpc_status_code::Type,
    pub status_details: *mut grpc_slice,
    #[doc = " If this is not nullptr, it will be populated with the full fidelity"]
    #[doc = " error string for debugging purposes. The application is responsible"]
    #[doc = " for freeing the data by using gpr_free()."]
    pub error_string: *mut *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_op_grpc_op_data_grpc_op_recv_close_on_server {
    #[doc = " out argument, set to 1 if the call failed at the server for"]
    #[doc = "a reason other than a non-OK status (cancel, deadline"]
    #[doc = "exceeded, network failure, etc.), 0 otherwise (RPC processing ran to"]
    #[doc = "completion and was able to provide any status from the server)"]
    pub cancelled: *mut ::std::os::raw::c_int,
}
impl ::std::fmt::Debug for grpc_op_grpc_op_data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "grpc_op_grpc_op_data {{ union }}")
    }
}
impl ::std::fmt::Debug for grpc_op {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_op {{ op: {:?}, reserved: {:?}, data: {:?} }}",
            self.op, self.reserved, self.data
        )
    }
}
#[doc = " Information requested from the channel."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_channel_info {
    #[doc = " If non-NULL, will be set to point to a string indicating the LB"]
    #[doc = " policy name.  Caller takes ownership."]
    pub lb_policy_name: *mut *mut ::std::os::raw::c_char,
    #[doc = " If non-NULL, will be set to point to a string containing the"]
    #[doc = " service config used by the channel in JSON form."]
    pub service_config_json: *mut *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_resource_quota {
    _unused: [u8; 0],
}
#[repr(u32)]
#[doc = " Completion queues internally MAY maintain a set of file descriptors in a"]
#[doc = "structure called 'pollset'. This enum specifies if a completion queue has an"]
#[doc = "associated pollset and any restrictions on the type of file descriptors that"]
#[doc = "can be present in the pollset."]
#[doc = ""]
#[doc = "I/O progress can only be made when grpc_completion_queue_next() or"]
#[doc = "grpc_completion_queue_pluck() are called on the completion queue (unless the"]
#[doc = "grpc_cq_polling_type is GRPC_CQ_NON_POLLING) and hence it is very important"]
#[doc = "to actively call these APIs"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_cq_polling_type {
    #[doc = " The completion queue will have an associated pollset and there is no"]
    #[doc = "restriction on the type of file descriptors the pollset may contain"]
    GRPC_CQ_DEFAULT_POLLING = 0,
    #[doc = " Similar to GRPC_CQ_DEFAULT_POLLING except that the completion queues will"]
    #[doc = "not contain any 'listening file descriptors' (i.e file descriptors used to"]
    #[doc = "listen to incoming channels)"]
    GRPC_CQ_NON_LISTENING = 1,
    #[doc = " The completion queue will not have an associated pollset. Note that"]
    #[doc = "grpc_completion_queue_next() or grpc_completion_queue_pluck() MUST still"]
    #[doc = "be called to pop events from the completion queue; it is not required to"]
    #[doc = "call them actively to make I/O progress"]
    GRPC_CQ_NON_POLLING = 2,
}
#[repr(u32)]
#[doc = " Specifies the type of APIs to use to pop events from the completion queue"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_cq_completion_type {
    #[doc = " Events are popped out by calling grpc_completion_queue_next() API ONLY"]
    GRPC_CQ_NEXT = 0,
    #[doc = " Events are popped out by calling grpc_completion_queue_pluck() API ONLY"]
    GRPC_CQ_PLUCK = 1,
    #[doc = " Events trigger a callback specified as the tag"]
    GRPC_CQ_CALLBACK = 2,
}
#[doc = " Specifies an interface class to be used as a tag for callback-based"]
#[doc = " completion queues. This can be used directly, as the first element of a"]
#[doc = " struct in C, or as a base class in C++. Its \"run\" value should be assigned to"]
#[doc = " some non-member function, such as a static method."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_completion_queue_functor {
    #[doc = " The run member specifies a function that will be called when this"]
    #[doc = "tag is extracted from the completion queue. Its arguments will be a"]
    #[doc = "pointer to this functor and a boolean that indicates whether the"]
    #[doc = "operation succeeded (non-zero) or failed (zero)"]
    pub functor_run: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut grpc_completion_queue_functor, arg2: ::std::os::raw::c_int),
    >,
    #[doc = " The inlineable member specifies whether this functor can be run inline."]
    #[doc = "This should only be used for trivial internally-defined functors."]
    pub inlineable: ::std::os::raw::c_int,
    #[doc = " The following fields are not API. They are meant for internal use."]
    pub internal_success: ::std::os::raw::c_int,
    pub internal_next: *mut grpc_completion_queue_functor,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_completion_queue_attributes {
    #[doc = " The version number of this structure. More fields might be added to this"]
    #[doc = "structure in future."]
    pub version: ::std::os::raw::c_int,
    #[doc = " Set to GRPC_CQ_CURRENT_VERSION"]
    pub cq_completion_type: grpc_cq_completion_type,
    pub cq_polling_type: grpc_cq_polling_type,
    #[doc = " When creating a callbackable CQ, pass in a functor to get invoked when"]
    #[doc = " shutdown is complete"]
    pub cq_shutdown_cb: *mut grpc_completion_queue_functor,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_completion_queue_factory {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " initialize a slice buffer"]
    pub fn grpc_slice_buffer_init(sb: *mut grpc_slice_buffer);
}
extern "C" {
    #[doc = " destroy a slice buffer - unrefs any held elements"]
    pub fn grpc_slice_buffer_destroy(sb: *mut grpc_slice_buffer);
}
extern "C" {
    #[doc = " Add an element to a slice buffer - takes ownership of the slice."]
    #[doc = "This function is allowed to concatenate the passed in slice to the end of"]
    #[doc = "some other slice if desired by the slice buffer."]
    pub fn grpc_slice_buffer_add(sb: *mut grpc_slice_buffer, slice: grpc_slice);
}
extern "C" {
    #[doc = " add an element to a slice buffer - takes ownership of the slice and returns"]
    #[doc = "the index of the slice."]
    #[doc = "Guarantees that the slice will not be concatenated at the end of another"]
    #[doc = "slice (i.e. the data for this slice will begin at the first byte of the"]
    #[doc = "slice at the returned index in sb->slices)"]
    #[doc = "The implementation MAY decide to concatenate data at the end of a small"]
    #[doc = "slice added in this fashion."]
    pub fn grpc_slice_buffer_add_indexed(sb: *mut grpc_slice_buffer, slice: grpc_slice) -> usize;
}
extern "C" {
    pub fn grpc_slice_buffer_addn(sb: *mut grpc_slice_buffer, slices: *mut grpc_slice, n: usize);
}
extern "C" {
    #[doc = " add a very small (less than 8 bytes) amount of data to the end of a slice"]
    #[doc = "buffer: returns a pointer into which to add the data"]
    pub fn grpc_slice_buffer_tiny_add(sb: *mut grpc_slice_buffer, len: usize) -> *mut u8;
}
extern "C" {
    #[doc = " pop the last buffer, but don't unref it"]
    pub fn grpc_slice_buffer_pop(sb: *mut grpc_slice_buffer);
}
extern "C" {
    #[doc = " clear a slice buffer, unref all elements"]
    pub fn grpc_slice_buffer_reset_and_unref(sb: *mut grpc_slice_buffer);
}
extern "C" {
    #[doc = " swap the contents of two slice buffers"]
    pub fn grpc_slice_buffer_swap(a: *mut grpc_slice_buffer, b: *mut grpc_slice_buffer);
}
extern "C" {
    #[doc = " move all of the elements of src into dst"]
    pub fn grpc_slice_buffer_move_into(src: *mut grpc_slice_buffer, dst: *mut grpc_slice_buffer);
}
extern "C" {
    #[doc = " remove n bytes from the end of a slice buffer"]
    pub fn grpc_slice_buffer_trim_end(
        sb: *mut grpc_slice_buffer,
        n: usize,
        garbage: *mut grpc_slice_buffer,
    );
}
extern "C" {
    #[doc = " move the first n bytes of src into dst"]
    pub fn grpc_slice_buffer_move_first(
        src: *mut grpc_slice_buffer,
        n: usize,
        dst: *mut grpc_slice_buffer,
    );
}
extern "C" {
    #[doc = " move the first n bytes of src into dst without adding references"]
    pub fn grpc_slice_buffer_move_first_no_ref(
        src: *mut grpc_slice_buffer,
        n: usize,
        dst: *mut grpc_slice_buffer,
    );
}
extern "C" {
    #[doc = " move the first n bytes of src into dst (copying them)"]
    pub fn grpc_slice_buffer_move_first_into_buffer(
        src: *mut grpc_slice_buffer,
        n: usize,
        dst: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " take the first slice in the slice buffer"]
    pub fn grpc_slice_buffer_take_first(sb: *mut grpc_slice_buffer) -> grpc_slice;
}
extern "C" {
    #[doc = " undo the above with (a possibly different) \\a slice"]
    pub fn grpc_slice_buffer_undo_take_first(sb: *mut grpc_slice_buffer, slice: grpc_slice);
}
extern "C" {
    #[doc = " Returns a RAW byte buffer instance over the given slices (up to \\a nslices)."]
    #[doc = ""]
    #[doc = " Increases the reference count for all \\a slices processed. The user is"]
    #[doc = " responsible for invoking grpc_byte_buffer_destroy on the returned instance."]
    pub fn grpc_raw_byte_buffer_create(
        slices: *mut grpc_slice,
        nslices: usize,
    ) -> *mut grpc_byte_buffer;
}
extern "C" {
    #[doc = " Returns a *compressed* RAW byte buffer instance over the given slices (up to"]
    #[doc = " \\a nslices). The \\a compression argument defines the compression algorithm"]
    #[doc = " used to generate the data in \\a slices."]
    #[doc = ""]
    #[doc = " Increases the reference count for all \\a slices processed. The user is"]
    #[doc = " responsible for invoking grpc_byte_buffer_destroy on the returned instance."]
    pub fn grpc_raw_compressed_byte_buffer_create(
        slices: *mut grpc_slice,
        nslices: usize,
        compression: grpc_compression_algorithm,
    ) -> *mut grpc_byte_buffer;
}
extern "C" {
    #[doc = " Copies input byte buffer \\a bb."]
    #[doc = ""]
    #[doc = " Increases the reference count of all the source slices. The user is"]
    #[doc = " responsible for calling grpc_byte_buffer_destroy over the returned copy."]
    pub fn grpc_byte_buffer_copy(bb: *mut grpc_byte_buffer) -> *mut grpc_byte_buffer;
}
extern "C" {
    #[doc = " Returns the size of the given byte buffer, in bytes."]
    pub fn grpc_byte_buffer_length(bb: *mut grpc_byte_buffer) -> usize;
}
extern "C" {
    #[doc = " Destroys \\a byte_buffer deallocating all its memory."]
    pub fn grpc_byte_buffer_destroy(bb: *mut grpc_byte_buffer);
}
extern "C" {
    #[doc = " Initialize \\a reader to read over \\a buffer."]
    #[doc = " Returns 1 upon success, 0 otherwise."]
    pub fn grpc_byte_buffer_reader_init(
        reader: *mut grpc_byte_buffer_reader,
        buffer: *mut grpc_byte_buffer,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Cleanup and destroy \\a reader"]
    pub fn grpc_byte_buffer_reader_destroy(reader: *mut grpc_byte_buffer_reader);
}
extern "C" {
    #[doc = " Updates \\a slice with the next piece of data from from \\a reader and returns"]
    #[doc = " 1. Returns 0 at the end of the stream. Caller is responsible for calling"]
    #[doc = " grpc_slice_unref on the result."]
    pub fn grpc_byte_buffer_reader_next(
        reader: *mut grpc_byte_buffer_reader,
        slice: *mut grpc_slice,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - This function may be removed and changed, in the future."]
    #[doc = ""]
    #[doc = " Updates \\a slice with the next piece of data from from \\a reader and returns"]
    #[doc = " 1. Returns 0 at the end of the stream. Caller is responsible for making sure"]
    #[doc = " the slice pointer remains valid when accessed."]
    #[doc = ""]
    #[doc = " NOTE: Do not use this function unless the caller can guarantee that the"]
    #[doc = "       underlying grpc_byte_buffer outlasts the use of the slice. This is only"]
    #[doc = "       safe when the underlying grpc_byte_buffer remains immutable while slice"]
    #[doc = "       is being accessed."]
    pub fn grpc_byte_buffer_reader_peek(
        reader: *mut grpc_byte_buffer_reader,
        slice: *mut *mut grpc_slice,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Merge all data from \\a reader into single slice"]
    pub fn grpc_byte_buffer_reader_readall(reader: *mut grpc_byte_buffer_reader) -> grpc_slice;
}
extern "C" {
    #[doc = " Returns a RAW byte buffer instance from the output of \\a reader."]
    pub fn grpc_raw_byte_buffer_from_reader(
        reader: *mut grpc_byte_buffer_reader,
    ) -> *mut grpc_byte_buffer;
}
extern "C" {
    #[doc = " Return if an algorithm is message compression algorithm."]
    pub fn grpc_compression_algorithm_is_message(
        algorithm: grpc_compression_algorithm,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Return if an algorithm is stream compression algorithm."]
    pub fn grpc_compression_algorithm_is_stream(
        algorithm: grpc_compression_algorithm,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Parses the \\a slice as a grpc_compression_algorithm instance and updating \\a"]
    #[doc = " algorithm. Returns 1 upon success, 0 otherwise."]
    pub fn grpc_compression_algorithm_parse(
        name: grpc_slice,
        algorithm: *mut grpc_compression_algorithm,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Updates \\a name with the encoding name corresponding to a valid \\a"]
    #[doc = " algorithm. Note that \\a name is statically allocated and must *not* be freed."]
    #[doc = " Returns 1 upon success, 0 otherwise."]
    pub fn grpc_compression_algorithm_name(
        algorithm: grpc_compression_algorithm,
        name: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the compression algorithm corresponding to \\a level for the"]
    #[doc = " compression algorithms encoded in the \\a accepted_encodings bitset."]
    pub fn grpc_compression_algorithm_for_level(
        level: grpc_compression_level,
        accepted_encodings: u32,
    ) -> grpc_compression_algorithm;
}
extern "C" {
    pub fn grpc_compression_options_init(opts: *mut grpc_compression_options);
}
extern "C" {
    #[doc = " Mark \\a algorithm as enabled in \\a opts."]
    pub fn grpc_compression_options_enable_algorithm(
        opts: *mut grpc_compression_options,
        algorithm: grpc_compression_algorithm,
    );
}
extern "C" {
    #[doc = " Mark \\a algorithm as disabled in \\a opts."]
    pub fn grpc_compression_options_disable_algorithm(
        opts: *mut grpc_compression_options,
        algorithm: grpc_compression_algorithm,
    );
}
extern "C" {
    #[doc = " Returns true if \\a algorithm is marked as enabled in \\a opts."]
    pub fn grpc_compression_options_is_algorithm_enabled(
        opts: *const grpc_compression_options,
        algorithm: grpc_compression_algorithm,
    ) -> ::std::os::raw::c_int;
}
#[repr(u32)]
#[doc = " Connectivity state of a channel."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_connectivity_state {
    #[doc = " channel is idle"]
    GRPC_CHANNEL_IDLE = 0,
    #[doc = " channel is connecting"]
    GRPC_CHANNEL_CONNECTING = 1,
    #[doc = " channel is ready for work"]
    GRPC_CHANNEL_READY = 2,
    #[doc = " channel has seen a failure but expects to recover"]
    GRPC_CHANNEL_TRANSIENT_FAILURE = 3,
    #[doc = " channel has seen a failure that it cannot recover from"]
    GRPC_CHANNEL_SHUTDOWN = 4,
}
extern "C" {
    #[doc = " \\mainpage GRPC Core"]
    #[doc = ""]
    #[doc = " The GRPC Core library is a low-level library designed to be wrapped by higher"]
    #[doc = " level libraries. The top-level API is provided in grpc.h. Security related"]
    #[doc = " functionality lives in grpc_security.h."]
    pub fn grpc_metadata_array_init(array: *mut grpc_metadata_array);
}
extern "C" {
    pub fn grpc_metadata_array_destroy(array: *mut grpc_metadata_array);
}
extern "C" {
    pub fn grpc_call_details_init(details: *mut grpc_call_details);
}
extern "C" {
    pub fn grpc_call_details_destroy(details: *mut grpc_call_details);
}
extern "C" {
    #[doc = " Initialize the grpc library."]
    #[doc = ""]
    #[doc = "After it's called, a matching invocation to grpc_shutdown() is expected."]
    #[doc = ""]
    #[doc = "It is not safe to call any other grpc functions before calling this."]
    #[doc = "(To avoid overhead, little checking is done, and some things may work. We"]
    #[doc = "do not warrant that they will continue to do so in future revisions of this"]
    #[doc = "library)."]
    pub fn grpc_init();
}
extern "C" {
    #[doc = " Shut down the grpc library."]
    #[doc = ""]
    #[doc = "Before it's called, there should haven been a matching invocation to"]
    #[doc = "grpc_init()."]
    #[doc = ""]
    #[doc = "The last call to grpc_shutdown will initiate cleaning up of grpc library"]
    #[doc = "internals, which can happen in another thread. Once the clean-up is done,"]
    #[doc = "no memory is used by grpc, nor are any instructions executing within the"]
    #[doc = "grpc library.  Prior to calling, all application owned grpc objects must"]
    #[doc = "have been destroyed."]
    pub fn grpc_shutdown();
}
extern "C" {
    #[doc = " EXPERIMENTAL. Returns 1 if the grpc library has been initialized."]
    #[doc = "TODO(ericgribkoff) Decide if this should be promoted to non-experimental as"]
    #[doc = "part of stabilizing the fork support API, as tracked in"]
    #[doc = "https://github.com/grpc/grpc/issues/15334"]
    pub fn grpc_is_initialized() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " DEPRECATED. Recommend to use grpc_shutdown only"]
    pub fn grpc_shutdown_blocking();
}
extern "C" {
    #[doc = " Return a string representing the current version of grpc"]
    pub fn grpc_version_string() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Return a string specifying what the 'g' in gRPC stands for"]
    pub fn grpc_g_stands_for() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Returns the completion queue factory based on the attributes. MAY return a"]
    #[doc = "NULL if no factory can be found"]
    pub fn grpc_completion_queue_factory_lookup(
        attributes: *const grpc_completion_queue_attributes,
    ) -> *const grpc_completion_queue_factory;
}
extern "C" {
    #[doc = " Helper function to create a completion queue with grpc_cq_completion_type"]
    #[doc = "of GRPC_CQ_NEXT and grpc_cq_polling_type of GRPC_CQ_DEFAULT_POLLING"]
    pub fn grpc_completion_queue_create_for_next(
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_completion_queue;
}
extern "C" {
    #[doc = " Helper function to create a completion queue with grpc_cq_completion_type"]
    #[doc = "of GRPC_CQ_PLUCK and grpc_cq_polling_type of GRPC_CQ_DEFAULT_POLLING"]
    pub fn grpc_completion_queue_create_for_pluck(
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_completion_queue;
}
extern "C" {
    #[doc = " Helper function to create a completion queue with grpc_cq_completion_type"]
    #[doc = "of GRPC_CQ_CALLBACK and grpc_cq_polling_type of GRPC_CQ_DEFAULT_POLLING."]
    #[doc = "This function is experimental."]
    pub fn grpc_completion_queue_create_for_callback(
        shutdown_callback: *mut grpc_completion_queue_functor,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_completion_queue;
}
extern "C" {
    #[doc = " Create a completion queue"]
    pub fn grpc_completion_queue_create(
        factory: *const grpc_completion_queue_factory,
        attributes: *const grpc_completion_queue_attributes,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_completion_queue;
}
extern "C" {
    #[doc = " Blocks until an event is available, the completion queue is being shut down,"]
    #[doc = "or deadline is reached."]
    #[doc = ""]
    #[doc = "Returns a grpc_event with type GRPC_QUEUE_TIMEOUT on timeout,"]
    #[doc = "otherwise a grpc_event describing the event that occurred."]
    #[doc = ""]
    #[doc = "Callers must not call grpc_completion_queue_next and"]
    #[doc = "grpc_completion_queue_pluck simultaneously on the same completion queue."]
    pub fn grpc_completion_queue_next(
        cq: *mut grpc_completion_queue,
        deadline: gpr_timespec,
        reserved: *mut ::std::os::raw::c_void,
    ) -> grpc_event;
}
extern "C" {
    #[doc = " Blocks until an event with tag 'tag' is available, the completion queue is"]
    #[doc = "being shutdown or deadline is reached."]
    #[doc = ""]
    #[doc = "Returns a grpc_event with type GRPC_QUEUE_TIMEOUT on timeout,"]
    #[doc = "otherwise a grpc_event describing the event that occurred."]
    #[doc = ""]
    #[doc = "Callers must not call grpc_completion_queue_next and"]
    #[doc = "grpc_completion_queue_pluck simultaneously on the same completion queue."]
    #[doc = ""]
    #[doc = "Completion queues support a maximum of GRPC_MAX_COMPLETION_QUEUE_PLUCKERS"]
    #[doc = "concurrently executing plucks at any time."]
    pub fn grpc_completion_queue_pluck(
        cq: *mut grpc_completion_queue,
        tag: *mut ::std::os::raw::c_void,
        deadline: gpr_timespec,
        reserved: *mut ::std::os::raw::c_void,
    ) -> grpc_event;
}
extern "C" {
    #[doc = " Begin destruction of a completion queue. Once all possible events are"]
    #[doc = "drained then grpc_completion_queue_next will start to produce"]
    #[doc = "GRPC_QUEUE_SHUTDOWN events only. At that point it's safe to call"]
    #[doc = "grpc_completion_queue_destroy."]
    #[doc = ""]
    #[doc = "After calling this function applications should ensure that no"]
    #[doc = "NEW work is added to be published on this completion queue."]
    pub fn grpc_completion_queue_shutdown(cq: *mut grpc_completion_queue);
}
extern "C" {
    #[doc = " Destroy a completion queue. The caller must ensure that the queue is"]
    #[doc = "drained and no threads are executing grpc_completion_queue_next"]
    pub fn grpc_completion_queue_destroy(cq: *mut grpc_completion_queue);
}
extern "C" {
    #[doc = " EXPERIMENTAL API ************/"]
    #[doc = " grpc_flush_cq_tls_cache() MUST be called on the same thread,"]
    #[doc = " with the same cq."]
    pub fn grpc_completion_queue_thread_local_cache_init(cq: *mut grpc_completion_queue);
}
extern "C" {
    #[doc = " EXPERIMENTAL API ************/"]
    #[doc = " Returns 1 if there was contents in the cache.  If there was an event"]
    #[doc = " in \\a cq tls cache, its tag is placed in tag, and ok is set to the"]
    #[doc = " event success."]
    pub fn grpc_completion_queue_thread_local_cache_flush(
        cq: *mut grpc_completion_queue,
        tag: *mut *mut ::std::os::raw::c_void,
        ok: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Check the connectivity state of a channel."]
    pub fn grpc_channel_check_connectivity_state(
        channel: *mut grpc_channel,
        try_to_connect: ::std::os::raw::c_int,
    ) -> grpc_connectivity_state;
}
extern "C" {
    #[doc = " Number of active \"external connectivity state watchers\" attached to a"]
    #[doc = " channel."]
    #[doc = " Useful for testing."]
    pub fn grpc_channel_num_external_connectivity_watchers(
        channel: *mut grpc_channel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Watch for a change in connectivity state."]
    #[doc = "Once the channel connectivity state is different from last_observed_state,"]
    #[doc = "tag will be enqueued on cq with success=1."]
    #[doc = "If deadline expires BEFORE the state is changed, tag will be enqueued on cq"]
    #[doc = "with success=0."]
    pub fn grpc_channel_watch_connectivity_state(
        channel: *mut grpc_channel,
        last_observed_state: grpc_connectivity_state,
        deadline: gpr_timespec,
        cq: *mut grpc_completion_queue,
        tag: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Check whether a grpc channel supports connectivity watcher"]
    pub fn grpc_channel_support_connectivity_watcher(
        channel: *mut grpc_channel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Create a call given a grpc_channel, in order to call 'method'. All"]
    #[doc = "completions are sent to 'completion_queue'. 'method' and 'host' need only"]
    #[doc = "live through the invocation of this function."]
    #[doc = "If parent_call is non-NULL, it must be a server-side call. It will be used"]
    #[doc = "to propagate properties from the server call to this new client call,"]
    #[doc = "depending on the value of \\a propagation_mask (see propagation_bits.h for"]
    #[doc = "possible values)."]
    pub fn grpc_channel_create_call(
        channel: *mut grpc_channel,
        parent_call: *mut grpc_call,
        propagation_mask: u32,
        completion_queue: *mut grpc_completion_queue,
        method: grpc_slice,
        host: *const grpc_slice,
        deadline: gpr_timespec,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call;
}
extern "C" {
    #[doc = " Pre-register a method/host pair on a channel."]
    #[doc = "method and host are not owned and must remain alive while the channel is"]
    #[doc = "alive."]
    pub fn grpc_channel_register_call(
        channel: *mut grpc_channel,
        method: *const ::std::os::raw::c_char,
        host: *const ::std::os::raw::c_char,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Create a call given a handle returned from grpc_channel_register_call."]
    #[doc = "\\sa grpc_channel_create_call."]
    pub fn grpc_channel_create_registered_call(
        channel: *mut grpc_channel,
        parent_call: *mut grpc_call,
        propagation_mask: u32,
        completion_queue: *mut grpc_completion_queue,
        registered_call_handle: *mut ::std::os::raw::c_void,
        deadline: gpr_timespec,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call;
}
extern "C" {
    #[doc = " Allocate memory in the grpc_call arena: this memory is automatically"]
    #[doc = "discarded at call completion"]
    pub fn grpc_call_arena_alloc(call: *mut grpc_call, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Start a batch of operations defined in the array ops; when complete, post a"]
    #[doc = "completion of type 'tag' to the completion queue bound to the call."]
    #[doc = "The order of ops specified in the batch has no significance."]
    #[doc = "Only one operation of each type can be active at once in any given"]
    #[doc = "batch."]
    #[doc = "If a call to grpc_call_start_batch returns GRPC_CALL_OK you must call"]
    #[doc = "grpc_completion_queue_next or grpc_completion_queue_pluck on the completion"]
    #[doc = "queue associated with 'call' for work to be performed. If a call to"]
    #[doc = "grpc_call_start_batch returns any value other than GRPC_CALL_OK it is"]
    #[doc = "guaranteed that no state associated with 'call' is changed and it is not"]
    #[doc = "appropriate to call grpc_completion_queue_next or"]
    #[doc = "grpc_completion_queue_pluck consequent to the failed grpc_call_start_batch"]
    #[doc = "call."]
    #[doc = "If a call to grpc_call_start_batch with an empty batch returns"]
    #[doc = "GRPC_CALL_OK, the tag is put in the completion queue immediately."]
    #[doc = "THREAD SAFETY: access to grpc_call_start_batch in multi-threaded environment"]
    #[doc = "needs to be synchronized. As an optimization, you may synchronize batches"]
    #[doc = "containing just send operations independently from batches containing just"]
    #[doc = "receive operations. Access to grpc_call_start_batch with an empty batch is"]
    #[doc = "thread-compatible."]
    pub fn grpc_call_start_batch(
        call: *mut grpc_call,
        ops: *const grpc_op,
        nops: usize,
        tag: *mut ::std::os::raw::c_void,
        reserved: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    #[doc = " Returns a newly allocated string representing the endpoint to which this"]
    #[doc = "call is communicating with. The string is in the uri format accepted by"]
    #[doc = "grpc_channel_create."]
    #[doc = "The returned string should be disposed of with gpr_free()."]
    #[doc = ""]
    #[doc = "WARNING: this value is never authenticated or subject to any security"]
    #[doc = "related code. It must not be used for any authentication related"]
    #[doc = "functionality. Instead, use grpc_auth_context."]
    pub fn grpc_call_get_peer(call: *mut grpc_call) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct census_context {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Set census context for a call; Must be called before first call to"]
    #[doc = "grpc_call_start_batch()."]
    pub fn grpc_census_call_set_context(call: *mut grpc_call, context: *mut census_context);
}
extern "C" {
    #[doc = " Retrieve the calls current census context."]
    pub fn grpc_census_call_get_context(call: *mut grpc_call) -> *mut census_context;
}
extern "C" {
    #[doc = " Return a newly allocated string representing the target a channel was"]
    #[doc = "created for."]
    pub fn grpc_channel_get_target(channel: *mut grpc_channel) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Request info about the channel."]
    #[doc = "\\a channel_info indicates what information is being requested and"]
    #[doc = "how that information will be returned."]
    #[doc = "\\a channel_info is owned by the caller."]
    pub fn grpc_channel_get_info(
        channel: *mut grpc_channel,
        channel_info: *const grpc_channel_info,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL.  Resets the channel's connect backoff."]
    #[doc = "TODO(roth): When we see whether this proves useful, either promote"]
    #[doc = "to non-experimental or remove it."]
    pub fn grpc_channel_reset_connect_backoff(channel: *mut grpc_channel);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_channel_credentials {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Releases a channel credentials object."]
    #[doc = "The creator of the credentials object is responsible for its release."]
    pub fn grpc_channel_credentials_release(creds: *mut grpc_channel_credentials);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_server_credentials {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Releases a server_credentials object."]
    #[doc = "The creator of the server_credentials object is responsible for its release."]
    pub fn grpc_server_credentials_release(creds: *mut grpc_server_credentials);
}
extern "C" {
    #[doc = " Creates a secure channel using the passed-in credentials. Additional"]
    #[doc = "channel level configuration MAY be provided by grpc_channel_args, though"]
    #[doc = "the expectation is that most clients will want to simply pass NULL. The"]
    #[doc = "user data in 'args' need only live through the invocation of this function."]
    #[doc = "However, if any args of the 'pointer' type are passed, then the referenced"]
    #[doc = "vtable must be maintained by the caller until grpc_channel_destroy"]
    #[doc = "terminates. See grpc_channel_args definition for more on this."]
    pub fn grpc_channel_create(
        target: *const ::std::os::raw::c_char,
        creds: *mut grpc_channel_credentials,
        args: *const grpc_channel_args,
    ) -> *mut grpc_channel;
}
extern "C" {
    #[doc = " Create a lame client: this client fails every operation attempted on it."]
    pub fn grpc_lame_client_channel_create(
        target: *const ::std::os::raw::c_char,
        error_code: grpc_status_code::Type,
        error_message: *const ::std::os::raw::c_char,
    ) -> *mut grpc_channel;
}
extern "C" {
    #[doc = " Close and destroy a grpc channel"]
    pub fn grpc_channel_destroy(channel: *mut grpc_channel);
}
extern "C" {
    #[doc = " Cancel an RPC."]
    #[doc = "Can be called multiple times, from any thread."]
    #[doc = "THREAD-SAFETY grpc_call_cancel and grpc_call_cancel_with_status"]
    #[doc = "are thread-safe, and can be called at any point before grpc_call_unref"]
    #[doc = "is called."]
    pub fn grpc_call_cancel(
        call: *mut grpc_call,
        reserved: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    #[doc = " Cancel an RPC."]
    #[doc = "Can be called multiple times, from any thread."]
    #[doc = "If a status has not been received for the call, set it to the status code"]
    #[doc = "and description passed in."]
    #[doc = "Importantly, this function does not send status nor description to the"]
    #[doc = "remote endpoint."]
    #[doc = "Note that \\a description doesn't need be a static string."]
    #[doc = "It doesn't need to be alive after the call to"]
    #[doc = "grpc_call_cancel_with_status completes."]
    pub fn grpc_call_cancel_with_status(
        call: *mut grpc_call,
        status: grpc_status_code::Type,
        description: *const ::std::os::raw::c_char,
        reserved: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpc_call_failed_before_recv_message(c: *const grpc_call) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Ref a call."]
    #[doc = "THREAD SAFETY: grpc_call_ref is thread-compatible"]
    pub fn grpc_call_ref(call: *mut grpc_call);
}
extern "C" {
    #[doc = " Unref a call."]
    #[doc = "THREAD SAFETY: grpc_call_unref is thread-compatible"]
    pub fn grpc_call_unref(call: *mut grpc_call);
}
extern "C" {
    #[doc = " Request notification of a new call."]
    #[doc = "Once a call is received, a notification tagged with \\a tag_new is added to"]
    #[doc = "\\a cq_for_notification. \\a call, \\a details and \\a request_metadata are"]
    #[doc = "updated with the appropriate call information. \\a cq_bound_to_call is bound"]
    #[doc = "to \\a call, and batch operation notifications for that call will be posted"]
    #[doc = "to \\a cq_bound_to_call."]
    #[doc = "Note that \\a cq_for_notification must have been registered to the server via"]
    #[doc = "\\a grpc_server_register_completion_queue."]
    pub fn grpc_server_request_call(
        server: *mut grpc_server,
        call: *mut *mut grpc_call,
        details: *mut grpc_call_details,
        request_metadata: *mut grpc_metadata_array,
        cq_bound_to_call: *mut grpc_completion_queue,
        cq_for_notification: *mut grpc_completion_queue,
        tag_new: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
#[repr(u32)]
#[doc = " How to handle payloads for a registered method"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_server_register_method_payload_handling {
    #[doc = " Don't try to read the payload"]
    GRPC_SRM_PAYLOAD_NONE = 0,
    #[doc = " Read the initial payload as a byte buffer"]
    GRPC_SRM_PAYLOAD_READ_INITIAL_BYTE_BUFFER = 1,
}
extern "C" {
    #[doc = " Registers a method in the server."]
    #[doc = "Methods to this (host, method) pair will not be reported by"]
    #[doc = "grpc_server_request_call, but instead be reported by"]
    #[doc = "grpc_server_request_registered_call when passed the appropriate"]
    #[doc = "registered_method (as returned by this function)."]
    #[doc = "Must be called before grpc_server_start."]
    #[doc = "Returns NULL on failure."]
    pub fn grpc_server_register_method(
        server: *mut grpc_server,
        method: *const ::std::os::raw::c_char,
        host: *const ::std::os::raw::c_char,
        payload_handling: grpc_server_register_method_payload_handling,
        flags: u32,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Request notification of a new pre-registered call. 'cq_for_notification'"]
    #[doc = "must have been registered to the server via"]
    #[doc = "grpc_server_register_completion_queue."]
    pub fn grpc_server_request_registered_call(
        server: *mut grpc_server,
        registered_method: *mut ::std::os::raw::c_void,
        call: *mut *mut grpc_call,
        deadline: *mut gpr_timespec,
        request_metadata: *mut grpc_metadata_array,
        optional_payload: *mut *mut grpc_byte_buffer,
        cq_bound_to_call: *mut grpc_completion_queue,
        cq_for_notification: *mut grpc_completion_queue,
        tag_new: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    #[doc = " Create a server. Additional configuration for each incoming channel can"]
    #[doc = "be specified with args. If no additional configuration is needed, args can"]
    #[doc = "be NULL. The user data in 'args' need only live through the invocation of"]
    #[doc = "this function. However, if any args of the 'pointer' type are passed, then"]
    #[doc = "the referenced vtable must be maintained by the caller until"]
    #[doc = "grpc_server_destroy terminates. See grpc_channel_args definition for more"]
    #[doc = "on this."]
    pub fn grpc_server_create(
        args: *const grpc_channel_args,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_server;
}
extern "C" {
    #[doc = " Register a completion queue with the server. Must be done for any"]
    #[doc = "notification completion queue that is passed to grpc_server_request_*_call"]
    #[doc = "and to grpc_server_shutdown_and_notify. Must be performed prior to"]
    #[doc = "grpc_server_start."]
    pub fn grpc_server_register_completion_queue(
        server: *mut grpc_server,
        cq: *mut grpc_completion_queue,
        reserved: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_serving_status_update {
    pub code: grpc_status_code::Type,
    pub error_message: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_server_xds_status_notifier {
    pub on_serving_status_update: ::std::option::Option<
        unsafe extern "C" fn(
            user_data: *mut ::std::os::raw::c_void,
            uri: *const ::std::os::raw::c_char,
            update: grpc_serving_status_update,
        ),
    >,
    pub user_data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_server_config_fetcher {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " EXPERIMENTAL.  Creates an xDS config fetcher."]
    pub fn grpc_server_config_fetcher_xds_create(
        notifier: grpc_server_xds_status_notifier,
        args: *const grpc_channel_args,
    ) -> *mut grpc_server_config_fetcher;
}
extern "C" {
    #[doc = " EXPERIMENTAL.  Destroys a config fetcher."]
    pub fn grpc_server_config_fetcher_destroy(config_fetcher: *mut grpc_server_config_fetcher);
}
extern "C" {
    #[doc = " EXPERIMENTAL.  Sets the server's config fetcher.  Takes ownership."]
    #[doc = "Must be called before adding ports"]
    pub fn grpc_server_set_config_fetcher(
        server: *mut grpc_server,
        config_fetcher: *mut grpc_server_config_fetcher,
    );
}
extern "C" {
    #[doc = " Add a HTTP2 over an encrypted link over tcp listener."]
    #[doc = "Returns bound port number on success, 0 on failure."]
    #[doc = "REQUIRES: server not started"]
    pub fn grpc_server_add_http2_port(
        server: *mut grpc_server,
        addr: *const ::std::os::raw::c_char,
        creds: *mut grpc_server_credentials,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Start a server - tells all listeners to start listening"]
    pub fn grpc_server_start(server: *mut grpc_server);
}
extern "C" {
    #[doc = " Begin shutting down a server."]
    #[doc = "After completion, no new calls or connections will be admitted."]
    #[doc = "Existing calls will be allowed to complete."]
    #[doc = "Send a GRPC_OP_COMPLETE event when there are no more calls being serviced."]
    #[doc = "Shutdown is idempotent, and all tags will be notified at once if multiple"]
    #[doc = "grpc_server_shutdown_and_notify calls are made. 'cq' must have been"]
    #[doc = "registered to this server via grpc_server_register_completion_queue."]
    pub fn grpc_server_shutdown_and_notify(
        server: *mut grpc_server,
        cq: *mut grpc_completion_queue,
        tag: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Cancel all in-progress calls."]
    #[doc = "Only usable after shutdown."]
    pub fn grpc_server_cancel_all_calls(server: *mut grpc_server);
}
extern "C" {
    #[doc = " Destroy a server."]
    #[doc = "Shutdown must have completed beforehand (i.e. all tags generated by"]
    #[doc = "grpc_server_shutdown_and_notify must have been received, and at least"]
    #[doc = "one call to grpc_server_shutdown_and_notify must have been made)."]
    pub fn grpc_server_destroy(server: *mut grpc_server);
}
extern "C" {
    #[doc = " Enable or disable a tracer."]
    #[doc = ""]
    #[doc = "Tracers (usually controlled by the environment variable GRPC_TRACE)"]
    #[doc = "allow printf-style debugging on GRPC internals, and are useful for"]
    #[doc = "tracking down problems in the field."]
    #[doc = ""]
    #[doc = "Use of this function is not strictly thread-safe, but the"]
    #[doc = "thread-safety issues raised by it should not be of concern."]
    pub fn grpc_tracer_set_enabled(
        name: *const ::std::os::raw::c_char,
        enabled: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Check whether a metadata key is legal (will be accepted by core)"]
    pub fn grpc_header_key_is_legal(slice: grpc_slice) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Check whether a non-binary metadata value is legal (will be accepted by"]
    #[doc = "core)"]
    pub fn grpc_header_nonbin_value_is_legal(slice: grpc_slice) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Check whether a metadata key corresponds to a binary value"]
    pub fn grpc_is_binary_header(slice: grpc_slice) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Convert grpc_call_error values to a string"]
    pub fn grpc_call_error_to_string(error: grpc_call_error) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Create a buffer pool"]
    pub fn grpc_resource_quota_create(
        trace_name: *const ::std::os::raw::c_char,
    ) -> *mut grpc_resource_quota;
}
extern "C" {
    #[doc = " Add a reference to a buffer pool"]
    pub fn grpc_resource_quota_ref(resource_quota: *mut grpc_resource_quota);
}
extern "C" {
    #[doc = " Drop a reference to a buffer pool"]
    pub fn grpc_resource_quota_unref(resource_quota: *mut grpc_resource_quota);
}
extern "C" {
    #[doc = " Update the size of a buffer pool"]
    pub fn grpc_resource_quota_resize(resource_quota: *mut grpc_resource_quota, new_size: usize);
}
extern "C" {
    #[doc = " Update the size of the maximum number of threads allowed"]
    pub fn grpc_resource_quota_set_max_threads(
        resource_quota: *mut grpc_resource_quota,
        new_max_threads: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL.  Dumps xDS configs as a serialized ClientConfig proto."]
    #[doc = "The full name of the proto is envoy.service.status.v3.ClientConfig."]
    pub fn grpc_dump_xds_configs() -> grpc_slice;
}
extern "C" {
    #[doc = " Fetch a vtable for a grpc_channel_arg that points to a grpc_resource_quota"]
    pub fn grpc_resource_quota_arg_vtable() -> *const grpc_arg_pointer_vtable;
}
extern "C" {
    #[doc = " CHANNELZ API *************/"]
    #[doc = "churn as the feature is implemented. This comment will be removed once"]
    #[doc = "channelz is officially supported, and these APIs become stable. For now"]
    #[doc = "you may track the progress by following this github issue:"]
    #[doc = "https://github.com/grpc/grpc/issues/15340"]
    #[doc = ""]
    #[doc = "the following APIs return allocated JSON strings that match the response"]
    #[doc = "objects from the channelz proto, found here:"]
    #[doc = "https://github.com/grpc/grpc/blob/master/src/proto/grpc/channelz/channelz.proto."]
    #[doc = ""]
    #[doc = "For easy conversion to protobuf, The JSON is formatted according to:"]
    #[doc = "https://developers.google.com/protocol-buffers/docs/proto3#json."]
    pub fn grpc_channelz_get_top_channels(start_channel_id: isize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpc_channelz_get_servers(start_server_id: isize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpc_channelz_get_server(server_id: isize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpc_channelz_get_server_sockets(
        server_id: isize,
        start_socket_id: isize,
        max_results: isize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpc_channelz_get_channel(channel_id: isize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpc_channelz_get_subchannel(subchannel_id: isize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpc_channelz_get_socket(socket_id: isize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " EXPERIMENTAL - Subject to change."]
    #[doc = " Fetch a vtable for grpc_channel_arg that points to"]
    #[doc = " grpc_authorization_policy_provider."]
    pub fn grpc_authorization_policy_provider_arg_vtable() -> *const grpc_arg_pointer_vtable;
}
extern "C" {
    pub fn grpc_cronet_secure_channel_create(
        engine: *mut ::std::os::raw::c_void,
        target: *const ::std::os::raw::c_char,
        args: *const grpc_channel_args,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_channel;
}
extern "C" {
    #[doc = " Create a secure channel to 'target' using file descriptor 'fd' and passed-in"]
    #[doc = "credentials. The 'target' argument will be used to indicate the name for"]
    #[doc = "this channel. Note that this API currently only supports insecure channel"]
    #[doc = "credentials. Using other types of credentials will result in a failure."]
    pub fn grpc_channel_create_from_fd(
        target: *const ::std::os::raw::c_char,
        fd: ::std::os::raw::c_int,
        creds: *mut grpc_channel_credentials,
        args: *const grpc_channel_args,
    ) -> *mut grpc_channel;
}
extern "C" {
    #[doc = " Add the connected secure communication channel based on file descriptor 'fd'"]
    #[doc = "to the 'server' and server credentials 'creds'. The 'fd' must be an open file"]
    #[doc = "descriptor corresponding to a connected socket. Events from the file"]
    #[doc = "descriptor may come on any of the server completion queues (i.e completion"]
    #[doc = "queues registered via the grpc_server_register_completion_queue API)."]
    #[doc = "Note that this API currently only supports inseure server credentials"]
    #[doc = "Using other types of credentials will result in a failure."]
    #[doc = "TODO(hork): add channel_args to this API to allow endpoints and transports"]
    #[doc = "created in this function to participate in the resource quota feature."]
    pub fn grpc_server_add_channel_from_fd(
        server: *mut grpc_server,
        fd: ::std::os::raw::c_int,
        creds: *mut grpc_server_credentials,
    );
}
#[repr(u32)]
#[doc = " Results for the SSL roots override callback."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_ssl_roots_override_result {
    GRPC_SSL_ROOTS_OVERRIDE_OK = 0,
    GRPC_SSL_ROOTS_OVERRIDE_FAIL_PERMANENTLY = 1,
    #[doc = " Do not try fallback options."]
    GRPC_SSL_ROOTS_OVERRIDE_FAIL = 2,
}
#[repr(u32)]
#[doc = " Callback results for dynamically loading a SSL certificate config."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_ssl_certificate_config_reload_status {
    GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_UNCHANGED = 0,
    GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_NEW = 1,
    GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_FAIL = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_ssl_client_certificate_request_type {
    #[doc = " Server does not request client certificate."]
    #[doc = "The certificate presented by the client is not checked by the server at"]
    #[doc = "all. (A client may present a self signed or signed certificate or not"]
    #[doc = "present a certificate at all and any of those option would be accepted)"]
    GRPC_SSL_DONT_REQUEST_CLIENT_CERTIFICATE = 0,
    #[doc = " Server requests client certificate but does not enforce that the client"]
    #[doc = "presents a certificate."]
    #[doc = ""]
    #[doc = "If the client presents a certificate, the client authentication is left to"]
    #[doc = "the application (the necessary metadata will be available to the"]
    #[doc = "application via authentication context properties, see grpc_auth_context)."]
    #[doc = ""]
    #[doc = "The client's key certificate pair must be valid for the SSL connection to"]
    #[doc = "be established."]
    GRPC_SSL_REQUEST_CLIENT_CERTIFICATE_BUT_DONT_VERIFY = 1,
    #[doc = " Server requests client certificate but does not enforce that the client"]
    #[doc = "presents a certificate."]
    #[doc = ""]
    #[doc = "If the client presents a certificate, the client authentication is done by"]
    #[doc = "the gRPC framework. (For a successful connection the client needs to either"]
    #[doc = "present a certificate that can be verified against the root certificate"]
    #[doc = "configured by the server or not present a certificate at all)"]
    #[doc = ""]
    #[doc = "The client's key certificate pair must be valid for the SSL connection to"]
    #[doc = "be established."]
    GRPC_SSL_REQUEST_CLIENT_CERTIFICATE_AND_VERIFY = 2,
    #[doc = " Server requests client certificate and enforces that the client presents a"]
    #[doc = "certificate."]
    #[doc = ""]
    #[doc = "If the client presents a certificate, the client authentication is left to"]
    #[doc = "the application (the necessary metadata will be available to the"]
    #[doc = "application via authentication context properties, see grpc_auth_context)."]
    #[doc = ""]
    #[doc = "The client's key certificate pair must be valid for the SSL connection to"]
    #[doc = "be established."]
    GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_BUT_DONT_VERIFY = 3,
    #[doc = " Server requests client certificate and enforces that the client presents a"]
    #[doc = "certificate."]
    #[doc = ""]
    #[doc = "The certificate presented by the client is verified by the gRPC framework."]
    #[doc = "(For a successful connection the client needs to present a certificate that"]
    #[doc = "can be verified against the root certificate configured by the server)"]
    #[doc = ""]
    #[doc = "The client's key certificate pair must be valid for the SSL connection to"]
    #[doc = "be established."]
    GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY = 4,
}
impl grpc_security_level {
    pub const GRPC_SECURITY_NONE: grpc_security_level = grpc_security_level::GRPC_SECURITY_MIN;
}
impl grpc_security_level {
    pub const GRPC_SECURITY_MAX: grpc_security_level =
        grpc_security_level::GRPC_PRIVACY_AND_INTEGRITY;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_security_level {
    GRPC_SECURITY_MIN = 0,
    GRPC_INTEGRITY_ONLY = 1,
    GRPC_PRIVACY_AND_INTEGRITY = 2,
}
#[repr(u32)]
#[doc = " Type of local connections for which local channel/server credentials will be"]
#[doc = " applied. It supports UDS and local TCP connections."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_local_connect_type {
    UDS = 0,
    LOCAL_TCP = 1,
}
#[repr(u32)]
#[doc = " The TLS versions that are supported by the SSL stack."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpc_tls_version {
    TLS1_2 = 0,
    TLS1_3 = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_auth_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_auth_property_iterator {
    pub ctx: *const grpc_auth_context,
    pub index: usize,
    pub name: *const ::std::os::raw::c_char,
}
#[doc = " value, if not NULL, is guaranteed to be NULL terminated."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_auth_property {
    pub name: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
    pub value_length: usize,
}
extern "C" {
    #[doc = " Returns NULL when the iterator is at the end."]
    pub fn grpc_auth_property_iterator_next(
        it: *mut grpc_auth_property_iterator,
    ) -> *const grpc_auth_property;
}
extern "C" {
    #[doc = " Iterates over the auth context."]
    pub fn grpc_auth_context_property_iterator(
        ctx: *const grpc_auth_context,
    ) -> grpc_auth_property_iterator;
}
extern "C" {
    #[doc = " Gets the peer identity. Returns an empty iterator (first _next will return"]
    #[doc = "NULL) if the peer is not authenticated."]
    pub fn grpc_auth_context_peer_identity(
        ctx: *const grpc_auth_context,
    ) -> grpc_auth_property_iterator;
}
extern "C" {
    #[doc = " Finds a property in the context. May return an empty iterator (first _next"]
    #[doc = "will return NULL) if no property with this name was found in the context."]
    pub fn grpc_auth_context_find_properties_by_name(
        ctx: *const grpc_auth_context,
        name: *const ::std::os::raw::c_char,
    ) -> grpc_auth_property_iterator;
}
extern "C" {
    #[doc = " Gets the name of the property that indicates the peer identity. Will return"]
    #[doc = "NULL if the peer is not authenticated."]
    pub fn grpc_auth_context_peer_identity_property_name(
        ctx: *const grpc_auth_context,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Returns 1 if the peer is authenticated, 0 otherwise."]
    pub fn grpc_auth_context_peer_is_authenticated(
        ctx: *const grpc_auth_context,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Gets the auth context from the call. Caller needs to call"]
    #[doc = "grpc_auth_context_release on the returned context."]
    pub fn grpc_call_auth_context(call: *mut grpc_call) -> *mut grpc_auth_context;
}
extern "C" {
    #[doc = " Releases the auth context returned from grpc_call_auth_context."]
    pub fn grpc_auth_context_release(context: *mut grpc_auth_context);
}
extern "C" {
    #[doc = " Add a property."]
    pub fn grpc_auth_context_add_property(
        ctx: *mut grpc_auth_context,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
        value_length: usize,
    );
}
extern "C" {
    #[doc = " Add a C string property."]
    pub fn grpc_auth_context_add_cstring_property(
        ctx: *mut grpc_auth_context,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Sets the property name. Returns 1 if successful or 0 in case of failure"]
    #[doc = "(which means that no property with this name exists)."]
    pub fn grpc_auth_context_set_peer_identity_property_name(
        ctx: *mut grpc_auth_context,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_ssl_session_cache {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Create LRU cache for client-side SSL sessions with the given capacity."]
    #[doc = "If capacity is < 1, a default capacity is used instead."]
    pub fn grpc_ssl_session_cache_create_lru(capacity: usize) -> *mut grpc_ssl_session_cache;
}
extern "C" {
    #[doc = " Destroy SSL session cache."]
    pub fn grpc_ssl_session_cache_destroy(cache: *mut grpc_ssl_session_cache);
}
extern "C" {
    #[doc = " Create a channel arg with the given cache object."]
    pub fn grpc_ssl_session_cache_create_channel_arg(
        cache: *mut grpc_ssl_session_cache,
    ) -> grpc_arg;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_call_credentials {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Releases a call credentials object."]
    #[doc = "The creator of the credentials object is responsible for its release."]
    pub fn grpc_call_credentials_release(creds: *mut grpc_call_credentials);
}
extern "C" {
    #[doc = " Creates default credentials to connect to a google gRPC service."]
    #[doc = "WARNING: Do NOT use this credentials to connect to a non-google service as"]
    #[doc = "this could result in an oauth2 token leak. The security level of the"]
    #[doc = "resulting connection is GRPC_PRIVACY_AND_INTEGRITY."]
    #[doc = ""]
    #[doc = "If specified, the supplied call credentials object will be attached to the"]
    #[doc = "returned channel credentials object. The call_credentials object must remain"]
    #[doc = "valid throughout the lifetime of the returned grpc_channel_credentials"]
    #[doc = "object. It is expected that the call credentials object was generated"]
    #[doc = "according to the Application Default Credentials mechanism and asserts the"]
    #[doc = "identity of the default service account of the machine. Supplying any other"]
    #[doc = "sort of call credential will result in undefined behavior, up to and"]
    #[doc = "including the sudden and unexpected failure of RPCs."]
    #[doc = ""]
    #[doc = "If nullptr is supplied, the returned channel credentials object will use a"]
    #[doc = "call credentials object based on the Application Default Credentials"]
    #[doc = "mechanism."]
    pub fn grpc_google_default_credentials_create(
        call_credentials: *mut grpc_call_credentials,
    ) -> *mut grpc_channel_credentials;
}
#[doc = " Callback for getting the SSL roots override from the application."]
#[doc = "In case of success, *pem_roots_certs must be set to a NULL terminated string"]
#[doc = "containing the list of PEM encoded root certificates. The ownership is passed"]
#[doc = "to the core and freed (laster by the core) with gpr_free."]
#[doc = "If this function fails and GRPC_DEFAULT_SSL_ROOTS_FILE_PATH environment is"]
#[doc = "set to a valid path, it will override the roots specified this func"]
pub type grpc_ssl_roots_override_callback = ::std::option::Option<
    unsafe extern "C" fn(
        pem_root_certs: *mut *mut ::std::os::raw::c_char,
    ) -> grpc_ssl_roots_override_result,
>;
extern "C" {
    #[doc = " Setup a callback to override the default TLS/SSL roots."]
    #[doc = "This function is not thread-safe and must be called at initialization time"]
    #[doc = "before any ssl credentials are created to have the desired side effect."]
    #[doc = "If GRPC_DEFAULT_SSL_ROOTS_FILE_PATH environment is set to a valid path, the"]
    #[doc = "callback will not be called."]
    pub fn grpc_set_ssl_roots_override_callback(cb: grpc_ssl_roots_override_callback);
}
#[doc = " Object that holds a private key / certificate chain pair in PEM format."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_ssl_pem_key_cert_pair {
    #[doc = " private_key is the NULL-terminated string containing the PEM encoding of"]
    #[doc = "the client's private key."]
    pub private_key: *const ::std::os::raw::c_char,
    #[doc = " cert_chain is the NULL-terminated string containing the PEM encoding of"]
    #[doc = "the client's certificate chain."]
    pub cert_chain: *const ::std::os::raw::c_char,
}
#[doc = " Deprecated in favor of grpc_ssl_verify_peer_options. It will be removed"]
#[doc = "after all of its call sites are migrated to grpc_ssl_verify_peer_options."]
#[doc = "Object that holds additional peer-verification options on a secure"]
#[doc = "channel."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct verify_peer_options {
    #[doc = " If non-NULL this callback will be invoked with the expected"]
    #[doc = "target_name, the peer's certificate (in PEM format), and whatever"]
    #[doc = "userdata pointer is set below. If a non-zero value is returned by this"]
    #[doc = "callback then it is treated as a verification failure. Invocation of"]
    #[doc = "the callback is blocking, so any implementation should be light-weight."]
    pub verify_peer_callback: ::std::option::Option<
        unsafe extern "C" fn(
            target_name: *const ::std::os::raw::c_char,
            peer_pem: *const ::std::os::raw::c_char,
            userdata: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    #[doc = " Arbitrary userdata that will be passed as the last argument to"]
    #[doc = "verify_peer_callback."]
    pub verify_peer_callback_userdata: *mut ::std::os::raw::c_void,
    #[doc = " A destruct callback that will be invoked when the channel is being"]
    #[doc = "cleaned up. The userdata argument will be passed to it. The intent is"]
    #[doc = "to perform any cleanup associated with that userdata."]
    pub verify_peer_destruct:
        ::std::option::Option<unsafe extern "C" fn(userdata: *mut ::std::os::raw::c_void)>,
}
#[doc = " Object that holds additional peer-verification options on a secure"]
#[doc = "channel."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_ssl_verify_peer_options {
    #[doc = " If non-NULL this callback will be invoked with the expected"]
    #[doc = "target_name, the peer's certificate (in PEM format), and whatever"]
    #[doc = "userdata pointer is set below. If a non-zero value is returned by this"]
    #[doc = "callback then it is treated as a verification failure. Invocation of"]
    #[doc = "the callback is blocking, so any implementation should be light-weight."]
    pub verify_peer_callback: ::std::option::Option<
        unsafe extern "C" fn(
            target_name: *const ::std::os::raw::c_char,
            peer_pem: *const ::std::os::raw::c_char,
            userdata: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    #[doc = " Arbitrary userdata that will be passed as the last argument to"]
    #[doc = "verify_peer_callback."]
    pub verify_peer_callback_userdata: *mut ::std::os::raw::c_void,
    #[doc = " A destruct callback that will be invoked when the channel is being"]
    #[doc = "cleaned up. The userdata argument will be passed to it. The intent is"]
    #[doc = "to perform any cleanup associated with that userdata."]
    pub verify_peer_destruct:
        ::std::option::Option<unsafe extern "C" fn(userdata: *mut ::std::os::raw::c_void)>,
}
extern "C" {
    #[doc = " Deprecated in favor of grpc_ssl_server_credentials_create_ex. It will be"]
    #[doc = "removed after all of its call sites are migrated to"]
    #[doc = "grpc_ssl_server_credentials_create_ex. Creates an SSL credentials object."]
    #[doc = "The security level of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY."]
    #[doc = "- pem_root_certs is the NULL-terminated string containing the PEM encoding"]
    #[doc = "of the server root certificates. If this parameter is NULL, the"]
    #[doc = "implementation will first try to dereference the file pointed by the"]
    #[doc = "GRPC_DEFAULT_SSL_ROOTS_FILE_PATH environment variable, and if that fails,"]
    #[doc = "try to get the roots set by grpc_override_ssl_default_roots. Eventually,"]
    #[doc = "if all these fail, it will try to get the roots from a well-known place on"]
    #[doc = "disk (in the grpc install directory)."]
    #[doc = ""]
    #[doc = "gRPC has implemented root cache if the underlying OpenSSL library supports"]
    #[doc = "it. The gRPC root certificates cache is only applicable on the default"]
    #[doc = "root certificates, which is used when this parameter is nullptr. If user"]
    #[doc = "provides their own pem_root_certs, when creating an SSL credential object,"]
    #[doc = "gRPC would not be able to cache it, and each subchannel will generate a"]
    #[doc = "copy of the root store. So it is recommended to avoid providing large room"]
    #[doc = "pem with pem_root_certs parameter to avoid excessive memory consumption,"]
    #[doc = "particularly on mobile platforms such as iOS."]
    #[doc = "- pem_key_cert_pair is a pointer on the object containing client's private"]
    #[doc = "key and certificate chain. This parameter can be NULL if the client does"]
    #[doc = "not have such a key/cert pair."]
    #[doc = "- verify_options is an optional verify_peer_options object which holds"]
    #[doc = "additional options controlling how peer certificates are verified. For"]
    #[doc = "example, you can supply a callback which receives the peer's certificate"]
    #[doc = "with which you can do additional verification. Can be NULL, in which"]
    #[doc = "case verification will retain default behavior. Any settings in"]
    #[doc = "verify_options are copied during this call, so the verify_options"]
    #[doc = "object can be released afterwards."]
    pub fn grpc_ssl_credentials_create(
        pem_root_certs: *const ::std::os::raw::c_char,
        pem_key_cert_pair: *mut grpc_ssl_pem_key_cert_pair,
        verify_options: *const verify_peer_options,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_channel_credentials;
}
extern "C" {
    pub fn grpc_ssl_credentials_create_ex(
        pem_root_certs: *const ::std::os::raw::c_char,
        pem_key_cert_pair: *mut grpc_ssl_pem_key_cert_pair,
        verify_options: *const grpc_ssl_verify_peer_options,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_channel_credentials;
}
extern "C" {
    #[doc = " Creates a composite channel credentials object. The security level of"]
    #[doc = " resulting connection is determined by channel_creds."]
    pub fn grpc_composite_channel_credentials_create(
        channel_creds: *mut grpc_channel_credentials,
        call_creds: *mut grpc_call_credentials,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_channel_credentials;
}
extern "C" {
    #[doc = " Creates a composite call credentials object."]
    pub fn grpc_composite_call_credentials_create(
        creds1: *mut grpc_call_credentials,
        creds2: *mut grpc_call_credentials,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call_credentials;
}
extern "C" {
    #[doc = " Creates a compute engine credentials object for connecting to Google."]
    #[doc = "WARNING: Do NOT use this credentials to connect to a non-google service as"]
    #[doc = "this could result in an oauth2 token leak."]
    pub fn grpc_google_compute_engine_credentials_create(
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call_credentials;
}
extern "C" {
    pub fn grpc_max_auth_token_lifetime() -> gpr_timespec;
}
extern "C" {
    #[doc = " Creates a JWT credentials object. May return NULL if the input is invalid."]
    #[doc = "- json_key is the JSON key string containing the client's private key."]
    #[doc = "- token_lifetime is the lifetime of each Json Web Token (JWT) created with"]
    #[doc = "this credentials.  It should not exceed grpc_max_auth_token_lifetime or"]
    #[doc = "will be cropped to this value."]
    pub fn grpc_service_account_jwt_access_credentials_create(
        json_key: *const ::std::os::raw::c_char,
        token_lifetime: gpr_timespec,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call_credentials;
}
extern "C" {
    #[doc = " Builds External Account credentials."]
    #[doc = "- json_string is the JSON string containing the credentials options."]
    #[doc = "- scopes_string contains the scopes to be binded with the credentials."]
    #[doc = "This API is used for experimental purposes for now and may change in the"]
    #[doc = "future."]
    pub fn grpc_external_account_credentials_create(
        json_string: *const ::std::os::raw::c_char,
        scopes_string: *const ::std::os::raw::c_char,
    ) -> *mut grpc_call_credentials;
}
extern "C" {
    #[doc = " Creates an Oauth2 Refresh Token credentials object for connecting to Google."]
    #[doc = "May return NULL if the input is invalid."]
    #[doc = "WARNING: Do NOT use this credentials to connect to a non-google service as"]
    #[doc = "this could result in an oauth2 token leak."]
    #[doc = "- json_refresh_token is the JSON string containing the refresh token itself"]
    #[doc = "along with a client_id and client_secret."]
    pub fn grpc_google_refresh_token_credentials_create(
        json_refresh_token: *const ::std::os::raw::c_char,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call_credentials;
}
extern "C" {
    #[doc = " Creates an Oauth2 Access Token credentials with an access token that was"]
    #[doc = "acquired by an out of band mechanism."]
    pub fn grpc_access_token_credentials_create(
        access_token: *const ::std::os::raw::c_char,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call_credentials;
}
extern "C" {
    #[doc = " Creates an IAM credentials object for connecting to Google."]
    pub fn grpc_google_iam_credentials_create(
        authorization_token: *const ::std::os::raw::c_char,
        authority_selector: *const ::std::os::raw::c_char,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call_credentials;
}
#[doc = " Options for creating STS Oauth Token Exchange credentials following the IETF"]
#[doc = "draft https://tools.ietf.org/html/draft-ietf-oauth-token-exchange-16."]
#[doc = "Optional fields may be set to NULL or empty string. It is the responsibility"]
#[doc = "of the caller to ensure that the subject and actor tokens are refreshed on"]
#[doc = "disk at the specified paths. This API is used for experimental purposes for"]
#[doc = "now and may change in the future."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_sts_credentials_options {
    pub token_exchange_service_uri: *const ::std::os::raw::c_char,
    pub resource: *const ::std::os::raw::c_char,
    pub audience: *const ::std::os::raw::c_char,
    pub scope: *const ::std::os::raw::c_char,
    pub requested_token_type: *const ::std::os::raw::c_char,
    pub subject_token_path: *const ::std::os::raw::c_char,
    pub subject_token_type: *const ::std::os::raw::c_char,
    pub actor_token_path: *const ::std::os::raw::c_char,
    pub actor_token_type: *const ::std::os::raw::c_char,
}
extern "C" {
    #[doc = " Creates an STS credentials following the STS Token Exchanged specifed in the"]
    #[doc = "IETF draft https://tools.ietf.org/html/draft-ietf-oauth-token-exchange-16."]
    #[doc = "This API is used for experimental purposes for now and may change in the"]
    #[doc = "future."]
    pub fn grpc_sts_credentials_create(
        options: *const grpc_sts_credentials_options,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call_credentials;
}
#[doc = " Callback function to be called by the metadata credentials plugin"]
#[doc = "implementation when the metadata is ready."]
#[doc = "- user_data is the opaque pointer that was passed in the get_metadata method"]
#[doc = "of the grpc_metadata_credentials_plugin (see below)."]
#[doc = "- creds_md is an array of credentials metadata produced by the plugin. It"]
#[doc = "may be set to NULL in case of an error."]
#[doc = "- num_creds_md is the number of items in the creds_md array."]
#[doc = "- status must be GRPC_STATUS_OK in case of success or another specific error"]
#[doc = "code otherwise."]
#[doc = "- error_details contains details about the error if any. In case of success"]
#[doc = "it should be NULL and will be otherwise ignored."]
pub type grpc_credentials_plugin_metadata_cb = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        creds_md: *const grpc_metadata,
        num_creds_md: usize,
        status: grpc_status_code::Type,
        error_details: *const ::std::os::raw::c_char,
    ),
>;
#[doc = " Context that can be used by metadata credentials plugin in order to create"]
#[doc = "auth related metadata."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_auth_metadata_context {
    #[doc = " The fully qualifed service url."]
    pub service_url: *const ::std::os::raw::c_char,
    #[doc = " The method name of the RPC being called (not fully qualified)."]
    #[doc = "The fully qualified method name can be built from the service_url:"]
    #[doc = "full_qualified_method_name = ctx->service_url + '/' + ctx->method_name."]
    pub method_name: *const ::std::os::raw::c_char,
    #[doc = " The auth_context of the channel which gives the server's identity."]
    pub channel_auth_context: *const grpc_auth_context,
    #[doc = " Reserved for future use."]
    pub reserved: *mut ::std::os::raw::c_void,
}
extern "C" {
    #[doc = " Performs a deep copy from \\a from to \\a to."]
    pub fn grpc_auth_metadata_context_copy(
        from: *mut grpc_auth_metadata_context,
        to: *mut grpc_auth_metadata_context,
    );
}
extern "C" {
    #[doc = " Releases internal resources held by \\a context."]
    pub fn grpc_auth_metadata_context_reset(context: *mut grpc_auth_metadata_context);
}
#[doc = " grpc_metadata_credentials plugin is an API user provided structure used to"]
#[doc = "create grpc_credentials objects that can be set on a channel (composed) or"]
#[doc = "a call. See grpc_credentials_metadata_create_from_plugin below."]
#[doc = "The grpc client stack will call the get_metadata method of the plugin for"]
#[doc = "every call in scope for the credentials created from it."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_metadata_credentials_plugin {
    #[doc = " The implementation of this method has to be non-blocking, but can"]
    #[doc = "be performed synchronously or asynchronously."]
    #[doc = ""]
    #[doc = "If processing occurs synchronously, returns non-zero and populates"]
    #[doc = "creds_md, num_creds_md, status, and error_details.  In this case,"]
    #[doc = "the caller takes ownership of the entries in creds_md and of"]
    #[doc = "error_details.  Note that if the plugin needs to return more than"]
    #[doc = "GRPC_METADATA_CREDENTIALS_PLUGIN_SYNC_MAX entries in creds_md, it must"]
    #[doc = "return asynchronously."]
    #[doc = ""]
    #[doc = "If processing occurs asynchronously, returns zero and invokes \\a cb"]
    #[doc = "when processing is completed.  \\a user_data will be passed as the"]
    #[doc = "first parameter of the callback.  NOTE: \\a cb MUST be invoked in a"]
    #[doc = "different thread, not from the thread in which \\a get_metadata() is"]
    #[doc = "invoked."]
    #[doc = ""]
    #[doc = "\\a context is the information that can be used by the plugin to create"]
    #[doc = "auth metadata."]
    pub get_metadata: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            context: grpc_auth_metadata_context,
            cb: grpc_credentials_plugin_metadata_cb,
            user_data: *mut ::std::os::raw::c_void,
            creds_md: *mut grpc_metadata,
            num_creds_md: *mut usize,
            status: *mut grpc_status_code::Type,
            error_details: *mut *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    #[doc = " Implements debug string of the given plugin. This method returns an"]
    #[doc = " allocated string that the caller needs to free using gpr_free()"]
    pub debug_string: ::std::option::Option<
        unsafe extern "C" fn(state: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_char,
    >,
    #[doc = " Destroys the plugin state."]
    pub destroy: ::std::option::Option<unsafe extern "C" fn(state: *mut ::std::os::raw::c_void)>,
    #[doc = " State that will be set as the first parameter of the methods above."]
    pub state: *mut ::std::os::raw::c_void,
    #[doc = " Type of credentials that this plugin is implementing."]
    pub type_: *const ::std::os::raw::c_char,
}
extern "C" {
    #[doc = " Creates a credentials object from a plugin with a specified minimum security"]
    #[doc = " level."]
    pub fn grpc_metadata_credentials_create_from_plugin(
        plugin: grpc_metadata_credentials_plugin,
        min_security_level: grpc_security_level,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_call_credentials;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_ssl_server_certificate_config {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Creates a grpc_ssl_server_certificate_config object."]
    #[doc = "- pem_roots_cert is the NULL-terminated string containing the PEM encoding of"]
    #[doc = "the client root certificates. This parameter may be NULL if the server does"]
    #[doc = "not want the client to be authenticated with SSL."]
    #[doc = "- pem_key_cert_pairs is an array private key / certificate chains of the"]
    #[doc = "server. This parameter cannot be NULL."]
    #[doc = "- num_key_cert_pairs indicates the number of items in the private_key_files"]
    #[doc = "and cert_chain_files parameters. It must be at least 1."]
    #[doc = "- It is the caller's responsibility to free this object via"]
    #[doc = "grpc_ssl_server_certificate_config_destroy()."]
    pub fn grpc_ssl_server_certificate_config_create(
        pem_root_certs: *const ::std::os::raw::c_char,
        pem_key_cert_pairs: *const grpc_ssl_pem_key_cert_pair,
        num_key_cert_pairs: usize,
    ) -> *mut grpc_ssl_server_certificate_config;
}
extern "C" {
    #[doc = " Destroys a grpc_ssl_server_certificate_config object."]
    pub fn grpc_ssl_server_certificate_config_destroy(
        config: *mut grpc_ssl_server_certificate_config,
    );
}
#[doc = " Callback to retrieve updated SSL server certificates, private keys, and"]
#[doc = "trusted CAs (for client authentication)."]
#[doc = "- user_data parameter, if not NULL, contains opaque data to be used by the"]
#[doc = "callback."]
#[doc = "- Use grpc_ssl_server_certificate_config_create to create the config."]
#[doc = "- The caller assumes ownership of the config."]
pub type grpc_ssl_server_certificate_config_callback = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        config: *mut *mut grpc_ssl_server_certificate_config,
    ) -> grpc_ssl_certificate_config_reload_status,
>;
extern "C" {
    #[doc = " Deprecated in favor of grpc_ssl_server_credentials_create_ex."]
    #[doc = "Creates an SSL server_credentials object."]
    #[doc = "- pem_roots_cert is the NULL-terminated string containing the PEM encoding of"]
    #[doc = "the client root certificates. This parameter may be NULL if the server does"]
    #[doc = "not want the client to be authenticated with SSL."]
    #[doc = "- pem_key_cert_pairs is an array private key / certificate chains of the"]
    #[doc = "server. This parameter cannot be NULL."]
    #[doc = "- num_key_cert_pairs indicates the number of items in the private_key_files"]
    #[doc = "and cert_chain_files parameters. It should be at least 1."]
    #[doc = "- force_client_auth, if set to non-zero will force the client to authenticate"]
    #[doc = "with an SSL cert. Note that this option is ignored if pem_root_certs is"]
    #[doc = "NULL."]
    pub fn grpc_ssl_server_credentials_create(
        pem_root_certs: *const ::std::os::raw::c_char,
        pem_key_cert_pairs: *mut grpc_ssl_pem_key_cert_pair,
        num_key_cert_pairs: usize,
        force_client_auth: ::std::os::raw::c_int,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_server_credentials;
}
extern "C" {
    #[doc = " Deprecated in favor of grpc_ssl_server_credentials_create_with_options."]
    #[doc = "Same as grpc_ssl_server_credentials_create method except uses"]
    #[doc = "grpc_ssl_client_certificate_request_type enum to support more ways to"]
    #[doc = "authenticate client certificates."]
    pub fn grpc_ssl_server_credentials_create_ex(
        pem_root_certs: *const ::std::os::raw::c_char,
        pem_key_cert_pairs: *mut grpc_ssl_pem_key_cert_pair,
        num_key_cert_pairs: usize,
        client_certificate_request: grpc_ssl_client_certificate_request_type,
        reserved: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_server_credentials;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_ssl_server_credentials_options {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Creates an options object using a certificate config. Use this method when"]
    #[doc = "the certificates and keys of the SSL server will not change during the"]
    #[doc = "server's lifetime."]
    #[doc = "- Takes ownership of the certificate_config parameter."]
    pub fn grpc_ssl_server_credentials_create_options_using_config(
        client_certificate_request: grpc_ssl_client_certificate_request_type,
        certificate_config: *mut grpc_ssl_server_certificate_config,
    ) -> *mut grpc_ssl_server_credentials_options;
}
extern "C" {
    #[doc = " Creates an options object using a certificate config fetcher. Use this"]
    #[doc = "method to reload the certificates and keys of the SSL server without"]
    #[doc = "interrupting the operation of the server. Initial certificate config will be"]
    #[doc = "fetched during server initialization."]
    #[doc = "- user_data parameter, if not NULL, contains opaque data which will be passed"]
    #[doc = "to the fetcher (see definition of"]
    #[doc = "grpc_ssl_server_certificate_config_callback)."]
    pub fn grpc_ssl_server_credentials_create_options_using_config_fetcher(
        client_certificate_request: grpc_ssl_client_certificate_request_type,
        cb: grpc_ssl_server_certificate_config_callback,
        user_data: *mut ::std::os::raw::c_void,
    ) -> *mut grpc_ssl_server_credentials_options;
}
extern "C" {
    #[doc = " Destroys a grpc_ssl_server_credentials_options object."]
    pub fn grpc_ssl_server_credentials_options_destroy(
        options: *mut grpc_ssl_server_credentials_options,
    );
}
extern "C" {
    #[doc = " Creates an SSL server_credentials object using the provided options struct."]
    #[doc = "- Takes ownership of the options parameter."]
    pub fn grpc_ssl_server_credentials_create_with_options(
        options: *mut grpc_ssl_server_credentials_options,
    ) -> *mut grpc_server_credentials;
}
extern "C" {
    #[doc = " Sets a credentials to a call. Can only be called on the client side before"]
    #[doc = "grpc_call_start_batch."]
    pub fn grpc_call_set_credentials(
        call: *mut grpc_call,
        creds: *mut grpc_call_credentials,
    ) -> grpc_call_error;
}
#[doc = " Callback function that is called when the metadata processing is done."]
#[doc = "- Consumed metadata will be removed from the set of metadata available on the"]
#[doc = "call. consumed_md may be NULL if no metadata has been consumed."]
#[doc = "- Response metadata will be set on the response. response_md may be NULL."]
#[doc = "- status is GRPC_STATUS_OK for success or a specific status for an error."]
#[doc = "Common error status for auth metadata processing is either"]
#[doc = "GRPC_STATUS_UNAUTHENTICATED in case of an authentication failure or"]
#[doc = "GRPC_STATUS PERMISSION_DENIED in case of an authorization failure."]
#[doc = "- error_details gives details about the error. May be NULL."]
pub type grpc_process_auth_metadata_done_cb = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::std::os::raw::c_void,
        consumed_md: *const grpc_metadata,
        num_consumed_md: usize,
        response_md: *const grpc_metadata,
        num_response_md: usize,
        status: grpc_status_code::Type,
        error_details: *const ::std::os::raw::c_char,
    ),
>;
#[doc = " Pluggable server-side metadata processor object."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_auth_metadata_processor {
    #[doc = " The context object is read/write: it contains the properties of the"]
    #[doc = "channel peer and it is the job of the process function to augment it with"]
    #[doc = "properties derived from the passed-in metadata."]
    #[doc = "The lifetime of these objects is guaranteed until cb is invoked."]
    pub process: ::std::option::Option<
        unsafe extern "C" fn(
            state: *mut ::std::os::raw::c_void,
            context: *mut grpc_auth_context,
            md: *const grpc_metadata,
            num_md: usize,
            cb: grpc_process_auth_metadata_done_cb,
            user_data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(state: *mut ::std::os::raw::c_void)>,
    pub state: *mut ::std::os::raw::c_void,
}
extern "C" {
    pub fn grpc_server_credentials_set_auth_metadata_processor(
        creds: *mut grpc_server_credentials,
        processor: grpc_auth_metadata_processor,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_alts_credentials_options {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " This method creates a grpc ALTS credentials client options instance."]
    #[doc = " It is used for experimental purpose for now and subject to change."]
    pub fn grpc_alts_credentials_client_options_create() -> *mut grpc_alts_credentials_options;
}
extern "C" {
    #[doc = " This method creates a grpc ALTS credentials server options instance."]
    #[doc = " It is used for experimental purpose for now and subject to change."]
    pub fn grpc_alts_credentials_server_options_create() -> *mut grpc_alts_credentials_options;
}
extern "C" {
    #[doc = " This method adds a target service account to grpc client's ALTS credentials"]
    #[doc = " options instance. It is used for experimental purpose for now and subject"]
    #[doc = " to change."]
    #[doc = ""]
    #[doc = " - options: grpc ALTS credentials options instance."]
    #[doc = " - service_account: service account of target endpoint."]
    pub fn grpc_alts_credentials_client_options_add_target_service_account(
        options: *mut grpc_alts_credentials_options,
        service_account: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " This method destroys a grpc_alts_credentials_options instance by"]
    #[doc = " de-allocating all of its occupied memory. It is used for experimental purpose"]
    #[doc = " for now and subject to change."]
    #[doc = ""]
    #[doc = " - options: a grpc_alts_credentials_options instance that needs to be"]
    #[doc = "   destroyed."]
    pub fn grpc_alts_credentials_options_destroy(options: *mut grpc_alts_credentials_options);
}
extern "C" {
    #[doc = " This method creates an ALTS channel credential object. The security"]
    #[doc = " level of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY."]
    #[doc = " It is used for experimental purpose for now and subject to change."]
    #[doc = ""]
    #[doc = " - options: grpc ALTS credentials options instance for client."]
    #[doc = ""]
    #[doc = " It returns the created ALTS channel credential object."]
    pub fn grpc_alts_credentials_create(
        options: *const grpc_alts_credentials_options,
    ) -> *mut grpc_channel_credentials;
}
extern "C" {
    #[doc = " This method creates an ALTS server credential object. It is used for"]
    #[doc = " experimental purpose for now and subject to change."]
    #[doc = ""]
    #[doc = " - options: grpc ALTS credentials options instance for server."]
    #[doc = ""]
    #[doc = " It returns the created ALTS server credential object."]
    pub fn grpc_alts_server_credentials_create(
        options: *const grpc_alts_credentials_options,
    ) -> *mut grpc_server_credentials;
}
extern "C" {
    #[doc = " This method creates a local channel credential object. The security level"]
    #[doc = " of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY for UDS and"]
    #[doc = " GRPC_SECURITY_NONE for LOCAL_TCP. It is used for experimental purpose"]
    #[doc = " for now and subject to change."]
    #[doc = ""]
    #[doc = " - type: local connection type"]
    #[doc = ""]
    #[doc = " It returns the created local channel credential object."]
    pub fn grpc_local_credentials_create(
        type_: grpc_local_connect_type,
    ) -> *mut grpc_channel_credentials;
}
extern "C" {
    #[doc = " This method creates a local server credential object. It is used for"]
    #[doc = " experimental purpose for now and subject to change."]
    #[doc = ""]
    #[doc = " - type: local connection type"]
    #[doc = ""]
    #[doc = " It returns the created local server credential object."]
    pub fn grpc_local_server_credentials_create(
        type_: grpc_local_connect_type,
    ) -> *mut grpc_server_credentials;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_tls_credentials_options {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_tls_certificate_provider {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_tls_identity_pairs {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Creates a grpc_tls_identity_pairs that stores a list of identity credential"]
    #[doc = " data, including identity private key and identity certificate chain."]
    pub fn grpc_tls_identity_pairs_create() -> *mut grpc_tls_identity_pairs;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Adds a identity private key and a identity certificate chain to"]
    #[doc = " grpc_tls_identity_pairs. This function will make an internal copy of"]
    #[doc = " |private_key| and |cert_chain|."]
    pub fn grpc_tls_identity_pairs_add_pair(
        pairs: *mut grpc_tls_identity_pairs,
        private_key: *const ::std::os::raw::c_char,
        cert_chain: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Destroys a grpc_tls_identity_pairs object. If this object is passed to a"]
    #[doc = " provider initiation function, the ownership is transferred so this function"]
    #[doc = " doesn't need to be called. Otherwise the creator of the"]
    #[doc = " grpc_tls_identity_pairs object is responsible for its destruction."]
    pub fn grpc_tls_identity_pairs_destroy(pairs: *mut grpc_tls_identity_pairs);
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Creates a grpc_tls_certificate_provider that will load credential data from"]
    #[doc = " static string during initialization. This provider will always return the"]
    #[doc = " same cert data for all cert names."]
    #[doc = " root_certificate and pem_key_cert_pairs can be nullptr, indicating the"]
    #[doc = " corresponding credential data is not needed."]
    #[doc = " This function will make a copy of |root_certificate|."]
    #[doc = " The ownership of |pem_key_cert_pairs| is transferred."]
    pub fn grpc_tls_certificate_provider_static_data_create(
        root_certificate: *const ::std::os::raw::c_char,
        pem_key_cert_pairs: *mut grpc_tls_identity_pairs,
    ) -> *mut grpc_tls_certificate_provider;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Creates a grpc_tls_certificate_provider that will watch the credential"]
    #[doc = " changes on the file system. This provider will always return the up-to-date"]
    #[doc = " cert data for all the cert names callers set through"]
    #[doc = " |grpc_tls_credentials_options|. Note that this API only supports one key-cert"]
    #[doc = " file and hence one set of identity key-cert pair, so SNI(Server Name"]
    #[doc = " Indication) is not supported."]
    #[doc = " - private_key_path is the file path of the private key. This must be set if"]
    #[doc = "   |identity_certificate_path| is set. Otherwise, it could be null if no"]
    #[doc = "   identity credentials are needed."]
    #[doc = " - identity_certificate_path is the file path of the identity certificate"]
    #[doc = "   chain. This must be set if |private_key_path| is set. Otherwise, it could"]
    #[doc = "   be null if no identity credentials are needed."]
    #[doc = " - root_cert_path is the file path to the root certificate bundle. This"]
    #[doc = "   may be null if no root certs are needed."]
    #[doc = " - refresh_interval_sec is the refreshing interval that we will check the"]
    #[doc = "   files for updates."]
    #[doc = " It does not take ownership of parameters."]
    pub fn grpc_tls_certificate_provider_file_watcher_create(
        private_key_path: *const ::std::os::raw::c_char,
        identity_certificate_path: *const ::std::os::raw::c_char,
        root_cert_path: *const ::std::os::raw::c_char,
        refresh_interval_sec: ::std::os::raw::c_uint,
    ) -> *mut grpc_tls_certificate_provider;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Releases a grpc_tls_certificate_provider object. The creator of the"]
    #[doc = " grpc_tls_certificate_provider object is responsible for its release."]
    pub fn grpc_tls_certificate_provider_release(provider: *mut grpc_tls_certificate_provider);
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Creates an grpc_tls_credentials_options."]
    pub fn grpc_tls_credentials_options_create() -> *mut grpc_tls_credentials_options;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Sets the credential provider in the options."]
    #[doc = " The |options| will implicitly take a new ref to the |provider|."]
    pub fn grpc_tls_credentials_options_set_certificate_provider(
        options: *mut grpc_tls_credentials_options,
        provider: *mut grpc_tls_certificate_provider,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " If set, gRPC stack will keep watching the root certificates with"]
    #[doc = " name |root_cert_name|."]
    #[doc = " If this is not set on the client side, we will use the root certificates"]
    #[doc = " stored in the default system location, since client side must provide root"]
    #[doc = " certificates in TLS."]
    #[doc = " If this is not set on the server side, we will not watch any root certificate"]
    #[doc = " updates, and assume no root certificates needed for the server(single-side"]
    #[doc = " TLS). Default root certs on the server side is not supported."]
    pub fn grpc_tls_credentials_options_watch_root_certs(
        options: *mut grpc_tls_credentials_options,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Sets the name of the root certificates being watched."]
    #[doc = " If not set, We will use a default empty string as the root certificate name."]
    pub fn grpc_tls_credentials_options_set_root_cert_name(
        options: *mut grpc_tls_credentials_options,
        root_cert_name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " If set, gRPC stack will keep watching the identity key-cert pairs"]
    #[doc = " with name |identity_cert_name|."]
    #[doc = " This is required on the server side, and optional on the client side."]
    pub fn grpc_tls_credentials_options_watch_identity_key_cert_pairs(
        options: *mut grpc_tls_credentials_options,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Sets the name of the identity certificates being watched."]
    #[doc = " If not set, We will use a default empty string as the identity certificate"]
    #[doc = " name."]
    pub fn grpc_tls_credentials_options_set_identity_cert_name(
        options: *mut grpc_tls_credentials_options,
        identity_cert_name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Sets the options of whether to request and/or verify client certs. This shall"]
    #[doc = " only be called on the server side."]
    pub fn grpc_tls_credentials_options_set_cert_request_type(
        options: *mut grpc_tls_credentials_options,
        type_: grpc_ssl_client_certificate_request_type,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " If set, gRPC will read all hashed x.509 CRL files in the directory and"]
    #[doc = " enforce the CRL files on all TLS handshakes. Only supported for OpenSSL"]
    #[doc = " version > 1.1."]
    #[doc = " It is used for experimental purpose for now and subject to change."]
    pub fn grpc_tls_credentials_options_set_crl_directory(
        options: *mut grpc_tls_credentials_options,
        crl_directory: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Sets the options of whether to verify server certs on the client side."]
    #[doc = " Passing in a non-zero value indicates verifying the certs."]
    pub fn grpc_tls_credentials_options_set_verify_server_cert(
        options: *mut grpc_tls_credentials_options,
        verify_server_cert: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Sets whether or not a TLS server should send a list of CA names in the"]
    #[doc = " ServerHello. This list of CA names is read from the server's trust bundle, so"]
    #[doc = " that the client can use this list as a hint to know which certificate it"]
    #[doc = " should send to the server."]
    #[doc = ""]
    #[doc = " WARNING: This API is extremely dangerous and should not be used. If the"]
    #[doc = " server's trust bundle is too large, then the TLS server will be unable to"]
    #[doc = " form a ServerHello, and hence will be unusable. The definition of \"too large\""]
    #[doc = " depends on the underlying SSL library being used and on the size of the CN"]
    #[doc = " fields of the certificates in the trust bundle."]
    pub fn grpc_tls_credentials_options_set_send_client_ca_list(
        options: *mut grpc_tls_credentials_options,
        send_client_ca_list: bool,
    );
}
#[doc = " EXPERIMENTAL API - Subject to change"]
#[doc = ""]
#[doc = " The read-only request information exposed in a verification call."]
#[doc = " Callers should not directly manage the ownership of it. We will make sure it"]
#[doc = " is always available inside verify() or cancel() call, and will destroy the"]
#[doc = " object at the end of custom verification."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_tls_custom_verification_check_request {
    pub target_name: *const ::std::os::raw::c_char,
    pub peer_info: grpc_tls_custom_verification_check_request_peer_info,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_tls_custom_verification_check_request_peer_info {
    pub common_name: *const ::std::os::raw::c_char,
    pub san_names: grpc_tls_custom_verification_check_request_peer_info_san_names,
    pub peer_cert: *const ::std::os::raw::c_char,
    pub peer_cert_full_chain: *const ::std::os::raw::c_char,
    pub verified_root_cert_subject: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_tls_custom_verification_check_request_peer_info_san_names {
    pub uri_names: *mut *mut ::std::os::raw::c_char,
    pub uri_names_size: usize,
    pub dns_names: *mut *mut ::std::os::raw::c_char,
    pub dns_names_size: usize,
    pub email_names: *mut *mut ::std::os::raw::c_char,
    pub email_names_size: usize,
    pub ip_names: *mut *mut ::std::os::raw::c_char,
    pub ip_names_size: usize,
}
#[doc = " EXPERIMENTAL API - Subject to change"]
#[doc = ""]
#[doc = " A callback function provided by gRPC as a parameter of the |verify| function"]
#[doc = " in grpc_tls_certificate_verifier_external. If |verify| is expected to be run"]
#[doc = " asynchronously, the implementer of |verify| will need to invoke this callback"]
#[doc = " with |callback_arg| and proper verification status at the end to bring the"]
#[doc = " control back to gRPC C core."]
pub type grpc_tls_on_custom_verification_check_done_cb = ::std::option::Option<
    unsafe extern "C" fn(
        request: *mut grpc_tls_custom_verification_check_request,
        callback_arg: *mut ::std::os::raw::c_void,
        status: grpc_status_code::Type,
        error_details: *const ::std::os::raw::c_char,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_tls_certificate_verifier {
    _unused: [u8; 0],
}
#[doc = " EXPERIMENTAL API - Subject to change"]
#[doc = ""]
#[doc = " A struct containing all the necessary functions a custom external verifier"]
#[doc = " needs to implement to be able to be converted to an internal verifier."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_tls_certificate_verifier_external {
    pub user_data: *mut ::std::os::raw::c_void,
    #[doc = " A function pointer containing the verification logic that will be"]
    #[doc = " performed after the TLS handshake is done. It could be processed"]
    #[doc = " synchronously or asynchronously."]
    #[doc = " - If expected to be processed synchronously, the implementer should"]
    #[doc = "   populate the verification result through |sync_status| and"]
    #[doc = "   |sync_error_details|, and then return true."]
    #[doc = " - If expected to be processed asynchronously, the implementer should return"]
    #[doc = "   false immediately, and then in the asynchronous thread invoke |callback|"]
    #[doc = "   with the verification result. The implementer MUST NOT invoke the async"]
    #[doc = "   |callback| in the same thread before |verify| returns, otherwise it can"]
    #[doc = "   lead to deadlocks."]
    #[doc = ""]
    #[doc = " user_data: any argument that is passed in the user_data of"]
    #[doc = "            grpc_tls_certificate_verifier_external during construction time"]
    #[doc = "            can be retrieved later here."]
    #[doc = " request: request information exposed to the function implementer."]
    #[doc = " callback: the callback that the function implementer needs to invoke, if"]
    #[doc = "           return a non-zero value. It is usually invoked when the"]
    #[doc = "           asynchronous verification is done, and serves to bring the"]
    #[doc = "           control back to gRPC."]
    #[doc = " callback_arg: A pointer to the internal ExternalVerifier instance. This is"]
    #[doc = "               mainly used as an argument in |callback|, if want to invoke"]
    #[doc = "               |callback| in async mode."]
    #[doc = " sync_status: indicates if a connection should be allowed. This should only"]
    #[doc = "              be used if the verification check is done synchronously."]
    #[doc = " sync_error_details: the error generated while verifying a connection. This"]
    #[doc = "                     should only be used if the verification check is done"]
    #[doc = "                     synchronously. the implementation must allocate the"]
    #[doc = "                     error string via gpr_malloc() or gpr_strdup()."]
    #[doc = " return: return 0 if |verify| is expected to be executed asynchronously,"]
    #[doc = "         otherwise return a non-zero value."]
    pub verify: ::std::option::Option<
        unsafe extern "C" fn(
            user_data: *mut ::std::os::raw::c_void,
            request: *mut grpc_tls_custom_verification_check_request,
            callback: grpc_tls_on_custom_verification_check_done_cb,
            callback_arg: *mut ::std::os::raw::c_void,
            sync_status: *mut grpc_status_code::Type,
            sync_error_details: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    #[doc = " A function pointer that cleans up the caller-specified resources when the"]
    #[doc = " verifier is still running but the whole connection got cancelled. This"]
    #[doc = " could happen when the verifier is doing some async operations, and the"]
    #[doc = " whole handshaker object got destroyed because of connection time limit is"]
    #[doc = " reached, or any other reasons. In such cases, function implementers might"]
    #[doc = " want to be notified, and properly clean up some resources."]
    #[doc = ""]
    #[doc = " user_data: any argument that is passed in the user_data of"]
    #[doc = "            grpc_tls_certificate_verifier_external during construction time"]
    #[doc = "            can be retrieved later here."]
    #[doc = " request: request information exposed to the function implementer. It will"]
    #[doc = "          be the same request object that was passed to verify(), and it"]
    #[doc = "          tells the cancel() which request to cancel."]
    pub cancel: ::std::option::Option<
        unsafe extern "C" fn(
            user_data: *mut ::std::os::raw::c_void,
            request: *mut grpc_tls_custom_verification_check_request,
        ),
    >,
    #[doc = " A function pointer that does some additional destruction work when the"]
    #[doc = " verifier is destroyed. This is used when the caller wants to associate some"]
    #[doc = " objects to the lifetime of external_verifier, and destroy them when"]
    #[doc = " external_verifier got destructed. For example, in C++, the class containing"]
    #[doc = " user-specified callback functions should not be destroyed before"]
    #[doc = " external_verifier, since external_verifier will invoke them while being"]
    #[doc = " used."]
    #[doc = " Note that the caller MUST delete the grpc_tls_certificate_verifier_external"]
    #[doc = " object itself in this function, otherwise it will cause memory leaks. That"]
    #[doc = " also means the user_data has to carries at least a self pointer, for the"]
    #[doc = " callers to later delete it in destruct()."]
    #[doc = ""]
    #[doc = " user_data: any argument that is passed in the user_data of"]
    #[doc = "            grpc_tls_certificate_verifier_external during construction time"]
    #[doc = "            can be retrieved later here."]
    pub destruct:
        ::std::option::Option<unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void)>,
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Converts an external verifier to an internal verifier."]
    #[doc = " Note that we will not take the ownership of the external_verifier. Callers"]
    #[doc = " will need to delete external_verifier in its own destruct function."]
    pub fn grpc_tls_certificate_verifier_external_create(
        external_verifier: *mut grpc_tls_certificate_verifier_external,
    ) -> *mut grpc_tls_certificate_verifier;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Factory function for an internal verifier that won't perform any"]
    #[doc = " post-handshake verification. Note: using this solely without any other"]
    #[doc = " authentication mechanisms on the peer identity will leave your applications"]
    #[doc = " to the MITM(Man-In-The-Middle) attacks. Users should avoid doing so in"]
    #[doc = " production environments."]
    pub fn grpc_tls_certificate_verifier_no_op_create() -> *mut grpc_tls_certificate_verifier;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Factory function for an internal verifier that will do the default hostname"]
    #[doc = " check."]
    pub fn grpc_tls_certificate_verifier_host_name_create() -> *mut grpc_tls_certificate_verifier;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Releases a grpc_tls_certificate_verifier object. The creator of the"]
    #[doc = " grpc_tls_certificate_verifier object is responsible for its release."]
    pub fn grpc_tls_certificate_verifier_release(verifier: *mut grpc_tls_certificate_verifier);
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Sets the verifier in options. The |options| will implicitly take a new ref to"]
    #[doc = " the |verifier|. If not set on the client side, we will verify server's"]
    #[doc = " certificates, and check the default hostname. If not set on the server side,"]
    #[doc = " we will verify client's certificates."]
    pub fn grpc_tls_credentials_options_set_certificate_verifier(
        options: *mut grpc_tls_credentials_options,
        verifier: *mut grpc_tls_certificate_verifier,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Sets the options of whether to check the hostname of the peer on a per-call"]
    #[doc = " basis. This is usually used in a combination with virtual hosting at the"]
    #[doc = " client side, where each individual call on a channel can have a different"]
    #[doc = " host associated with it."]
    #[doc = " This check is intended to verify that the host specified for the individual"]
    #[doc = " call is covered by the cert that the peer presented."]
    #[doc = " The default is a non-zero value, which indicates performing such checks."]
    pub fn grpc_tls_credentials_options_set_check_call_host(
        options: *mut grpc_tls_credentials_options,
        check_call_host: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Performs the verification logic of an internal verifier."]
    #[doc = " This is typically used when composing the internal verifiers as part of the"]
    #[doc = " custom verification."]
    #[doc = " If |grpc_tls_certificate_verifier_verify| returns true, inspect the"]
    #[doc = " verification result through request->status and request->error_details."]
    #[doc = " Otherwise, inspect through the parameter of |callback|."]
    pub fn grpc_tls_certificate_verifier_verify(
        verifier: *mut grpc_tls_certificate_verifier,
        request: *mut grpc_tls_custom_verification_check_request,
        callback: grpc_tls_on_custom_verification_check_done_cb,
        callback_arg: *mut ::std::os::raw::c_void,
        sync_status: *mut grpc_status_code::Type,
        sync_error_details: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Performs the cancellation logic of an internal verifier."]
    #[doc = " This is typically used when composing the internal verifiers as part of the"]
    #[doc = " custom verification."]
    pub fn grpc_tls_certificate_verifier_cancel(
        verifier: *mut grpc_tls_certificate_verifier,
        request: *mut grpc_tls_custom_verification_check_request,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Creates a TLS channel credential object based on the"]
    #[doc = " grpc_tls_credentials_options specified by callers. The"]
    #[doc = " grpc_channel_credentials will take the ownership of the |options|. The"]
    #[doc = " security level of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY."]
    pub fn grpc_tls_credentials_create(
        options: *mut grpc_tls_credentials_options,
    ) -> *mut grpc_channel_credentials;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " Creates a TLS server credential object based on the"]
    #[doc = " grpc_tls_credentials_options specified by callers. The"]
    #[doc = " grpc_server_credentials will take the ownership of the |options|."]
    pub fn grpc_tls_server_credentials_create(
        options: *mut grpc_tls_credentials_options,
    ) -> *mut grpc_server_credentials;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " This method creates an insecure channel credentials object."]
    pub fn grpc_insecure_credentials_create() -> *mut grpc_channel_credentials;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " This method creates an insecure server credentials object."]
    pub fn grpc_insecure_server_credentials_create() -> *mut grpc_server_credentials;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " This method creates an xDS channel credentials object."]
    #[doc = ""]
    #[doc = " Creating a channel with credentials of this type indicates that the channel"]
    #[doc = " should get credentials configuration from the xDS control plane."]
    #[doc = ""]
    #[doc = " \\a fallback_credentials are used if the channel target does not have the"]
    #[doc = " 'xds:///' scheme or if the xDS control plane does not provide information on"]
    #[doc = " how to fetch credentials dynamically. Does NOT take ownership of the \\a"]
    #[doc = " fallback_credentials. (Internally takes a ref to the object.)"]
    pub fn grpc_xds_credentials_create(
        fallback_credentials: *mut grpc_channel_credentials,
    ) -> *mut grpc_channel_credentials;
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change"]
    #[doc = ""]
    #[doc = " This method creates an xDS server credentials object."]
    #[doc = ""]
    #[doc = " \\a fallback_credentials are used if the xDS control plane does not provide"]
    #[doc = " information on how to fetch credentials dynamically."]
    #[doc = ""]
    #[doc = " Does NOT take ownership of the \\a fallback_credentials. (Internally takes"]
    #[doc = " a ref to the object.)"]
    pub fn grpc_xds_server_credentials_create(
        fallback_credentials: *mut grpc_server_credentials,
    ) -> *mut grpc_server_credentials;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_authorization_policy_provider {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " EXPERIMENTAL - Subject to change."]
    #[doc = " Creates a grpc_authorization_policy_provider using gRPC authorization policy"]
    #[doc = " from static string."]
    #[doc = " - authz_policy is the input gRPC authorization policy."]
    #[doc = " - code is the error status code on failure. On success, it equals"]
    #[doc = "   GRPC_STATUS_OK."]
    #[doc = " - error_details contains details about the error if any. If the"]
    #[doc = "   initialization is successful, it will be null. Caller must use gpr_free to"]
    #[doc = "   destroy this string."]
    pub fn grpc_authorization_policy_provider_static_data_create(
        authz_policy: *const ::std::os::raw::c_char,
        code: *mut grpc_status_code::Type,
        error_details: *mut *const ::std::os::raw::c_char,
    ) -> *mut grpc_authorization_policy_provider;
}
extern "C" {
    #[doc = " EXPERIMENTAL - Subject to change."]
    #[doc = " Creates a grpc_authorization_policy_provider by watching for gRPC"]
    #[doc = " authorization policy changes in filesystem."]
    #[doc = " - authz_policy is the file path of gRPC authorization policy."]
    #[doc = " - refresh_interval_sec is the amount of time the internal thread would wait"]
    #[doc = "   before checking for file updates."]
    #[doc = " - code is the error status code on failure. On success, it equals"]
    #[doc = "   GRPC_STATUS_OK."]
    #[doc = " - error_details contains details about the error if any. If the"]
    #[doc = "   initialization is successful, it will be null. Caller must use gpr_free to"]
    #[doc = "   destroy this string."]
    pub fn grpc_authorization_policy_provider_file_watcher_create(
        authz_policy_path: *const ::std::os::raw::c_char,
        refresh_interval_sec: ::std::os::raw::c_uint,
        code: *mut grpc_status_code::Type,
        error_details: *mut *const ::std::os::raw::c_char,
    ) -> *mut grpc_authorization_policy_provider;
}
extern "C" {
    #[doc = " EXPERIMENTAL - Subject to change."]
    #[doc = " Releases grpc_authorization_policy_provider object. The creator of"]
    #[doc = " grpc_authorization_policy_provider is responsible for its release."]
    pub fn grpc_authorization_policy_provider_release(
        provider: *mut grpc_authorization_policy_provider,
    );
}
extern "C" {
    #[doc = " EXPERIMENTAL API - Subject to change."]
    #[doc = " Configures a grpc_tls_credentials_options object with tls session key"]
    #[doc = " logging capability. TLS channels using these credentials have tls session"]
    #[doc = " key logging enabled."]
    #[doc = " - options is the grpc_tls_credentials_options object"]
    #[doc = " - path is a string pointing to the location where TLS session keys would be"]
    #[doc = "   stored."]
    pub fn grpc_tls_credentials_options_set_tls_session_key_log_file_path(
        options: *mut grpc_tls_credentials_options,
        path: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " malloc."]
    #[doc = " If size==0, always returns NULL. Otherwise this function never returns NULL."]
    #[doc = " The pointer returned is suitably aligned for any kind of variable it could"]
    #[doc = " contain."]
    pub fn gpr_malloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " like malloc, but zero all bytes before returning them"]
    pub fn gpr_zalloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " free"]
    pub fn gpr_free(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = " realloc, never returns NULL"]
    pub fn gpr_realloc(p: *mut ::std::os::raw::c_void, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " aligned malloc, never returns NULL, will align to alignment, which"]
    #[doc = " must be a power of 2."]
    pub fn gpr_malloc_aligned(size: usize, alignment: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " free memory allocated by gpr_malloc_aligned"]
    pub fn gpr_free_aligned(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = " Return the number of CPU cores on the current system. Will return 0 if"]
    #[doc = "the information is not available."]
    pub fn gpr_cpu_num_cores() -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " Return the CPU on which the current thread is executing; N.B. This should"]
    #[doc = "be considered advisory only - it is possible that the thread is switched"]
    #[doc = "to a different CPU at any time. Returns a value in range"]
    #[doc = "[0, gpr_cpu_num_cores() - 1]"]
    pub fn gpr_cpu_current_cpu() -> ::std::os::raw::c_uint;
}
#[repr(u32)]
#[doc = " The severity of a log message - use the #defines below when calling into"]
#[doc = "gpr_log to additionally supply file and line data"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum gpr_log_severity {
    GPR_LOG_SEVERITY_DEBUG = 0,
    GPR_LOG_SEVERITY_INFO = 1,
    GPR_LOG_SEVERITY_ERROR = 2,
}
extern "C" {
    #[doc = " Returns a string representation of the log severity"]
    pub fn gpr_log_severity_string(severity: gpr_log_severity) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Log a message. It's advised to use GPR_xxx above to generate the context"]
    #[doc = " for each message"]
    pub fn gpr_log(
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        severity: gpr_log_severity,
        format: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn gpr_should_log(severity: gpr_log_severity) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gpr_log_message(
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        severity: gpr_log_severity,
        message: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Set global log verbosity"]
    pub fn gpr_set_log_verbosity(min_severity_to_print: gpr_log_severity);
}
extern "C" {
    pub fn gpr_log_verbosity_init();
}
#[doc = " Log overrides: applications can use this API to intercept logging calls"]
#[doc = "and use their own implementations"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_log_func_args {
    pub file: *const ::std::os::raw::c_char,
    pub line: ::std::os::raw::c_int,
    pub severity: gpr_log_severity,
    pub message: *const ::std::os::raw::c_char,
}
pub type gpr_log_func = ::std::option::Option<unsafe extern "C" fn(args: *mut gpr_log_func_args)>;
extern "C" {
    pub fn gpr_set_log_function(func: gpr_log_func);
}
extern "C" {
    pub fn gpr_assertion_failed(
        filename: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        message: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Returns a string allocated with gpr_malloc that contains a UTF-8"]
    #[doc = " formatted error message, corresponding to the error messageid."]
    #[doc = " Use in conjunction with GetLastError() et al."]
    pub fn gpr_format_message(messageid: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Returns a copy of src that can be passed to gpr_free()."]
    #[doc = "If allocation fails or if src is NULL, returns NULL."]
    pub fn gpr_strdup(src: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " printf to a newly-allocated string.  The set of supported formats may vary"]
    #[doc = "between platforms."]
    #[doc = ""]
    #[doc = "On success, returns the number of bytes printed (excluding the final '\\0'),"]
    #[doc = "and *strp points to a string which must later be destroyed with gpr_free()."]
    #[doc = ""]
    #[doc = "On error, returns -1 and sets *strp to NULL. If the format string is bad,"]
    #[doc = "the result is undefined."]
    pub fn gpr_asprintf(
        strp: *mut *mut ::std::os::raw::c_char,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
pub type gpr_thd_id = usize;
extern "C" {
    #[doc = " Returns the identifier of the current thread."]
    pub fn gpr_thd_currentid() -> gpr_thd_id;
}
#[doc = " Reader for byte buffers. Iterates over slices in the byte buffer"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpc_byte_buffer_reader {
    pub buffer_in: *mut grpc_byte_buffer,
    pub buffer_out: *mut grpc_byte_buffer,
    pub current: grpc_byte_buffer_reader_grpc_byte_buffer_reader_current,
}
#[doc = " Different current objects correspond to different types of byte buffers"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union grpc_byte_buffer_reader_grpc_byte_buffer_reader_current {
    #[doc = " Index into a slice buffer's array of slices"]
    pub index: ::std::os::raw::c_uint,
}
impl ::std::fmt::Debug for grpc_byte_buffer_reader_grpc_byte_buffer_reader_current {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_byte_buffer_reader_grpc_byte_buffer_reader_current {{ union }}"
        )
    }
}
impl ::std::fmt::Debug for grpc_byte_buffer_reader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "grpc_byte_buffer_reader {{ buffer_in: {:?}, buffer_out: {:?}, current: {:?} }}",
            self.buffer_in, self.buffer_out, self.current
        )
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpcwrap_batch_context {
    pub send_initial_metadata: grpc_metadata_array,
    pub send_message: *mut grpc_byte_buffer,
    pub send_status_from_server: grpcwrap_batch_context__bindgen_ty_1,
    pub recv_initial_metadata: grpc_metadata_array,
    pub recv_message: *mut grpc_byte_buffer,
    pub recv_status_on_client: grpcwrap_batch_context__bindgen_ty_2,
    pub recv_close_on_server_cancelled: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpcwrap_batch_context__bindgen_ty_1 {
    pub trailing_metadata: grpc_metadata_array,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpcwrap_batch_context__bindgen_ty_2 {
    pub trailing_metadata: grpc_metadata_array,
    pub status: grpc_status_code::Type,
    pub status_details: grpc_slice,
    pub error_string: *const ::std::os::raw::c_char,
}
impl ::std::fmt::Debug for grpcwrap_batch_context__bindgen_ty_2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "grpcwrap_batch_context__bindgen_ty_2 {{ trailing_metadata: {:?}, status: {:?}, status_details: {:?}, error_string: {:?} }}" , self . trailing_metadata , self . status , self . status_details , self . error_string)
    }
}
impl ::std::fmt::Debug for grpcwrap_batch_context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "grpcwrap_batch_context {{ send_initial_metadata: {:?}, send_message: {:?}, send_status_from_server: {:?}, recv_initial_metadata: {:?}, recv_message: {:?}, recv_status_on_client: {:?}, recv_close_on_server_cancelled: {:?} }}" , self . send_initial_metadata , self . send_message , self . send_status_from_server , self . recv_initial_metadata , self . recv_message , self . recv_status_on_client , self . recv_close_on_server_cancelled)
    }
}
extern "C" {
    pub fn grpcwrap_batch_context_create() -> *mut grpcwrap_batch_context;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpcwrap_request_call_context {
    pub call: *mut grpc_call,
    pub call_details: grpc_call_details,
    pub request_metadata: grpc_metadata_array,
}
impl ::std::fmt::Debug for grpcwrap_request_call_context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "grpcwrap_request_call_context {{ call: {:?}, call_details: {:?}, request_metadata: {:?} }}" , self . call , self . call_details , self . request_metadata)
    }
}
extern "C" {
    pub fn grpcwrap_request_call_context_create() -> *mut grpcwrap_request_call_context;
}
extern "C" {
    pub fn grpcwrap_metadata_array_destroy_metadata_only(array: *mut grpc_metadata_array);
}
extern "C" {
    pub fn grpcwrap_metadata_array_destroy_metadata_including_entries(
        array: *mut grpc_metadata_array,
    );
}
extern "C" {
    pub fn grpcwrap_metadata_array_destroy_full(array: *mut grpc_metadata_array);
}
extern "C" {
    pub fn grpcwrap_metadata_array_init(array: *mut grpc_metadata_array, capacity: usize);
}
extern "C" {
    pub fn grpcwrap_metadata_array_add(
        array: *mut grpc_metadata_array,
        key: *const ::std::os::raw::c_char,
        key_length: usize,
        value: *const ::std::os::raw::c_char,
        value_length: usize,
    );
}
extern "C" {
    pub fn grpcwrap_metadata_array_get_key(
        array: *const grpc_metadata_array,
        index: usize,
        key_length: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpcwrap_metadata_array_get_value(
        array: *const grpc_metadata_array,
        index: usize,
        value_length: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpcwrap_metadata_array_cleanup(array: *mut grpc_metadata_array);
}
extern "C" {
    pub fn grpcwrap_metadata_array_shrink_to_fit(array: *mut grpc_metadata_array);
}
extern "C" {
    pub fn grpcwrap_metadata_array_move(
        dest: *mut grpc_metadata_array,
        src: *mut grpc_metadata_array,
    );
}
extern "C" {
    pub fn grpcwrap_batch_context_destroy(ctx: *mut grpcwrap_batch_context);
}
extern "C" {
    pub fn grpcwrap_request_call_context_destroy(ctx: *mut grpcwrap_request_call_context);
}
extern "C" {
    pub fn grpcwrap_batch_context_take_recv_initial_metadata(
        ctx: *mut grpcwrap_batch_context,
        res: *mut grpc_metadata_array,
    );
}
extern "C" {
    pub fn grpcwrap_batch_context_take_recv_status_on_client_trailing_metadata(
        ctx: *mut grpcwrap_batch_context,
        res: *mut grpc_metadata_array,
    );
}
extern "C" {
    pub fn grpcwrap_slice_raw_offset(
        slice: *const grpc_slice,
        offset: usize,
        len: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpcwrap_slice_copy(slice: *const grpc_slice) -> grpc_slice;
}
extern "C" {
    pub fn grpcwrap_slice_unref(slice: *const grpc_slice);
}
extern "C" {
    pub fn grpcwrap_slice_ref(slice: *const grpc_slice) -> grpc_slice;
}
extern "C" {
    pub fn grpcwrap_slice_length(slice: *const grpc_slice) -> usize;
}
extern "C" {
    pub fn grpcwrap_batch_context_take_recv_message(
        ctx: *mut grpcwrap_batch_context,
    ) -> *mut grpc_byte_buffer;
}
extern "C" {
    pub fn grpcwrap_batch_context_recv_status_on_client_status(
        ctx: *const grpcwrap_batch_context,
    ) -> grpc_status_code::Type;
}
extern "C" {
    pub fn grpcwrap_batch_context_recv_status_on_client_details(
        ctx: *const grpcwrap_batch_context,
        details_length: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpcwrap_batch_context_recv_status_on_client_trailing_metadata(
        ctx: *const grpcwrap_batch_context,
    ) -> *const grpc_metadata_array;
}
extern "C" {
    pub fn grpcwrap_batch_context_recv_status_on_client_error_string(
        ctx: *const grpcwrap_batch_context,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpcwrap_request_call_context_ref_call(
        ctx: *mut grpcwrap_request_call_context,
    ) -> *mut grpc_call;
}
extern "C" {
    pub fn grpcwrap_request_call_context_get_call(
        ctx: *mut grpcwrap_request_call_context,
    ) -> *mut grpc_call;
}
extern "C" {
    pub fn grpcwrap_request_call_context_method(
        ctx: *const grpcwrap_request_call_context,
        method_length: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpcwrap_request_call_context_host(
        ctx: *const grpcwrap_request_call_context,
        host_length: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn grpcwrap_request_call_context_deadline(
        ctx: *const grpcwrap_request_call_context,
    ) -> gpr_timespec;
}
extern "C" {
    pub fn grpcwrap_request_call_context_metadata_array(
        ctx: *const grpcwrap_request_call_context,
    ) -> *const grpc_metadata_array;
}
extern "C" {
    pub fn grpcwrap_batch_context_recv_close_on_server_cancelled(
        ctx: *const grpcwrap_batch_context,
    ) -> i32;
}
extern "C" {
    pub fn grpcwrap_channel_create_call(
        channel: *mut grpc_channel,
        parent_call: *mut grpc_call,
        propagation_mask: u32,
        cq: *mut grpc_completion_queue,
        method: *const ::std::os::raw::c_char,
        method_len: usize,
        host: *const ::std::os::raw::c_char,
        host_len: usize,
        deadline: gpr_timespec,
    ) -> *mut grpc_call;
}
extern "C" {
    pub fn grpcwrap_channel_args_create(num_args: usize) -> *mut grpc_channel_args;
}
extern "C" {
    pub fn grpcwrap_channel_args_set_string(
        args: *mut grpc_channel_args,
        index: usize,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn grpcwrap_channel_args_set_integer(
        args: *mut grpc_channel_args,
        index: usize,
        key: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn grpcwrap_channel_args_set_pointer_vtable(
        args: *mut grpc_channel_args,
        index: usize,
        key: *const ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_void,
        vtable: *const grpc_arg_pointer_vtable,
    );
}
extern "C" {
    pub fn grpcwrap_channel_args_destroy(args: *mut grpc_channel_args);
}
extern "C" {
    pub fn grpcwrap_call_start_unary(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        send_buffer: *mut grpc_slice,
        write_flags: u32,
        initial_metadata: *mut grpc_metadata_array,
        initial_metadata_flags: u32,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_start_client_streaming(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        initial_metadata: *mut grpc_metadata_array,
        initial_metadata_flags: u32,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_start_server_streaming(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        send_buffer: *mut grpc_slice,
        write_flags: u32,
        initial_metadata: *mut grpc_metadata_array,
        initial_metadata_flags: u32,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_start_duplex_streaming(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        initial_metadata: *mut grpc_metadata_array,
        initial_metadata_flags: u32,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_recv_initial_metadata(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_send_message(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        send_buffer: *mut grpc_slice,
        write_flags: u32,
        initial_metadata: *mut grpc_metadata_array,
        initial_metadata_flags: u32,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_send_close_from_client(
        call: *mut grpc_call,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_send_status_from_server(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        status_code: grpc_status_code::Type,
        status_details: *const ::std::os::raw::c_char,
        status_details_len: usize,
        initial_metadata: *mut grpc_metadata_array,
        initial_metadata_flags: u32,
        trailing_metadata: *mut grpc_metadata_array,
        optional_send_buffer: *mut grpc_slice,
        write_flags: u32,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_recv_message(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_start_serverside(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_call_send_initial_metadata(
        call: *mut grpc_call,
        ctx: *mut grpcwrap_batch_context,
        initial_metadata: *mut grpc_metadata_array,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    #[doc = " Kick call's completion queue, it should be called after there is an event"]
    #[doc = "ready to poll."]
    #[doc = "THREAD SAFETY: grpcwrap_call_kick_completion_queue is thread-safe"]
    #[doc = "because it does not change the call's state."]
    pub fn grpcwrap_call_kick_completion_queue(
        call: *mut grpc_call,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
extern "C" {
    pub fn grpcwrap_server_request_call(
        server: *mut grpc_server,
        cq: *mut grpc_completion_queue,
        ctx: *mut grpcwrap_request_call_context,
        tag: *mut ::std::os::raw::c_void,
    ) -> grpc_call_error;
}
