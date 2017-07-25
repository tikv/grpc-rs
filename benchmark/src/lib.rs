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

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(not(feature = "dev"), allow(unknown_lints))]

#[macro_use]
extern crate futures;
extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
extern crate grpcio_sys as grpc_sys;
extern crate libc;
extern crate rand;
extern crate tokio_timer;

mod bench;
mod client;
mod error;
mod server;
mod worker;
mod util;

pub use worker::Worker;
