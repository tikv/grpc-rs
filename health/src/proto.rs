// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "prost-codec")]
mod health_prost;

#[cfg(feature = "prost-codec")]
pub use self::health_prost::*;

#[cfg(feature = "protobuf-codec")]
mod health;
#[cfg(feature = "protobuf-codec")]
mod health_grpc;

#[cfg(feature = "protobuf-codec")]
pub use self::{health::*, health_grpc::*};
