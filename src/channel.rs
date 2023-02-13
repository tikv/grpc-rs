// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use std::{cmp, i32, ptr};

use crate::{
    grpc_sys::{self, gpr_timespec, grpc_arg_pointer_vtable, grpc_channel, grpc_channel_args},
    Deadline,
};
use libc::{self, c_char, c_int};

use crate::call::{Call, Method};
use crate::cq::CompletionQueue;
use crate::env::Environment;
use crate::error::Result;
use crate::task::CallTag;
use crate::task::Kicker;
use crate::{CallOption, ChannelCredentials};
use crate::{ResourceQuota, RpcStatusCode};

pub use crate::grpc_sys::{
    grpc_compression_algorithm as CompressionAlgorithms,
    grpc_compression_level as CompressionLevel, grpc_connectivity_state as ConnectivityState,
};

/// Ref: http://www.grpc.io/docs/guides/wire.html#user-agents
fn format_user_agent_string(agent: &str) -> CString {
    let version = env!("CARGO_PKG_VERSION");
    let trimed_agent = agent.trim();
    let val = if trimed_agent.is_empty() {
        format!("grpc-rust/{version}")
    } else {
        format!("{trimed_agent} grpc-rust/{version}")
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
    Pointer(ResourceQuota, *const grpc_arg_pointer_vtable),
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
    credentials: Option<ChannelCredentials>,
}

impl ChannelBuilder {
    /// Initialize a new [`ChannelBuilder`].
    pub fn new(env: Arc<Environment>) -> ChannelBuilder {
        ChannelBuilder {
            env,
            options: HashMap::new(),
            credentials: None,
        }
    }

    /// Set default authority to pass if none specified on call construction.
    pub fn default_authority<S: Into<Vec<u8>>>(mut self, authority: S) -> ChannelBuilder {
        let authority = CString::new(authority).unwrap();
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_DEFAULT_AUTHORITY),
            Options::String(authority),
        );
        self
    }

    /// Set resource quota by consuming a ResourceQuota
    pub fn set_resource_quota(mut self, quota: ResourceQuota) -> ChannelBuilder {
        unsafe {
            self.options.insert(
                Cow::Borrowed(grpcio_sys::GRPC_ARG_RESOURCE_QUOTA),
                Options::Pointer(quota, grpc_sys::grpc_resource_quota_arg_vtable()),
            );
        }
        self
    }

    /// Set maximum number of concurrent incoming streams to allow on a HTTP/2 connection.
    pub fn max_concurrent_stream(mut self, num: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_MAX_CONCURRENT_STREAMS),
            Options::Integer(num),
        );
        self
    }

    /// Set maximum message length that the channel can receive. `-1` means unlimited.
    pub fn max_receive_message_len(mut self, len: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_MAX_RECEIVE_MESSAGE_LENGTH),
            Options::Integer(len),
        );
        self
    }

    /// Set maximum message length that the channel can send. `-1` means unlimited.
    pub fn max_send_message_len(mut self, len: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_MAX_SEND_MESSAGE_LENGTH),
            Options::Integer(len),
        );
        self
    }

    /// Set maximum time between subsequent connection attempts.
    pub fn max_reconnect_backoff(mut self, backoff: Duration) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_MAX_RECONNECT_BACKOFF_MS),
            Options::Integer(dur_to_ms(backoff)),
        );
        self
    }

    /// Set time between the first and second connection attempts.
    pub fn initial_reconnect_backoff(mut self, backoff: Duration) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_INITIAL_RECONNECT_BACKOFF_MS),
            Options::Integer(dur_to_ms(backoff)),
        );
        self
    }

    /// Set initial sequence number for HTTP/2 transports.
    pub fn https_initial_seq_number(mut self, number: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_INITIAL_SEQUENCE_NUMBER),
            Options::Integer(number),
        );
        self
    }

    /// Set amount to read ahead on individual streams. Defaults to 64KB. Larger
    /// values help throughput on high-latency connections.
    pub fn stream_initial_window_size(mut self, window_size: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_STREAM_LOOKAHEAD_BYTES),
            Options::Integer(window_size),
        );
        self
    }

    /// Set primary user agent, which goes at the start of the user-agent metadata sent on
    /// each request.
    pub fn primary_user_agent(mut self, agent: &str) -> ChannelBuilder {
        let agent_string = format_user_agent_string(agent);
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_PRIMARY_USER_AGENT_STRING),
            Options::String(agent_string),
        );
        self
    }

    /// Set whether to allow the use of `SO_REUSEPORT` if available. Defaults to `true`.
    pub fn reuse_port(mut self, reuse: bool) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_ALLOW_REUSEPORT),
            Options::Integer(reuse as i32),
        );
        self
    }

    /// Set the size of slice to try and read from the wire each time.
    pub fn tcp_read_chunk_size(mut self, bytes: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_TCP_READ_CHUNK_SIZE),
            Options::Integer(bytes),
        );
        self
    }

    /// Set the minimum size of slice to try and read from the wire each time.
    pub fn tcp_min_read_chunk_size(mut self, bytes: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_TCP_MIN_READ_CHUNK_SIZE),
            Options::Integer(bytes),
        );
        self
    }

    /// Set the maximum size of slice to try and read from the wire each time.
    pub fn tcp_max_read_chunk_size(mut self, bytes: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_TCP_MAX_READ_CHUNK_SIZE),
            Options::Integer(bytes),
        );
        self
    }

    /// How much data are we willing to queue up per stream if
    /// write_buffer_hint is set. This is an upper bound.
    pub fn http2_write_buffer_size(mut self, size: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_WRITE_BUFFER_SIZE),
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
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_MAX_FRAME_SIZE),
            Options::Integer(size),
        );
        self
    }

    /// Set whether to enable BDP probing.
    pub fn http2_bdp_probe(mut self, enable: bool) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_BDP_PROBE),
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
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_MIN_SENT_PING_INTERVAL_WITHOUT_DATA_MS),
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
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_MIN_RECV_PING_INTERVAL_WITHOUT_DATA_MS),
            Options::Integer(dur_to_ms(interval)),
        );
        self
    }

    /// How many pings can we send before needing to send a data frame or header
    /// frame? (0 indicates that an infinite number of pings can be sent without
    /// sending a data frame or header frame)
    pub fn http2_max_pings_without_data(mut self, num: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_MAX_PINGS_WITHOUT_DATA),
            Options::Integer(num),
        );
        self
    }

    /// How many misbehaving pings the server can bear before sending goaway and
    /// closing the transport? (0 indicates that the server can bear an infinite
    /// number of misbehaving pings)
    pub fn http2_max_ping_strikes(mut self, num: i32) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_HTTP2_MAX_PING_STRIKES),
            Options::Integer(num),
        );
        self
    }

    /// If set to zero, disables use of http proxies.
    pub fn enable_http_proxy(mut self, num: bool) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_ENABLE_HTTP_PROXY),
            Options::Integer(num as i32),
        );
        self
    }

    /// Set default compression algorithm for the channel.
    pub fn default_compression_algorithm(mut self, algo: CompressionAlgorithms) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_COMPRESSION_CHANNEL_DEFAULT_ALGORITHM),
            Options::Integer(algo as i32),
        );
        self
    }

    /// Set default gzip compression level.
    #[cfg(feature = "nightly")]
    pub fn default_gzip_compression_level(mut self, level: usize) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_GZIP_COMPRESSION_LEVEL),
            Options::Integer(level as i32),
        );
        self
    }

    /// Set default grpc min message size to compression.
    #[cfg(feature = "nightly")]
    pub fn default_grpc_min_message_size_to_compress(
        mut self,
        lower_bound: usize,
    ) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_MIN_MESSAGE_SIZE_TO_COMPRESS),
            Options::Integer(lower_bound as i32),
        );
        self
    }

    /// Set default compression level for the channel.
    pub fn default_compression_level(mut self, level: CompressionLevel) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_COMPRESSION_CHANNEL_DEFAULT_LEVEL),
            Options::Integer(level as i32),
        );
        self
    }

    /// After a duration of this time the client/server pings its peer to see
    /// if the transport is still alive.
    pub fn keepalive_time(mut self, timeout: Duration) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_KEEPALIVE_TIME_MS),
            Options::Integer(dur_to_ms(timeout)),
        );
        self
    }

    /// After waiting for a duration of this time, if the keepalive ping sender does
    /// not receive the ping ack, it will close the transport.
    pub fn keepalive_timeout(mut self, timeout: Duration) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_KEEPALIVE_TIMEOUT_MS),
            Options::Integer(dur_to_ms(timeout)),
        );
        self
    }

    /// Is it permissible to send keepalive pings without any outstanding streams.
    pub fn keepalive_permit_without_calls(mut self, allow: bool) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_KEEPALIVE_PERMIT_WITHOUT_CALLS),
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
            Cow::Borrowed(grpcio_sys::GRPC_ARG_OPTIMIZATION_TARGET),
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
            Cow::Borrowed(grpcio_sys::GRPC_ARG_LB_POLICY_NAME),
            Options::String(val.unwrap()),
        );
        self
    }

    /// Set use local subchannel pool
    ///
    /// This method allows channel use it's owned subchannel pool.
    pub fn use_local_subchannel_pool(mut self, enable: bool) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_USE_LOCAL_SUBCHANNEL_POOL),
            Options::Integer(enable as i32),
        );
        self
    }

    /// Enables retry functionality.  Defaults to true.  When enabled, transparent
    /// retries will be performed as appropriate, and configurable retries are
    /// enabled when they are configured via the service config. For details, see:
    ///   https://github.com/grpc/proposal/blob/master/A6-client-retries.md
    /// NOTE: Hedging functionality is not yet implemented.
    pub fn enable_retry(mut self, enable: bool) -> ChannelBuilder {
        self.options.insert(
            Cow::Borrowed(grpcio_sys::GRPC_ARG_ENABLE_RETRIES),
            Options::Integer(enable as i32),
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
    #[allow(clippy::useless_conversion)]
    #[allow(clippy::cmp_owned)]
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
                Options::Pointer(ref quota, vtable) => unsafe {
                    grpc_sys::grpcwrap_channel_args_set_pointer_vtable(
                        args,
                        i,
                        key,
                        quota.get_ptr() as _,
                        vtable,
                    )
                },
            }
        }
        ChannelArgs { args }
    }

    fn prepare_connect_args(&mut self) -> ChannelArgs {
        if let Entry::Vacant(e) = self.options.entry(Cow::Borrowed(
            grpcio_sys::GRPC_ARG_PRIMARY_USER_AGENT_STRING,
        )) {
            e.insert(Options::String(format_user_agent_string("")));
        }
        self.build_args()
    }

    /// Build an [`Channel`] that connects to a specific address.
    pub fn connect(mut self, addr: &str) -> Channel {
        let args = self.prepare_connect_args();
        let addr = CString::new(addr).unwrap();
        let addr_ptr = addr.as_ptr();
        let mut creds = self
            .credentials
            .unwrap_or_else(ChannelCredentials::insecure);
        let channel =
            unsafe { grpcio_sys::grpc_channel_create(addr_ptr, creds.as_mut_ptr(), args.args) };

        unsafe { Channel::new(self.env.pick_cq(), self.env, channel) }
    }

    /// Build an [`Channel`] taking over an established connection from
    /// a file descriptor. The target string given is purely informative to
    /// describe the endpoint of the connection. Takes ownership of the given
    /// file descriptor and will close it when the connection is closed.
    ///
    /// This function is available on posix systems only.
    ///
    /// # Safety
    ///
    /// The file descriptor must correspond to a connected stream socket. After
    /// this call, the socket must not be accessed (read / written / closed)
    /// by other code.
    #[cfg(unix)]
    pub unsafe fn connect_from_fd(mut self, target: &str, fd: ::std::os::raw::c_int) -> Channel {
        let args = self.prepare_connect_args();
        let target = CString::new(target).unwrap();
        let target_ptr = target.as_ptr();
        // Actually only insecure credentials are supported currently.
        let mut creds = self
            .credentials
            .unwrap_or_else(ChannelCredentials::insecure);
        let channel =
            grpcio_sys::grpc_channel_create_from_fd(target_ptr, fd, creds.as_mut_ptr(), args.args);

        Channel::new(self.env.pick_cq(), self.env, channel)
    }
}

