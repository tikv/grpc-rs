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

use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::Arc;
use std::time::Duration;
use std::{cmp, i32, ptr};

use grpc_sys::{self, GprTimespec, GrpcChannel, GrpcChannelArgs};
use libc::{self, c_char, c_int};

use async::Kicker;
use call::{Call, Method};
use cq::CompletionQueue;
use env::Environment;
use error::Result;
use CallOption;

pub use grpc_sys::{
    GrpcCompressionAlgorithms as CompressionAlgorithms, GrpcCompressionLevel as CompressionLevel,
};

// hack: add a '\0' to be compatible with c string without extra allocation.
const OPT_DEFAULT_AUTHORITY: &[u8] = b"grpc.default_authority\0";
const OPT_MAX_CONCURRENT_STREAMS: &[u8] = b"grpc.max_concurrent_streams\0";
const OPT_MAX_RECEIVE_MESSAGE_LENGTH: &[u8] = b"grpc.max_receive_message_length\0";
const OPT_MAX_SEND_MESSAGE_LENGTH: &[u8] = b"grpc.max_send_message_length\0";
const OPT_MAX_RECONNECT_BACKOFF_MS: &[u8] = b"grpc.max_reconnect_backoff_ms\0";
const OPT_INITIAL_RECONNECT_BACKOFF_MS: &[u8] = b"grpc.initial_reconnect_backoff_ms\0";
const OPT_HTTP2_INITIAL_SEQUENCE_NUMBER: &[u8] = b"grpc.http2.initial_sequence_number\0";
const OPT_SO_REUSE_PORT: &[u8] = b"grpc.so_reuseport\0";
const OPT_STREAM_INITIAL_WINDOW_SIZE: &[u8] = b"grpc.http2.lookahead_bytes\0";
const OPT_TCP_READ_CHUNK_SIZE: &[u8] = b"grpc.experimental.tcp_read_chunk_size\0";
const OPT_TCP_MIN_READ_CHUNK_SIZE: &[u8] = b"grpc.experimental.tcp_min_read_chunk_size\0";
const OPT_TCP_MAX_READ_CHUNK_SIZE: &[u8] = b"grpc.experimental.tcp_max_read_chunk_size\0";
const OPT_HTTP2_WRITE_BUFFER_SIZE: &[u8] = b"grpc.http2.write_buffer_size\0";
const OPT_HTTP2_MAX_FRAME_SIZE: &[u8] = b"grpc.http2.max_frame_size\0";
const OPT_HTTP2_BDP_PROBE: &[u8] = b"grpc.http2.bdp_probe\0";
const OPT_HTTP2_MIN_SENT_PING_INTERVAL_WITHOUT_DATA_MS: &[u8] =
    b"grpc.http2.min_time_between_pings_ms\0";
const OPT_HTTP2_MIN_RECV_PING_INTERVAL_WITHOUT_DATA_MS: &[u8] =
    b"grpc.http2.min_ping_interval_without_data_ms\0";
const OPT_HTTP2_MAX_PINGS_WITHOUT_DATA: &[u8] = b"grpc.http2.max_pings_without_data\0";
const OPT_HTTP2_MAX_PING_STRIKES: &[u8] = b"grpc.http2.max_ping_strikes\0";
const OPT_DEFALUT_COMPRESSION_ALGORITHM: &[u8] = b"grpc.default_compression_algorithm\0";
const OPT_DEFAULT_COMPRESSION_LEVEL: &[u8] = b"grpc.default_compression_level\0";
const OPT_KEEPALIVE_TIME_MS: &[u8] = b"grpc.keepalive_time_ms\0";
const OPT_KEEPALIVE_TIMEOUT_MS: &[u8] = b"grpc.keepalive_timeout_ms\0";
const OPT_KEEPALIVE_PERMIT_WITHOUT_CALLS: &[u8] = b"grpc.keepalive_permit_without_calls\0";
const OPT_OPTIMIZATION_TARGET: &[u8] = b"grpc.optimization_target\0";
const PRIMARY_USER_AGENT_STRING: &[u8] = b"grpc.primary_user_agent\0";
const OPT_GRPC_ARG_LB_POLICY_NAME: &[u8] = b"grpc.lb_policy_name\0";

