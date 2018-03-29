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

extern crate cc;
extern crate cmake;

use std::path::Path;
use std::{env, fs, io};

use cmake::Config;
use cc::Build;

fn prepare_grpc() {
    let mut modules = vec![
        "grpc",
        "grpc/third_party/zlib",
        "grpc/third_party/cares/cares",
    ];

    if cfg!(feature = "secure") {
        modules.push("grpc/third_party/boringssl");
    }

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
    let mut entries = fs::read_dir(p)?;
    Ok(entries.next().is_none())
}

fn build_grpc(cc: &mut Build, library: &str) {
    prepare_grpc();

    let dst = {
        let mut config = Config::new("grpc");
        if cfg!(target_os = "macos") {
            config.cxxflag("-stdlib=libc++");
        }
        config.build_target(library).uses_cxx11().build()
    };

    let mut zlib = "z";
    let build_dir = format!("{}/build", dst.display());
    let third_party = vec![
        "cares/cares/lib",
        "zlib",
        "boringssl/ssl",
        "boringssl/crypto",
    ];
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
        for path in third_party {
            println!(
                "cargo:rustc-link-search=native={}/third_party/{}/{}",
                build_dir, path, profile
            );
        }
    } else {
        println!("cargo:rustc-link-search=native={}", build_dir);
        for path in third_party {
            println!(
                "cargo:rustc-link-search=native={}/third_party/{}",
                build_dir, path,
            );
        }
    }

    println!("cargo:rustc-link-lib=static={}", zlib);
    println!("cargo:rustc-link-lib=static=cares");
    println!("cargo:rustc-link-lib=static=gpr");
    println!("cargo:rustc-link-lib=static={}", library);

    if cfg!(feature = "secure") {
        println!("cargo:rustc-link-lib=static=ssl");
        println!("cargo:rustc-link-lib=static=crypto");
    }

    cc.include("grpc/include");
}

fn main() {
    let mut cc = Build::new();

    println!("cargo:rerun-if-changed=grpc_wrap.c");
    println!("cargo:rerun-if-changed=grpc");

    let library = if cfg!(feature = "secure") {
        cc.define("GRPC_SYS_SECURE", None);
        "grpc"
    } else {
        "grpc_unsecure"
    };

    build_grpc(&mut cc, library);
    cc.cpp(true).file("grpc_wrap.cc");

    if cfg!(target_os = "windows") {
        // At lease win7
        cc.define("_WIN32_WINNT", Some("0x0700"));
    }
    if !cfg!(target_env = "msvc") {
        cc.flag("-std=c++11");
    }

    cc.warnings_into_errors(true).compile("libgrpc_wrap.a");
}
