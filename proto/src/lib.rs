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

#[allow(dead_code)]
pub mod testing {
    include!(concat!(env!("OUT_DIR"), "/grpc.testing.rs"));
    include!(concat!(env!("OUT_DIR"), "/wrapper_grpc.testing.rs"));

    // Wrapper functions for oneof fields.
    impl ClientArgs {
        pub fn get_mark(&self) -> &Mark {
            match &self.argtype {
                ::std::option::Option::Some(client_args::Argtype::Mark(v)) => v,
                _ => <Mark as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn get_setup(&self) -> &ClientConfig {
            match &self.argtype {
                ::std::option::Option::Some(client_args::Argtype::Setup(v)) => v,
                _ => <ClientConfig as ::protobuf::Message>::default_instance(),
            }
        }
    }
    impl ServerArgs {
        pub fn get_mark(&self) -> &Mark {
            match &self.argtype {
                ::std::option::Option::Some(server_args::Argtype::Mark(v)) => v,
                _ => <Mark as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn get_setup(&self) -> &ServerConfig {
            match &self.argtype {
                ::std::option::Option::Some(server_args::Argtype::Setup(v)) => v,
                _ => <ServerConfig as ::protobuf::Message>::default_instance(),
            }
        }
    }
    impl ChannelArg {
        pub fn get_str_value(&self) -> &str {
            match &self.value {
                ::std::option::Option::Some(channel_arg::Value::StrValue(v)) => v,
                _ => "",
            }
        }
        pub fn get_int_value(&self) -> i32 {
            match self.value {
                ::std::option::Option::Some(channel_arg::Value::IntValue(v)) => v,
                _ => 0,
            }
        }
        pub fn has_str_value(&self) -> bool {
            match self.value {
                ::std::option::Option::Some(channel_arg::Value::StrValue(_)) => true,
                _ => false,
            }
        }
        pub fn has_int_value(&self) -> bool {
            match self.value {
                ::std::option::Option::Some(channel_arg::Value::IntValue(_)) => true,
                _ => false,
            }
        }
    }
    impl LoadParams {
        pub fn get_closed_loop(&self) -> &ClosedLoopParams {
            match &self.load {
                ::std::option::Option::Some(load_params::Load::ClosedLoop(v)) => v,
                _ => <ClosedLoopParams as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn get_poisson(&self) -> &PoissonParams {
            match &self.load {
                ::std::option::Option::Some(load_params::Load::Poisson(v)) => v,
                _ => <PoissonParams as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn has_poisson(&self) -> bool {
            match self.load {
                ::std::option::Option::Some(load_params::Load::Poisson(_)) => true,
                _ => false,
            }
        }
    }
    impl PayloadConfig {
        pub fn get_simple_params(&self) -> &SimpleProtoParams {
            match &self.payload {
                ::std::option::Option::Some(payload_config::Payload::SimpleParams(v)) => v,
                _ => <SimpleProtoParams as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn get_bytebuf_params(&self) -> &ByteBufferParams {
            match &self.payload {
                ::std::option::Option::Some(payload_config::Payload::BytebufParams(v)) => v,
                _ => <ByteBufferParams as ::protobuf::Message>::default_instance(),
            }
        }
        pub fn has_bytebuf_params(&self) -> bool {
            match self.payload {
                ::std::option::Option::Some(payload_config::Payload::BytebufParams(_)) => true,
                _ => false,
            }
        }
    }
}

#[allow(dead_code)]
pub mod example {
    pub mod helloworld {
        include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));
        include!(concat!(env!("OUT_DIR"), "/wrapper_helloworld.rs"));
    }
    pub mod route_guide {
        include!(concat!(env!("OUT_DIR"), "/routeguide.rs"));
        include!(concat!(env!("OUT_DIR"), "/wrapper_routeguide.rs"));
    }
}

#[allow(dead_code)]
pub mod health {
    pub mod v1 {
        pub mod health {
            include!(concat!(env!("OUT_DIR"), "/grpc.health.v1.rs"));
            include!(concat!(env!("OUT_DIR"), "/wrapper_grpc.health.v1.rs"));
        }
    }
}

pub mod util;