#[cfg(feature = "_secure")]
mod secure_channel {
    use std::borrow::Cow;
    use std::ffi::CString;

    use crate::ChannelCredentials;

    use super::{ChannelBuilder, Options};

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

        /// Set the credentials used to build the connection.
        pub fn set_credentials(mut self, creds: ChannelCredentials) -> ChannelBuilder {
            self.credentials = Some(creds);
            self
        }
    }
}

pub struct ChannelArgs {
    args: *mut grpc_channel_args,
}

impl ChannelArgs {
    pub fn as_ptr(&self) -> *const grpc_channel_args {
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
    channel: *mut grpc_channel,
}

impl ChannelInner {
    // If try_to_connect is true, the channel will try to establish a connection, potentially
    // changing the state.
    fn check_connectivity_state(&self, try_to_connect: bool) -> ConnectivityState {
        unsafe {
            grpc_sys::grpc_channel_check_connectivity_state(self.channel, try_to_connect as _)
        }
    }
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

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}

impl Channel {
    /// Create a new channel. Avoid using this directly and use
    /// [`ChannelBuilder`] to build a [`Channel`] instead.
    ///
    /// # Safety
    ///
    /// The given grpc_channel must correspond to an instantiated grpc core
    /// channel. Takes exclusive ownership of the channel and will close it after
    /// use.
    pub unsafe fn new(
        cq: CompletionQueue,
        env: Arc<Environment>,
        channel: *mut grpc_channel,
    ) -> Channel {
        Channel {
            inner: Arc::new(ChannelInner { _env: env, channel }),
            cq,
        }
    }

