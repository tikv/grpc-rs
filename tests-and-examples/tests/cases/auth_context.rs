// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

use futures_util::future::{FutureExt as _, TryFutureExt as _};
use grpcio::*;
use grpcio_proto::example::helloworld::*;

use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::sync::*;
use std::time::*;

use tests_and_examples::util::{read_cert_pair, read_single_crt};

#[derive(Clone)]
struct GreeterService {
    tx: Sender<Option<HashMap<String, String>>>,
}

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, req: HelloRequest, sink: UnarySink<HelloReply>) {
        if let Some(auth_context) = ctx.auth_context() {
            let mut ctx_map = HashMap::new();
            for (key, value) in auth_context
                .into_iter()
                .map(|x| (x.name(), x.value_str().unwrap()))
            {
                ctx_map.insert(key.to_owned(), value.to_owned());
            }
            self.tx.send(Some(ctx_map)).unwrap();
        }

        let mut resp = HelloReply::default();
        resp.message = format!("hello {}", req.name);
        ctx.spawn(
            sink.success(resp)
                .map_err(|e| panic!("failed to reply {:?}", e))
                .map(|_| ()),
        );
    }
}

#[test]
fn test_auth_context() {
    let env = Arc::new(EnvBuilder::new().build());
    let (tx, rx) = mpsc::channel();
    let service = create_greeter(GreeterService { tx });
    let (server_crt, server_key) = read_cert_pair("server1").unwrap();
    let server_credentials = grpcio::ServerCredentialsBuilder::new()
        .root_cert(
            read_single_crt("ca").unwrap(),
            CertificateRequestType::RequestClientCertificateAndVerify,
        )
        .add_cert(server_crt.into(), server_key.into())
        .build();
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", server_credentials)
        .unwrap();
    server.start();

    let (client_crt, client_key) = read_cert_pair("client1").unwrap();
    let client_credentials = ChannelCredentialsBuilder::new()
        .root_cert(read_single_crt("ca").unwrap().into())
        .cert(client_crt.clone().into(), client_key.into())
        .build();
    let ch = ChannelBuilder::new(env)
        .override_ssl_target("rust.test.google.fr")
        .set_credentials(client_credentials)
        .connect(&format!("127.0.0.1:{port}"));
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.name = "world".to_owned();
    let resp = client.say_hello(&req).unwrap();

    assert_eq!(resp.message, "hello world");

    // Test auth_context keys
    let ctx_map = rx.recv_timeout(Duration::from_secs(1)).unwrap().unwrap();

    assert_eq!(ctx_map.get("transport_security_type").unwrap(), "ssl");
    assert_eq!(ctx_map.get("x509_common_name").unwrap(), "testclient1");
    assert_eq!(
        ctx_map.get("x509_pem_cert").unwrap(),
        &client_crt.replace("\r\n", "\n")
    );
    assert_eq!(
        ctx_map.get("security_level").unwrap(),
        "TSI_PRIVACY_AND_INTEGRITY"
    );
    assert_eq!(ctx_map.get("ssl_session_reused").unwrap(), "false");
    assert!(ctx_map.get("x509_subject_alternative_name").is_none());
}

#[test]
fn test_no_crash_on_insecure() {
    let env = Arc::new(EnvBuilder::new().build());
    let (tx, rx) = mpsc::channel();
    let service = create_greeter(GreeterService { tx });
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();

    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.name = "world".to_owned();
    let resp = client.say_hello(&req).unwrap();

    assert_eq!(resp.message, "hello world");

    // Test auth_context keys
    let ctx_map = rx.recv_timeout(Duration::from_secs(1)).unwrap().unwrap();
    assert_eq!(ctx_map.get("transport_security_type").unwrap(), "insecure");
    assert_eq!(ctx_map.get("security_level").unwrap(), "TSI_SECURITY_NONE");
}