/// Ref: http://www.grpc.io/docs/guides/wire.html#user-agents
fn format_user_agent_string(agent: &str) -> CString {
    let version = env!("CARGO_PKG_VERSION");
    let trimed_agent = agent.trim();
    let val = if trimed_agent.is_empty() {
        format!("grpc-rust/{}", version)
    } else {
        format!("{} grpc-rust/{}", trimed_agent, version)
    };
    CString::new(val).unwrap()
}

fn dur_to_ms(dur: Duration) -> i32 {
    let millis = dur.as_secs() * 1000 + dur.subsec_nanos() as u64 / 1_000_000;
    cmp::min(i32::MAX as u64, millis) as i32
}

enum Options {
    Integer(i32),
    String(CString),
}

/// The optimization target for a [`Channel`].
#[derive(Clone, Copy)]
pub enum OptTarget {
    /// Minimize latency at the cost of throughput.
    Latency,
    /// Balance latency and throughput.
    Blend,
    /// Maximize throughput at the expense of latency.
    Throughput,
}

#[derive(Clone, Copy)]
pub enum LbPolicy {
    PickFirst,
    RoundRobin,
}

/// [`Channel`] factory in order to configure the properties.
pub struct ChannelBuilder {
    env: Arc<Environment>,
    options: HashMap<Cow<'static, [u8]>, Options>,
}

impl ChannelBuilder {
    /// Initialize a new [`ChannelBuilder`].
    pub fn new(env: Arc<Environment>) -> ChannelBuilder {
        ChannelBuilder {
            env,
            options: HashMap::new(),
        }
    }

    /// Set default authority to pass if none specified on call construction.
    pub fn default_authority<S: Into<Vec<u8>>>(mut self, authority: S) -> ChannelBuilder {
        let authority = CString::new(authority).unwrap();
        self.options.insert(
            Cow::Borrowed(OPT_DEFAULT_AUTHORITY),
            Options::String(authority),
        );
        self
    }

