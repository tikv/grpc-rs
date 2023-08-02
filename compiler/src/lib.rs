// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
pub mod codegen;
#[cfg(feature = "prost-codec")]
pub mod prost_codegen;

#[cfg(any(feature = "protobuf-codec", feature = "prost-codec"))]
mod util;
