use futures::*;
use grpcio::*;
use grpcio_proto::example::helloworld::*;
use grpcio_proto::example::helloworld_grpc::*;
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
    let server_credentials = grpcio::ServerCredentialsBuilder::new()
        .root_cert(CA_CRT.as_bytes(), true)
        .add_cert(
            SERVER_CRT.as_bytes().to_owned(),
            SERVER_KEY.as_bytes().to_owned(),
        )
        .build();
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_secure("127.0.0.1", 0, server_credentials)
        .build()
        .unwrap();
    server.start();
    let port = server.bind_addrs()[0].1;

    let client_credentials = ChannelCredentialsBuilder::new()
        .root_cert(CA_CRT.as_bytes().to_owned())
        .cert(
            CLIENT_CRT.as_bytes().to_owned(),
            CLIENT_KEY.as_bytes().to_owned(),
        )
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
        (
            "x509_common_name".to_owned(),
            "grpc-rs Test Client".to_owned()
        )
    );
    let keys = rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(
        keys,
        ("x509_pem_cert".to_owned(), CLIENT_CRT.to_owned() + "\n")
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
    let port = server.bind_addrs()[0].1;

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

static CA_CRT: &str = r#"-----BEGIN CERTIFICATE-----
MIIDozCCAougAwIBAgIUbDIVUVThvKdy/e+Ohe5PFZz5vE0wDQYJKoZIhvcNAQEL
BQAwYDELMAkGA1UEBhMCRlIxDzANBgNVBAgMBkZyYW5jZTEOMAwGA1UEBwwFUGFy
aXMxEDAOBgNVBAoMB2dycGMtcnMxHjAcBgNVBAMMFWdycGMtcnMgdGVzdHMgUm9v
dCBDQTAgFw0xOTA1MDcwOTMzNDlaGA8zMDE4MDkwNzA5MzM0OVowYDELMAkGA1UE
BhMCRlIxDzANBgNVBAgMBkZyYW5jZTEOMAwGA1UEBwwFUGFyaXMxEDAOBgNVBAoM
B2dycGMtcnMxHjAcBgNVBAMMFWdycGMtcnMgdGVzdHMgUm9vdCBDQTCCASIwDQYJ
KoZIhvcNAQEBBQADggEPADCCAQoCggEBAORLQaYDhLWtMdPPD+LU4QOT1t2Uh4WS
l08H/ahNHvQo1ZZJFvFftM9i0jg8Fg6nntJOFhniy7ctGgWnJYf2gzEdFzmWdTYu
S6tQghzwJ0EGT69jTHStgLm4v/BiuVEWhI8ChXkwpUcRjBG6U1nOm8wRJOFbmV5j
ha1f7vbyqk8I3RaDpKYhiJvuSxRrUwhqdYdM2TYjXuEc2/LQfRmeIqjlnxPIUD9Z
cbPtble8SRcYKDlMa8rHKSCrmRtDwKL928ISrO5f4JyOXwMt/k0EOLu6F4/Hb7ZE
iajUheycuskLIt6m/3mGjWJN5ueE20wXnGESKnUgyu7iqMqmGq6kcFMCAwEAAaNT
MFEwHQYDVR0OBBYEFO01joDHYIk56VpZ2qBGn4bLUJdkMB8GA1UdIwQYMBaAFO01
joDHYIk56VpZ2qBGn4bLUJdkMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQEL
BQADggEBALcvExHiGqRedzckcTczcothpiOxSc4ijZD70iMhqQH4sR09hoPP0uAs
U1avxftki5oV15EgOAUhw8EGZHI8fCRfLnLkcy1OPJz8NLypr792g8SLiaabhsVO
MzFemDgAkz9KY2ZReqkV4IxYaEG75f0/EJpWiXFkkjpz6GrFrMzArhUBH6/AqXjB
IQPic4x4CKs3cbw7WUgXSWki59Ynpa4RQmp4N7CRZIsR2qutNcbwwZhLFJUNCIf5
ZM31ga7ARFnFNoWS189mcybu41+EZ0+U7i+nxCYaXb1ztGuu+nRexxnfM/ABnwla
Y4aqBDTLfluWe78NdR7XDEIZ176ukug=
-----END CERTIFICATE-----"#;

static SERVER_CRT: &str = r#"-----BEGIN CERTIFICATE-----
MIIDPTCCAiUCFBH6c/YUjvTe8EmdS5jhnePyW3XwMA0GCSqGSIb3DQEBCwUAMGAx
CzAJBgNVBAYTAkZSMQ8wDQYDVQQIDAZGcmFuY2UxDjAMBgNVBAcMBVBhcmlzMRAw
DgYDVQQKDAdncnBjLXJzMR4wHAYDVQQDDBVncnBjLXJzIHRlc3RzIFJvb3QgQ0Ew
IBcNMTkwNTA3MDkzODU0WhgPMzAxODA5MDcwOTM4NTRaMFQxCzAJBgNVBAYTAkZS
MQ8wDQYDVQQIDAZGcmFuY2UxDjAMBgNVBAcMBVBhcmlzMRAwDgYDVQQKDAdncnBj
LXJzMRIwEAYDVQQDDAlsb2NhbGhvc3QwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAw
ggEKAoIBAQDUnmhaasfyckI7nG1qqc52W8HpbUClv7P2XqxP78FYZ9KSRwCTo2oH
6DVKRdeSuc0fyXgQd5cN0wCOBn8uufCWjmAjRmATFK3fyi/cN3yMAh90I1dPeBI5
UOow4XoPKNGit/+Hh0tD+7q380rMqYIW4JjLctrP6Nk7ZlnHx6XSS3MhrrCHX7SD
jYfPW5Dky2uOWRYOJ5Oz0jVvhbmgZx0U6/L/OoK2Gb/CmVKm62nrMGYodUFYIseC
GtpmFGlWLjVtfqx/+5xY4b06ZVxFGn6/0mvPzuCQsMWLeq0Q2LbiTXeMLcw8u3u4
+kS5mz8s55eU6VQHtqHZoHkWok9bHv95AgMBAAEwDQYJKoZIhvcNAQELBQADggEB
AHlts23/rrtIuwrEDfGcwNyOqKTddQKZ4hxg6HzWewEw5vM6kUHsOUZWHrpOxolU
Lb4UPmxw5b5emBkEqMt3MMHRFRWEBV/Kke2hRkj2F8ejyoSu80gBFeAb9FSYLHEg
P4oLc2NdIYhewLiq8f+lsPEfa7XAkk1TNYAtUUb7CCV4ObuNjA4ENQdUOggslhcE
IY+x+pdPTYnaw7o+ADC0DSl7BFXVrJxA12YCCh1Yg5rFkhBxCydQkIDMTBKnXLoC
47cieX2q516T2xrudAXky2hPI7ofySBBXhNDLxXDosL0c5/1LRWKVACHf8PodUuX
MCWjE24WEc6m0Mwe0P1z+30=
-----END CERTIFICATE-----"#;

static SERVER_KEY: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA1J5oWmrH8nJCO5xtaqnOdlvB6W1Apb+z9l6sT+/BWGfSkkcA
k6NqB+g1SkXXkrnNH8l4EHeXDdMAjgZ/Lrnwlo5gI0ZgExSt38ov3Dd8jAIfdCNX
T3gSOVDqMOF6DyjRorf/h4dLQ/u6t/NKzKmCFuCYy3Laz+jZO2ZZx8el0ktzIa6w
h1+0g42Hz1uQ5MtrjlkWDieTs9I1b4W5oGcdFOvy/zqCthm/wplSputp6zBmKHVB
WCLHghraZhRpVi41bX6sf/ucWOG9OmVcRRp+v9Jrz87gkLDFi3qtENi24k13jC3M
PLt7uPpEuZs/LOeXlOlUB7ah2aB5FqJPWx7/eQIDAQABAoIBAAuj8IqitfesEbw5
wDPKwCbytxcUiCmK6Utfu4dU1P0GEh04DewHUlLEf8ZlNHwv9+75lL0yDu8O0kT0
Tho8y9MyUEkeHTxuMWbDJwOjdgokQ5yUIr+TD0xcBbZPYeNalL+X20GMQq6iJYLU
qw+xaBHgkfzxeNFRKjN6UwgBAi8VINGRmnanKxVywzwAnyn45YoBPSvN/1OScpX8
Uc7Ih6uYbnkvZwVsCzGjUGKuf63Z+xaqGhTasjh5DEVxbTXkpbpr55X84oDmgdGR
aRuGZuNkbRMmcNbJIOl+p7zi5T3nOKyNv58MvSgFGV64+CYCqV45Z2454o4ztvrC
kaGP/kECgYEA9HsIN8wUjDHiVaJDSMxYM/iJ1yI6Jdc0CsDccoijw+1L3oVLJnzr
9I0fhJZcZUHu1qjHnnxDPoD3//PxupJUxwqJlXanaT3aAQnEnoBjVJETbKxF1DAK
srvtsXRXqh+xtvw4TxCDWmjnV5frkhscxdYj7EphNkidqGpAWa0yqEUCgYEA3qMN
f0cFZaKW4lp7qKjbItRl/lkCCiWNdbXfzZQHYf48VEoj27yHFCZ0zxBaVkJlKv5P
HfBdvwDnJwohWg0QQRM3KJiQYhGKQJu00ECcvHZt6TGN5p4nYRt/MieW35xVE1fZ
OOf75ZxmKaXxZcsZWPWnmyMgR+LB8ixqNVAyj6UCgYEAkQYajd7FoZRFyr6CY0C0
Yb/hJHjtPV03Rdm560cavjKZetpfBem8nkjIlYIMNRSzdSqRn3Xe/cjLVE2E60O6
fofFrZ/BFzDVjSAaPluykIuulhxrH7+P2Q6ZpVZ6WjSK2x0q2dvVz1/DO7amyvjC
TKkGjjhhCi5xN2Iv4gJXtnECgYByyeLlorqXonLlFf34Aceqd5R2xXGVxbLf4/wW
RKJh1fFJDEjcvKFwtDs5n5bULrt2pVQpfYr4NgD4UfLMOTwOMVgZKEAZMC32DdH2
JboqOW2F9DTTmOt41/KG636QoEwFAyBOhN2Nj0vlYVfLQiI5ib727+9u4O1abaf9
APJwuQKBgQDvnkqP6MgNvoKhKH1WclF2mPrtC4uZgA1KhVqTyEtreyw6A9UV5lSX
tQDKG2lRRgJ+U1178KtuJsh1bzFItqkkEPBArwAbA44DTrSu+AEpmDPVQXscsONR
SQEXSC7d6N3mzuUK1S4TJc6pcb789YYuOl3dXrO/fLPSwBJtUPjISw==
-----END RSA PRIVATE KEY-----"#;

static CLIENT_CRT: &str = r#"-----BEGIN CERTIFICATE-----
MIIDRzCCAi8CFBH6c/YUjvTe8EmdS5jhnePyW3XxMA0GCSqGSIb3DQEBCwUAMGAx
CzAJBgNVBAYTAkZSMQ8wDQYDVQQIDAZGcmFuY2UxDjAMBgNVBAcMBVBhcmlzMRAw
DgYDVQQKDAdncnBjLXJzMR4wHAYDVQQDDBVncnBjLXJzIHRlc3RzIFJvb3QgQ0Ew
IBcNMTkwNTA3MDkzOTE5WhgPMzAxODA5MDcwOTM5MTlaMF4xCzAJBgNVBAYTAkZS
MQ8wDQYDVQQIDAZGcmFuY2UxDjAMBgNVBAcMBVBhcmlzMRAwDgYDVQQKDAdncnBj
LXJzMRwwGgYDVQQDDBNncnBjLXJzIFRlc3QgQ2xpZW50MIIBIjANBgkqhkiG9w0B
AQEFAAOCAQ8AMIIBCgKCAQEA0zxpjVNg4kntLL31VR/zQO4YE6MtyqJOBVKbeCc6
R15KLYC5zjz4NchPouquJX23JmLoLtMgBnoIlEC6LbwckqxrYoOrT8FfBWyIHMrI
LGGQtQv/1wFKtOsbZugaRoUpklIkv6h2kOHO9Sj7TKpVRzNLXADUtiCClzxncg1T
z98G/PrbCw7LZB09S9KiJnUV6QEHuhqiG0ROrSW+/rH5IHjtsaFDh9IBQNCkneMD
D2KfbpQ/k/Z+8KckBSNpPRRLIioG6ZJGu0W/ZNHW7/CvHfxnM1lW+w4aNcmW0FZv
zAP1sA2GHfYp7N3QXt0ORdPjyOtQwM0xS3aPf1CaCEj5DwIDAQABMA0GCSqGSIb3
DQEBCwUAA4IBAQAQGVYG3wXu1EZuQTwW8ec0ROwKEoJltKsoRAvHHXvwpEOCmDyl
8Gxk5j+O8Xytc4lBcViAz8EJ6senGk1RdNiSQLAf0dGZPsfuCGrp13gGG4ZbMXZX
oI4Z2lTvxTEVMMpAoHHQheYRYZk1z3gKHeKuiu4tAiti39iCS1SninrINBlvU5ZS
ijUszQEhmGoYb2OxjBHw8iYwE9GC33rCNWsONGXGkM7wNt3A64SYwOsXXT3wG8CX
kqX0HFP2JaX7x0BqG1Jh0VZ+QoEKmfPOREv20q8aO6+s0mEu+88eGpnJWlBXhtfD
LOqbHJD2Bktma6qlvdjMFHgkJVon+Z1pIoes
-----END CERTIFICATE-----"#;

static CLIENT_KEY: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQEA0zxpjVNg4kntLL31VR/zQO4YE6MtyqJOBVKbeCc6R15KLYC5
zjz4NchPouquJX23JmLoLtMgBnoIlEC6LbwckqxrYoOrT8FfBWyIHMrILGGQtQv/
1wFKtOsbZugaRoUpklIkv6h2kOHO9Sj7TKpVRzNLXADUtiCClzxncg1Tz98G/Prb
Cw7LZB09S9KiJnUV6QEHuhqiG0ROrSW+/rH5IHjtsaFDh9IBQNCkneMDD2KfbpQ/
k/Z+8KckBSNpPRRLIioG6ZJGu0W/ZNHW7/CvHfxnM1lW+w4aNcmW0FZvzAP1sA2G
HfYp7N3QXt0ORdPjyOtQwM0xS3aPf1CaCEj5DwIDAQABAoIBAGqkLy2YEGttov1Z
kUe+b5IyIZmYYf/Q3M63G6IXO8bYBV7fg+5ovW55iCrZi72FsDcC6/DnyV07sqUV
4GhCdbJetX/wtUX4e75DQpw2i8RMJUAhpLGpB0w6/YSAXPOJOqmSKtdPdzxBo7gF
Tokv3QmrDbqO+NRRqVwstJTRwaRJK3KpVIuCbB2JRoGfY9ztDnWFd3qmbFTpeVzA
Bd3zvRuHtMWmxYtY6mQ9ZwCO+3zw2/6Vj6eBjNd/EMm4plbZXMye7ih7IzFnswub
q+6jkRJNnICF17En4CYBLyMNmpeFgczqZiRhMv/2s/Z1t6d0SCE29QB8bpApbnEb
7f56e/ECgYEA6YIm+3OkrZCodM0LoDG3sDRMEaImZWWqkyDeS6I3oG69a+QgZNlJ
thA89EqqTTYryV5Y7Px7dDNVVS+uE39S+5y02Vu8uC1lq3eqJe9aempOQo7zI7XO
+LEGHuH8ufN+pOARDX+46G99q0liXheIuuYcI3MPFE8ZAFdVtgxasIUCgYEA55UR
IP8+O5mR0EmfRbPM5HCyE0NA3GI9ONSVDVX/PG1rl7xMhlXH3cYYo4sOB8nMcIWc
mHPJT5gqTGvgwrv6UpXg7FEV42ILL+z6sbY0AT/EvuDzFNbaQYV++8pJ0gg+jfc2
nguyoxyFXMTPvKjykc8l2YrzLwpS5HL/xaZ5oYMCgYBaSj5CioPJrR7GEycEqVTK
qF4lj1pjaL1dKJ9sBWyW91r9Jfe0pyROZtb66wgQ1Pp3Yajm8fXWh6beSpJ37Yqr
yftP9J3zZXJjq+C5zGRi40ohiF7RlxJRrTmdpWpwiIuWuyE84/8euUWRfs2vblAr
XKDSwEG7a55SOHLm8r36dQKBgA/S0a7NgfUUkzAGN8nNdUyOnrxONhpY5BIEBIM6
wwvf9iASD7CGH/f5VymlkLnbggqWNpL8Jl836Hv8SwlDSuFluE6tY697vYPyz1Zh
OsBzGQBp4nIkFBmyA3E060JcqdmNcL/v0K7wffROxlr/uENaQh3RLriiHfPaH7vj
mh/9AoGAEqDij1mu1KtwgjDo/2vRZDC8/iWARPdkM+/5KX+NQyEueQ7DVCJ395of
yq+fZMhZpCMrnHsTIbTa3T1+GMzghqYASI0DJPLk9z18AeMP/5rWCe8KjitTvYDH
iEmsL1oMGCGH/W5H68gboDXxWcvS8PE7gA4NrNJhLzLorRF8SoQ=
-----END RSA PRIVATE KEY-----"#;

// CA key:
// -----BEGIN PRIVATE KEY-----
// MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDkS0GmA4S1rTHT
// zw/i1OEDk9bdlIeFkpdPB/2oTR70KNWWSRbxX7TPYtI4PBYOp57SThYZ4su3LRoF
// pyWH9oMxHRc5lnU2LkurUIIc8CdBBk+vY0x0rYC5uL/wYrlRFoSPAoV5MKVHEYwR
// ulNZzpvMESThW5leY4WtX+728qpPCN0Wg6SmIYib7ksUa1MIanWHTNk2I17hHNvy
// 0H0ZniKo5Z8TyFA/WXGz7W5XvEkXGCg5TGvKxykgq5kbQ8Ci/dvCEqzuX+Ccjl8D
// Lf5NBDi7uhePx2+2RImo1IXsnLrJCyLepv95ho1iTebnhNtMF5xhEip1IMru4qjK
// phqupHBTAgMBAAECggEAO7vlRYcOJmtW6lD3myaQads5EWlXvH+Kz8cLfAoR21kD
// 6frd4fJk4Q6+arBuJQ9+4xBDgCpzm05VzLzbeSPrV+KAvTS9HpwmeIDsSp4b50u7
// dezJiI4NbIsGzeNHEgyOSs0iF3++/0V8HPdf79O7uD2UJNNO4f+wbpj3ce402dC9
// QrNW3Oy1J4sSDS7nBhm0UJfboTAPngifrk0FSPwOaNDZUYC8Nq9oUWVWr6dIW7Us
// exqJczXQRn5oYWMBXiWzMWS8vCKfdPR7m7ZmS2QjCbehcOXddDsaAUr2pZhQhG/1
// PRqV8QRkB7Z8mLyphVXi3VNIo/hr7AXhKGf4y/7EoQKBgQD9QOxr/xgPgMhHAX7w
// gqDe6YHYUQF5kBgYLLDUcbhdPmGf5CTf0rwHuziosaU2LtbgciXLUUA+g9qkiSdJ
// Jo1oH7Bri2sEDTer4Lve5hQuSMw3ZjtbItCsG1hiCaRexjOQ+LxneTxeibXTR84k
// pph5+tzwOKw4ajwt47qu+3tFhQKBgQDmxQpjlXyxWq6uQlK75tBraGeiOwH+5zME
// wqZSwVD4KUH6u2bNaOsPEpkQGV8H3/3MlLS9xNbSyBHiAJgJWojqPh9m7zzTIZUn
// VnzXaxTZVc7WD//VXaQFvDpGZM6WOO/ophVN9b67WtsDVUod++nnDcwKDpP7+A2m
// lGBEDJ759wKBgBbwOnnNsTA2Se2khQrjyg5muAwPykJoesY1xg53/mrzq4P/9hcP
// z1gj7Q3aiUxPEcdij2nLAjanWrZsddJ6W3SC6kfTaO4FBHXc9SdLT6ihj121NMsc
// bCoqj+bRWMH80c7fuLDmmkE11/I7HsDTFhx2TGy1tWOmu6ysY8xVmuTBAoGBAOS8
// 6k9LHQ2ulPalRtx5LJ1KH27ujkvbXvaY+cONWnSgcIXWu09fb57BEHakr01e+HKP
// 85cmRMkpBpC9hfW/h3annPiqqd+cqfsT/yqHOWMzapFDypHy/2Gv8RNVWHAOf75a
// 4UMlEqkipXPoVk7iJDahtl1Ble1rMhyrdb/9wpaBAoGBANlm0mBib8pDEtEoUU/5
// BjF0kdaL9o5assZwoEE0UPofratCRYpaH/jJfcMMuQtHAsj8c0ugB1LWjjUPfQdA
// kWaYFfzHsTbJNeK7++zVnXWxvR4vR+/faLMGMYaTb3WNIDM9iLdHTq1mHKS2sHE4
// jy1VpDZpl0D2G9hY54alQV4v
// -----END PRIVATE KEY-----
