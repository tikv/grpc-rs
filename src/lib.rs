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

#![allow(new_without_default_derive)]
#![allow(new_without_default)]

extern crate grpcio_sys as grpc_sys;
#[macro_use]
extern crate futures;
extern crate libc;
#[cfg(feature = "protobuf-codec")]
extern crate protobuf;

mod async;
mod call;
mod channel;
mod codec;
mod cq;
mod client;
mod credentials;
mod env;
mod error;
mod server;

pub use call::{Method, MethodType, RpcStatus, RpcStatusCode, WriteFlags};
pub use call::client::{CallOption, ClientCStreamReceiver, ClientCStreamSender,
                       ClientDuplexReceiver, ClientDuplexSender, ClientSStreamReceiver,
                       ClientUnaryReceiver};
pub use call::server::{ClientStreamingSink, ClientStreamingSinkResult, Deadline, DuplexSink,
                       DuplexSinkFailure, RequestStream, RpcContext, ServerStreamingSink,
                       ServerStreamingSinkFailure, UnarySink, UnarySinkResult};
pub use channel::{Channel, ChannelBuilder, CompressionLevel, CompressionAlgorithms};
pub use client::Client;
pub use codec::Marshaller;
#[cfg(feature = "protobuf-codec")]
pub use codec::pb_codec::{de as pb_de, ser as pb_ser};
pub use credentials::{ChannelCredentials, ChannelCredentialsBuilder, ServerCredentials,
                      ServerCredentialsBuilder};
pub use env::{EnvBuilder, Environment};
pub use error::{Error, Result};
pub use server::{Server, ServerBuilder, Service, ServiceBuilder, ShutdownFuture};
