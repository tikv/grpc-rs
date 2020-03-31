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

#![allow(unknown_lints)]

extern crate libc;

use libc::{c_char, c_int, c_uint, c_void, size_t};
use std::time::Duration;

/// The clocks gRPC supports.
///
/// Based on `gpr_clock_type`.
#[derive(Clone, Copy)]
#[repr(C)]
pub enum GprClockType {
    /// Monotonic clock. Epoch undefined. Always moves forward.
    Monotonic = 0,

    /// Realtime clock. May jump forwards or backwards. Settable by the system administrator.
    /// Has its epoch at 0:00:00 UTC 1 Jan 1970.
    Realtime,

    /// CPU cycle time obtained by rdtsc instruction on x86 platforms. Epoch undefined. Degrades
    /// to [`GprClockType::Realtime`] on other platforms.
    Precise,

    /// Unmeasurable clock type: no base, created by taking the difference between two times.
    Timespan,
}

/// Analogous to struct `timespec`. On some machines, absolute times may be in local time.
///
/// Based on `gpr_timespec`.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GprTimespec {
    pub tv_sec: i64,
    pub tv_nsec: i32,

    /// Against which clock was this time measured? (or [`GprClockType::Timespan`] if this is a
    /// relative time measure)
    pub clock_type: GprClockType,
}

impl GprTimespec {
    pub fn inf_future() -> GprTimespec {
        unsafe { gpr_inf_future(GprClockType::Realtime) }
    }
}

impl From<Duration> for GprTimespec {
    fn from(dur: Duration) -> GprTimespec {
        GprTimespec {
            tv_sec: dur.as_secs() as i64,
            tv_nsec: dur.subsec_nanos() as i32,
            clock_type: GprClockType::Timespan,
        }
    }
}

/// Result of a remote procedure call.
///
/// Based on `grpc_status_code`.
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GrpcStatusCode {
    /// Not an error; returned on success.
    Ok = 0,

    /// The operation was cancelled (typically by the caller).
    Cancelled = 1,

    /// Unknown error. An example of where this error may be returned is if a Status value received
    /// from another address space belongs to an error-space that is not known in this address
    /// space. Also errors raised by APIs that do not return enough error information may be
    /// converted to this error.
    Unknown = 2,

    /// Client specified an invalid argument. Note that this differs from `FailedPrecondition`.
    /// `InvalidArgument` indicates arguments that are problematic regardless of the state of the
    /// system (e.g., a malformed file name).
    InvalidArgument = 3,

    /// Deadline expired before operation could complete. For operations that change the state of
    /// the system, this error may be returned even if the operation has completed successfully.
    /// For example, a successful response from a server could have been delayed long enough for
    /// the deadline to expire.
    DeadlineExceeded = 4,

    /// Some requested entity (e.g., file or directory) was not found.
    NotFound = 5,

    /// Some entity that we attempted to create (e.g., file or directory) already exists.
    AlreadyExists = 6,

    /// The caller does not have permission to execute the specified operation.
    /// `PermissionDenied` must not be used for rejections caused by exhausting
    /// some resource (use `ResourceExhausted` instead for those errors).
    /// `PermissionDenied` must not be used if the caller can not be
    /// identified (use `Unauthenticated` instead for those errors).
    PermissionDenied = 7,

    /// The request does not have valid authentication credentials for the operation.
    Unauthenticated = 16,

    /// Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file
    /// system is out of space.
    ResourceExhausted = 8,

    /// Operation was rejected because the system is not in a state required for the operation's
    /// execution. For example, directory to be deleted may be non-empty, an rmdir operation is
    /// applied to a non-directory, etc.
    FailedPrecondition = 9,

    /// The operation was aborted, typically due to a concurrency issue like sequencer check
    /// failures, transaction aborts, etc.
    Aborted = 10,

    /// Operation was attempted past the valid range. E.g., seeking or reading past end of file.
    OutOfRange = 11,

    /// Operation is not implemented or not supported/enabled in this service.
    Unimplemented = 12,

    /// Internal errors. Means some invariants expected by underlying system has been broken. If
    /// you see one of these errors, something is very broken.
    Internal = 13,

    /// The service is currently unavailable. This is a most likely a transient condition and may
    /// be corrected by retrying with a backoff.
    Unavailable = 14,

    /// Unrecoverable data loss or corruption.
    DataLoss = 15,
}

