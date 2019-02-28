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
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use grpcio_compiler::codegen as grpc_gen;
use pb_gen::Customize;
use prost::Message;
use prost_types::FileDescriptorSet as ProstFileDescriptorSet;
use protobuf::compiler_plugin::GenResult;
use protobuf::descriptor::{FileDescriptorProto, FileDescriptorSet};
use protobuf_codegen as pb_gen;

fn write_files<W: Write>(
    results: impl Iterator<Item = (String, Vec<u8>)>,
    output_dir: &Path,
    mut out: W,
) {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    for (name, content) in results {
        let out_file = output_dir.join(&name);
        let mut f = File::create(&out_file).unwrap();
        f.write_all(&content).unwrap();
        let (module_name, _) = name.split_at(name.len() - 3); // ".rs".len() == 3
        writeln!(out, "pub mod {};", module_name).unwrap();
    }
}

/// Descriptor file to module file using rust-protobuf.
fn desc_to_module_rust_protobuf<G, W>(
    descriptor: &Path,
    output_dir: &Path,
    mut generate_files: G,
    out: W,
) where
    G: FnMut(&[FileDescriptorProto], &[String]) -> Vec<GenResult>,
    W: Write,
{
    let mut f = File::open(descriptor).unwrap();
    let proto_set: FileDescriptorSet = protobuf::parse_from_reader(&mut f).unwrap();

    let files: Vec<_> = proto_set
        .get_file()
        .into_iter()
        .map(|f| f.get_name().to_owned())
        .collect();

    // All files need to be generated in our case.
    let results = generate_files(proto_set.get_file(), &files);
    write_files(
        results.into_iter().map(|res| (res.name, res.content)),
        output_dir,
        out,
    );
}

/// Descriptor file to module file using Prost.
fn desc_to_module_prost(descriptor: &Path) {
    let buf = fs::read(descriptor).unwrap();
    let proto_set: ProstFileDescriptorSet = Message::decode(buf).unwrap();
    let files: Vec<_> = proto_set
        .file
        .into_iter()
        .map(|f| {
            let mut path = PathBuf::new();
            path.push("proto");
            path.push(f.name.unwrap());
            path.to_str().unwrap().to_owned()
        })
        .collect();

    let packages = protobuf_build::generate_prost_files(&files, &env::var("OUT_DIR").unwrap());
    assert!(!packages.is_empty());
    protobuf_build::generate_wrappers(
        &packages
            .iter()
            .map(|m| format!("{}/{}.rs", env::var("OUT_DIR").unwrap(), m))
            .collect::<Vec<_>>(),
        &env::var("OUT_DIR").unwrap(),
    );
}

/// Compile all related proto file to `FileDescriptorSet` and use it to generate
/// rust source using rust-protobuf.
///
/// Using `FileDescriptorSet` here so we don't need to compile the binaries like
/// protoc-gen-rust and grpc_rust_plugin.
fn compile_protobuf<P: AsRef<Path>>(desc_path: P, package: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let module_path = Path::new(&out_dir).join(package);
    if !module_path.exists() {
        fs::create_dir_all(&module_path).unwrap();
    }

    let mod_rs = module_path.join("mod.rs");
    let mut module_file = File::create(mod_rs).unwrap();

    desc_to_module_rust_protobuf(
        desc_path.as_ref(),
        &module_path,
        |a, b| {
            let c = Customize::default();
            pb_gen::gen(a, b, &c)
        },
        &mut module_file,
    );
    desc_to_module_rust_protobuf(
        desc_path.as_ref(),
        &module_path,
        grpc_gen::gen,
        &mut module_file,
    );
}

// Generate Prost code.
fn compile_prost<P: AsRef<Path>>(desc_path: P) {
    desc_to_module_prost(desc_path.as_ref());
}

fn main() {
    for package in &["testing", "example", "health"] {
        compile_protobuf(format!("{}.desc", package), package);
        compile_prost(format!("{}.desc", package));
    }
}
