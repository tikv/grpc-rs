// Copyright 2022 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "prost-codec")]
pub mod prost {
    pub mod example {
        pub mod helloworld;
        pub mod routeguide;
    }
    pub mod google {
        pub mod rpc {
            #[path = "google.rpc.rs"]
            mod status;

            pub use status::*;
        }
    }
    #[path = "testing/grpc.testing.rs"]
    pub mod testing;

    use testing::*;

    impl Mark {
        fn default_instance() -> &'static Mark {
            static MARK: Mark = Mark { reset: false };
            &MARK
        }
    }

    impl ClientConfig {
        fn default_instance() -> &'static ClientConfig {
            static CLIENT_CONFIG: ClientConfig = ClientConfig {
                server_targets: Vec::new(),
                client_type: 0,
                security_params: None,
                outstanding_rpcs_per_channel: 0,
                client_channels: 0,
                async_client_threads: 0,
                rpc_type: 0,
                load_params: None,
                payload_config: None,
                histogram_params: None,
                core_list: Vec::new(),
                core_limit: 0,
                other_client_api: String::new(),
                channel_args: Vec::new(),
                threads_per_cq: 0,
                messages_per_stream: 0,
            };
            &CLIENT_CONFIG
        }
    }

    impl ServerConfig {
        fn default_instance() -> &'static ServerConfig {
            static SERVER_CONFIG: ServerConfig = ServerConfig {
                server_type: 0,
                security_params: None,
                port: 0,
                async_server_threads: 0,
                core_limit: 0,
                payload_config: None,
                core_list: Vec::new(),
                other_server_api: String::new(),
                threads_per_cq: 0,
                resource_quota_size: 0,
                channel_args: Vec::new(),
            };
            &SERVER_CONFIG
        }
    }

    impl ClosedLoopParams {
        fn default_instance() -> &'static ClosedLoopParams {
            static CLOSED_LOOP_PARAMS: ClosedLoopParams = ClosedLoopParams {};
            &CLOSED_LOOP_PARAMS
        }
    }

    impl PoissonParams {
        fn default_instance() -> &'static PoissonParams {
            static POISSON_PARAMS: PoissonParams = PoissonParams { offered_load: 0f64 };
            &POISSON_PARAMS
        }
    }

    impl SimpleProtoParams {
        fn default_instance() -> &'static SimpleProtoParams {
            static SIMPLE_PROTO_PARAMS: SimpleProtoParams = SimpleProtoParams {
                req_size: 0,
                resp_size: 0,
            };
            &SIMPLE_PROTO_PARAMS
        }
    }

    impl ByteBufferParams {
        fn default_instance() -> &'static ByteBufferParams {
            static BYTE_BUFFER_PARAMS: ByteBufferParams = ByteBufferParams {
                req_size: 0,
                resp_size: 0,
            };
            &BYTE_BUFFER_PARAMS
        }
    }

    // Wrapper functions for oneof fields.
    impl ClientArgs {
        pub fn get_mark(&self) -> &Mark {
            match &self.argtype {
                ::std::option::Option::Some(client_args::Argtype::Mark(v)) => v,
                _ => Mark::default_instance(),
            }
        }
        pub fn get_setup(&self) -> &ClientConfig {
            match &self.argtype {
                ::std::option::Option::Some(client_args::Argtype::Setup(v)) => v,
                _ => ClientConfig::default_instance(),
            }
        }
    }
    impl ServerArgs {
        pub fn get_mark(&self) -> &Mark {
            match &self.argtype {
                ::std::option::Option::Some(server_args::Argtype::Mark(v)) => v,
                _ => Mark::default_instance(),
            }
        }
        pub fn get_setup(&self) -> &ServerConfig {
            match &self.argtype {
                ::std::option::Option::Some(server_args::Argtype::Setup(v)) => v,
                _ => ServerConfig::default_instance(),
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
                _ => ClosedLoopParams::default_instance(),
            }
        }
        pub fn get_poisson(&self) -> &PoissonParams {
            match &self.load {
                ::std::option::Option::Some(load_params::Load::Poisson(v)) => v,
                _ => PoissonParams::default_instance(),
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
                _ => SimpleProtoParams::default_instance(),
            }
        }
        pub fn get_bytebuf_params(&self) -> &ByteBufferParams {
            match &self.payload {
                ::std::option::Option::Some(payload_config::Payload::BytebufParams(v)) => v,
                _ => ByteBufferParams::default_instance(),
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

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
#[cfg_attr(feature = "protobuf-codec", path = "proto/protobuf")]
#[cfg_attr(feature = "protobufv3-codec", path = "proto/protobuf_v3")]
#[allow(deprecated)]
pub mod protobuf {
    pub mod example {
        pub mod helloworld;
        pub mod helloworld_grpc;

        pub mod route_guide;
        pub mod route_guide_grpc;
    }
    pub mod google {
        pub mod rpc {
            pub mod status;

            pub use status::*;
        }
    }
    pub mod testing {
        pub mod control;
        pub mod empty;
        pub mod messages;
        pub mod payloads;
        pub mod services;
        pub mod services_grpc;
        pub mod stats;
        pub mod test;
        pub mod test_grpc;
    }
}
