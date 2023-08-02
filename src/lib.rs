// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!

[grpcio] is a Rust implementation of [gRPC], which is a high performance, open source universal RPC
framework that puts mobile and HTTP/2 first. grpcio is built on [gRPC Core] and [futures-rs].

[grpcio]: https://github.com/tikv/grpc-rs/
[gRPC]: https://grpc.io/
[gRPC Core]: https://github.com/grpc/grpc
[futures-rs]: https://github.com/rust-lang/futures-rs

## Optional features

- **`boringssl`** *(enabled by default)* - Enables support for TLS encryption and some authentication
  mechanisms.
- **`openssl`** - Same as `boringssl`, but base on the system openssl.
- **`openssl-vendored`** - Same as `openssl`, but build openssl from source.

*/

#![allow(clippy::new_without_default)]
#![allow(clippy::new_without_default)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::option_map_unit_fn)]
#![allow(clippy::derive_partial_eq_without_eq)]

use grpcio_sys as grpc_sys;
#[macro_use]
extern crate log;

mod buf;
mod call;
mod channel;
pub mod channelz;
mod client;
mod codec;
mod cq;
mod env;
mod error;
mod log_util;
mod metadata;
mod quota;
mod security;
mod server;
mod task;

pub use crate::buf::GrpcSlice;
pub use crate::call::client::{
    CallOption, ClientCStreamReceiver, ClientCStreamSender, ClientDuplexReceiver,
    ClientDuplexSender, ClientSStreamReceiver, ClientUnaryReceiver, StreamingCallSink,
};
pub use crate::call::server::{
    ClientStreamingSink, ClientStreamingSinkResult, Deadline, DuplexSink, DuplexSinkFailure,
    RequestStream, RpcContext, ServerStreamingSink, ServerStreamingSinkFailure, UnarySink,
    UnarySinkResult,
};
pub use crate::call::{MessageReader, Method, MethodType, RpcStatus, RpcStatusCode, WriteFlags};
pub use crate::channel::{
    Channel, ChannelBuilder, CompressionAlgorithms, CompressionLevel, ConnectivityState, LbPolicy,
    OptTarget,
};
pub use crate::client::Client;

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
pub use crate::codec::pb_codec::{de as pb_de, ser as pb_ser};
#[cfg(feature = "prost-codec")]
pub use crate::codec::pr_codec::{de as pr_de, ser as pr_ser};

pub use crate::codec::{Marshaller, MAX_MESSAGE_SIZE};
pub use crate::env::{EnvBuilder, Environment};
pub use crate::error::{Error, Result};
pub use crate::log_util::redirect_log;
pub use crate::metadata::{Metadata, MetadataBuilder, MetadataIter};
pub use crate::quota::ResourceQuota;
pub use crate::security::*;
pub use crate::server::{
    CheckResult, Server, ServerBuilder, ServerChecker, Service, ServiceBuilder, ShutdownFuture,
};

/// A shortcut for implementing a service method by returning `UNIMPLEMENTED` status code.
///
/// Compiler will provide a default implementations for all methods to invoke this macro, so
/// you usually won't call it directly. If you really need to, just call it like:
/// ```ignored
/// fn method(&self, ctx: grpcio::RpcContext, req: Request, resp: UnarySink<Response>) {
///     unimplemented_call!(ctx, resp);
/// }
/// ```
#[macro_export]
macro_rules! unimplemented_call {
    ($ctx:ident, $sink:ident) => {{
        let f = async move {
            let _ = $sink
                .fail($crate::RpcStatus::new($crate::RpcStatusCode::UNIMPLEMENTED))
                .await;
        };
        $ctx.spawn(f)
    }};
}
