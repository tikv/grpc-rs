use futures::Future;
use grpcio::{
    CertificateRequestType, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder, RpcContext,
    ServerBuilder, ServerCredentialsBuilder, ServerCredentialsFetcher, UnarySink,
};
use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};

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

fn read_root_crt() -> Result<String, std::io::Error> {
    let mut root = String::new();
    fs::File::open("certs/root.crt")
        .unwrap()
        .read_to_string(&mut root)
        .unwrap();
    Ok(root)
}

fn read_server1_creds() -> Result<(String, String), std::io::Error> {
    let mut server1_crt = String::new();
    let mut server1_key = String::new();
    fs::File::open("certs/server1.crt")?.read_to_string(&mut server1_crt)?;
    fs::File::open("certs/server1.key")?.read_to_string(&mut server1_key)?;
    Ok((server1_crt, server1_key))
}

fn read_server2_creds() -> Result<(String, String), std::io::Error> {
    let mut server2_crt = String::new();
    let mut server2_key = String::new();
    fs::File::open("certs/server2.crt")?.read_to_string(&mut server2_crt)?;
    fs::File::open("certs/server2.key")?.read_to_string(&mut server2_key)?;
    Ok((server2_crt, server2_key))
}

struct DataReload {
    flag: Arc<Mutex<(bool, bool)>>,
}

impl ServerCredentialsFetcher for DataReload {
    fn fetch(&self) -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
        let mut guard_flag = self.flag.lock().unwrap();
        if (*guard_flag).1 {
            return Ok(None);
        }
        if !(*guard_flag).0 {
            let root = read_root_crt()?;
            let (server1_crt, server1_key) = read_server1_creds()?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(server1_crt.into(), server1_key.into())
                .root_cert(root, CertificateRequestType::DontRequestClientCertificate);
            (*guard_flag).0 = true;
            Ok(Some(new_cred))
        } else {
            let root = read_root_crt()?;
            let (server2_crt, server2_key) = read_server2_creds()?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(server2_crt.into(), server2_key.into())
                .root_cert(root, CertificateRequestType::DontRequestClientCertificate);
            (*guard_flag).1 = true;
            Ok(Some(new_cred))
        }
    }
}

struct DataMeetFail {
    flag: Arc<Mutex<bool>>,
}

impl ServerCredentialsFetcher for DataMeetFail {
    fn fetch(&self) -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
        let mut guard_flag = self.flag.lock().unwrap();
        if *guard_flag {
            // Should return io::Error here.
            let _f = fs::File::open("Forsaken/Land")?;
            unimplemented!("You cannot execute here");
        } else {
            let root = read_root_crt()?;
            let (server1_crt, server1_key) = read_server1_creds()?;
            let new_cred = ServerCredentialsBuilder::new()
                .add_cert(server1_crt.into(), server1_key.into())
                .root_cert(root, CertificateRequestType::DontRequestClientCertificate);
            *guard_flag = true;
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
                flag: Arc::new(Mutex::new((false, false))),
            }),
            CertificateRequestType::DontRequestClientCertificate,
        )
        .build()
        .unwrap();
    server.start();

    let port = server.bind_addrs().next().unwrap().1;
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(read_root_crt().unwrap().into())
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

#[test]
fn test_reload_fail_open() {
    let env = Arc::new(EnvBuilder::new().build());
    let service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_with_fetcher(
            "localhost",
            0,
            Box::new(DataMeetFail {
                flag: Arc::new(Mutex::new(false)),
            }),
            CertificateRequestType::DontRequestClientCertificate,
        )
        .build()
        .unwrap();
    server.start();

    let port = server.bind_addrs().next().unwrap().1;
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(read_root_crt().unwrap().into())
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
