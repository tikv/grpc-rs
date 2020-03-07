use futures::Future;
use grpcio::{
    CertificateRequestType, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder, RpcContext,
    ServerBuilder, ServerCredentialsBuilder, ServerCredentialsFetcher, UnarySink,
};
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tests_and_examples::util::{read_cert_pair, read_single_crt};

use grpcio_proto::example::helloworld::*;
use grpcio_proto::example::helloworld_grpc::*;

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let msg = format!("Hello {}", req.get_name());
        let mut resp = HelloReply::default();
        resp.set_message(msg);
        let f = sink
            .success(resp)
            .map_err(move |e| panic!("failed to reply {:?}", e));
        ctx.spawn(f)
    }
}

struct DataReload {
    initial: AtomicBool,
    reload: AtomicBool,
}

impl ServerCredentialsFetcher for DataReload {
    fn fetch(&self) -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
        if self.reload.load(Ordering::Relaxed) {
            return Ok(None);
        }
        if self.initial.load(Ordering::Relaxed) {
            let root = read_single_crt("root")?;
            let (server2_crt, server2_key) = read_cert_pair("server2")?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(server2_crt.into(), server2_key.into())
                .root_cert(root, CertificateRequestType::DontRequestClientCertificate);
            self.reload.store(true, Ordering::Relaxed);
            Ok(Some(new_cred))
        } else {
            let root = read_single_crt("root")?;
            let (server1_crt, server1_key) = read_cert_pair("server1")?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(server1_crt.into(), server1_key.into())
                .root_cert(root, CertificateRequestType::DontRequestClientCertificate);
            self.initial.store(true, Ordering::Relaxed);
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
            let _f = fs::File::open("Forsaken/Land")?;
            unimplemented!("You cannot execute here");
        } else {
            let root = read_single_crt("root")?;
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
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_with_fetcher(
            "localhost",
            0,
            Box::new(DataReload {
                initial: AtomicBool::new(false),
                reload: AtomicBool::new(false),
            }),
            CertificateRequestType::DontRequestClientCertificate,
        )
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;

    // To connect the server whose CN is "remotehost"
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(read_single_crt("root").unwrap().into())
        .build();
    let ch = ChannelBuilder::new(env)
        .override_ssl_target("remotehost")
        .secure_connect(&format!("localhost:{}", port), cred);
    let client = GreeterClient::new(ch);
    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let reply = client.say_hello(&req).expect("rpc");
    assert_eq!(reply.get_message(), "Hello world");
}

#[test]
fn test_reload_fail() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_with_fetcher(
            "localhost",
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
        .root_cert(read_single_crt("root").unwrap().into())
        .build();
    let ch = ChannelBuilder::new(env).secure_connect(&format!("localhost:{}", port), cred);
    let client = GreeterClient::new(ch);

    for _ in 0..3 {
        let mut req = HelloRequest::default();
        req.set_name("world".to_owned());
        let reply = client.say_hello(&req).expect("rpc");
        assert_eq!(reply.get_message(), "Hello world");
    }
}
