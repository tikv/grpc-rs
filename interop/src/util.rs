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


use proto::testing::messages::{Payload, ResponseParameters};

pub fn new_payload(size: usize) -> Payload {
    let mut payload = Payload::new();
    payload.set_body(vec![0; size]);
    payload
}

pub fn new_parameters(size: i32) -> ResponseParameters {
    let mut parameter = ResponseParameters::new();
    parameter.set_size(size);
    parameter
}
