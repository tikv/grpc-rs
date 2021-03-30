// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::google::rpc::Status;
use grpcio::{
    ChannelCredentials, ChannelCredentialsBuilder, ServerCredentials, ServerCredentialsBuilder,
};
use std::convert::TryFrom;

#[cfg(all(feature = "protobuf-codec", not(feature = "prost-codec")))]
use crate::testing::messages::{Payload, ResponseParameters};
#[cfg(feature = "prost-codec")]
use crate::testing::{Payload, ResponseParameters};

/// Create a payload with the specified size.
pub fn new_payload(size: usize) -> Payload {
    let mut payload = Payload::default();
    payload.set_body(vec![0; size]);
    payload
}

pub fn new_parameters(size: i32) -> ResponseParameters {
    let mut parameter = ResponseParameters::default();
    parameter.set_size(size);
    parameter
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
        #[cfg(feature = "protobuf-codec")]
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
        #[cfg(feature = "protobuf-codec")]
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
