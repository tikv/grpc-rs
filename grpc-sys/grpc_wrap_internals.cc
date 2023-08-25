// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

#include <src/core/lib/debug/stats.h>
#include <src/core/lib/debug/stats_data.h>

#include <grpc/support/log.h>

#ifdef GPR_WINDOWS
#define GPR_EXPORT extern "C" __declspec(dllexport)
#define GPR_CALLTYPE __cdecl
#endif

#ifndef GPR_EXPORT
#define GPR_EXPORT extern "C"
#endif

#ifndef GPR_CALLTYPE
#define GPR_CALLTYPE
#endif

/** gRPC stats.
    See: grpc/src/core/lib/debug/stats_data.yaml */
typedef struct grpcwrap_stats grpcwrap_stats;

enum grpcwrap_stats_counter {
  ClientCallsCreated,
  ServerCallsCreated,
  ClientChannelsCreated,
  ClientSubchannelsCreated,
  ServerChannelsCreated,
  InsecureConnectionsCreated,
  SyscallWrite,
  SyscallRead,
  TcpReadAlloc8k,
  TcpReadAlloc64k,
  Http2SettingsWrites,
  Http2PingsSent,
  Http2WritesBegun,
  Http2TransportStalls,
  Http2StreamStalls,
  CqPluckCreates,
  CqNextCreates,
  CqCallbackCreates,
  COUNTER_COUNT
};
// Just make sure they have the same number of counters.
static_assert(static_cast<int>(grpcwrap_stats_counter::COUNTER_COUNT) ==
                  static_cast<int>(grpc_core::GlobalStats::Counter::COUNT),
              "Counter count must be the same");

enum grpcwrap_stats_histogram {
  CallInitialSize,
  TcpWriteSize,
  TcpWriteIovSize,
  TcpReadSize,
  TcpReadOffer,
  TcpReadOfferIovSize,
  Http2SendMessageSize,
  Http2MetadataSize,
  HISTOGRAM_COUNT
};
// Just make sure they have the same number of histograms.
static_assert(static_cast<int>(grpcwrap_stats_histogram::HISTOGRAM_COUNT) ==
                  static_cast<int>(grpc_core::GlobalStats::Histogram::COUNT),
              "Histogram count must be the same");

GPR_EXPORT grpcwrap_stats* GPR_CALLTYPE grpcwrap_stats_collect() {
  return (grpcwrap_stats*)grpc_core::global_stats().Collect().release();
}

GPR_EXPORT void GPR_CALLTYPE grpcwrap_stats_free(grpcwrap_stats* stats) {
  auto s = (grpc_core::GlobalStats*)stats;
  delete s;
}

GPR_EXPORT uint64_t GPR_CALLTYPE grpcwrap_stats_get_counter(
    const grpcwrap_stats* stats, grpcwrap_stats_counter which) {
  auto s = (const grpc_core::GlobalStats*)stats;
  return s->counters[which];
}

GPR_EXPORT grpc_slice GPR_CALLTYPE
grpcwrap_stats_counter_name(grpcwrap_stats_counter which) {
  auto name = grpc_core::GlobalStats::counter_name[which];
  auto slice = grpc_slice_from_static_buffer(name.data(), name.size());
  return slice;
}

GPR_EXPORT grpc_slice GPR_CALLTYPE
grpcwrap_stats_counter_doc(grpcwrap_stats_counter which) {
  auto doc = grpc_core::GlobalStats::counter_doc[which];
  auto slice = grpc_slice_from_static_buffer(doc.data(), doc.size());
  return slice;
}

GPR_EXPORT double GPR_CALLTYPE grpcwrap_stats_get_histogram_percentile(
    const grpcwrap_stats* stats, grpcwrap_stats_histogram which,
    double percentile) {
  auto s = (const grpc_core::GlobalStats*)stats;
  return s->histogram(static_cast<grpc_core::GlobalStats::Histogram>(which))
      .Percentile(percentile);
}

GPR_EXPORT double GPR_CALLTYPE grpcwrap_stats_get_histogram_count(
    const grpcwrap_stats* stats, grpcwrap_stats_histogram which) {
  auto s = (const grpc_core::GlobalStats*)stats;
  return s->histogram(static_cast<grpc_core::GlobalStats::Histogram>(which))
      .Count();
}

GPR_EXPORT grpc_slice GPR_CALLTYPE
grpcwrap_stats_histogram_name(grpcwrap_stats_histogram which) {
  auto name = grpc_core::GlobalStats::histogram_name[which];
  auto slice = grpc_slice_from_static_buffer(name.data(), name.size());
  return slice;
}

GPR_EXPORT grpc_slice GPR_CALLTYPE
grpcwrap_stats_histogram_doc(grpcwrap_stats_histogram which) {
  auto doc = grpc_core::GlobalStats::histogram_doc[which];
  auto slice = grpc_slice_from_static_buffer(doc.data(), doc.size());
  return slice;
}
