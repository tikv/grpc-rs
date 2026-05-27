// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::ffi::CString;
use std::{error, fmt, result};

use crate::call::RpcStatus;
use crate::grpc_sys::grpc_call_error;

#[cfg(feature = "protobuf-codec")]
use protobuf::ProtobufError;

#[cfg(feature = "protobufv3-codec")]
use protobufv3::Error as ProtobufError;

/// Errors generated from this library.
#[derive(Debug)]
pub enum Error {
    /// Codec error.
    Codec(Box<dyn error::Error + Send + Sync>),
    /// Failed to start an internal async call.
    CallFailure(grpc_call_error),
    /// Rpc request fail.
    RpcFailure(RpcStatus),
    /// Try to write to a finished rpc call.
    RpcFinished(Option<RpcStatus>),
    /// Remote is stopped.
    RemoteStopped,
    /// Failed to shutdown.
    ShutdownFailed,
    /// Failed to bind.
    BindFail(CString),
    /// gRPC completion queue is shutdown.
    QueueShutdown,
    /// Failed to create Google default credentials.
    GoogleAuthenticationFailed,
    /// Invalid format of metadata.
    InvalidMetadata(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RpcFailure(s) => {
                if s.message().is_empty() {
                    write!(fmt, "RpcFailure: {}", s.code())
                } else {
                    write!(fmt, "RpcFailure: {} {}", s.code(), s.message())
                }
            }
            other_error => write!(fmt, "{other_error:?}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Codec(ref e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
impl From<ProtobufError> for Error {
    fn from(e: ProtobufError) -> Error {
        Error::Codec(Box::new(e))
    }
}

#[cfg(feature = "prost-codec")]
impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError) -> Error {
        Error::Codec(Box::new(e))
    }
}

#[cfg(feature = "prost-codec")]
impl From<prost::EncodeError> for Error {
    fn from(e: prost::EncodeError) -> Error {
        Error::Codec(Box::new(e))
    }
}

/// Type alias to use this library's [`Error`] type in a `Result`.
pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;
    use std::ffi::CString;

    use crate::call::{RpcStatus, RpcStatusCode};
    use crate::grpc_sys::grpc_call_error;

    use super::Error;

    #[test]
    fn test_rpc_failure_display_without_message() {
        let status = RpcStatus::new(RpcStatusCode::UNKNOWN);
        let err = Error::RpcFailure(status);
        assert_eq!(err.to_string(), "RpcFailure: 2-UNKNOWN");
        assert!(err.source().is_none());
    }

    #[test]
    fn test_rpc_failure_display_with_message() {
        let status = RpcStatus::with_message(RpcStatusCode::INTERNAL, "something went wrong".to_owned());
        let err = Error::RpcFailure(status);
        assert_eq!(err.to_string(), "RpcFailure: 13-INTERNAL something went wrong");
        assert!(err.source().is_none());
    }

    #[test]
    fn test_shutdown_failed_display() {
        let err = Error::ShutdownFailed;
        assert_eq!(err.to_string(), "ShutdownFailed");
        assert!(err.source().is_none());
    }

    #[test]
    fn test_rpc_finished_display() {
        let err_none = Error::RpcFinished(None);
        assert_eq!(err_none.to_string(), "RpcFinished(None)");

        let status = RpcStatus::new(RpcStatusCode::CANCELLED);
        let err_some = Error::RpcFinished(Some(status));
        assert_eq!(
            err_some.to_string(),
            "RpcFinished(Some(RpcStatus { code: 1-CANCELLED, message: \"\", details: [], debug_error_string: \"\" }))"
        );
    }

    #[test]
    fn test_other_error_variants_display_and_source() {
        let variants: Vec<Error> = vec![
            Error::RemoteStopped,
            Error::QueueShutdown,
            Error::GoogleAuthenticationFailed,
            Error::InvalidMetadata("bad header".to_owned()),
            Error::BindFail(CString::new("addr").unwrap()),
            Error::CallFailure(grpc_call_error::GRPC_CALL_ERROR),
        ];
        for err in variants {
            let _ = err.to_string();
            assert!(err.source().is_none());
        }
    }

    #[cfg(feature = "protobuf-codec")]
    #[test]
    fn test_convert() {
        use protobuf::error::WireError;
        use protobuf::ProtobufError;

        let error = ProtobufError::WireError(WireError::UnexpectedEof);
        let e: Error = error.into();
        assert_eq!(e.to_string(), "Codec(WireError(UnexpectedEof))");
        assert!(e.source().is_some());
    }
}
