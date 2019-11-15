// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#[allow(clippy::all)]
mod bindings {
    include!(env!("BINDING_PATH"));
}
mod grpc_wrap;

pub use bindings::*;
pub use grpc_wrap::*;
