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

// TODO: Remove it once Rust's tool_lints is stabilized.
// There are some clippy lints in the generated protobuf files.
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]

extern crate futures;
extern crate grpcio;
extern crate protobuf;

pub mod testing {
    include!(concat!(env!("OUT_DIR"), "/testing/mod.rs"));
}

pub mod example {
    include!(concat!(env!("OUT_DIR"), "/example/mod.rs"));
}

pub mod health {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/health/mod.rs"));
    }
}

pub mod util;
