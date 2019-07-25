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

use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let modules = &[
        ("grpc/testing", "testing"),
        ("grpc/health/v1/", "health"),
        ("grpc/example", "example"),
    ];
    for (dir, package) in modules {
        let out_dir = format!("{}/{}", out_dir, package);
        let files: Vec<_> = walkdir::WalkDir::new(format!("proto/{}", dir))
            .into_iter()
            .filter_map(|p| {
                let dent = p.expect("Error happened when search protos");
                if !dent.file_type().is_file() {
                    return None;
                }
                Some(format!("{}", dent.path().display()).replace('\\', "/"))
            })
            .collect();
        protobuf_build::generate_files(&["proto".to_owned()], &files, &out_dir);
    }
}
