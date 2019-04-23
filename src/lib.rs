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

#![allow(clippy::new_without_default)]
#![allow(clippy::new_without_default)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::option_map_unit_fn)]

#[macro_use]
extern crate futures;
use grpcio_sys as grpc_sys;
#[macro_use]
extern crate log;

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
mod task;

pub use crate::call::client::{
    CallOption, ClientCStreamReceiver, ClientCStreamSender, ClientDuplexReceiver,
    ClientDuplexSender, ClientSStreamReceiver, ClientUnaryReceiver, StreamingCallSink,
};
pub use crate::call::server::{
    ClientStreamingSink, ClientStreamingSinkResult, Deadline, DuplexSink, DuplexSinkFailure,
    RequestStream, RpcContext, ServerStreamingSink, ServerStreamingSinkFailure, UnarySink,
    UnarySinkResult,
};
pub use crate::call::{
    MessageReader, MessageWriter, Method, MethodType, RpcStatus, RpcStatusCode, WriteFlags,
};
pub use crate::channel::{
    Channel, ChannelBuilder, CompressionAlgorithms, CompressionLevel, ConnectivityState, LbPolicy,
    OptTarget,
};
pub use crate::client::Client;

#[cfg(feature = "protobuf-codec")]
pub use crate::codec::pb_codec::{de as pb_de, ser as pb_ser};
#[cfg(feature = "prost-codec")]
pub use crate::codec::pr_codec::{de as pr_de, ser as pr_ser};

pub use crate::codec::Marshaller;
#[cfg(feature = "secure")]
pub use crate::credentials::{
    ChannelCredentials, ChannelCredentialsBuilder, ServerCredentials, ServerCredentialsBuilder,
};
pub use crate::env::{EnvBuilder, Environment};
pub use crate::error::{Error, Result};
pub use crate::log_util::redirect_log;
pub use crate::metadata::{Metadata, MetadataBuilder, MetadataIter};
pub use crate::server::{Server, ServerBuilder, Service, ServiceBuilder, ShutdownFuture};
