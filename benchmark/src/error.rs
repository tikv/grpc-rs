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

use std::result;

use grpc;
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
