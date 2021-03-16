// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "prost-codec")]
mod reexports {
    include!("proto/grpc.health.v1.rs");

    pub use self::health_check_response::ServingStatus;
}

#[cfg(feature = "protobuf-codec")]
#[allow(deprecated)]
mod health;
#[cfg(feature = "protobuf-codec")]
mod health_grpc;
#[cfg(feature = "protobuf-codec")]
mod reexports {
    pub use super::health::*;
    pub use HealthCheckResponseServingStatus as ServingStatus;
}

pub use self::reexports::*;
