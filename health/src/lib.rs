// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

//! grpcio-health provides health check protos as well as some helper structs to make
//! health check easily. For the detail design of health checking service, see
//! https://github.com/grpc/grpc/blob/master/doc/health-checking.md.
//!
//! ### Usage
//!
//! The crate provides a default implementation of `Health` service, you can use it
//! to maintain the service states. First, you need to register it to the server builder
//! so that it can serve health check service later.
//! ```ignore
//! use grpcio_health::{HealthService, create_health};
//!
//! let service = HealthService::default();
//! let builder = builder.register_service(create_health(service.clone()));
//! ```
//! Then insert service status for query.
//! ```ignore
//! service.set_serving_status("", ServingStatus::Serving);
//! ```
//! `""` means overall health status. You can also provide specific service name.
//!
//! Client can either use `check` to do one time query or `watch` to observe status changes.
//! ```ignore
//! use grpcio_health::proto::HealthCheckRequest;
//!
//! let client = HealthClient::new(ch);
//! let req = HealthCheckRequest { service: "".to_string(), ..Default::default() };
//! let status_resp = client.check_async(&req).await.unwrap();
//! assert_eq!(statuss_resp.status, ServingStatus::Serving);
//! ```

pub mod proto;
mod service;

pub use self::proto::{create_health, HealthClient};
pub use self::service::HealthService;

#[cfg(feature = "protobuf-codec")]
pub use self::proto::ServingStatus;

#[cfg(feature = "protobufv3-codec")]
pub use self::proto::health_check_response::ServingStatus;
