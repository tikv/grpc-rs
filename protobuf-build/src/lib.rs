// Copyright 2019 PingCAP, Inc.
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

//! Utility functions for generating Rust code from protobuf specifications.
//!
//! These functions panic liberally, they are designed to be used from build
//! scripts, not in production.

use regex::Regex;
use std::fs::read_dir;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;

mod wrapper;

/// Check that the user's installed version of the protobuf compiler is 3.1.x.
pub fn check_protoc_version() {
    let ver_re = Regex::new(r"([0-9]+)\.([0-9]+)\.[0-9]").unwrap();
    let ver = Command::new("protoc")
        .arg("--version")
        .output()
        .expect("Program `protoc` not installed (is it in PATH?).");
    let caps = ver_re.captures(from_utf8(&ver.stdout).unwrap()).unwrap();
    let major = caps.get(1).unwrap().as_str().parse::<i16>().unwrap();
    let minor = caps.get(2).unwrap().as_str().parse::<i16>().unwrap();
    if major == 3 && minor < 1 || major < 3 {
        panic!(
            "Invalid version of protoc (required 3.1.x, get {}.{}.x).",
            major, minor,
        );
    }
}

/// Use protobuf-rs to generate Rust files from proto files (`file_names`).
///
/// Uses `["proto", "include"]` as the include lists.
pub fn generate_protobuf_files<T: AsRef<str>>(file_names: &[T], out_dir: &str) {
    let file_names = &file_names.iter().map(|f| f.as_ref()).collect::<Vec<_>>();
    protoc_rust::run(protoc_rust::Args {
        out_dir,
        input: file_names,
        includes: &["proto", "include"],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    })
    .unwrap();

    protoc_grpcio::compile_grpc_protos(file_names, &["proto", "include"], out_dir).unwrap();
}

/// Use Prost to generate Rust files from proto files (`file_names`).
///
/// Uses `["proto", "include"]` as the include lists.
///
/// Returns a list of the package names of the protocols that were compiled.
pub fn generate_prost_files<T: AsRef<str>>(file_names: &[T], out_dir: &str) -> Vec<String> {
    let packages = grpcio_compiler::prost_codegen::compile_protos(
        &file_names.iter().map(|f| f.as_ref()).collect::<Vec<_>>(),
        &["proto", "include"],
        out_dir,
    )
    .unwrap();
    for package in &packages {
        let mut file_name = PathBuf::new();
        file_name.push(out_dir);
        file_name.push(&format!("{}.rs", package));
        rustfmt(&file_name);
    }

    packages
}

fn rustfmt(file_path: &Path) {
    let output = Command::new("rustfmt")
        .arg(file_path.to_str().unwrap())
        .output();
    if !output.map(|o| o.status.success()).unwrap_or(false) {
        eprintln!("Rustfmt failed");
    }
}

pub fn generate_wrappers<T: AsRef<str>>(file_names: &[T], out_dir: &str) {
    for file in file_names {
        let gen = wrapper::WrapperGen::new(file.as_ref());
        gen.write(out_dir);
    }
}

/// Returns a list of module names corresponding to the Rust files in a directory.
///
/// Note that this does not read the files so will miss inline modules, it only
/// looks at filenames,
pub fn module_names_for_dir(directory_name: &str) -> Vec<String> {
    let mut mod_names: Vec<_> = read_dir(directory_name)
        .expect("Couldn't read directory")
        .filter_map(|e| {
            let file_name = e.expect("Couldn't list file").file_name();
            let file_name = file_name.to_string_lossy();
            if !file_name.ends_with(".rs") {
                return None;
            }
            file_name
                .split(".rs")
                .next()
                .filter(|n| !n.starts_with("wrapper_"))
                .map(ToOwned::to_owned)
        })
        .collect();

    mod_names.sort();
    mod_names
}

/// Convert protobuf files to use the old way of reading protobuf enums.
// FIXME: Remove this once stepancheg/rust-protobuf#233 is resolved.
pub fn replace_read_unknown_fields<T: AsRef<str>>(file_names: &[T]) {
    let regex =
        Regex::new(r"::protobuf::rt::read_proto3_enum_with_unknown_fields_into\(([^,]+), ([^,]+), &mut ([^,]+), [^\)]+\)\?").unwrap();
    for file_name in file_names {
        let mut text = String::new();
        {
            let mut f = File::open(file_name.as_ref()).unwrap();
            f.read_to_string(&mut text)
                .expect("Couldn't read source file");
        }

        // FIXME Rustfmt bug in string literals
        #[rustfmt::skip]
        let text = {
            regex.replace_all(
                &text,
                "if $1 == ::protobuf::wire_format::WireTypeVarint {\
                    $3 = $2.read_enum()?;\
                 } else {\
                    return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));\
                 }",
            )
        };
        let mut out = File::create(file_name.as_ref()).unwrap();
        out.write_all(text.as_bytes())
            .expect("Could not write source file");
    }
}
