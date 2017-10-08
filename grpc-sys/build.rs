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


extern crate cmake;
extern crate cc;
extern crate pkg_config;

use std::path::Path;
use std::{env, fs, io};
use std::env::VarError;

use cmake::Config;
use cc::Build;
use pkg_config::Config as PkgConfig;

const GRPC_VERSION: &'static str = "1.6.1";

fn link_grpc(cc: &mut Build) {
    if let Ok(lib) = PkgConfig::new().atleast_version(GRPC_VERSION).probe("grpc") {
        for inc_path in lib.include_paths {
            cc.include(inc_path);
        }
    } else {
        panic!("can't find a grpc library via pkg-config");
    }
}

fn prepare_grpc() {
    let modules = vec![
        "grpc",
        "grpc/third_party/zlib",
        "grpc/third_party/boringssl",
        "grpc/third_party/cares/cares",
    ];

    for module in modules {
        if is_directory_empty(module).unwrap_or(true) {
            panic!(
                "Can't find module {}. You need to run `git submodule \
                 update --init --recursive` first to build the project.",
                module
            );
        }
    }
}

fn is_directory_empty<P: AsRef<Path>>(p: P) -> Result<bool, io::Error> {
    let mut entries = try!(fs::read_dir(p));
    Ok(entries.next().is_none())
}

fn build_grpc(cc: &mut Build) {
    prepare_grpc();

    let dst = Config::new("grpc").build_target("grpc").build();

    let mut zlib = "z";
    let build_dir = format!("{}/build", dst.display());
    if cfg!(target_os = "windows") {
        let profile = match &*env::var("PROFILE").unwrap_or("debug".to_owned()) {
            "bench" | "release" => {
                zlib = "zlibstatic";
                "Release"
            }
            _ => {
                zlib = "zlibstaticd";
                "Debug"
            }
        };
        println!("cargo:rustc-link-search=native={}/{}", build_dir, profile);
        println!(
            "cargo:rustc-link-search=native={}/third_party/cares/{}",
            build_dir,
            profile
        );
        println!(
            "cargo:rustc-link-search=native={}/third_party/zlib/{}",
            build_dir,
            profile
        );
        println!(
            "cargo:rustc-link-search=native={}/third_party/boringssl/ssl/{}",
            build_dir,
            profile
        );
        println!(
            "cargo:rustc-link-search=native={}/third_party/boringssl/crypto/{}",
            build_dir,
            profile
        );
    } else {
        println!("cargo:rustc-link-search=native={}", build_dir);
        println!(
            "cargo:rustc-link-search=native={}/third_party/cares",
            build_dir
        );
        println!(
            "cargo:rustc-link-search=native={}/third_party/zlib",
            build_dir
        );
        println!(
            "cargo:rustc-link-search=native={}/third_party/boringssl/ssl",
            build_dir
        );
        println!(
            "cargo:rustc-link-search=native={}/third_party/boringssl/crypto",
            build_dir
        );
    }

    println!("cargo:rustc-link-lib=static={}", zlib);
    println!("cargo:rustc-link-lib=static=cares");
    println!("cargo:rustc-link-lib=static=gpr");
    println!("cargo:rustc-link-lib=static=grpc");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");

    cc.include("grpc/include");
}

fn get_env(name: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={}", name);
    match env::var(name) {
        Ok(s) => Some(s),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(s)) => {
            panic!("unrecognize env var of {}: {:?}", name, s.to_string_lossy());
        }
    }
}

fn main() {
    let mut cc = Build::new();

    if get_env("GRPCIO_SYS_USE_PKG_CONFIG").map_or(false, |s| s == "1") {
        link_grpc(&mut cc);
    } else {
        build_grpc(&mut cc);
    }

    cc.file("grpc_wrap.c");

    if cfg!(target_os = "windows") {
        // At lease win7
        cc.define("_WIN32_WINNT", Some("0x0700"));
    }

    cc.warnings_into_errors(true).compile("libgrpc_wrap.a");
}
