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

#![allow(dead_code, unused_imports)]

extern crate gcc;
extern crate cmake;
extern crate pkg_config;

use std::path::Path;
use std::process::Command;


const GRPC_VERSION: &'static str = "1.2.5";
const ZLIB_VERSION: &'static str = "1.2.8";
const BORINGSSL_GIT_HASH: &'static str = "78684e5b222645828ca302e56b40b9daff2b2d27";


#[cfg(not(feature = "static-link"))]
fn build_or_link_grpc(cc: &mut gcc::Config) {
    if let Ok(lib) = pkg_config::Config::new().atleast_version(GRPC_VERSION).statik(true).probe("grpc_unsecure") {
        for inc_path in &lib.include_paths {
            cc.include(inc_path);
        }
        return;
    }
}

fn wget(url: &str, out: &str) -> Result<(), String> {
    Command::new("wget").args(&["-q", "-c", "-O", out, url])
        .status()
        .map_err(|err| format!("wget execute failed: {}", err))
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(format!("wget exit with {}", status))
            }
        })
}

fn tar_xf(file: &str) -> Result<(), String> {
    Command::new("tar").args(&["zxf", file])
        .status()
        .map_err(|err| format!("tar execute failed: {}", err))
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(format!("tar exit with {}", status))
            }
        })
}

#[cfg(feature = "static-link")]
fn build_or_link_grpc(cc: &mut gcc::Config) {
    if !Path::new(&format!("grpc-{}", GRPC_VERSION)).exists() {
        wget(&format!("https://github.com/grpc/grpc/archive/v{}.tar.gz", GRPC_VERSION),
             "grpc.tar.gz")
            .and_then(|_| tar_xf("grpc.tar.gz"))
            .unwrap();
    }

    if !Path::new(&format!("zlib-{}", ZLIB_VERSION)).exists() {
        wget(&format!("https://github.com/madler/zlib/archive/v{}.tar.gz", ZLIB_VERSION),
             "zlib.tar.gz")
            .and_then(|_| tar_xf("zlib.tar.gz"))
            .unwrap();
    }

    if !Path::new(&format!("boringssl-{}", BORINGSSL_GIT_HASH)).exists() {
        wget(&format!("https://github.com/google/boringssl/archive/{}.tar.gz", BORINGSSL_GIT_HASH),
             "boringssl.tar.gz")
            .and_then(|_| tar_xf("boringssl.tar.gz"))
            .unwrap();
    }

    let dst = cmake::Config::new(format!("zlib-{}", ZLIB_VERSION))
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    let dst = cmake::Config::new(format!("grpc-{}", GRPC_VERSION))
        .define("ZLIB_ROOT_DIR", format!("../zlib-{}", ZLIB_VERSION)) // relative to grpc dir
        .define("BORINGSSL_ROOT_DIR", format!("../boringssl-{}", BORINGSSL_GIT_HASH))
        .build_target("grpc_unsecure")
        .build();

    cc.include(format!("grpc-{}/include", GRPC_VERSION));

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-search=native={}/build/third_party/zlib", dst.display());

    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=gpr");
    println!("cargo:rustc-link-lib=static=grpc_unsecure");
}


fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut cc = gcc::Config::new();

    build_or_link_grpc(&mut cc);

    cc.file("grpc_wrap.c")
        .flag("-fPIC")
        .flag("-O2")
        .compile("libgrpc_wrap.a");
}
