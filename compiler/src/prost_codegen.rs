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

use super::util::{self, fq_grpc, to_snake_case, MethodType};
use prost_build::{Config, Method, Service, ServiceGenerator};
use std::fmt::Write;
use std::io;
use std::path::Path;

// TODO might move this to lib.rs
pub fn compile_protos<P>(protos: &[P], includes: &[P]) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut prost_config = Config::new();
    prost_config.service_generator(Box::new(Generator {}));
    prost_config.compile_protos(protos, includes)
}

struct Generator {
    // TODO
}

impl ServiceGenerator for Generator {
    fn generate(&mut self, service: Service, buf: &mut String) {
        generate_methods(&service, buf);
        generate_client(&service, buf);
        generate_server(&service, buf);
    }
}

fn generate_methods(service: &Service, buf: &mut String) {
    let service_path = if service.package.is_empty() {
        format!("/{}", service.proto_name)
    } else {
        format!("/{}.{}", service.package, service.proto_name)
    };

    for method in &service.methods {
        generate_method(&service.name, &service_path, method, buf);
    }
}

fn generate_method(service_name: &str, service_path: &str, method: &Method, buf: &mut String) {
    let name = format!(
        "METHOD_{}_{}",
        to_snake_case(service_name).to_uppercase(),
        method.name.to_uppercase()
    );
    let ty = format!(
        "{}<{}, {}>",
        fq_grpc("Method"),
        method.input_type,
        method.output_type
    );

    buf.push_str(" const ");
    buf.push_str(&name);
    buf.push_str(": ");
    buf.push_str(&ty);
    buf.push_str(" = ");
    generate_method_body(service_path, method, buf);
}

fn generate_method_body(service_path: &str, method: &Method, buf: &mut String) {
    let ty = match (method.client_streaming, method.server_streaming) {
        (false, false) => fq_grpc("MethodType::Unary"),
        (true, false) => fq_grpc("MethodType::ClientStreaming"),
        (false, true) => fq_grpc("MethodType::ServerStreaming"),
        (true, true) => fq_grpc("MethodType::Duplex"),
    };
    let pr_mar = format!(
        "{} {{ ser: {}, de: {} }}",
        fq_grpc("Marshaller"),
        fq_grpc("pr_ser"),
        fq_grpc("pr_de")
    );

    buf.push('{');
    generate_field_init("ty", &ty, buf);
    generate_field_init(
        "name",
        &format!("\"{}/{}\"", service_path, method.proto_name),
        buf,
    );
    generate_field_init("req_ma", &pr_mar, buf);
    generate_field_init("res_ma", &pr_mar, buf);
    buf.push('}');
}

fn generate_field_init(name: &str, value: &str, buf: &mut String) {
    buf.push_str(name);
    buf.push(':');
    buf.push_str(value);
    buf.push_str(", ");
}

fn generate_client(service: &Service, buf: &mut String) {
    // TODO
}

fn generate_server(service: &Service, buf: &mut String) {
    // TODO
}
