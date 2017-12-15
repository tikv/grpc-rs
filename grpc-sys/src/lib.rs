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

use libc::{c_char, c_int, c_uint, c_void, size_t, int32_t, int64_t, uint32_t};
use std::time::Duration;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum GprClockType {
    Monotonic = 0,
    Realtime,
    Precise,
    Timespan,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GprTimespec {
    pub tv_sec: int64_t,
    pub tv_nsec: int32_t,
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
            tv_sec: dur.as_secs() as int64_t,
            tv_nsec: dur.subsec_nanos() as int32_t,
            clock_type: GprClockType::Timespan,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GrpcStatusCode {
    Ok = 0,
    Cancelled = 1,
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    Unauthenticated = 16,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
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

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum GrpcCallStatus {
    Ok = 0,
    Error,
    ErrorNotOnServer,
    ErrorNotOnClient,
    ErrorAlreadyAccepted,
    ErrorAlreadyInvoked,
    ErrorNotInvoked,
    ErrorAlreadyFinished,
    ErrorTooManyOperations,
    ErrorInvalidFlags,
    ErrorInvalidMetadata,
    ErrorInvalidMessage,
    ErrorNotServerCompletionQueue,
    ErrorBatchTooBig,
    ErrorPayloadTypeMismatch,
}

#[repr(C)]
pub enum GrpcCompletionType {
    QueueShutdown,
    QueueTimeout,
    OpComplete,
}

#[repr(C)]
pub struct GrpcEvent {
    pub event_type: GrpcCompletionType,
    pub success: c_int,
    pub tag: *mut c_void,
}

pub enum GrpcChannelArgs {}

#[repr(C)]
pub enum GrpcConnectivityState {
    Init = -1,
    Idle,
    Connecting,
    Ready,
    TransientFailure,
    Shutdown,
}

#[repr(C)]
pub enum GrpcCompressionLevel {
    None = 0,
    Low,
    Med,
    High,
}

#[repr(C)]
pub enum GrpcCompressionAlgorithms {
    None = 0,
    Deflate,
    Gzip,
}

#[repr(C)]
pub enum GrpcServerRegisterMethodPayloadHandling {
    None,
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

pub const GRPC_INITIAL_METADATA_IDEMPOTENT_REQUEST: uint32_t = 0x00000010;
pub const GRPC_INITIAL_METADATA_WAIT_FOR_READY: uint32_t = 0x00000020;
pub const GRPC_INITIAL_METADATA_CACHEABLE_REQUEST: uint32_t = 0x00000040;

pub const GRPC_WRITE_BUFFER_HINT: uint32_t = 0x00000001;
pub const GRPC_WRITE_NO_COMPRESS: uint32_t = 0x00000002;

pub enum GrpcSlice {}
pub enum GrpcMetadataArray {}
pub enum GrpcCallDetails {}
pub enum GrpcCompletionQueue {}
pub enum GrpcChannel {}
pub enum GrpcCall {}
pub enum GrpcByteBuffer {}
pub enum GrpcBatchContext {}
pub enum GrpcServer {}
pub enum GrpcRequestCallContext {}
pub enum GrpcAlarm {}

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
    pub fn grpcwrap_channel_args_destroy(args: *mut GrpcChannelArgs);

    pub fn grpc_channel_check_connectivity_state(
        channel: *mut GrpcChannel,
        try_to_connect: c_int,
    ) -> GrpcConnectivityState;
    pub fn grpcwrap_channel_create_call(
        channel: *mut GrpcChannel,
        parent_call: *mut GrpcCall,
        propagation_mask: uint32_t,
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
    pub fn grpcwrap_batch_context_recv_close_on_server_cancelled(
        ctx: *mut GrpcBatchContext,
    ) -> int32_t;

    pub fn grpcwrap_call_start_unary(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        send_bufer: *const c_char,
        send_buffer_len: size_t,
        write_flags: uint32_t,
        initial_metadata: *mut GrpcMetadataArray,
        initial_metadata_flags: uint32_t,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_start_client_streaming(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        initial_metadata: *mut GrpcMetadataArray,
        initial_metadata_flags: uint32_t,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_start_server_streaming(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        send_bufer: *const c_char,
        send_buffer_len: size_t,
        write_flags: uint32_t,
        initial_metadata: *mut GrpcMetadataArray,
        initial_metadata_flags: uint32_t,
        tag: *mut c_void,
    ) -> GrpcCallStatus;
    pub fn grpcwrap_call_start_duplex_streaming(
        call: *mut GrpcCall,
        ctx: *mut GrpcBatchContext,
        initial_metadata: *mut GrpcMetadataArray,
        initial_metadata_flags: uint32_t,
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
        write_flags: uint32_t,
        send_empty_initial_metadata: uint32_t,
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
        send_empty_metadata: int32_t,
        optional_send_buffer: *const c_char,
        buffer_len: size_t,
        write_flags: uint32_t,
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
    pub fn grpc_call_unref(call: *mut GrpcCall);

    pub fn grpc_server_register_method(
        server: *mut GrpcServer,
        method: *const c_char,
        host: *const c_char,
        paylod_handling: GrpcServerRegisterMethodPayloadHandling,
        flags: uint32_t,
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
    pub fn grpcwrap_request_call_context_take_call(
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
    pub fn grpcwrap_request_call_context_metadata(
        ctx: *const GrpcRequestCallContext,
    ) -> *const GrpcMetadataArray;
    pub fn grpcwrap_server_request_call(
        server: *mut GrpcServer,
        cq: *mut GrpcCompletionQueue,
        ctx: *mut GrpcRequestCallContext,
        tag: *mut c_void,
    ) -> GrpcCallStatus;

    pub fn grpc_alarm_create(reserved: *mut c_void) -> *mut GrpcAlarm;
    pub fn grpc_alarm_set(
        alarm: *mut GrpcAlarm,
        cq: *mut GrpcCompletionQueue,
        deadline: GprTimespec,
        tag: *mut c_void,
        reserved: *mut c_void,
    ) -> *mut GrpcAlarm;
    pub fn grpc_alarm_cancel(alarm: *mut GrpcAlarm);
    pub fn grpc_alarm_destroy(alarm: *mut GrpcAlarm);
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
        pub fn grpc_server_credentials_release(credentials: *mut GrpcServerCredentials);
    }
}

#[cfg(feature = "secure")]
pub use secure_component::*;

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