    /// Create a lame channel that will fail all its operations.
    pub fn lame(env: Arc<Environment>, target: &str) -> Channel {
        unsafe {
            let target = CString::new(target).unwrap();
            let ch = grpc_sys::grpc_lame_client_channel_create(
                target.as_ptr(),
                RpcStatusCode::UNAVAILABLE.into(),
                b"call on lame client\0".as_ptr() as _,
            );
            Self::new(env.pick_cq(), env, ch)
        }
    }

    /// If try_to_connect is true, the channel will try to establish a connection, potentially
    /// changing the state.
    pub fn check_connectivity_state(&self, try_to_connect: bool) -> ConnectivityState {
        self.inner.check_connectivity_state(try_to_connect)
    }

    /// Blocking wait for channel state change or deadline expiration.
    ///
    /// `check_connectivity_state` needs to be called to get the current state. Returns false
    /// means deadline excceeds before observing any state changes.
    pub fn wait_for_state_change(
        &self,
        last_observed: ConnectivityState,
        deadline: impl Into<Deadline>,
    ) -> impl Future<Output = bool> {
        let (cq_f, prom) = CallTag::action_pair();
        let prom_box = Box::new(prom);
        let tag = Box::into_raw(prom_box);
        let should_wait = if let Ok(cq_ref) = self.cq.borrow() {
            unsafe {
                grpcio_sys::grpc_channel_watch_connectivity_state(
                    self.inner.channel,
                    last_observed,
                    deadline.into().spec(),
                    cq_ref.as_ptr(),
                    tag as *mut _,
                )
            }
            true
        } else {
            // It's already shutdown.
            false
        };
        async move { should_wait && cq_f.await.unwrap() }
    }