    /// Set maximum number of concurrent incoming streams to allow on a HTTP/2 connection.
    pub fn max_concurrent_stream(mut self, num: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_MAX_CONCURRENT_STREAMS),
            Options::Integer(num),
        );
        self
    }

    /// Set maximum message length that the channel can receive. `usize::MAX` means unlimited.
    pub fn max_receive_message_len(mut self, len: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_MAX_RECEIVE_MESSAGE_LENGTH),
            Options::Integer(len),
        );
        self
    }

    /// Set maximum message length that the channel can send. `-1` means unlimited.
    pub fn max_send_message_len(mut self, len: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_MAX_SEND_MESSAGE_LENGTH),
            Options::Integer(len),
        );
        self
    }

    /// Set maximum time between subsequent connection attempts.
    pub fn max_reconnect_backoff(mut self, backoff: Duration) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_MAX_RECONNECT_BACKOFF_MS),
            Options::Integer(dur_to_ms(backoff)),
        );
        self
    }

    /// Set time between the first and second connection attempts.
    pub fn initial_reconnect_backoff(mut self, backoff: Duration) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_INITIAL_RECONNECT_BACKOFF_MS),
            Options::Integer(dur_to_ms(backoff)),
        );
        self
    }

    /// Set initial sequence number for HTTP/2 transports.
    pub fn https_initial_seq_number(mut self, number: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_HTTP2_INITIAL_SEQUENCE_NUMBER),
            Options::Integer(number),
        );
        self
    }

    /// Set amount to read ahead on individual streams. Defaults to 64KB. Larger
    /// values help throughput on high-latency connections.
    pub fn stream_initial_window_size(mut self, window_size: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_STREAM_INITIAL_WINDOW_SIZE),
            Options::Integer(window_size),
        );
        self
    }

    /// Set primary user agent, which goes at the start of the user-agent metadata sent on
    /// each request.
    pub fn primary_user_agent(mut self, agent: &str) -> ChannelBuilder {
        let agent_string = format_user_agent_string(agent);
        self.options.insert(
            Cow::Borrowed(PRIMARY_USER_AGENT_STRING),
            Options::String(agent_string),
        );
        self
    }

    /// Set whether to allow the use of `SO_REUSEPORT` if available. Defaults to `true`.
    pub fn reuse_port(mut self, reuse: bool) -> ChannelBuilder {
        let opt = if reuse { 1 } else { 0 };
        self.options
            .insert(Cow::Borrowed(OPT_SO_REUSE_PORT), Options::Integer(opt));
        self
    }

    /// Set the size of slice to try and read from the wire each time.
    pub fn tcp_read_chunk_size(mut self, bytes: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_TCP_READ_CHUNK_SIZE),
            Options::Integer(bytes),
        );
        self
    }

    /// Set the minimum size of slice to try and read from the wire each time.
    pub fn tcp_min_read_chunk_size(mut self, bytes: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_TCP_MIN_READ_CHUNK_SIZE),
            Options::Integer(bytes),
        );
        self
    }

    /// Set the maximum size of slice to try and read from the wire each time.
    pub fn tcp_max_read_chunk_size(mut self, bytes: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_TCP_MAX_READ_CHUNK_SIZE),
            Options::Integer(bytes),
        );
        self
    }

    /// How much data are we willing to queue up per stream if
    /// write_buffer_hint is set. This is an upper bound.
    pub fn http2_write_buffer_size(mut self, size: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_HTTP2_WRITE_BUFFER_SIZE),
            Options::Integer(size),
        );
        self
    }

    /// How big a frame are we willing to receive via HTTP/2.
    /// Min 16384, max 16777215.
    /// Larger values give lower CPU usage for large messages, but more head of line
    /// blocking for small messages.
    pub fn http2_max_frame_size(mut self, size: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_HTTP2_MAX_FRAME_SIZE),
            Options::Integer(size),
        );
        self
    }

    /// Set whether to enable BDP probing.
    pub fn http2_bdp_probe(mut self, enable: bool) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_HTTP2_BDP_PROBE),
            Options::Integer(enable as i32),
        );
        self
    }

    /// Minimum time between sending successive ping frames without receiving any
    /// data frame.
    pub fn http2_min_sent_ping_interval_without_data(
        mut self,
        interval: Duration,
    ) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_HTTP2_MIN_SENT_PING_INTERVAL_WITHOUT_DATA_MS),
            Options::Integer(dur_to_ms(interval)),
        );
        self
    }

    /// Minimum allowed time between receiving successive ping frames without
    /// sending any data frame.
    pub fn http2_min_recv_ping_interval_without_data(
        mut self,
        interval: Duration,
    ) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_HTTP2_MIN_RECV_PING_INTERVAL_WITHOUT_DATA_MS),
            Options::Integer(dur_to_ms(interval)),
        );
        self
    }

    /// How many pings can we send before needing to send a data frame or header
    /// frame? (0 indicates that an infinite number of pings can be sent without
    /// sending a data frame or header frame)
    pub fn http2_max_pings_without_data(mut self, num: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_HTTP2_MAX_PINGS_WITHOUT_DATA),
            Options::Integer(num),
        );
        self
    }

    /// How many misbehaving pings the server can bear before sending goaway and
    /// closing the transport? (0 indicates that the server can bear an infinite
    /// number of misbehaving pings)
    pub fn http2_max_ping_strikes(mut self, num: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_HTTP2_MAX_PING_STRIKES),
            Options::Integer(num),
        );
        self
    }

    /// Set default compression algorithm for the channel.
    pub fn default_compression_algorithm(mut self, algo: CompressionAlgorithms) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_DEFALUT_COMPRESSION_ALGORITHM),
            Options::Integer(algo as i32),
        );
        self
    }

    /// Set default compression level for the channel.
    pub fn default_compression_level(mut self, level: CompressionLevel) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_DEFAULT_COMPRESSION_LEVEL),
            Options::Integer(level as i32),
        );
        self
    }

    /// After a duration of this time the client/server pings its peer to see
    /// if the transport is still alive.
    pub fn keepalive_time(mut self, timeout: Duration) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_KEEPALIVE_TIME_MS),
            Options::Integer(dur_to_ms(timeout)),
        );
        self
    }

    /// After waiting for a duration of this time, if the keepalive ping sender does
    /// not receive the ping ack, it will close the transport.
    pub fn keepalive_timeout(mut self, timeout: Duration) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_KEEPALIVE_TIMEOUT_MS),
            Options::Integer(dur_to_ms(timeout)),
        );
        self
    }

    /// Is it permissible to send keepalive pings without any outstanding streams.
    pub fn keepalive_permit_without_calls(mut self, allow: bool) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(OPT_KEEPALIVE_PERMIT_WITHOUT_CALLS),
            Options::Integer(allow as i32),
        );
        self
    }

    /// Set optimization target for the channel. See [`OptTarget`] for all available
    /// optimization targets. Defaults to `OptTarget::Blend`.
    pub fn optimize_for(mut self, target: OptTarget) -> ChannelBuilder {
        let val = match target {
            OptTarget::Latency => CString::new("latency"),
            OptTarget::Blend => CString::new("blend"),
            OptTarget::Throughput => CString::new("throughput"),
        };
        self.options.insert(
            Cow::Borrowed(OPT_OPTIMIZATION_TARGET),
            Options::String(val.unwrap()),
        );
        self
    }

    /// Set LbPolicy for channel
    ///
    /// This method allows one to set the load-balancing policy for a given channel.
    pub fn load_balancing_policy(mut self, lb_policy: LbPolicy) -> ChannelBuilder {
        let val = match lb_policy {
            LbPolicy::PickFirst => CString::new("pick_first"),
            LbPolicy::RoundRobin => CString::new("round_robin"),
        };
        self.options.insert(
            Cow::Borrowed(OPT_GRPC_ARG_LB_POLICY_NAME),
            Options::String(val.unwrap()),
        );
        self
    }

    /// Set a raw integer configuration.
    ///
    /// This method is only for bench usage, users should use the encapsulated API instead.
    #[doc(hidden)]
    pub fn raw_cfg_int(mut self, key: CString, val: i32) -> ChannelBuilder {
        self.options
            .insert(Cow::Owned(key.into_bytes_with_nul()), Options::Integer(val));
        self
    }

    /// Set a raw string configuration.
    ///
    /// This method is only for bench usage, users should use the encapsulated API instead.
    #[doc(hidden)]
    pub fn raw_cfg_string(mut self, key: CString, val: CString) -> ChannelBuilder {
        self.options
            .insert(Cow::Owned(key.into_bytes_with_nul()), Options::String(val));
        self
    }

    /// Build `ChannelArgs` from the current configuration.
    ///
    /// This method is only for bench usage, users should use the encapsulated API instead.
    #[doc(hidden)]
    #[cfg_attr(feature = "cargo-clippy", allow(identity_conversion))]
    pub fn build_args(&self) -> ChannelArgs {
        let args = unsafe { grpc_sys::grpcwrap_channel_args_create(self.options.len()) };
        for (i, (k, v)) in self.options.iter().enumerate() {
            let key = k.as_ptr() as *const c_char;
            match *v {
                Options::Integer(val) => unsafe {
                    // On most modern compiler and architect, c_int is the same as i32,
                    // panic directly to simplify signature.
                    assert!(
                        val <= i32::from(libc::INT_MAX) && val >= i32::from(libc::INT_MIN),
                        "{} is out of range for {:?}",
                        val,
                        CStr::from_bytes_with_nul(k).unwrap()
                    );
                    grpc_sys::grpcwrap_channel_args_set_integer(args, i, key, val as c_int)
                },
                Options::String(ref val) => unsafe {
                    grpc_sys::grpcwrap_channel_args_set_string(args, i, key, val.as_ptr())
                },
            }
        }
        ChannelArgs { args }
    }

    fn prepare_connect_args(&mut self) -> ChannelArgs {
        if let Entry::Vacant(e) = self.options.entry(Cow::Borrowed(PRIMARY_USER_AGENT_STRING)) {
            e.insert(Options::String(format_user_agent_string("")));
        }
        self.build_args()
    }

    /// Build an insecure [`Channel`] that connects to a specific address.
    pub fn connect(mut self, addr: &str) -> Channel {
        let args = self.prepare_connect_args();
        let addr = CString::new(addr).unwrap();
        let addr_ptr = addr.as_ptr();
        let channel =
            unsafe { grpc_sys::grpc_insecure_channel_create(addr_ptr, args.args, ptr::null_mut()) };

        Channel::new(self.env.pick_cq(), self.env, channel)
    }
}

