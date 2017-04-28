// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


use grpc_sys::GrpcCallStatus;
use protobuf::ProtobufError;
use std::{error, result};
use std::fmt::{self, Display, Formatter};

use call::RpcStatus;

#[derive(Debug)]
pub enum Error {
    Protobuf(ProtobufError),
    // return when failed to start an internal async call.
    CallFailure(GrpcCallStatus),
    // fail when the rpc request fail.
    RpcFailure(RpcStatus),
    RemoteStopped,
    ShutdownFailed,
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
            Error::RemoteStopped => "Remote is stopped.",
            Error::ShutdownFailed => "Failed to shutdown.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Protobuf(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<ProtobufError> for Error {
    fn from(e: ProtobufError) -> Error {
        Error::Protobuf(e)
    }
}

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;

    use protobuf::ProtobufError;

    use super::Error;

    #[test]
    fn test_convert() {
        let error = ProtobufError::WireError("test".to_owned());
        let e: Error = error.into();
        assert_eq!(e.description(), "Grpc Protobuf Error");
        assert!(e.cause().is_some());
    }
}
