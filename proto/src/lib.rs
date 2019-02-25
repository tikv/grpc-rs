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

// There are some clippy lints in the generated protobuf files.
#![allow(clippy::renamed_and_removed_lints)]

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

#[allow(dead_code)]
pub mod testing_prost {
    include!(concat!(env!("OUT_DIR"), "/grpc.testing.rs"));
}

#[allow(dead_code)]
pub mod example_prost {
    pub mod helloworld {
        include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));
    }
    pub mod helloworld_grpc {}
    pub mod route_guide {
        include!(concat!(env!("OUT_DIR"), "/routeguide.rs"));
    }
    pub mod route_guide_grpc {}
}

#[allow(dead_code)]
pub mod health_prost {
    pub mod v1 {
        pub mod health {
            include!(concat!(env!("OUT_DIR"), "/grpc.health.v1.rs"));
        }
    }
}

pub mod util;
