// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
extern crate interop;

macro_rules! mk_test {
    ($case_name:ident, $func:ident, $use_tls:expr) => {
        #[test]
        fn $case_name() {
            let env = Arc::new(Environment::new(2));

            let service = create_test_service(InteropTestService);
            let mut server = ServerBuilder::new(env.clone())
                .register_service(service)
                .build()
                .unwrap();
            let creds = if $use_tls {
                util::create_test_server_credentials()
            } else {
                grpcio::ServerCredentials::insecure()
            };
            let port = server.add_listening_port("127.0.0.1:0", creds).unwrap();
            server.start();

            let mut builder =
                ChannelBuilder::new(env.clone()).override_ssl_target("foo.test.google.fr");
            if $use_tls {
                let creds = util::create_test_channel_credentials();
                builder = builder.set_credentials(creds);
            }
            let channel = builder.connect(&format!("127.0.0.1:{port}"));
            let client = Client::new(channel);
            block_on(client.$func()).unwrap();
        }
    };
    ($func:ident) => {
        mod $func {
            use std::sync::Arc;

            use futures_executor::block_on;
            use grpc::{ChannelBuilder, Environment, ServerBuilder};
            use grpc_proto::testing::test_grpc::create_test_service;
            use grpc_proto::util;
            use interop::{Client, InteropTestService};

            mk_test!(test_insecure, $func, false);
            mk_test!(test_secure, $func, true);
        }
    };
}

mk_test!(empty_unary);
mk_test!(large_unary);
// FIXME(#305) Intermittent test.
#[cfg(not(feature = "openssl"))]
mk_test!(client_streaming);
mk_test!(server_streaming);
// FIXME(#306) Intermittent test.
#[cfg(not(feature = "openssl"))]
mk_test!(ping_pong);
mk_test!(custom_metadata);
mk_test!(empty_stream);
mk_test!(cancel_after_begin);
mk_test!(cancel_after_first_response);
mk_test!(timeout_on_sleeping_server);
mk_test!(status_code_and_message);
mk_test!(unimplemented_method);
mk_test!(unimplemented_service);
