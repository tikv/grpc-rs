#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpcwrap_stats {
    _unused: [u8; 0],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpcwrap_stats_counter {
    ClientCallsCreated = 0,
    ServerCallsCreated = 1,
    ClientChannelsCreated = 2,
    ClientSubchannelsCreated = 3,
    ServerChannelsCreated = 4,
    InsecureConnectionsCreated = 5,
    SyscallWrite = 6,
    SyscallRead = 7,
    TcpReadAlloc8k = 8,
    TcpReadAlloc64k = 9,
    Http2SettingsWrites = 10,
    Http2PingsSent = 11,
    Http2WritesBegun = 12,
    Http2TransportStalls = 13,
    Http2StreamStalls = 14,
    CqPluckCreates = 15,
    CqNextCreates = 16,
    CqCallbackCreates = 17,
    COUNTER_COUNT = 18,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum grpcwrap_stats_histogram {
    CallInitialSize = 0,
    TcpWriteSize = 1,
    TcpWriteIovSize = 2,
    TcpReadSize = 3,
    TcpReadOffer = 4,
    TcpReadOfferIovSize = 5,
    Http2SendMessageSize = 6,
    Http2MetadataSize = 7,
    HISTOGRAM_COUNT = 8,
}
extern "C" {
    pub fn grpcwrap_stats_collect() -> *mut grpcwrap_stats;
}
extern "C" {
    pub fn grpcwrap_stats_free(stats: *mut grpcwrap_stats);
}
extern "C" {
    pub fn grpcwrap_stats_get_counter(
        stats: *const grpcwrap_stats,
        which: grpcwrap_stats_counter,
    ) -> u64;
}
extern "C" {
    pub fn grpcwrap_stats_counter_name(which: grpcwrap_stats_counter) -> grpc_slice;
}
extern "C" {
    pub fn grpcwrap_stats_counter_doc(which: grpcwrap_stats_counter) -> grpc_slice;
}
extern "C" {
    pub fn grpcwrap_stats_get_histogram_percentile(
        stats: *const grpcwrap_stats,
        which: grpcwrap_stats_histogram,
        percentile: f64,
    ) -> f64;
}
extern "C" {
    pub fn grpcwrap_stats_get_histogram_count(
        stats: *const grpcwrap_stats,
        which: grpcwrap_stats_histogram,
    ) -> u64;
}
extern "C" {
    pub fn grpcwrap_stats_histogram_name(which: grpcwrap_stats_histogram) -> grpc_slice;
}
extern "C" {
    pub fn grpcwrap_stats_histogram_doc(which: grpcwrap_stats_histogram) -> grpc_slice;
}