impl From<i32> for GrpcStatusCode {
    fn from(code: i32) -> GrpcStatusCode {
        match code {
            0 => GrpcStatusCode::Ok,
            1 => GrpcStatusCode::Cancelled,
            3 => GrpcStatusCode::InvalidArgument,
            4 => GrpcStatusCode::DeadlineExceeded,
            5 => GrpcStatusCode::NotFound,
            6 => GrpcStatusCode::AlreadyExists,
            7 => GrpcStatusCode::PermissionDenied,
            16 => GrpcStatusCode::Unauthenticated,
            8 => GrpcStatusCode::ResourceExhausted,
            9 => GrpcStatusCode::FailedPrecondition,
            10 => GrpcStatusCode::Aborted,
            11 => GrpcStatusCode::OutOfRange,
            12 => GrpcStatusCode::Unimplemented,
            13 => GrpcStatusCode::Internal,
            14 => GrpcStatusCode::Unavailable,
            15 => GrpcStatusCode::DataLoss,
            _ => GrpcStatusCode::Unknown,
        }
    }
}

/// Result of a gRPC call.
///
/// If the caller satisfies the prerequisites of a
/// particular operation, the `GrpcCallStatus` returned will be `Ok`.
/// Receiving any other value listed here is an indication of a bug in the caller.
///
/// Based on `grpc_call_error`.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum GrpcCallStatus {
    /// Everything went ok.
    Ok = 0,

    /// Something failed, we don't know what.
    Error,

    /// This method is not available on the server.
    ErrorNotOnServer,

    /// This method is not available on the client.
    ErrorNotOnClient,

    /// This method must be called before server_accept.
    ErrorAlreadyAccepted,

    /// This method must be called before invoke.
    ErrorAlreadyInvoked,

    /// This method must be called after invoke.
    ErrorNotInvoked,

    /// This call is already finished (writes_done or write_status has already been called).
    ErrorAlreadyFinished,

    /// There is already an outstanding read/write operation on the call.
    ErrorTooManyOperations,

    /// The flags value was illegal for this call.
    ErrorInvalidFlags,

    /// Invalid metadata was passed to this call.
    ErrorInvalidMetadata,

    /// Invalid message was passed to this call.
    ErrorInvalidMessage,

    /// Completion queue for notification has not been registered with the server.
    ErrorNotServerCompletionQueue,

    /// This batch of operations leads to more operations than allowed.
    ErrorBatchTooBig,

    /// Payload type requested is not the type registered.
    ErrorPayloadTypeMismatch,

    /// Completion queue has been shut down.
    ErrorCompletionQueueShutdown,
}

/// The type of completion.
///
/// Based on `grpc_completion_type`.
#[repr(C)]
pub enum GrpcCompletionType {
    /// Shutting down.
    QueueShutdown,

    /// No event before timeout.
    QueueTimeout,

    /// Operation completion.
    OpComplete,
}

/// The result of an operation.
///
/// Returned by a completion queue when the operation started with tag.
#[repr(C)]
pub struct GrpcEvent {
    pub event_type: GrpcCompletionType,
    pub success: c_int,
    pub tag: *mut c_void,
}

pub enum GrpcChannelArgs {}

/// Connectivity state of a channel.
///
/// Based on `grpc_connectivity_state`.
#[repr(C)]
pub enum GrpcConnectivityState {
    /// Channel has just been initialized.
    Init = -1,

    /// Channel is idle.
    Idle,

    /// Channel is connecting.
    Connecting,

