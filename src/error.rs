use std::{error, result};
use std::fmt::{self, Display, Formatter};

use protobuf::ProtobufError;
use grpc_sys::GrpcCallStatus;
use call::RpcStatus;

#[derive(Debug)]
pub enum Error {
    Protobuf(ProtobufError),
    CallFailure(GrpcCallStatus),
    RpcFailure(RpcStatus),
    FutureStale,
    RemoteStopped,
}


impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Protobuf(_) => "Grpc Protobuf Error",
            Error::CallFailure(_) => "Grpc Call Error",
            Error::RpcFailure(_) => "Grpc Request Error",
            Error::FutureStale => "Future is polled after being resolved.",
            Error::RemoteStopped => "Remote is stopped.",
        }
    }
}

impl From<ProtobufError> for Error {
    fn from(e: ProtobufError) -> Error {
        Error::Protobuf(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
