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

extern crate grpcio_compiler;
extern crate protobuf;
extern crate protobuf_codegen;

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use grpcio_compiler::codegen as grpc_gen;
use pb_gen::Customize;
use protobuf::compiler_plugin::GenResult;
use protobuf::descriptor::{FileDescriptorProto, FileDescriptorSet};
use protobuf_codegen as pb_gen;

/// Descriptor file to module file.
fn desc_to_module<G, W>(descriptor: &Path, output_dir: &Path, mut gen: G, mut module: W)
where
    G: FnMut(&[FileDescriptorProto], &[String]) -> Vec<GenResult>,
    W: Write,
{
    let proto_set: FileDescriptorSet = {
        let mut f = File::open(descriptor).unwrap();
        protobuf::parse_from_reader(&mut f).unwrap()
    };
    let files: Vec<_> = proto_set
        .get_file()
        .into_iter()
        .map(|f| f.get_name().to_owned())
        .collect();
    // All files need to be generated in our case.
    let results = gen(proto_set.get_file(), &files);
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    for res in results {
        let out_file = output_dir.join(&res.name);
        let mut f = File::create(&out_file).unwrap();
        f.write_all(&res.content).unwrap();
        let (module_name, _) = res.name.split_at(res.name.len() - 3); // ".rs".len() == 3
        writeln!(module, "pub mod {};", module_name).unwrap();
    }
}

/// Compile all related proto file to `FileDescriptorSet` and use it to generate
/// rust source.
///
/// Using `FileDescriptorSet` here so we don't need to compile the binaries like
/// protoc-gen-rust and grpc_rust_plugin.
fn compile<P: AsRef<Path>>(desc_path: P, module: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let module_path = Path::new(&out_dir).join(module);
    if !module_path.exists() {
        fs::create_dir_all(&module_path).unwrap();
    }

    let mod_rs = module_path.join("mod.rs");
    let mut module = File::create(mod_rs).unwrap();
    desc_to_module(
        desc_path.as_ref(),
        &module_path,
        |a, b| {
            let c = Customize::default();
            pb_gen::gen(a, b, &c)
        },
        &mut module,
    );
    desc_to_module(desc_path.as_ref(), &module_path, grpc_gen::gen, &mut module);
}

fn main() {
    for package in &["testing", "example", "health"] {
        compile(format!("{}.desc", package), package);
    }
}
