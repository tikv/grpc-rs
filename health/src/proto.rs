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
#[cfg(feature = "protobufv3-codec")]
mod protobuf_v3 {
    #[allow(deprecated)]
    pub mod health;
    pub mod health_grpc;
}
#[cfg(feature = "protobuf-codec")]
mod reexports {
    pub use super::protobuf::health::*;
    pub use super::protobuf::health_grpc::*;
}
#[cfg(feature = "protobufv3-codec")]
mod reexports {
    pub use super::protobuf_v3::health::*;
    pub use super::protobuf_v3::health_grpc::*;
}

pub use self::reexports::*;
