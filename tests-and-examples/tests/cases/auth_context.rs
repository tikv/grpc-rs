use futures::*;
use grpcio::*;
use grpcio_proto::example::helloworld::*;
use grpcio_proto::example::helloworld_grpc::*;
use std::fs;
use std::io::Read;
use std::sync::mpsc::{self, Sender};
use std::sync::*;
use std::time::*;

#[derive(Clone)]
struct GreeterService {
    tx: Sender<(String, String)>,
}

impl Greeter for GreeterService {
    fn say_hello(
        &mut self,
        ctx: RpcContext<'_>,
        mut req: HelloRequest,
        sink: UnarySink<HelloReply>,
    ) {
        let auth_context = ctx.auth_context();
        self.tx
            .send((
                "AuthContextPresent".to_string(),
                (if auth_context.is_some() { "Y" } else { "N" }).to_string(),
            ))
            .unwrap();
        if let Some(auth_context) = auth_context {
            for (key, value) in auth_context
                .into_iter()
                .map(|x| (x.name(), x.value_str().unwrap()))
            {
                self.tx.send((key.to_owned(), value.to_owned())).unwrap();
            }
        }

        let mut resp = HelloReply::default();
        resp.set_message(format!("hello {}", req.take_name()));
        ctx.spawn(
            sink.success(resp)
                .map_err(|e| panic!("failed to reply {:?}", e)),
        );
    }
}

#[test]
fn test_auth_context() {
    let env = Arc::new(EnvBuilder::new().build());
    let (tx, rx) = mpsc::channel();
    let service = create_greeter(GreeterService { tx: tx });
    let (server_crt, server_key) = read_server1_creds().unwrap();
    let server_credentials = grpcio::ServerCredentialsBuilder::new()
        .root_cert(
            read_root_crt().unwrap(),
            CertificateRequestType::RequestClientCertificateAndVerify,
        )
        .add_cert(server_crt.into(), server_key.into())
        .build();
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_with_cred("127.0.0.1", 0, server_credentials)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;

    let (client_crt, client_key) = read_client1_creds().unwrap();
    let client_credentials = ChannelCredentialsBuilder::new()
        .root_cert(read_root_crt().unwrap().into())
        .cert(client_crt.clone().into(), client_key.into())
        .build();
    let ch = ChannelBuilder::new(env)
        .override_ssl_target("localhost")
        .secure_connect(&format!("127.0.0.1:{}", port), client_credentials);
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let resp = client.say_hello(&req).unwrap();

    assert_eq!(resp.get_message(), "hello world");

    let keys = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(keys, ("AuthContextPresent".to_owned(), "Y".to_owned()));
    // Test auth_context keys
    let keys = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(
        keys,
        ("transport_security_type".to_owned(), "ssl".to_owned())
    );
    let keys = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(
        keys,
        ("x509_common_name".to_owned(), "grpc-client-1".to_owned())
    );
    let keys = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(
        keys,
        ("x509_pem_cert".to_owned(), client_crt.replace("\r\n", "\n"))
    );
    let keys = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(keys, ("ssl_session_reused".to_owned(), "false".to_owned()));
    let _empty_keys: mpsc::RecvTimeoutError = rx
        .recv_timeout(Duration::from_millis(100))
        .expect_err("Received more auth_context vars than expected");
}

#[test]
fn test_no_crash_on_insecure() {
    let env = Arc::new(EnvBuilder::new().build());
    let (tx, rx) = mpsc::channel();
    let service = create_greeter(GreeterService { tx: tx });
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs().next().unwrap().1;

    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let resp = client.say_hello(&req).unwrap();

    assert_eq!(resp.get_message(), "hello world");

    // Test auth_context keys
    let keys = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(keys, ("AuthContextPresent".to_owned(), "N".to_owned()));
    let _empty_keys: mpsc::RecvTimeoutError = rx
        .recv_timeout(Duration::from_millis(100))
        .expect_err("Received auth context even though not authenticated");
}

fn read_root_crt() -> std::result::Result<String, std::io::Error> {
    let mut root = String::new();
    fs::File::open("certs/root.crt")
        .unwrap()
        .read_to_string(&mut root)
        .unwrap();
    Ok(root)
}

fn read_server1_creds() -> std::result::Result<(String, String), std::io::Error> {
    let mut server1_crt = String::new();
    let mut server1_key = String::new();
    fs::File::open("certs/server1.crt")?.read_to_string(&mut server1_crt)?;
    fs::File::open("certs/server1.key")?.read_to_string(&mut server1_key)?;
    Ok((server1_crt, server1_key))
}

fn read_client1_creds() -> std::result::Result<(String, String), std::io::Error> {
    let mut client1_crt = String::new();
    let mut client1_key = String::new();
    fs::File::open("certs/client1.crt")?.read_to_string(&mut client1_crt)?;
    fs::File::open("certs/client1.key")?.read_to_string(&mut client1_key)?;
    Ok((client1_crt, client1_key))
}
