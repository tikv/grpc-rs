// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::google::rpc::Status;
use grpcio::{
    ChannelCredentials, ChannelCredentialsBuilder, ServerCredentials, ServerCredentialsBuilder,
};
use std::convert::TryFrom;

#[cfg(all(
    any(feature = "protobuf-codec", feature = "protobufv3-codec"),
    not(feature = "prost-codec")
))]
use crate::proto::protobuf::testing::messages::{Payload, ResponseParameters};
#[cfg(feature = "prost-codec")]
use crate::testing::{Payload, ResponseParameters};

#[cfg(feature = "protobufv3-codec")]
use protobufv3 as protobuf;

/// Create a payload with the specified size.
pub fn new_payload(size: usize) -> Payload {
    Payload {
        body: vec![0; size],
        ..Default::default()
    }
}

pub fn new_parameters(size: i32) -> ResponseParameters {
    ResponseParameters {
        size,
        ..Default::default()
    }
}

pub fn create_test_server_credentials() -> ServerCredentials {
    let private_key = include_str!("../data/server1.key");
    let cert = include_str!("../data/server1.pem");
    ServerCredentialsBuilder::new()
        .add_cert(cert.into(), private_key.into())
        .build()
}

pub fn create_test_channel_credentials() -> ChannelCredentials {
    let ca = include_str!("../data/ca.pem");
    ChannelCredentialsBuilder::new()
        .root_cert(ca.into())
        .build()
}

impl TryFrom<grpcio::RpcStatus> for Status {
    type Error = grpcio::Error;

    fn try_from(value: grpcio::RpcStatus) -> grpcio::Result<Self> {
        let mut s = Status::default();
        #[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
        protobuf::Message::merge_from_bytes(&mut s, value.details())?;
        #[cfg(feature = "prost-codec")]
        prost::Message::merge(&mut s, value.details())?;
        if s.code == value.code().into() {
            if s.message == value.message() {
                Ok(s)
            } else {
                Err(grpcio::Error::Codec(
                    format!(
                        "message doesn't match {:?} != {:?}",
                        s.message,
                        value.message()
                    )
                    .into(),
                ))
            }
        } else {
            Err(grpcio::Error::Codec(
                format!("code doesn't match {} != {}", s.code, value.code()).into(),
            ))
        }
    }
}

impl TryFrom<Status> for grpcio::RpcStatus {
    type Error = grpcio::Error;

    fn try_from(value: Status) -> grpcio::Result<Self> {
        #[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
        let details = protobuf::Message::write_to_bytes(&value)?;
        #[cfg(feature = "prost-codec")]
        let details = {
            let mut v = vec![];
            prost::Message::encode(&value, &mut v).unwrap();
            v
        };
        Ok(grpcio::RpcStatus::with_details(
            value.code,
            value.message,
            details,
        ))
    }
}