    /// Wait for this channel to be connected.
    ///
    /// Returns false means deadline excceeds before connection is connected.
    pub async fn wait_for_connected(&self, deadline: impl Into<Deadline>) -> bool {
        // Fast path, it's probably connected.
        let mut state = self.check_connectivity_state(true);
        if ConnectivityState::GRPC_CHANNEL_READY == state {
            return true;
        }
        let deadline = deadline.into();
        loop {
            if self.wait_for_state_change(state, deadline).await {
                state = self.check_connectivity_state(true);
                match state {
                    ConnectivityState::GRPC_CHANNEL_READY => return true,
                    ConnectivityState::GRPC_CHANNEL_SHUTDOWN => return false,
                    _ => (),
                }
                continue;
            }
            return false;
        }
    }

    /// Create a Kicker.
    pub(crate) fn create_kicker(&self) -> Result<Kicker> {
        let cq_ref = self.cq.borrow()?;
        let raw_call = unsafe {
            let ch = self.inner.channel;
            let cq = cq_ref.as_ptr();
            // Do not timeout.
            let timeout = gpr_timespec::inf_future();
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
                .map_or_else(gpr_timespec::inf_future, gpr_timespec::from);
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
            )
        };

        unsafe { Ok(Call::from_raw(raw_call, self.cq.clone())) }
    }

    pub(crate) fn cq(&self) -> &CompletionQueue {
        &self.cq
    }
}

#[cfg(test)]
#[cfg(feature = "nightly")]
mod tests {
    use crate::env::Environment;
    use crate::ChannelBuilder;
    use std::sync::Arc;

    #[test]
    #[cfg(feature = "nightly")]
    fn test_grpc_min_message_size_to_compress() {
        let env = Arc::new(Environment::new(1));
        let cb = ChannelBuilder::new(env);
        cb.default_grpc_min_message_size_to_compress(1);
    }
    #[test]
    #[cfg(feature = "nightly")]
    fn test_gzip_compression_level() {
        let env = Arc::new(Environment::new(1));
        let cb = ChannelBuilder::new(env);
        cb.default_gzip_compression_level(1);
    }
}
