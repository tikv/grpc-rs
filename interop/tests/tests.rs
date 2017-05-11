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


extern crate grpc;
extern crate interop;
extern crate futures;
extern crate futures_cpupool;
extern crate grpc_proto;

macro_rules! mk_test {
    ($case_name:ident, $func:ident, $use_tls:expr) => (
        #[test]
        fn $case_name() {
            let pool = CpuPool::new(1);
            let env = Arc::new(Environment::new(2));

            let instance = InteropTestService::new(pool.clone());
            let service = test_grpc::create_test_service(instance);
            let mut builder = ServerBuilder::new(env.clone()).register_service(service);

            builder = if $use_tls {
                let creds = util::create_test_server_credentials();
                builder.bind_secure("localhost", 0, creds)
            } else {
                builder.bind("localhost", 0)
            };

            let mut server = builder.build();
            server.start();

            let builder = ChannelBuilder::new(env).override_ssl_target("foo.test.google.fr");
            let channel = {
                let (ref host, port) = server.bind_addrs()[0];
                if $use_tls {
                    let creds = util::create_test_channel_credentials();
                    builder.secure_connect(&format!("{}:{}", host, port), creds)
                } else {
                    builder.connect(&format!("{}:{}", host, port))
                }
            };

            let client = Client::new(channel);

            client.$func();

            let _ = server.shutdown().wait();
        }
    );
    ($func:ident) => {
        mod $func {
            use std::sync::Arc;

            use grpc::{Environment, ServerBuilder, ChannelBuilder};
            use interop::{InteropTestService, Client};
            use grpc_proto::testing::test_grpc;
            use grpc_proto::util;
            use futures::Future;
            use futures_cpupool::CpuPool;

            mk_test!(test_insecure, $func, false);
            mk_test!(test_secure, $func, true);
        }
    };
}

mk_test!(empty_unary);
mk_test!(large_unary);
mk_test!(client_streaming);
mk_test!(server_streaming);
mk_test!(ping_pong);
mk_test!(empty_stream);
mk_test!(cancel_after_begin);
mk_test!(cancel_after_first_response);
mk_test!(timeout_on_sleeping_server);
mk_test!(status_code_and_message);
mk_test!(unimplemented_method);
mk_test!(unimplemented_service);
