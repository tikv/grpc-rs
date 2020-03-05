use futures::Future;
use grpcio::{
    CertificateRequestType, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder, RpcContext,
    ServerBuilder, ServerCredentialsBuilder, ServerCredentialsFetcher, UnarySink,
};
use std::fs;
use std::io::Read;
use std::sync::Arc;

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

fn get_ca_crt() -> String {
    let mut buf = String::new();
    fs::File::open("certs/ca.crt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    buf
}

struct DataInitial {
    flag: bool,
}

impl ServerCredentialsFetcher for DataInitial {
    fn fetch(&mut self) -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
        if self.flag {
            return Ok(None);
        }
        let mut pd_crt = String::new();
        fs::File::open("certs/pd.crt")?.read_to_string(&mut pd_crt)?;
        let mut pd_key = String::new();
        fs::File::open("certs/pd.key")?.read_to_string(&mut pd_key)?;
        let mut ca_crt = String::new();
        fs::File::open("certs/ca.crt")?.read_to_string(&mut ca_crt)?;
        let new_cred = ServerCredentialsBuilder::new()
            .add_cert(pd_crt.into(), pd_key.into())
            .root_cert(ca_crt, CertificateRequestType::DontRequestClientCertificate);
        self.flag = true;
        Ok(Some(new_cred))
    }
}

struct DataMeetFail {
    flag: bool,
}

impl ServerCredentialsFetcher for DataMeetFail {
    fn fetch(&mut self) -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
        if self.flag {
            let mut f = fs::File::open("Forsaken/Land")?;
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            return Ok(None);
        } else {
            let mut pd_crt = String::new();
            fs::File::open("certs/pd.crt")?.read_to_string(&mut pd_crt)?;
            let mut pd_key = String::new();
            fs::File::open("certs/pd.key")?.read_to_string(&mut pd_key)?;
            let mut ca_crt = String::new();
            fs::File::open("certs/ca.crt")?.read_to_string(&mut ca_crt)?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(pd_crt.into(), pd_key.into())
                .root_cert(ca_crt, CertificateRequestType::DontRequestClientCertificate);
            self.flag = true;
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
            Box::new(DataInitial { flag: false }),
            CertificateRequestType::DontRequestClientCertificate,
        )
        .build()
        .unwrap();
    server.start();

    let port = server.bind_addrs().next().unwrap().1;
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(get_ca_crt().into())
        .build();
    let ch = ChannelBuilder::new(env).secure_connect(&format!("localhost:{}", port), cred);
    let client = GreeterClient::new(ch);

    for _ in 0..10 {
        let mut req = HelloRequest::default();
        req.set_name("world".to_owned());
        let reply = client.say_hello(&req).expect("rpc");
        assert_eq!(reply.get_message(), "Hello world");
    }
}

#[test]
fn test_reload_fail_open() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_with_fetcher(
            "localhost",
            0,
            Box::new(DataMeetFail { flag: false }),
            CertificateRequestType::DontRequestClientCertificate,
        )
        .build()
        .unwrap();
    server.start();

    let port = server.bind_addrs().next().unwrap().1;
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(get_ca_crt().into())
        .build();
    let ch = ChannelBuilder::new(env).secure_connect(&format!("localhost:{}", port), cred);
    let client = GreeterClient::new(ch);

    for _ in 0..10 {
        let mut req = HelloRequest::default();
        req.set_name("world".to_owned());
        let reply = client.say_hello(&req).expect("rpc");
        assert_eq!(reply.get_message(), "Hello world");
    }
}
