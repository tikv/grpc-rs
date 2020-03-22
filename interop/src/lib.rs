// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#![allow(unknown_lints)]

extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
#[macro_use]
extern crate log;
extern crate futures_timer;

mod client;
mod server;

pub use self::client::Client;
pub use self::server::InteropTestService;