    /// Channel is ready for work.
    Ready,

    /// Channel has seen a failure but expects to recover.
    TransientFailure,

    /// Channel has seen a failure that it cannot recover from.
    Shutdown,
}

/// Compression levels supported by gRPC.
///
/// Compression levels allow a party with knowledge of its peer's accepted
/// encodings to request compression in an abstract way. The level-algorithm
/// mapping is performed internally and depends on the peer's supported
/// compression algorithms.
///
/// Based on `grpc_compression_level`.
#[repr(C)]
pub enum GrpcCompressionLevel {
    /// No compression.
    None = 0,

    /// Low compression.
    Low,

    /// Medium compression.
    // TODO: Change to `Medium`.
    Med,

    /// High compression.
    High,
}

/// Various compression algorithms supported by gRPC.
///
/// Based on `grpc_compression_algorithm`.
#[repr(C)]
pub enum GrpcCompressionAlgorithms {
    None = 0,
    Deflate,
    Gzip,
}

/// How to handle payloads for a registered method.
///
/// Based on `grpc_server_register_method_payload_handling`.
#[repr(C)]
pub enum GrpcServerRegisterMethodPayloadHandling {
    /// Don't try to read the payload.
    None,

    /// Read the initial payload as a byte buffer.
    ReadInitialByteBuffer,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum GprLogSeverity {
    Debug,
    Info,
    Error,
}

#[repr(C)]
pub struct GprLogFuncArgs {
    pub file: *const c_char,
    pub line: c_int,
    pub severity: GprLogSeverity,
    pub message: *const c_char,
}

#[repr(C)]
pub struct GrpcMetadataArray {
    pub count: size_t,
    pub capacity: size_t,
    pub metadata: *mut GrpcMetadata,
}

pub const GRPC_INITIAL_METADATA_IDEMPOTENT_REQUEST: u32 = 0x0000_0010;
pub const GRPC_INITIAL_METADATA_WAIT_FOR_READY: u32 = 0x0000_0020;
pub const GRPC_INITIAL_METADATA_CACHEABLE_REQUEST: u32 = 0x0000_0040;

pub const GRPC_WRITE_BUFFER_HINT: u32 = 0x0000_0001;
pub const GRPC_WRITE_NO_COMPRESS: u32 = 0x0000_0002;

pub enum GrpcMetadata {}
pub enum GrpcSlice {}
pub enum GrpcCallDetails {}
pub enum GrpcCompletionQueue {}
pub enum GrpcChannel {}
pub enum GrpcCall {}
pub enum GrpcByteBuffer {}
pub enum GrpcBatchContext {}
pub enum GrpcServer {}
pub enum GrpcRequestCallContext {}
pub enum GrpcResourceQuota {}
pub enum GrpcArgPointerVtable {}

pub const GRPC_MAX_COMPLETION_QUEUE_PLUCKERS: usize = 6;

extern "C" {
    pub fn grpc_init();
    pub fn grpc_shutdown();
    pub fn grpc_version_string() -> *const c_char;

    pub fn grpc_call_details_init(details: *mut GrpcCallDetails);
    pub fn grpc_call_details_destroy(details: *mut GrpcCallDetails);

    pub fn grpc_register_plugin(init: Option<extern "C" fn()>, destroy: Option<extern "C" fn()>);

    pub fn gpr_inf_future(clock_type: GprClockType) -> GprTimespec;
    pub fn gpr_now(clock_type: GprClockType) -> GprTimespec;
    pub fn gpr_time_cmp(lhs: GprTimespec, rhs: GprTimespec) -> c_int;
    pub fn gpr_convert_clock_type(t: GprTimespec, clock_type: GprClockType) -> GprTimespec;

    pub fn gpr_set_log_verbosity(severity: GprLogSeverity);
    pub fn gpr_set_log_function(func: Option<extern "C" fn(*mut GprLogFuncArgs)>);

    pub fn gpr_cpu_num_cores() -> c_uint;

    pub fn grpc_completion_queue_create_for_next(reserved: *mut c_void)
        -> *mut GrpcCompletionQueue;
    pub fn grpc_completion_queue_next(
        cq: *mut GrpcCompletionQueue,
        deadline: GprTimespec,
        reserved: *mut c_void,
    ) -> GrpcEvent;
    pub fn grpc_completion_queue_pluck(
        cq: *mut GrpcCompletionQueue,
        tag: *mut c_void,
        deadline: GprTimespec,
        reversed: *mut c_void,
    ) -> GrpcEvent;
    pub fn grpc_completion_queue_shutdown(cq: *mut GrpcCompletionQueue);
    pub fn grpc_completion_queue_destroy(cq: *mut GrpcCompletionQueue);

    pub fn grpcwrap_channel_args_create(num_args: size_t) -> *mut GrpcChannelArgs;
    pub fn grpcwrap_channel_args_set_string(
        args: *mut GrpcChannelArgs,
        index: size_t,
        key: *const c_char,
        value: *const c_char,
    );
    pub fn grpcwrap_channel_args_set_integer(
        args: *mut GrpcChannelArgs,
        index: size_t,
        key: *const c_char,
        value: c_int,
    );
    pub fn grpcwrap_channel_args_set_pointer_vtable(
        args: *mut GrpcChannelArgs,
        index: usize,
        key: *const c_char,
        value: *mut c_void,
        vtable: *const GrpcArgPointerVtable,
    );
    pub fn grpcwrap_channel_args_destroy(args: *mut GrpcChannelArgs);

    pub fn grpc_channel_check_connectivity_state(
        channel: *mut GrpcChannel,
        try_to_connect: c_int,
    ) -> GrpcConnectivityState;
    pub fn grpcwrap_channel_create_call(
        channel: *mut GrpcChannel,
        parent_call: *mut GrpcCall,
        propagation_mask: u32,
        cq: *mut GrpcCompletionQueue,
        method: *const c_char,
        method_len: size_t,
        host: *const c_char,
        host_len: size_t,
        deadline: GprTimespec,
        reserved: *mut c_void,
    ) -> *mut GrpcCall;
    pub fn grpc_channel_get_target(channel: *mut GrpcChannel) -> *mut c_char;
    pub fn grpc_insecure_channel_create(
        target: *const c_char,
        args: *const GrpcChannelArgs,
        reserved: *mut c_void,
    ) -> *mut GrpcChannel;
    pub fn grpc_channel_destroy(channel: *mut GrpcChannel);

    pub fn grpcwrap_batch_context_create() -> *mut GrpcBatchContext;
    pub fn grpcwrap_batch_context_destroy(ctx: *mut GrpcBatchContext);
    pub fn grpcwrap_batch_context_recv_initial_metadata(
        ctx: *mut GrpcBatchContext,
    ) -> *const GrpcMetadataArray;
    pub fn grpcwrap_batch_context_recv_message_length(ctx: *mut GrpcBatchContext) -> size_t;
    pub fn grpcwrap_batch_context_recv_message_to_buffer(
        ctx: *mut GrpcBatchContext,
        buffer: *mut c_char,
        buffer_len: size_t,
    );
    pub fn grpcwrap_batch_context_recv_status_on_client_status(
        ctx: *mut GrpcBatchContext,
    ) -> GrpcStatusCode;
    pub fn grpcwrap_batch_context_recv_status_on_client_details(
        ctx: *mut GrpcBatchContext,
        details_length: *mut size_t,
    ) -> *const c_char;
    pub fn grpcwrap_batch_context_recv_status_on_client_trailing_metadata(
        ctx: *mut GrpcBatchContext,
    ) -> *const GrpcMetadataArray;
    pub fn grpcwrap_batch_context_recv_close_on_server_cancelled(ctx: *mut GrpcBatchContext)
        -> i32;

    pub fn grpcwrap_call_kick_completion_queue(
        call: *mut GrpcCall,
        tag: *mut c_void,
    ) -> GrpcCallStatus;

    pub fn grpcwrap_call_start_unary(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        send_bufer: *const c_char,
        send_buffer_len: size_t,
        write_flags: u32,
        initial_metadata: *mut GrpcMetadataArray,
        initial_metadata_flags: u32,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_start_client_streaming(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        initial_metadata: *mut GrpcMetadataArray,
        initial_metadata_flags: u32,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_start_server_streaming(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        send_bufer: *const c_char,
        send_buffer_len: size_t,
        write_flags: u32,
        initial_metadata: *mut GrpcMetadataArray,
        initial_metadata_flags: u32,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_start_duplex_streaming(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        initial_metadata: *mut GrpcMetadataArray,
        initial_metadata_flags: u32,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_recv_initial_metadata(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_send_message(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        send_bufer: *const c_char,
        send_buffer_len: size_t,
        write_flags: u32,
        send_empty_initial_metadata: u32,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_send_close_from_client(
        call: *mut GrpcCall,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_send_status_from_server(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        status: GrpcStatusCode,
        status_details: *const c_char,
        status_details_len: size_t,
        trailing_metadata: *mut GrpcMetadataArray,
        send_empty_metadata: i32,
        optional_send_buffer: *const c_char,
        buffer_len: size_t,
        write_flags: u32,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_recv_message(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_start_serverside(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_send_initial_metadata(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        initial_metadata: *mut GrpcMetadataArray,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpc_call_get_peer(call: *mut GrpcCall) -> *mut c_char;
    pub fn grpc_call_get_target(call: *mut GrpcCall) -> *mut c_char;
    pub fn grpc_call_cancel(call: *mut GrpcCall, reserved: *mut c_void);
    pub fn grpc_call_cancel_with_status(
        call: *mut GrpcCall,
        status: GrpcStatusCode,
        description: *const c_char,
        reserved: *mut c_void,
    );
    pub fn grpc_call_ref(call: *mut GrpcCall);
    pub fn grpc_call_unref(call: *mut GrpcCall);

    pub fn grpc_server_register_method(
        server: *mut GrpcServer,
        method: *const c_char,
        host: *const c_char,
        paylod_handling: GrpcServerRegisterMethodPayloadHandling,
        flags: u32,
    ) -> *mut c_void;
    pub fn grpc_server_create(
        args: *const GrpcChannelArgs,
        reserved: *mut c_void,
    ) -> *mut GrpcServer;
    pub fn grpc_server_register_completion_queue(
        server: *mut GrpcServer,
        cq: *mut GrpcCompletionQueue,
        reserved: *mut c_void,
    );
    pub fn grpc_server_add_insecure_http2_port(
        server: *mut GrpcServer,
        addr: *const c_char,
    ) -> c_int;
    pub fn grpc_server_start(server: *mut GrpcServer);
    pub fn grpc_server_shutdown_and_notify(
        server: *mut GrpcServer,
        cq: *mut GrpcCompletionQueue,
        tag: *mut c_void,
    );
    pub fn grpc_server_cancel_all_calls(server: *mut GrpcServer);
    pub fn grpc_server_destroy(server: *mut GrpcServer);

    pub fn grpcwrap_request_call_context_create() -> *mut GrpcRequestCallContext;
    pub fn grpcwrap_request_call_context_destroy(ctx: *mut GrpcRequestCallContext);
    pub fn grpcwrap_request_call_context_get_call(
        ctx: *const GrpcRequestCallContext,
    ) -> *mut GrpcCall;
    pub fn grpcwrap_request_call_context_ref_call(
        ctx: *const GrpcRequestCallContext,
    ) -> *mut GrpcCall;
    pub fn grpcwrap_request_call_context_method(
        ctx: *const GrpcRequestCallContext,
        len: *mut size_t,
    ) -> *const c_char;
    pub fn grpcwrap_request_call_context_host(
        ctx: *const GrpcRequestCallContext,
        len: *mut size_t,
    ) -> *const c_char;
    pub fn grpcwrap_request_call_context_deadline(
        ctx: *const GrpcRequestCallContext,
    ) -> GprTimespec;
    pub fn grpcwrap_request_call_context_metadata_array(
        ctx: *const GrpcRequestCallContext,
    ) -> *const GrpcMetadataArray;
    pub fn grpcwrap_server_request_call(
        server: *mut GrpcServer,
        cq: *mut GrpcCompletionQueue,
        ctx: *mut GrpcRequestCallContext,
        tag: *mut c_void,
    ) -> GrpcCallStatus;

    pub fn grpcwrap_metadata_array_init(array: *mut GrpcMetadataArray, capacity: size_t);
    pub fn grpcwrap_metadata_array_add(
        array: *mut GrpcMetadataArray,
        key: *const c_char,
        key_len: size_t,
        val: *const c_char,
        val_len: size_t,
    );
    pub fn grpcwrap_metadata_array_get_key(
        array: *const GrpcMetadataArray,
        index: size_t,
        key_len: *mut size_t,
    ) -> *const c_char;
    pub fn grpcwrap_metadata_array_get_value(
        array: *const GrpcMetadataArray,
        index: size_t,
        val_len: *mut size_t,
    ) -> *const c_char;
    pub fn grpcwrap_metadata_array_shrink_to_fit(array: *mut GrpcMetadataArray);
    pub fn grpcwrap_metadata_array_cleanup(array: *mut GrpcMetadataArray);

    pub fn gpr_free(p: *mut c_void);

    pub fn grpc_resource_quota_create(trace_name: *const c_char) -> *mut GrpcResourceQuota;
    pub fn grpc_resource_quota_ref(resource_quota: *mut GrpcResourceQuota);
    pub fn grpc_resource_quota_unref(resource_quota: *mut GrpcResourceQuota);
    pub fn grpc_resource_quota_resize(resource_quota: *mut GrpcResourceQuota, new_size: size_t);
    pub fn grpc_resource_quota_set_max_threads(
        resource_quota: *mut GrpcResourceQuota,
        new_max_threads: c_int,
    );
    pub fn grpc_resource_quota_arg_vtable() -> *const GrpcArgPointerVtable;
}

#[cfg(feature = "secure")]
mod secure_component {
    use libc::{c_char, c_int, c_void, size_t};

    use super::{GrpcChannel, GrpcChannelArgs, GrpcServer};

    pub enum GrpcChannelCredentials {}
    pub enum GrpcServerCredentials {}

    extern "C" {
        pub fn grpcwrap_ssl_credentials_create(
            root_certs: *const c_char,
            cert_chain: *const c_char,
            private_key: *const c_char,
        ) -> *mut GrpcChannelCredentials;

        pub fn grpc_secure_channel_create(
            creds: *mut GrpcChannelCredentials,
            target: *const c_char,
            args: *const GrpcChannelArgs,
            reserved: *mut c_void,
        ) -> *mut GrpcChannel;

        pub fn grpc_google_default_credentials_create() -> *mut GrpcChannelCredentials;
        pub fn grpc_server_add_secure_http2_port(
            server: *mut GrpcServer,
            addr: *const c_char,
            creds: *mut GrpcServerCredentials,
        ) -> c_int;

        pub fn grpcwrap_override_default_ssl_roots(certs: *const c_char);
        pub fn grpc_channel_credentials_release(credentials: *mut GrpcChannelCredentials);
        pub fn grpcwrap_ssl_server_credentials_create(
            root_certs: *const c_char,
            cert_chain_array: *mut *const c_char,
            private_key_array: *mut *const c_char,
            num_pairs: size_t,
            force_client_auth: c_int,
        ) -> *mut GrpcServerCredentials;
        pub fn grpcwrap_ssl_server_certificate_config_create(
            root_certs: *const c_char,
            cert_chain_array: *mut *const c_char,
            private_key_array: *mut *const c_char,
            num_pairs: size_t,
        ) -> *mut grpc_ssl_server_certificate_config;
        pub fn grpc_server_credentials_release(credentials: *mut GrpcServerCredentials);
    }

    #[repr(u32)]
    #[doc = " Callback results for dynamically loading a SSL certificate config."]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum grpc_ssl_certificate_config_reload_status {
        GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_UNCHANGED = 0,
        GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_NEW = 1,
        GRPC_SSL_CERTIFICATE_CONFIG_RELOAD_FAIL = 2,
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

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
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

    #[doc = " Callback to retrieve updated SSL server certificates, private keys, and"]
    #[doc = "trusted CAs (for client authentication)."]
    #[doc = "- user_data parameter, if not NULL, contains opaque data to be used by the"]
    #[doc = "callback."]
    #[doc = "- Use grpc_ssl_server_certificate_config_create to create the config."]
    #[doc = "- The caller assumes ownership of the config."]
    #[allow(non_camel_case_types)]
    pub type grpc_ssl_server_certificate_config_callback = ::std::option::Option<
        unsafe extern "C" fn(
            user_data: *mut ::std::os::raw::c_void,
            config: *mut *mut grpc_ssl_server_certificate_config,
        ) -> grpc_ssl_certificate_config_reload_status,
    >;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct grpc_ssl_server_credentials_options {
        _unused: [u8; 0],
    }

    extern "C" {
        #[doc = " Creates an SSL server_credentials object using the provided options struct."]
        #[doc = "- Takes ownership of the options parameter."]
        pub fn grpc_ssl_server_credentials_create_with_options(
            options: *mut grpc_ssl_server_credentials_options,
        ) -> *mut grpc_server_credentials;
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct grpc_server_credentials {
        _unused: [u8; 0],
    }
}

#[cfg(feature = "secure")]
pub use secure_component::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_auth_context {
    _unused: [u8; 0],
}

#[doc = " value, if not NULL, is guaranteed to be NULL terminated."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_auth_property {
    pub name: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
    pub value_length: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_auth_property_iterator {
    pub ctx: *const grpc_auth_context,
    pub index: usize,
    pub name: *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct grpc_call {
    _unused: [u8; 0],
}

extern "C" {
    #[doc = " Gets the auth context from the call. Caller needs to call"]
    #[doc = "grpc_auth_context_release on the returned context."]
    pub fn grpc_call_auth_context(call: *mut grpc_call) -> *mut grpc_auth_context;
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
    #[doc = " Gets the peer identity. Returns an empty iterator (first _next will return"]
    #[doc = "NULL) if the peer is not authenticated."]
    pub fn grpc_auth_context_peer_identity(
        ctx: *const grpc_auth_context,
    ) -> grpc_auth_property_iterator;
}

extern "C" {
    #[doc = " Iterates over the auth context."]
    pub fn grpc_auth_context_property_iterator(
        ctx: *const grpc_auth_context,
    ) -> grpc_auth_property_iterator;
}

extern "C" {
    #[doc = " Returns NULL when the iterator is at the end."]
    pub fn grpc_auth_property_iterator_next(
        it: *mut grpc_auth_property_iterator,
    ) -> *const grpc_auth_property;
}

extern "C" {
    #[doc = " Releases the auth context returned from grpc_call_auth_context."]
    pub fn grpc_auth_context_release(context: *mut grpc_auth_context);
}

// TODO: more tests.
#[cfg(test)]
mod tests {
    use std::ptr;

    #[test]
    fn smoke() {
        unsafe {
            super::grpc_init();
            let cq = super::grpc_completion_queue_create_for_next(ptr::null_mut());
            super::grpc_completion_queue_destroy(cq);
            super::grpc_shutdown();
        }
    }
}
