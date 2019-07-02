// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use futures::*;
use grpcio::*;
use grpcio_proto::health::v1::health::*;
use std::collections::*;
use std::sync::*;

type StatusRegistry = HashMap<String, health_check_response::ServingStatus>;

#[derive(Clone)]
struct HealthService {
    status: Arc<RwLock<StatusRegistry>>,
}

impl Health for HealthService {
    fn check(
        &mut self,
        ctx: RpcContext<'_>,
        req: HealthCheckRequest,
        sink: UnarySink<HealthCheckResponse>,
    ) {
        let status = self.status.read().unwrap();
        let res = match status.get(req.get_service()) {
            None => sink.fail(RpcStatus::new(RpcStatusCode::GRPC_STATUS_NOT_FOUND, None)),
            Some(s) => {
                let mut resp = HealthCheckResponse::new_();
                resp.set_status_(*s);
                sink.success(resp)
            }
        };
        ctx.spawn(res.map_err(|e| println!("failed to report result: {:?}", e)));
    }
}

fn check_health(
    client: &HealthClient,
    status: &Arc<RwLock<StatusRegistry>>,
    service: &str,
    exp: health_check_response::ServingStatus,
) {
    status.write().unwrap().insert(service.to_owned(), exp);
    let mut req = HealthCheckRequest::new_();
    req.set_service(service.to_owned());
    let status = client.check(&req).unwrap().get_status();
    assert_eq!(status, exp);
}

#[test]
fn test_health_check() {
    let env = Arc::new(Environment::new(1));
    let status: Arc<RwLock<StatusRegistry>> = Arc::default();
    let service = create_health(HealthService {
        status: status.clone(),
    });
    let mut server = ServerBuilder::new(env.clone())
        .register_service(service)
        .bind("127.0.0.1", 0)
        .build()
        .unwrap();
    server.start();
    let (_, port) = server.bind_addrs()[0];

    let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
    let client = HealthClient::new(ch);

    check_health(
        &client,
        &status,
        "test",
        health_check_response::ServingStatus::Serving,
    );
    check_health(
        &client,
        &status,
        "test",
        health_check_response::ServingStatus::NotServing,
    );
    check_health(
        &client,
        &status,
        "test",
        health_check_response::ServingStatus::Unknown,
    );

    let mut req = HealthCheckRequest::new_();
    req.set_service("not-exist".to_owned());
    let err = client.check(&req).unwrap_err();
    match err {
        Error::RpcFailure(s) => assert_eq!(s.status, RpcStatusCode::GRPC_STATUS_NOT_FOUND),
        e => panic!("unexpected error: {:?}", e),
    }
}
