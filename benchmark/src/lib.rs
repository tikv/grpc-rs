// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#![allow(unknown_lints)]

extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
extern crate grpcio_sys as grpc_sys;
#[cfg(target_os = "linux")]
extern crate libc;
#[macro_use]
extern crate log;

// Since impl trait is not stable yet, implement this as a function is impossible without box.
macro_rules! spawn {
    ($exec:ident, $keep_running:expr, $tag:expr, $f:expr) => {
        $exec.spawn(
            $f.map_err(move |e: grpcio::Error| {
                if $keep_running.load(Ordering::SeqCst) {
                    error!("failed to execute {}: {:?}", $tag, e);
                }
            })
            .map(|_| ()),
        )
    };
}

mod bench;
mod client;
mod server;
mod util;
mod worker;

pub use crate::util::log_util::init_log;
pub use crate::worker::Worker;
