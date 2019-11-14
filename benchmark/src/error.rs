// Copyright 2018 TiKV Project Authors. Licensed under Apache-2.0.

use std::result;

use crate::grpc;
use tokio_timer::TimerError;

#[derive(Debug)]
pub enum Error {
    Grpc(grpc::Error),
    Timer(TimerError),
}

impl From<grpc::Error> for Error {
    fn from(e: grpc::Error) -> Error {
        Error::Grpc(e)
    }
}

impl From<TimerError> for Error {
    fn from(e: TimerError) -> Error {
        Error::Timer(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
