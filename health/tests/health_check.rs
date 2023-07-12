// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use futures_executor::block_on;
use futures_util::StreamExt as _;
use grpcio::*;
use grpcio_health::proto::*;
use grpcio_health::*;
use std::sync::*;
use std::time::Duration;

const TEST_SERVICE: &str = "grpc.test.TestService";

#[cfg(feature = "protobuf-codec")]
mod constants {
    use crate::proto::ServingStatus;
    pub const SERVING: ServingStatus = ServingStatus::Serving;
    pub const NOT_SERVING: ServingStatus = ServingStatus::NotServing;
    pub const SERVICE_UNKNOWN: ServingStatus = ServingStatus::ServiceUnknown;
    pub const UNKNOWN: ServingStatus = ServingStatus::Unknown;
}

#[cfg(feature = "protobufv3-codec")]
mod constants {
    use grpcio_health::ServingStatus;
    pub const SERVING: ServingStatus = ServingStatus::SERVING;
    pub const NOT_SERVING: ServingStatus = ServingStatus::NOT_SERVING;
    pub const SERVICE_UNKNOWN: ServingStatus = ServingStatus::SERVICE_UNKNOWN;
    pub const UNKNOWN: ServingStatus = ServingStatus::UNKNOWN;
}

#[cfg(feature = "protobuf-codec")]
fn response_status_equals(resp: HealthCheckResponse, status: ServingStatus) -> bool {
    resp.status == status
}

#[cfg(feature = "protobufv3-codec")]
fn response_status_equals(resp: HealthCheckResponse, status: ServingStatus) -> bool {
    resp.status.enum_value().unwrap() == status
}

#[track_caller]
fn assert_status(status: ServingStatus, client: &HealthClient, name: &str) {
    let req = HealthCheckRequest {
        service: name.to_string(),
        ..Default::default()
    };
    let resp: HealthCheckResponse = client.check(&req).unwrap();
    assert_eq!(response_status_equals(resp, status), true);
}

#[track_caller]
fn watch(client: &HealthClient, name: &str) -> ClientSStreamReceiver<HealthCheckResponse> {
    let req = HealthCheckRequest {
        service: name.to_string(),
        ..Default::default()
    };
    let opt = CallOption::default().timeout(Duration::from_millis(500));
    client.watch_opt(&req, opt).unwrap()
}

#[track_caller]
fn assert_code(code: RpcStatusCode, client: &HealthClient, name: &str) {
    let req = HealthCheckRequest {
        service: name.to_string(),
        ..Default::default()
    };
    match client.check(&req) {
        Err(Error::RpcFailure(s)) if s.code() == code => (),
        r => panic!("{} != {:?}", code, r),
    }
}

#[track_caller]
fn assert_next(status: ServingStatus, ss: &mut ClientSStreamReceiver<HealthCheckResponse>) {
    let resp = block_on(ss.next()).unwrap().unwrap();
    assert_eq!(response_status_equals(resp, status), true);
}

fn setup() -> (Server, HealthService, HealthClient) {
    let env = Arc::new(Environment::new(1));
    let service = HealthService::default();
    let health_service = create_health(service.clone());
    let mut server = ServerBuilder::new(env.clone())
        .register_service(health_service)
        .build()
        .unwrap();
    let port = server
        .add_listening_port("127.0.0.1:0", ServerCredentials::insecure())
        .unwrap();
    server.start();

    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{port}"));
    let client = HealthClient::new(ch);
    (server, service, client)
}

#[test]
fn test_health_check() {
    let (_server, service, client) = setup();

    // Not exist service should return NOT_FOUND.
    assert_code(RpcStatusCode::NOT_FOUND, &client, "");
    assert_code(RpcStatusCode::NOT_FOUND, &client, TEST_SERVICE);

    // Service status can be updated
    service.set_serving_status("", constants::SERVING);
    assert_status(constants::SERVING, &client, "");
    service.set_serving_status("", constants::NOT_SERVING);
    assert_status(constants::NOT_SERVING, &client, "");
    service.set_serving_status("", constants::UNKNOWN);
    assert_status(constants::UNKNOWN, &client, "");
    service.set_serving_status(TEST_SERVICE, constants::SERVING);
    assert_status(constants::SERVING, &client, TEST_SERVICE);
    assert_status(constants::UNKNOWN, &client, "");

    // After shutdown, further updates will be abandonded.
    service.shutdown();
    service.set_serving_status(TEST_SERVICE, constants::SERVING);
    assert_status(constants::NOT_SERVING, &client, TEST_SERVICE);
    assert_status(constants::NOT_SERVING, &client, "");
}

#[test]
fn test_health_watch() {
    let (_server, service, client) = setup();

    // Not existed service should return ServiceUnknown.
    let mut statuses = watch(&client, "");
    assert_next(constants::SERVICE_UNKNOWN, &mut statuses);
    service.set_serving_status("", constants::SERVING);
    assert_next(constants::SERVING, &mut statuses);
    service.set_serving_status("", constants::NOT_SERVING);
    assert_next(constants::NOT_SERVING, &mut statuses);
    service.set_serving_status("", constants::UNKNOWN);
    assert_next(constants::UNKNOWN, &mut statuses);

    // Updating other service should not notify the stream.
    service.set_serving_status(TEST_SERVICE, constants::NOT_SERVING);
    match block_on(statuses.next()).unwrap() {
        Err(Error::RpcFailure(r)) if r.code() == RpcStatusCode::DEADLINE_EXCEEDED => (),
        r => panic!("unexpected status {:?}", r),
    }

    // Watch should fetch init status immediately.
    statuses = watch(&client, TEST_SERVICE);
    assert_next(constants::NOT_SERVING, &mut statuses);

    // Only latest state can be watched.
    service.set_serving_status(TEST_SERVICE, constants::SERVING);
    service.set_serving_status(TEST_SERVICE, constants::NOT_SERVING);
    service.set_serving_status(TEST_SERVICE, constants::SERVICE_UNKNOWN);
    service.set_serving_status(TEST_SERVICE, constants::UNKNOWN);
    let mut seen = 0;
    loop {
        let resp = block_on(statuses.next()).unwrap().unwrap();
        if response_status_equals(resp, constants::UNKNOWN) {
            seen += 1;
            continue;
        }
        break;
    }
    assert!(seen <= 1);
}

#[test]
fn test_health_watch_multiple() {
    let (_server, service, client) = setup();

    // Watch should fetch service status immediately.
    let mut statuses0 = vec![watch(&client, "")];
    assert_next(constants::SERVICE_UNKNOWN, &mut statuses0[0]);

    service.set_serving_status("", constants::SERVING);
    statuses0.push(watch(&client, ""));
    for s in &mut statuses0 {
        assert_next(constants::SERVING, s);
    }

    service.set_serving_status("", constants::NOT_SERVING);
    statuses0.push(watch(&client, ""));
    for s in &mut statuses0 {
        assert_next(constants::NOT_SERVING, s);
    }

    // Multiple watchers for multiple service should work correctly.
    let mut statuses1 = vec![watch(&client, TEST_SERVICE)];
    assert_next(constants::SERVICE_UNKNOWN, &mut statuses1[0]);
    service.set_serving_status(TEST_SERVICE, constants::NOT_SERVING);
    service.set_serving_status("", constants::SERVING);
    for s in &mut statuses0 {
        assert_next(constants::SERVING, s);
    }
    for s in &mut statuses1 {
        assert_next(constants::NOT_SERVING, s);
    }
}