#[cfg(feature = "secure")]
mod secure_channel {
    use std::borrow::Cow;
    use std::ffi::CString;
    use std::ptr;

    use grpc_sys;

    use credentials::ChannelCredentials;

    use super::{Channel, ChannelBuilder, Options};

    const OPT_SSL_TARGET_NAME_OVERRIDE: &[u8] = b"grpc.ssl_target_name_override\0";

    impl ChannelBuilder {
        /// The caller of the secure_channel_create functions may override the target name used
        /// for SSL host name checking using this channel argument.
        ///
        /// This *should* be used for testing only.
        #[doc(hidden)]
        pub fn override_ssl_target<S: Into<Vec<u8>>>(mut self, target: S) -> ChannelBuilder {
            let target = CString::new(target).unwrap();
            self.options.insert(
                Cow::Borrowed(OPT_SSL_TARGET_NAME_OVERRIDE),
                Options::String(target),
            );
            self
        }

        /// Build a secure [`Channel`] that connects to a specific address.
        pub fn secure_connect(mut self, addr: &str, mut creds: ChannelCredentials) -> Channel {
            let args = self.prepare_connect_args();
            let addr = CString::new(addr).unwrap();
            let addr_ptr = addr.as_ptr();
            let channel = unsafe {
                grpc_sys::grpc_secure_channel_create(
                    creds.as_mut_ptr(),
                    addr_ptr,
                    args.args,
                    ptr::null_mut(),
                )
            };

            Channel::new(self.env.pick_cq(), self.env, channel)
        }
    }
}

