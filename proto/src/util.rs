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


use grpcio::{ChannelCredentials, ChannelCredentialsBuilder, ServerCredentials,
             ServerCredentialsBuilder};
use slog_async;
use slog::{Drain, Logger};
use slog_scope::{self, GlobalLoggerGuard};
use slog_stdlog;
use slog_term::{FullFormat, TermDecorator};

use testing::messages::{Payload, ResponseParameters};

/// Create a payload with the specified size.
pub fn new_payload(size: usize) -> Payload {
    let mut payload = Payload::new();
    payload.set_body(vec![0; size]);
    payload
}

pub fn new_parameters(size: i32) -> ResponseParameters {
    let mut parameter = ResponseParameters::new();
    parameter.set_size(size);
    parameter
}

pub fn create_test_server_credentials() -> ServerCredentials {
    let private_key = include_str!("../data/server1.key");
    let cert = include_str!("../data/server1.pem");
    ServerCredentialsBuilder::new()
        .add_cert(cert, private_key)
        .build()
}

pub fn create_test_channel_credentials() -> ChannelCredentials {
    let ca = include_str!("../data/ca.pem");
    ChannelCredentialsBuilder::new().root_cert(ca).build()
}

pub fn init_log() -> GlobalLoggerGuard {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, slog_o!());

    let guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    guard
}
