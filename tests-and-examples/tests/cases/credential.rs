use futures::Future;
use grpcio::{
    CertificateRequestType, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder, RpcContext,
    ServerBuilder, ServerCredentialsBuilder, UnarySink,
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

static mut RELOADED: bool = false;

fn reload_new() -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
    if unsafe { RELOADED } {
        return Ok(None);
    }
    let new_cred = ServerCredentialsBuilder::new()
        .add_cert(PD_CRT.into(), PD_KEY.into())
        .root_cert(CA_CRT, CertificateRequestType::DontRequestClientCertificate);
    unsafe {
        RELOADED = true;
    }
    Ok(Some(new_cred))
}

fn fail_open() -> Result<Option<ServerCredentialsBuilder>, Box<dyn std::error::Error>> {
    let mut f = fs::File::open("Forsaken/Land")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(None)
}

#[test]
fn test_reload_new() {
    let env = Arc::new(EnvBuilder::new().build());
    let cred = ServerCredentialsBuilder::new()
        .add_cert(SERVER_CRT.into(), SERVER_KEY.into())
        .root_cert(CA_CRT, CertificateRequestType::DontRequestClientCertificate)
        .add_reload_fetcher(reload_new)
        .build();
    let service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_secure("localhost", 0, cred)
        .build()
        .unwrap();
    server.start();

    let port = server.bind_addrs()[0].1;
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(CA_CRT.into())
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
    let cred = ServerCredentialsBuilder::new()
        .add_cert(SERVER_CRT.into(), SERVER_KEY.into())
        .root_cert(CA_CRT, CertificateRequestType::DontRequestClientCertificate)
        .add_reload_fetcher(fail_open)
        .build();
    let service = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind_secure("localhost", 0, cred)
        .build()
        .unwrap();
    server.start();

    let port = server.bind_addrs()[0].1;
    let cred = ChannelCredentialsBuilder::new()
        .root_cert(CA_CRT.into())
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

#[allow(dead_code)]
static CA_CRT: &str = "-----BEGIN CERTIFICATE-----
MIIDRTCCAi2gAwIBAgIUGrzcBxX5bVapB86XYGPetE10wiEwDQYJKoZIhvcNAQEL
BQAwFDESMBAGA1UEAwwJbG9jYWxob3N0MB4XDTIwMDMwMTE1MTUxOVoXDTMwMDIy
NzE1MTUxOVowFDESMBAGA1UEAwwJbG9jYWxob3N0MIIBIjANBgkqhkiG9w0BAQEF
AAOCAQ8AMIIBCgKCAQEA5+GQsuC/qWJtasd+T+lFomOjo0M2kgPrYH7uowl6zvN2
jEiA92eChgNMa8VRw7oXNi7sfivZ9zUC11DPtfkE6JHZsxIK3nJbS1RscZ05yna9
eT/sxDpaGF39KtNTzSOQhFdomRu5Hr4krud3Z2Qteh3WUA+vfsyI88U2lHqwT3E1
nAFC0bEcSaYkPGnWd7ui8ZRKuuiSixLz/CV5ucvzFEhGKHFHpU5Hh6ManyIE7hvR
IC3/SsxYzk9IOdV+eFAxPrX6l+IkyIF16sLNMJLX3+ponlHElhwWBke6FtD8Kifn
goKAq4GOgGTpiBjB6lo3WjX6AOVZpULFUPUuH5Br5QIDAQABo4GOMIGLMB0GA1Ud
DgQWBBTanMVSdZoVp7zx+5qCwAe/jsH7GDBPBgNVHSMESDBGgBTanMVSdZoVp7zx
+5qCwAe/jsH7GKEYpBYwFDESMBAGA1UEAwwJbG9jYWxob3N0ghQavNwHFfltVqkH
zpdgY960TXTCITAMBgNVHRMEBTADAQH/MAsGA1UdDwQEAwIBBjANBgkqhkiG9w0B
AQsFAAOCAQEAEHiYOCjk84D/UYyJS+Zu9tsoF59mMyDJLxulPfEhqnUchb84Ofo8
BPU/8EkSFimBubrM3bK0OY5JYfUexHhWEdCQamRDTQgsqkjtlB8V9Ld7BS8kE87+
8lkGDmS+wpTXQGWR9SCZtqHiy1maFtE7JfCUOK01tfRRy2Mv8zOjT92nEbF8lB+L
V3JhTnjjet3w+UCDso7ASP2IzW6L3J4CIa6mNiei1UOnRBFb+0vFHXB6suWU3m3Q
50LvLNpdC0GHPNHuhI9iVIY7G1BEtj/nNdSgBiy7ndsu7R/0srPHn24AXE3jv7XW
iTgDg+mI/zAegK2gW7yY7HiAix+wpw59qA==
-----END CERTIFICATE-----";

#[allow(dead_code)]
static SERVER_CRT: &str = "-----BEGIN CERTIFICATE-----
MIIDajCCAlKgAwIBAgIRAPohMR1je4Mnr36vET6d1EswDQYJKoZIhvcNAQELBQAw
FDESMBAGA1UEAwwJbG9jYWxob3N0MB4XDTIwMDMwMTE1MTUzMloXDTIzMDIxNDE1
MTUzMlowFDESMBAGA1UEAwwJbG9jYWxob3N0MIIBIjANBgkqhkiG9w0BAQEFAAOC
AQ8AMIIBCgKCAQEAyVM0TLepoM8MAl23sfO49m1vdtmjZt+zSYxcAnw3DifqTTiJ
q2uGldaRlX5qBrhw4ghTQK/wj02dBBBZb3qvbOAv5jXVfdNHtoGXrMyzQWp6Ncub
vIZ3qIrlTRYBl6HQ+kMiuRL8b2Voq8ZLIRm+KYpAlexyDw5cYvzPyVdbYfuCfjPp
IymaUAt1asQ1bUFFjBvi3eDq0BXzEuZoVUgZktCyOIrBcsPOOOM0Gl7Ce8HW8QFI
0ARC8CGz5GMxsDbQVBm35WaBETrut8AEItlSQPsJ8RJADxDY9EMV813SQrMBUPxU
ZeR+UE/drTX8+2v2vZpu5aTdbUQjeMfmt/vWmQIDAQABo4G2MIGzMAkGA1UdEwQC
MAAwHQYDVR0OBBYEFDJik6Kuaho0kf7p1h/ICL7tZm/zME8GA1UdIwRIMEaAFNqc
xVJ1mhWnvPH7moLAB7+OwfsYoRikFjAUMRIwEAYDVQQDDAlsb2NhbGhvc3SCFBq8
3AcV+W1WqQfOl2Bj3rRNdMIhMBMGA1UdJQQMMAoGCCsGAQUFBwMBMAsGA1UdDwQE
AwIFoDAUBgNVHREEDTALgglsb2NhbGhvc3QwDQYJKoZIhvcNAQELBQADggEBAMTO
zeRXVwfx2pEV+dCvmuIkzVyxP8+gzv1mThP/oQISaRvy0c4ZcncVsh3Rs+oA5cn0
yxzrTcLzbvEIgLBBrj38vZ0kvEh/BJaDso8/th7bGqaI1wicEofZY2+E2usQk3+p
Wq8GYyv/fCuU1XhJx9o071mXX8XBRCnrm4tRGIlMCqNTFfoXTaQxEe3D49ZI/p34
+SzQpNzAbOngobI8Gx0jLD1BBnuePB1AJG0qEq+QbPwHHnHZO7lGfmsEyN87lkWA
2/rqII90nGqELF5dVZF3So8X8TDbRSrAX1p6vbxSQMx1jxheispZK1tZubUY1q6W
j2vq/Dn85xWFj37Sqo0=
-----END CERTIFICATE-----";

#[allow(dead_code)]
static PD_CRT: &str = "-----BEGIN CERTIFICATE-----
MIIDaTCCAlGgAwIBAgIQdlwwuddxLTcDKK+f/X4djTANBgkqhkiG9w0BAQsFADAU
MRIwEAYDVQQDDAlsb2NhbGhvc3QwHhcNMjAwMzAxMTUxNTI2WhcNMjMwMjE0MTUx
NTI2WjAUMRIwEAYDVQQDDAlsb2NhbGhvc3QwggEiMA0GCSqGSIb3DQEBAQUAA4IB
DwAwggEKAoIBAQCvFQJBfbBPqyxvu5U6YIntrf2TZ00DI6ci/XohoRqBbt80xsmX
xfv/RVqbZY7Od/KnmRq6MD7Cvlu1msHNLis8V22oyOcWVoAoP0iURjDhOXXLoMNk
3QKDsklj0Atnvth6n5WqHV0b+1zoWYdIjAYJ/+dQ1YKuHA0rC7akhOVHWGKXc17m
g9l6Xr8NHHOxNhAajCPUJD3jPemkUOgCdBPkFW6ATjxqzJOVtzdM9DchNiTBKDsI
KEpU196/eFqYhAQtc0HhiBdGkoKG27gGilr90SWyIyaKry5G4O1l8TGpgUMXdnYL
/ce8vDQHxvSISfeldYFYdUxKvSmZAqGmjfCBAgMBAAGjgbYwgbMwCQYDVR0TBAIw
ADAdBgNVHQ4EFgQU2Ee1pOsCMmP3CyXxTa8CeG+4JgAwTwYDVR0jBEgwRoAU2pzF
UnWaFae88fuagsAHv47B+xihGKQWMBQxEjAQBgNVBAMMCWxvY2FsaG9zdIIUGrzc
BxX5bVapB86XYGPetE10wiEwEwYDVR0lBAwwCgYIKwYBBQUHAwEwCwYDVR0PBAQD
AgWgMBQGA1UdEQQNMAuCCWxvY2FsaG9zdDANBgkqhkiG9w0BAQsFAAOCAQEAetSe
DvguVSpnX5vNc9JorOLOBFriiKNgkxvxdr//LHXfeROlyFWf0tzlMybFP/1htGwV
FYzILVNoigw5nqUNxLGdboGm60QahPp84BR0afcCE+gxVCs+QsmN3HMa/hcJuXho
0KSGH0pRMTH3OcYlIWaGkXyPL/w0BVM8kmEfIr7jZStEjL+lpvSRntqN8ZK4LV7A
qrOptgRc5VN1j2wmlKQG0RgwPY1Q0sO3vhzsJn4C+xTFhGN4pflJDlk9e/KCl2H1
bejewKalclfPQ/PRCV7/sVpA51L+J4ewpbQg/WdAvL835VL4N7srDuPNZ2Dd8W9W
w1XOW5h/vq0G+N0n8Q==
-----END CERTIFICATE-----";

#[allow(dead_code)]
static SERVER_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDJUzRMt6mgzwwC
Xbex87j2bW922aNm37NJjFwCfDcOJ+pNOImra4aV1pGVfmoGuHDiCFNAr/CPTZ0E
EFlveq9s4C/mNdV900e2gZeszLNBano1y5u8hneoiuVNFgGXodD6QyK5EvxvZWir
xkshGb4pikCV7HIPDlxi/M/JV1th+4J+M+kjKZpQC3VqxDVtQUWMG+Ld4OrQFfMS
5mhVSBmS0LI4isFyw8444zQaXsJ7wdbxAUjQBELwIbPkYzGwNtBUGbflZoEROu63
wAQi2VJA+wnxEkAPENj0QxXzXdJCswFQ/FRl5H5QT92tNfz7a/a9mm7lpN1tRCN4
x+a3+9aZAgMBAAECggEAXKiZ0pFZVfTVLyWZDSMCRxDZE1dZuFhO+RPLqS0oXbCL
St0xp57Gg6IMwUQ8YjlzYuS4BHa1fg8XrC4mK4PIx5BrG0Y9qzqZr+r3NwXDnuD1
vFqoo90zp3O903vEYdiNCRHrIkkVu+NtiFmsbGCfwBisXxRC7qEKbliy/ZTiLzaz
z/F/q5voizfE2EoHQCBVCaptZD8yP58nlQZqlsB7TSp2R9gb3uX2DMYEk5UZH+SB
pL/0fXm0gXd8CX8Fq1lEKKhvG8F39UOg/0LHFZK5xgXQQKvBdPRLEmJrFu8neRO1
cpKcwDR47NY1Zh2AjEhYx75atRyEc7AuZJ7BgtOw3QKBgQD5ku4EuklldcE6+saS
SZVeNByuvMx9r0mObnW77aTkSJu4+3WdiBxuPmIyxAIAy+jneauDxgWoISSKux5U
8f14ih4VjmkdJvu6OcxiY3ZyuSXEtbI1vNqFTVJy7kRjcaKOIbNZj1DUAAB8O/LY
Lz4wjPUb59n/VL1tes+jFqoLhwKBgQDOgj20kPkW9arFFY1SQTxsM+NoUeSudK7D
NJZKxmE4NmQqz1KSvh9vgQAU1fzG8MlHylxE09PUTK0dESxbt5rBQqQFxkLGlvce
8aJn/TVwKdc0BAepbM4G8FmrrIQjpLVgq/QN0TGrtiN7lI+8OnZux4We3ySoVgTs
dMxOyTDU3wKBgHkuvOASzDy1tsO6rNyvWhXJ2/uEtQ56JI9CV2yO8bffAa4Ke4uo
YZWuOORJhGd6m3Z5/6wWZPzESwgJ36qDbf4vonf5FFURbr13/yPvA76k5A/l4chy
EPEVIGmqBA79SF8XQgzEqAyoi5PHUB1kox/T8dhA9fsi3G/iaBvaj2rVAoGBAKt7
YWwQupdDNyRjai/0tg0P5XXaEzB++iRlBaxgIZkBKZqXAHWUZRw9neivbTFx/2Ka
+pW45P01EsfBW/lpdJupD/UvloG8GMdcVcVOcVKLnAW4lvgsvsTqTzwMjMngWzFE
eOwEW9+/5qUXT2g5j+Eb06yClbknfP0xW/rMfL03AoGBAJ+OsKEb/OgUKc6QVnl+
QG0KX6GqLuZxr8v3K9xtZYP8p5iHbzS+qWzpb+6t9knhMITugvATgnRJAwdDJ/gK
f3zCwiHVvymd6z57BVdkrr3qqs/Fsg1pSW8ksIaebQO/Nrr8PM6jeIV2CE/Z/RT2
1WJTpU+jJom+PMqwMjuyr82w
-----END PRIVATE KEY-----";

#[allow(dead_code)]
static PD_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCvFQJBfbBPqyxv
u5U6YIntrf2TZ00DI6ci/XohoRqBbt80xsmXxfv/RVqbZY7Od/KnmRq6MD7Cvlu1
msHNLis8V22oyOcWVoAoP0iURjDhOXXLoMNk3QKDsklj0Atnvth6n5WqHV0b+1zo
WYdIjAYJ/+dQ1YKuHA0rC7akhOVHWGKXc17mg9l6Xr8NHHOxNhAajCPUJD3jPemk
UOgCdBPkFW6ATjxqzJOVtzdM9DchNiTBKDsIKEpU196/eFqYhAQtc0HhiBdGkoKG
27gGilr90SWyIyaKry5G4O1l8TGpgUMXdnYL/ce8vDQHxvSISfeldYFYdUxKvSmZ
AqGmjfCBAgMBAAECggEAAosTlU/2F7Is2xKmgEqWtlUosdN2Uu1ZqDB2bbahY+/w
XhvlIcdGedbQEiHPiUBEzhpzQPEieEG7o0+0MP7SsfWzKhiBAtoO0obYzw6V9y+W
fna/X9+2AaO9XS3QfJlRG7blvarfh2CHYPir9mnSTKLjAVGYrF7L8iwhYxJrIjYo
TrCyTFAOIYOMUe+7oLiz3Br/HDh4qcbTyTKfuk2h4r75CIwOF5kJcqvds7yVBDOs
5XQ0LVdKc9DB4cHE7PlJmNHPOZL8clKKqL/gV+K9AUKMnvsjKV8aNfzLND+0iWwv
9/wpnERWb+IxdEU7FZaaCKuH7u848Uzz+PDPZYsugQKBgQDhpC8DMz5N9MAY5d+s
cBLAICqaNyRuLHe5yPtg0JWn7UfNgNhlDyiJ3JO6CLnhRGWpKzMdEm7ghBpdxoFU
doyDIdfC967USyb6YH/xe00GGUrPdrqekvNBySikZrsHAhObFITJGaB516OMz1qw
4vHbPsg43Bn2GSA/W2Ud1uxOfQKBgQDGo2aDOisSoSJ1WccdLsNszYMXzpc80uCo
UhS9zXgvoIWe+cobN020060Rz3IQz7iGCdRo1qyPs+kl8YeDPmnnvT76xtV9yjaP
3q9hf6eUWvbFr6QwtG+qlabBL+iCjWshguikOehouebQgetcs/MachaIXFCdC3TG
PFRIBLg1VQKBgGe3PBTQfRcAJTxMjV+JtFSzdGGmbwxlTDMNet6pgeDKZHLFgrIR
IaUcypp8YFR3R/NWdAws6OIyVpw8Hj5gr32nFLH9f1L9qFD2AllRP7pJ7eUmwekZ
ys62VAQdZGJ4ReLGr7JwIR9gS0DyHtGnv1vzo7zfDt/b9MKqk/P/UmdRAoGAdXqe
Xm5AmzwBjTYTtQa7PdYt+h6NA8QUanhhHAAxXLmrTWKeTYpfwtqOOUJfS+IOV/fo
UDjmxfa5pV87rjtzl0MtxFNkYIj/zAb/4sIE8pD58eD3UnbktBbeENOsx8mr2p35
yWcWNRn9kMPF14X1+WYQPYWbh8YGwldb4VwpNiECgYA+DkzmuRWRljbEK9yEjCtT
SQsYoVpewJ0xzU9HJ268yLlx+whqlVQqbbUVa2i3dN0ranibXNXwhPbl/HjOdZ2/
pubtGCKpSrlAo2AOsivva8INHZSH3731Kio1LMfsKnwnGkLgxXu5rj6mXzMt++dF
AS9M5mT/zOidMp46GWdCNw==
-----END PRIVATE KEY-----";
