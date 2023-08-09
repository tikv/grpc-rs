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
use crate::proto::ServingStatus;

#[cfg(feature = "protobufv3-codec")]
use grpcio_health::ServingStatus;

#[cfg(any(feature = "protobuf-codec", feature = "prost-codec"))]
fn response_status_equals(resp: HealthCheckResponse, status: ServingStatus) -> bool {
    resp.status == status.into()
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
    assert!(response_status_equals(resp, status));
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
    assert!(response_status_equals(resp, status));
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
    service.set_serving_status("", ServingStatus::Serving);
    assert_status(ServingStatus::Serving, &client, "");
    service.set_serving_status("", ServingStatus::NotServing);
    assert_status(ServingStatus::NotServing, &client, "");
    service.set_serving_status("", ServingStatus::Unknown);
    assert_status(ServingStatus::Unknown, &client, "");
    service.set_serving_status(TEST_SERVICE, ServingStatus::Serving);
    assert_status(ServingStatus::Serving, &client, TEST_SERVICE);
    assert_status(ServingStatus::Unknown, &client, "");

    // After shutdown, further updates will be abandonded.
    service.shutdown();
    service.set_serving_status(TEST_SERVICE, ServingStatus::Serving);
    assert_status(ServingStatus::NotServing, &client, TEST_SERVICE);
    assert_status(ServingStatus::NotServing, &client, "");
}

#[test]
fn test_health_watch() {
    let (_server, service, client) = setup();

    // Not existed service should return ServiceUnknown.
    let mut statuses = watch(&client, "");
    assert_next(ServingStatus::ServiceUnknown, &mut statuses);
    service.set_serving_status("", ServingStatus::Serving);
    assert_next(ServingStatus::Serving, &mut statuses);
    service.set_serving_status("", ServingStatus::NotServing);
    assert_next(ServingStatus::NotServing, &mut statuses);
    service.set_serving_status("", ServingStatus::Unknown);
    assert_next(ServingStatus::Unknown, &mut statuses);

    // Updating other service should not notify the stream.
    service.set_serving_status(TEST_SERVICE, ServingStatus::NotServing);
    match block_on(statuses.next()).unwrap() {
        Err(Error::RpcFailure(r)) if r.code() == RpcStatusCode::DEADLINE_EXCEEDED => (),
        r => panic!("unexpected status {:?}", r),
    }

    // Watch should fetch init status immediately.
    statuses = watch(&client, TEST_SERVICE);
    assert_next(ServingStatus::NotServing, &mut statuses);

    // Only latest state can be watched.
    service.set_serving_status(TEST_SERVICE, ServingStatus::Serving);
    service.set_serving_status(TEST_SERVICE, ServingStatus::NotServing);
    service.set_serving_status(TEST_SERVICE, ServingStatus::ServiceUnknown);
    service.set_serving_status(TEST_SERVICE, ServingStatus::Unknown);
    let mut seen = 0;
    loop {
        match block_on(statuses.next()).unwrap() {
            Err(e) => panic!("unexpected error {:?}", e),
            Ok(r) => {
                if !response_status_equals(r, ServingStatus::Unknown) {
                    seen += 1;
                    continue;
                }
                break;
            }
        }
    }
    assert!(seen <= 1);
}

#[test]
fn test_health_watch_multiple() {
    let (_server, service, client) = setup();

    // Watch should fetch service status immediately.
    let mut statuses0 = vec![watch(&client, "")];
    assert_next(ServingStatus::ServiceUnknown, &mut statuses0[0]);

    service.set_serving_status("", ServingStatus::Serving);
    statuses0.push(watch(&client, ""));
    for s in &mut statuses0 {
        assert_next(ServingStatus::Serving, s);
    }

    service.set_serving_status("", ServingStatus::NotServing);
    statuses0.push(watch(&client, ""));
    for s in &mut statuses0 {
        assert_next(ServingStatus::NotServing, s);
    }

    // Multiple watchers for multiple service should work correctly.
    let mut statuses1 = vec![watch(&client, TEST_SERVICE)];
    assert_next(ServingStatus::ServiceUnknown, &mut statuses1[0]);
    service.set_serving_status(TEST_SERVICE, ServingStatus::NotServing);
    service.set_serving_status("", ServingStatus::Serving);
    for s in &mut statuses0 {
        assert_next(ServingStatus::Serving, s);
    }
    for s in &mut statuses1 {
        assert_next(ServingStatus::NotServing, s);
    }
}
