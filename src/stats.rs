// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

use std::{
    fmt::{self, Debug, Display},
    slice, str,
};

use grpcio_sys::*;

unsafe fn slice_to_string(slice: grpc_slice) -> String {
    let mut len = 0;
    let ptr = grpcwrap_slice_raw_offset(&slice, 0, &mut len);
    let string = str::from_utf8_unchecked(slice::from_raw_parts(ptr as _, len)).to_owned();
    grpc_slice_unref(slice);
    string
}

macro_rules! stats_item {
    (
        $(#[$item_attr:meta])*
        $item:ident($inner:ident);
        $name_func:ident;
        $doc_func:ident;
        $(
            $(#[$konst_attr:meta])* ($num:path, $konst:ident);
        )+
    ) => {
        $(#[$item_attr])*
        pub struct $item($inner);

        impl $item {
        $(
            $(#[$konst_attr])*
            pub const $konst: $item = $item($num);
        )+
            /// All kinds of this stat.
            pub const ALL: &[$item] = &[ $( $item::$konst, )+ ];

            // Returns name of this stat.
            pub fn name(&self) -> String {
                unsafe {
                    let slice = $name_func(self.0);
                    slice_to_string(slice)
                }
            }

            // Returns doc of this stat.
            pub fn doc(&self) -> String {
                unsafe {
                    let slice = $doc_func(self.0);
                    slice_to_string(slice)
                }
            }
        }

        impl Display for $item {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                Debug::fmt(self, f)
            }
        }

        impl Debug for $item {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.name(),
                )
            }
        }
    }
}

stats_item! {
    /// gRPC stats counter.
    #[derive(PartialEq, Eq, Clone, Copy)]
    Counter(grpcwrap_stats_counter);
    grpcwrap_stats_counter_name;
    grpcwrap_stats_counter_doc;
    /// Number of client side calls created by this process
    (grpcwrap_stats_counter::ClientCallsCreated, CLIENT_CALLS_CREATED);
    /// Number of server side calls created by this process
    (grpcwrap_stats_counter::ServerCallsCreated, SERVER_CALLS_CREATED);
    /// Number of client channels created
    (grpcwrap_stats_counter::ClientChannelsCreated, CLIENT_CHANNELS_CREATED);
    /// Number of client subchannels created
    (grpcwrap_stats_counter::ClientSubchannelsCreated, CLIENT_SUBCHANNELS_CREATED);
    /// Number of server channels created
    (grpcwrap_stats_counter::ServerChannelsCreated, SERVER_CHANNELS_CREATED);
    /// Number of insecure connections created
    (grpcwrap_stats_counter::InsecureConnectionsCreated, INSECURE_CONNECTIONS_CREATED);
    /// Number of write syscalls (or equivalent - eg sendmsg) made by this process
    (grpcwrap_stats_counter::SyscallWrite, SYSCALL_WRITE);
    /// Number of read syscalls (or equivalent - eg recvmsg) made by this process
    (grpcwrap_stats_counter::SyscallRead, SYSCALL_READ);
    /// Number of 8k allocations by the TCP subsystem for reading
    (grpcwrap_stats_counter::TcpReadAlloc8k, TCP_READ_ALLOC_8K);
    /// Number of 64k allocations by the TCP subsystem for reading
    (grpcwrap_stats_counter::TcpReadAlloc64k, TCP_READ_ALLOC_64K);
    /// Number of settings frames sent
    (grpcwrap_stats_counter::Http2SettingsWrites, HTTP2_SETTINGS_WRITES);
    /// Number of HTTP2 pings sent by process
    (grpcwrap_stats_counter::Http2PingsSent, HTTP_2PINGS_SENT);
    /// Number of HTTP2 writes initiated
    (grpcwrap_stats_counter::Http2WritesBegun, HTTP2_WRITES_BEGUN);
    /// Number of times sending was completely stalled by the transport flow control window
    (grpcwrap_stats_counter::Http2TransportStalls, HTTP2_TRANSPORT_STALLS);
    /// Number of times sending was completely stalled by the stream flow control window
    (grpcwrap_stats_counter::Http2StreamStalls, HTTP2_STREAM_STALLS);
    /// Number of completion queues created for cq_pluck (indicates sync api usage)
    (grpcwrap_stats_counter::CqPluckCreates, CQ_PLUCK_CREATES);
    /// Number of completion queues created for cq_next (indicates cq async api usage)
    (grpcwrap_stats_counter::CqNextCreates, CQ_NEXT_CREATES);
    /// Number of completion queues created for cq_callback (indicates callback api usage)
    (grpcwrap_stats_counter::CqCallbackCreates, CQ_CALLBACK_CREATES);
}

stats_item! {
    /// gRPC stats histogram.
    #[derive(PartialEq, Eq, Clone, Copy)]
    Histogram(grpcwrap_stats_histogram);
    grpcwrap_stats_histogram_name;
    grpcwrap_stats_histogram_doc;
    /// Initial size of the grpc_call arena created at call start
    (grpcwrap_stats_histogram::CallInitialSize, CALL_INITIAL_SIZE);
    /// Number of bytes offered to each syscall_write
    (grpcwrap_stats_histogram::TcpWriteSize, TCP_WRITE_SIZE);
    /// Number of byte segments offered to each syscall_write
    (grpcwrap_stats_histogram::TcpWriteIovSize, TCP_WRITE_IOV_SIZE);
    /// Number of bytes received by each syscall_read
    (grpcwrap_stats_histogram::TcpReadSize, TCP_READ_SIZE);
    /// Number of bytes offered to each syscall_read
    (grpcwrap_stats_histogram::TcpReadOffer, TCP_READ_OFFER);
    /// Number of byte segments offered to each syscall_read
    (grpcwrap_stats_histogram::TcpReadOfferIovSize, TCP_READ_OFFER_IOV_SIZE);
    /// Size of messages received by HTTP2 transport
    (grpcwrap_stats_histogram::Http2SendMessageSize, HTTP2_SEND_MESSAGE_SIZE);
    /// Number of bytes consumed by metadata, according to HPACK accounting rules
    (grpcwrap_stats_histogram::Http2MetadataSize, HTTP2_METADATA_SIZE);
}

pub struct Stats {
    stats: *mut grpcwrap_stats,
}

impl Drop for Stats {
    fn drop(&mut self) {
        unsafe {
            grpcwrap_stats_free(self.stats);
        }
    }
}

impl Stats {
    pub fn collect() -> Stats {
        let stats = unsafe { grpcwrap_stats_collect() };
        Stats { stats }
    }

    pub fn counter(&self, which: Counter) -> u64 {
        unsafe { grpcwrap_stats_get_counter(self.stats, which.0) }
    }

    pub fn histogram_percentile(&self, which: Histogram, percentile: f64) -> f64 {
        unsafe { grpcwrap_stats_get_histogram_percentile(self.stats, which.0, percentile) }
    }

    pub fn histogram_count(&self, which: Histogram) -> u64 {
        unsafe { grpcwrap_stats_get_histogram_count(self.stats, which.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_doc() {
        for i in Counter::ALL {
            let _ = i.name();
            let _ = i.doc();
        }
        for i in Histogram::ALL {
            let _ = i.name();
            let _ = i.doc();
        }
    }

    #[test]
    fn test_counter() {
        let stats = Stats::collect();
        for i in Counter::ALL {
            let _ = stats.counter(*i);
        }
    }

    #[test]
    fn test_histogram() {
        let stats = Stats::collect();
        for i in Histogram::ALL {
            let _ = stats.histogram_count(*i);
            let _ = stats.histogram_percentile(*i, 0.99);
        }
    }
}
