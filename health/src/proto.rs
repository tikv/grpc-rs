// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "prost-codec")]
mod reexports {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));

    pub use self::grpc::health::v1::*;
    pub use self::health_check_response::ServingStatus;
}

#[cfg(feature = "protobuf-codec")]
#[allow(non_upper_case_globals)]
mod reexports {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));

    pub use self::health::*;
    pub use self::health_grpc::*;

    impl HealthCheckResponse_ServingStatus {
        pub const Unknown: HealthCheckResponse_ServingStatus =
            HealthCheckResponse_ServingStatus::UNKNOWN;
        pub const Serving: HealthCheckResponse_ServingStatus =
            HealthCheckResponse_ServingStatus::SERVING;
        pub const NotServing: HealthCheckResponse_ServingStatus =
            HealthCheckResponse_ServingStatus::NOT_SERVING;
        pub const ServiceUnknown: HealthCheckResponse_ServingStatus =
            HealthCheckResponse_ServingStatus::SERVICE_UNKNOWN;
    }

    pub use HealthCheckResponse_ServingStatus as ServingStatus;
}

pub use self::reexports::*;
