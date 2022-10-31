// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "prost-codec")]
mod reexports {
    include!("proto/prost/grpc.health.v1.rs");

    pub use self::health_check_response::ServingStatus;
}

#[cfg(feature = "protobuf-codec")]
mod protobuf {
    #[allow(deprecated)]
    pub mod health;
    pub mod health_grpc;
}
#[cfg(feature = "protobuf-codec")]
mod reexports {
    pub use super::protobuf::health::*;
}

pub use self::reexports::*;
