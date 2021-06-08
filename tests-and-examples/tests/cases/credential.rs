// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use futures::prelude::*;
use grpcio::{
    CertificateRequestType, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder, RpcContext,
    ServerBuilder, ServerCredentialsBuilder, ServerCredentialsFetcher, UnarySink,
};
use grpcio_proto::example::helloworld::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tests_and_examples::util::{read_cert_pair, read_single_crt};

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let msg = format!("Hello {}", req.get_name());
        let mut resp = HelloReply::default();
        resp.set_message(msg);
        let f = sink
            .success(resp)
            .map_err(move |e| panic!("failed to reply {:?}", e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

struct DataReload {
    switch: Arc<AtomicBool>,
}

impl ServerCredentialsFetcher for DataReload {
    fn fetch(&self) -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
        if self.switch.load(Ordering::Relaxed) {
            // The CN field in the certificate of server1 is "*.test.google.fr".
            let root = read_single_crt("ca")?;
            let (server1_crt, server1_key) = read_cert_pair("server1")?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(server1_crt.into(), server1_key.into())
                .root_cert(root, CertificateRequestType::DontRequestClientCertificate);
            Ok(Some(new_cred))
        } else {
            // The CN field in the certificate of server0 is "*.test.google.com.au".
            let root = read_single_crt("ca")?;
            let (server0_crt, server0_key) = read_cert_pair("server0")?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(server0_crt.into(), server0_key.into())
                .root_cert(root, CertificateRequestType::DontRequestClientCertificate);
            Ok(Some(new_cred))
        }
    }
}

struct DataReloadFail {
    initial: AtomicBool,
}

impl ServerCredentialsFetcher for DataReloadFail {
    fn fetch(&self) -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
        if self.initial.load(Ordering::Relaxed) {
            // Should return io::Error here.
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "")))
        } else {
            let root = read_single_crt("ca")?;
            let (server1_crt, server1_key) = read_cert_pair("server1")?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(server1_crt.into(), server1_key.into())
                .root_cert(root, CertificateRequestType::DontRequestClientCertificate);
            self.initial.store(true, Ordering::Relaxed);
            Ok(Some(new_cred))
        }
    }
}

#[test]
fn test_reload_new() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_greeter(GreeterService);
    let switch = Arc::new(AtomicBool::new(false));
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_with_fetcher(
            "127.0.0.1",
            0,
            Box::new(DataReload {
                switch: switch.clone(),
            }),
            CertificateRequestType::DontRequestClientCertificate,
        )
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;

    // To connect the server whose CN is "*.test.google.com.au".
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(read_single_crt("ca").unwrap().into())
        .build();
    let ch = ChannelBuilder::new(env.clone())
        .override_ssl_target("rust.test.google.com.au")
        .secure_connect(&format!("127.0.0.1:{}", port.clone()), cred);
    let client1 = GreeterClient::new(ch);
    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let reply = client1.say_hello(&req).expect("rpc");
    assert_eq!(reply.get_message(), "Hello world");

    // To connect the server whose CN is "*.test.google.fr".
    switch.store(true, Ordering::Relaxed);
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(read_single_crt("ca").unwrap().into())
        .build();
    let ch = ChannelBuilder::new(env.clone())
        .override_ssl_target("rust.test.google.fr")
        .secure_connect(&format!("127.0.0.1:{}", port.clone()), cred);
    let client2 = GreeterClient::new(ch);
    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let reply = client2.say_hello(&req).expect("rpc");
    assert_eq!(reply.get_message(), "Hello world");

    // Existing connection is still going to work.
    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let reply = client1.say_hello(&req).expect("rpc");
    assert_eq!(reply.get_message(), "Hello world");
}

#[test]
fn test_reload_fail() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_with_fetcher(
            "127.0.0.1",
            0,
            Box::new(DataReloadFail {
                initial: AtomicBool::new(false),
            }),
            CertificateRequestType::DontRequestClientCertificate,
        )
        .build()
        .unwrap();
    server.start();

    let port = server.bind_addrs().next().unwrap().1;
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(read_single_crt("ca").unwrap().into())
        .build();
    let ch = ChannelBuilder::new(env)
        .override_ssl_target("rust.test.google.fr")
        .secure_connect(&format!("127.0.0.1:{}", port), cred);
    let client = GreeterClient::new(ch);

    for _ in 0..10 {
        let mut req = HelloRequest::default();
        req.set_name("world".to_owned());
        let reply = client.say_hello(&req).expect("rpc");
        assert_eq!(reply.get_message(), "Hello world");
    }
}
