// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

extern crate clap;
extern crate grpcio as grpc;
extern crate grpcio_proto as grpc_proto;
extern crate interop;

use crate::grpc::{ChannelBuilder, ChannelCredentialsBuilder, Environment};
use crate::grpc_proto::util;
use clap::Parser;
use std::sync::Arc;

use interop::Client;

/// Interoperability Test Client
///
/// ref https://github.com/grpc/grpc/blob/v1.3.x/doc/interop-test-descriptions.md.
#[derive(Parser)]
struct ClientCli {
    /// The server host to connect to. For example, "localhost" or "127.0.0.1"
    #[arg(long)]
    host: Option<String>,
    /// The server host client pretend to connect. It's used for testing SSL/TLS to an arbitrary host.
    #[arg(long)]
    host_override: Option<String>,
    /// The server port to connect to. For example, 8080
    #[arg(long)]
    port: Option<u16>,
    /// The name of the test case to execute. For example, "empty_unary"
    #[arg(long)]
    case: Option<String>,
    /// Whether to use a plaintext or encrypted connection
    #[arg(long)]
    use_tls: Option<bool>,
    /// Whether to replace platform root CAs with ca.pem as the CA root
    #[arg(long)]
    use_test_ca: Option<bool>,
}

fn main() {
    let cli = ClientCli::parse();
    let host = cli.host.as_deref().unwrap_or("127.0.0.1");
    let host_override = cli.host_override.as_deref().unwrap_or("foo.test.google.fr");
    let port = cli.port.unwrap_or(8080);
    let case = cli.case.as_deref();
    let use_tls = cli.use_tls.unwrap_or(false);
    let use_test_ca = cli.use_test_ca.unwrap_or(false);

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
