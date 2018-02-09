extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;

#[path = "../log_util.rs"]
mod log_util;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use grpcio_proto::example::helloworld::HelloRequest;
use grpcio_proto::example::helloworld_grpc::GreeterClient;

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let meta_arr = grpcio::MetadataArrayBuilder::new()
        .add(
            "hello".as_bytes().to_vec(),
            "world!".as_bytes().to_vec()
        )
        .build();
    let opt = grpcio::CallOption::default()
        .metadata(meta_arr);

    let mut req = HelloRequest::new();
    req.set_name("world".to_owned());
    let reply = client.say_hello_opt(&req, opt).expect("rpc");
    info!("Greeter received: {}", reply.get_message());
}
