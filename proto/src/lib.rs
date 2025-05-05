// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[allow(renamed_and_removed_lints)]
#[allow(static_mut_refs)]
mod proto;

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
pub use proto::protobuf::*;

#[cfg(feature = "prost-codec")]
pub use proto::prost::*;

pub mod util;
