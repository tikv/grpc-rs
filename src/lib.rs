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

/*!

[grpcio] is a Rust implementation of [gRPC], which is a high performance, open source universal RPC
framework that puts mobile and HTTP/2 first. grpcio is built on [gRPC Core] and [futures-rs].

[grpcio]: https://github.com/pingcap/grpc-rs/
[gRPC]: https://grpc.io/
[gRPC Core]: https://github.com/grpc/grpc
[futures-rs]: https://github.com/rust-lang-nursery/futures-rs

## Optional features

- **`secure`** *(enabled by default)* - Enables support for TLS encryption and some authentication
  mechanisms.

*/

// TODO: Remove it once Rust's tool_lints is stabilized.
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]
#![cfg_attr(feature = "cargo-clippy", allow(new_without_default))]
#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
#![cfg_attr(feature = "cargo-clippy", allow(option_map_unit_fn))]

#[macro_use]
extern crate futures;
extern crate grpcio_sys as grpc_sys;
extern crate libc;
#[macro_use]
extern crate log;
#[cfg(feature = "protobuf-codec")]
extern crate protobuf;

mod async;
mod call;
mod channel;
mod client;
mod codec;
mod cq;
#[cfg(feature = "secure")]
mod credentials;
mod env;
mod error;
mod log_util;
mod metadata;
mod server;

pub use call::client::{
    CallOption, ClientCStreamReceiver, ClientCStreamSender, ClientDuplexReceiver,
    ClientDuplexSender, ClientSStreamReceiver, ClientUnaryReceiver, StreamingCallSink,
};
pub use call::server::{
    ClientStreamingSink, ClientStreamingSinkResult, Deadline, DuplexSink, DuplexSinkFailure,
    RequestStream, RpcContext, ServerStreamingSink, ServerStreamingSinkFailure, UnarySink,
    UnarySinkResult,
};
pub use call::{Method, MethodType, RpcStatus, RpcStatusCode, WriteFlags};
pub use channel::{
    Channel, ChannelBuilder, CompressionAlgorithms, CompressionLevel, LbPolicy, OptTarget,
};
pub use client::Client;
#[cfg(feature = "protobuf-codec")]
pub use codec::pb_codec::{de as pb_de, ser as pb_ser};
pub use codec::Marshaller;
#[cfg(feature = "secure")]
pub use credentials::{
    ChannelCredentials, ChannelCredentialsBuilder, ServerCredentials, ServerCredentialsBuilder,
};
pub use env::{EnvBuilder, Environment};
pub use error::{Error, Result};
pub use log_util::redirect_log;
pub use metadata::{Metadata, MetadataBuilder, MetadataIter};
pub use server::{Server, ServerBuilder, Service, ServiceBuilder, ShutdownFuture};
