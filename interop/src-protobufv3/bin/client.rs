// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate clap;
extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
extern crate interop;

use crate::grpc::{ChannelBuilder, ChannelCredentialsBuilder, Environment};
use crate::grpc_proto::util;
use clap::{App, Arg};
use std::sync::Arc;

use interop::Client;

fn main() {
    let matches = App::new("Interoperability Test Client")
        .about("ref https://github.com/grpc/grpc/blob/v1.3.x/doc/interop-test-descriptions.md")
        .arg(
            Arg::with_name("host")
                .long("server_host")
                .help("The server host to connect to. For example, \"localhost\" or \"127.0.0.1\"")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("host_override")
                .long("server_host_override")
                .help("The server host to connect to. For example, \"localhost\" or \"127.0.0.1\"")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("server_port")
                .help("The server port to connect to. For example, \"8080\"")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("case")
                .long("test_case")
                .help("The name of the test case to execute. For example, \"empty_unary\"")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("use_tls")
                .long("use_tls")
                .help("Whether to use a plaintext or encrypted connection")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("use_test_ca")
                .long("use_test_ca")
                .help("Whether to replace platform root CAs with ca.pem as the CA root")
                .takes_value(true),
        )
        .get_matches();
    let host = matches.value_of("host").unwrap_or("127.0.0.1");
    let host_override = matches
        .value_of("host_override")
        .unwrap_or("foo.test.google.fr");
    let port = matches.value_of("port").unwrap_or("8080");
    let case = matches.value_of("case");
    let use_tls: bool = matches
        .value_of("use_tls")
        .unwrap_or("false")
        .parse()
        .unwrap();
    let use_test_ca: bool = matches
        .value_of("use_test_ca")
        .unwrap_or("false")
        .parse()
        .unwrap();

    let env = Arc::new(Environment::new(1));
    let mut builder = ChannelBuilder::new(env).override_ssl_target(host_override.to_owned());
    if use_tls {
        let creds = if use_test_ca {
            util::create_test_channel_credentials()
        } else {
            ChannelCredentialsBuilder::new().build()
        };
        builder = builder.set_credentials(creds);
    }
    let channel = builder.connect(&format!("{host}:{port}"));

    let client = Client::new(channel);
    futures_executor::block_on(run_test(client, case)).unwrap();
}

async fn run_test(client: Client, case: Option<&str>) -> grpcio::Result<()> {
    let case_str = match case {
        None => {
            return client.test_all().await;
        }
        Some(s) => s,
    };

    match case_str.to_uppercase().as_str() {
        "EMPTY_UNARY" => client.empty_unary().await,
        "LARGE_UNARY" => client.large_unary().await,
        "CLIENT_STREAMING" => client.client_streaming().await,
        "SERVER_STREAMING" => client.server_streaming().await,
        "PING_PONG" => client.ping_pong().await,
        "CUSTOM_METADATA" => client.custom_metadata().await,
        "EMPTY_STREAM" => client.empty_stream().await,
        "CANCEL_AFTER_BEGIN" => client.cancel_after_begin().await,
        "CANCEL_AFTER_FIRST_RESPONSE" => client.cancel_after_first_response().await,
        "TIMEOUT_ON_SLEEPING_SERVER" => client.timeout_on_sleeping_server().await,
        "STATUS_CODE_AND_MESSAGE" => client.status_code_and_message().await,
        "UNIMPLEMENTED_METHOD" => client.unimplemented_method().await,
        "UNIMPLEMENTED_SERVICE" => client.unimplemented_service().await,
        _ => panic!("unknown case: {:?}", case),
    }
}
