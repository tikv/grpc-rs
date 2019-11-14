// Copyright 2018 TiKV Project Authors. Licensed under Apache-2.0.

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
                // rust-protobuf is bad at dealing with path, keep it the same style.
                Some(format!("{}", dent.path().display()).replace('\\', "/"))
            })
            .collect();
        protobuf_build::generate_files(&["proto".to_owned()], &files, &out_dir);
    }
}
