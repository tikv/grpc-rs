// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[allow(renamed_and_removed_lints)]
#[allow(static_mut_refs)]
mod proto;

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
pub use proto::protobuf::*;

#[cfg(feature = "prost-codec")]
pub use proto::prost::*;

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
pub mod offload {
    #[cfg(feature = "protobuf-codec")]
    use protobuf::Message;

    #[cfg(feature = "protobufv3-codec")]
    use protobufv3::Message;

    // These aliases let downstream service implementations compile against one
    // signature while the generated bindings switch between raw and offloaded
    // protobuf payloads at feature time.
    #[cfg(feature = "offload-codec")]
    pub type Request<T> = grpcio::pb_codec::Req<T>;
    #[cfg(not(feature = "offload-codec"))]
    pub type Request<T> = T;

    #[cfg(feature = "offload-codec")]
    pub type Response<T> = grpcio::pb_codec::Resp<T>;
    #[cfg(not(feature = "offload-codec"))]
    pub type Response<T> = T;

    #[cfg(feature = "offload-codec")]
    pub fn decode<T: Message>(request: Request<T>) -> grpcio::Result<T> {
        request.get()
    }

    #[cfg(not(feature = "offload-codec"))]
    pub fn decode<T>(request: Request<T>) -> grpcio::Result<T> {
        Ok(request)
    }

    #[cfg(feature = "offload-codec")]
    pub fn encode<T: Message>(response: T) -> grpcio::Result<Response<T>> {
        grpcio::pb_codec::Resp::new(response)
    }

    #[cfg(not(feature = "offload-codec"))]
    pub fn encode<T>(response: T) -> grpcio::Result<Response<T>> {
        Ok(response)
    }
}

pub mod util;