pub struct ChannelArgs {
    args: *mut GrpcChannelArgs,
}

impl ChannelArgs {
    pub fn as_ptr(&self) -> *const GrpcChannelArgs {
        self.args
    }
}

impl Drop for ChannelArgs {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpcwrap_channel_args_destroy(self.args) }
    }
}

struct ChannelInner {
    _env: Arc<Environment>,
    channel: *mut GrpcChannel,
}

impl Drop for ChannelInner {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_channel_destroy(self.channel);
        }
    }
}

/// A gRPC channel.
///
/// Channels are an abstraction of long-lived connections to remote servers. More client objects
/// can reuse the same channel.
///
/// Use [`ChannelBuilder`] to build a [`Channel`].
#[derive(Clone)]
pub struct Channel {
    inner: Arc<ChannelInner>,
    cq: CompletionQueue,
}

unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}

impl Channel {
    fn new(cq: CompletionQueue, env: Arc<Environment>, channel: *mut GrpcChannel) -> Channel {
        Channel {
            inner: Arc::new(ChannelInner { _env: env, channel }),
            cq,
        }
    }

    /// Create a Kicker.
    pub(crate) fn create_kicker(&self) -> Result<Kicker> {
        let cq_ref = self.cq.borrow()?;
        let raw_call = unsafe {
            let ch = self.inner.channel;
            let cq = cq_ref.as_ptr();
            // Do not timeout.
            let timeout = GprTimespec::inf_future();
            grpc_sys::grpcwrap_channel_create_call(
                ch,
                ptr::null_mut(),
                0,
                cq,
                ptr::null(),
                0,
                ptr::null(),
                0,
                timeout,
                ptr::null_mut(),
            )
        };
        let call = unsafe { Call::from_raw(raw_call, self.cq.clone()) };
        Ok(Kicker::from_call(call))
    }

    /// Create a call using the method and option.
    pub(crate) fn create_call<Req, Resp>(
        &self,
        method: &Method<Req, Resp>,
        opt: &CallOption,
    ) -> Result<Call> {
        let cq_ref = self.cq.borrow()?;
        let raw_call = unsafe {
            let ch = self.inner.channel;
            let cq = cq_ref.as_ptr();
            let method_ptr = method.name.as_ptr();
            let method_len = method.name.len();
            let timeout = opt
                .get_timeout()
                .map_or_else(GprTimespec::inf_future, GprTimespec::from);
            grpc_sys::grpcwrap_channel_create_call(
                ch,
                ptr::null_mut(),
                0,
                cq,
                method_ptr as *const _,
                method_len,
                ptr::null(),
                0,
                timeout,
                ptr::null_mut(),
            )
        };

        unsafe { Ok(Call::from_raw(raw_call, self.cq.clone())) }
    }

    pub(crate) fn cq(&self) -> &CompletionQueue {
        &self.cq
    }
}
