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


extern crate clap;
extern crate grpc;
extern crate interop;

use std::sync::Arc;

use clap::{App, Arg};
use grpc::{ChannelBuilder, Environment};

use interop::Client;

fn main() {
    let matches = App::new("Interoperability Test Client")
        .about("ref https://github.com/grpc/grpc/blob/v1.2.x/doc/interop-test-descriptions.md")
        .arg(Arg::with_name("host")
                 .long("server_host")
                 .help("The server host to connect to. For example, \"localhost\" or \"127.0.0.1\"")
                 .takes_value(true))
        .arg(Arg::with_name("port")
                 .long("server_port")
                 .help("The server port to connect to. For example, \"8080\"")
                 .takes_value(true))
        .arg(Arg::with_name("case")
                 .long("test_case")
                 .help("The name of the test case to execute. For example, \"empty_unary\"")
                 .takes_value(true))
        .get_matches();
    let host = matches.value_of("host").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("8080");
    let case = matches.value_of("case");

    let env = Arc::new(Environment::new(1));
    let channel = ChannelBuilder::new(env).connect(&format!("{}:{}", host, port));
    let client = Client::new(channel);

    let case_str = match case {
        None => {
            client.test_all();
            return;
        }
        Some(s) => s,
    };

    match case_str.to_uppercase().as_str() {
        "EMPTY_UNARY" => client.empty_unary(),
        "LARGE_UNARY" => client.large_unary(),
        "CLIENT_STREAMING" => client.client_streaming(),
        "SERVER_STREAMING" => client.server_streaming(),
        "PING_PONG" => client.ping_pong(),
        "EMPTY_STREAM" => client.empty_stream(),
        "CANCEL_AFTER_BEGIN" => client.cancel_after_begin(),
        "CANCEL_AFTER_FIRST_RESPONSE" => client.cancel_after_first_response(),
        "TIMEOUT_ON_SLEEPING_SERVER" => client.timeout_on_sleeping_server(),
        "STATUS_CODE_AND_MESSAGE" => client.status_code_and_message(),
        _ => panic!("unknown case: {:?}", case),
    }
}
