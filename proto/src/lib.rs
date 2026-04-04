// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[allow(renamed_and_removed_lints)]
#[allow(static_mut_refs)]
mod proto;

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
pub use proto::protobuf::*;

#[cfg(feature = "prost-codec")]
pub use proto::prost::*;

#[cfg(any(
    feature = "protobuf-codec",
    feature = "protobufv3-codec",
    feature = "prost-codec"
))]
pub mod offload {
    #[cfg(all(
        feature = "offload-codec",
        feature = "prost-codec",
        any(feature = "protobuf-codec", feature = "protobufv3-codec")
    ))]
    compile_error!("offload helpers require exactly one codec backend feature");

    #[cfg(not(feature = "offload-codec"))]
    mod imp {
        pub type Request<T> = T;
        pub type Response<T> = T;

        pub fn decode<T>(request: Request<T>) -> grpcio::Result<T> {
            Ok(request)
        }

        pub fn encode<T>(response: T) -> grpcio::Result<Response<T>> {
            Ok(response)
        }
    }

    #[cfg(all(
        feature = "offload-codec",
        any(feature = "protobuf-codec", feature = "protobufv3-codec"),
        not(feature = "prost-codec")
    ))]
    mod imp {
        #[cfg(feature = "protobuf-codec")]
        use protobuf::Message;
        #[cfg(feature = "protobufv3-codec")]
        use protobufv3::Message;

        pub type Request<T> = grpcio::pb_codec::Req<T>;
        pub type Response<T> = grpcio::pb_codec::Resp<T>;

        pub fn decode<T: Message>(request: Request<T>) -> grpcio::Result<T> {
            request.get()
        }

        pub fn encode<T: Message>(response: T) -> grpcio::Result<Response<T>> {
            grpcio::pb_codec::Resp::new(response)
        }
    }

    #[cfg(all(
        feature = "offload-codec",
        feature = "prost-codec",
        not(any(feature = "protobuf-codec", feature = "protobufv3-codec"))
    ))]
    mod imp {
        use prost::Message;

        pub type Request<T> = grpcio::pr_codec::Req<T>;
        pub type Response<T> = grpcio::pr_codec::Resp<T>;

        pub fn decode<T: Message + Default>(request: Request<T>) -> grpcio::Result<T> {
            request.get()
        }

        pub fn encode<T: Message>(response: T) -> grpcio::Result<Response<T>> {
            grpcio::pr_codec::Resp::new(response)
        }
    }

    pub use imp::{decode, encode, Request, Response};
}

pub mod util;
