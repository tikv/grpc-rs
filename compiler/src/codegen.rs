use std::collections::HashMap;

use protobuf;
use protobuf::compiler_plugin;
use protobuf::code_writer::CodeWriter;
use protobuf::descriptor::*;
use protobuf::descriptorx::*;

use super::util::snake_name;
use grpc::MethodType;

struct MethodGen<'a> {
    proto: &'a MethodDescriptorProto,
    service_path: String,
    root_scope: &'a RootScope<'a>,
}

impl<'a> MethodGen<'a> {
    fn new(proto: &'a MethodDescriptorProto,
           service_path: String,
           root_scope: &'a RootScope<'a>)
           -> MethodGen<'a> {
        MethodGen {
            proto: proto,
            service_path: service_path,
            root_scope: root_scope,
        }
    }

    fn input(&self) -> String {
        format!("super::{}",
                self.root_scope
                    .find_message(self.proto.get_input_type())
                    .rust_fq_name())
    }

    fn output(&self) -> String {
        format!("super::{}",
                self.root_scope
                    .find_message(self.proto.get_output_type())
                    .rust_fq_name())
    }

    fn method_type(&self) -> (MethodType, String) {
        match (self.proto.get_client_streaming(), self.proto.get_server_streaming()) {
            (false, false) => (MethodType::Unary, "::grpc::MethodType::Unary".to_owned()),
            (true, false) => {
                (MethodType::ClientStreaming, "::grpc::MethodType::ClientStreaming".to_owned())
            }
            (false, true) => {
                (MethodType::ServerStreaming, "::grpc::MethodType::ServerStreaming".to_owned())
            }
            (true, true) => (MethodType::Dulex, "::grpc::MethodType::Dulex".to_owned()),
        }
    }

    fn name(&self) -> String {
        snake_name(self.proto.get_name())
    }

    fn fq_name(&self) -> String {
        format!("\"{}/{}\"", self.service_path, &self.proto.get_name())
    }

    fn write_definition(&self, w: &mut CodeWriter) {
        let method_name = self.name();
        w.write_line(&format!("const METHOD_{}: ::grpc::Method = ::grpc::Method {{",
                              method_name.to_uppercase()));
        w.indented(|w| {
                       w.field_entry("ty", &self.method_type().1);
                       w.field_entry("name", &self.fq_name());
                   });
        w.write_line(&format!("}};"));
    }

    // Method signatures
    fn unary(&self, method_name: &str) -> String {
        format!("{}(&self, req: {}) -> ::grpc::Result<{}>",
                method_name,
                self.input(),
                self.output())
    }

    fn unary_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, req: {}, opt: ::grpc::CallOption) -> ::grpc::Result<{}>",
                method_name,
                self.input(),
                self.output())
    }

    fn unary_async(&self, method_name: &str) -> String {
        format!("{}_async(&self, req: {}) -> ::grpc::Result<::grpc::UnaryCallHandler<{}>>",
                method_name,
                self.input(),
                self.output())
    }

    fn unary_async_opt(&self, method_name: &str) -> String {
        format!("{}_async_opt(&self, req: {}, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::UnaryCallHandler<{}>>",
                method_name,
                self.input(),
                self.output())
    }

    fn client_streaming(&self, method_name: &str) -> String {
        format!("{}(&self) -> ::grpc::Result<::grpc::ClientStreamingCallHandler<{}, {}>>",
                method_name,
                self.input(),
                self.output())
    }

    fn client_streaming_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::ClientStreamingCallHandler<{}, {}>>",
                method_name,
                self.input(),
                self.output())
    }

    fn server_streaming(&self, method_name: &str) -> String {
        format!("{}(&self, req: {}) -> ::grpc::Result<::grpc::ServerStreamingCallHandler<{}>>",
                method_name,
                self.input(),
                self.output())
    }

    fn server_streaming_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, req: {}, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::ServerStreamingCallHandler<{}>>",
                method_name,
                self.input(),
                self.output())
    }

    fn duplex_streaming(&self, method_name: &str) -> String {
        format!("{}(&self) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<{}, {}>>",
                method_name,
                self.input(),
                self.output())
    }

    fn duplex_streaming_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<::grpc::DuplexStreamingCallHandler<{}, {}>>",
                method_name,
                self.input(),
                self.output())
    }

    fn write_client(&self, w: &mut CodeWriter) {
        let method_name = self.name();
        match self.method_type().0 {
            // Unary
            MethodType::Unary => {
                w.pub_fn(&self.unary_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.unary_call(&METHOD_{}, req, opt)",
                                          method_name.to_uppercase()));
                });
                w.write_line("");

                w.pub_fn(&self.unary(&method_name), |w| {
                    w.write_line(&format!("self.{}_opt(req, ::grpc::CallOption::default())",
                                          method_name));
                });
                w.write_line("");

                w.pub_fn(&self.unary_async_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.unary_call_async(&METHOD_{}, req, opt)",
                                          method_name.to_uppercase()));
                });
                w.write_line("");

                w.pub_fn(&self.unary_async(&method_name), |w| {
                    w.write_line(&format!("self.{}_async_opt(req, ::grpc::CallOption::default())",
                                          method_name));
                });
            }

            // Client streaming
            MethodType::ClientStreaming => {
                w.pub_fn(&self.client_streaming_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.client_streaming(&METHOD_{}, opt)",
                                          method_name.to_uppercase()));
                });
                w.write_line("");

                w.pub_fn(&self.client_streaming(&method_name), |w| {
                    w.write_line(&format!("self.{}_opt(::grpc::CallOption::default())",
                                          method_name));
                });
            }

            // Server streaming
            MethodType::ServerStreaming => {
                w.pub_fn(&self.server_streaming_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.server_streaming(&METHOD_{}, req, opt)",
                                          method_name.to_uppercase()));
                });
                w.write_line("");

                w.pub_fn(&self.server_streaming(&method_name), |w| {
                    w.write_line(&format!("self.{}_opt(req, ::grpc::CallOption::default())",
                                          method_name));
                });
            }

            // Duplex streaming
            MethodType::Dulex => {
                w.pub_fn(&self.duplex_streaming_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.duplex_streaming(&METHOD_{}, opt)",
                                          method_name.to_uppercase()));
                });
                w.write_line("");

                w.pub_fn(&self.duplex_streaming(&method_name), |w| {
                    w.write_line(&format!("self.{}_opt(::grpc::CallOption::default())",
                                          method_name));
                });
            }
        };
    }
}

struct ServiceGen<'a> {
    proto: &'a ServiceDescriptorProto,
    methods: Vec<MethodGen<'a>>,
}

impl<'a> ServiceGen<'a> {
    fn new(proto: &'a ServiceDescriptorProto,
           file: &FileDescriptorProto,
           root_scope: &'a RootScope)
           -> ServiceGen<'a> {
        let service_path = if file.get_package().is_empty() {
            format!("/{}", proto.get_name())
        } else {
            format!("/{}.{}", file.get_package(), proto.get_name())
        };
        let methods = proto
            .get_method()
            .into_iter()
            .map(|m| MethodGen::new(m, service_path.clone(), root_scope))
            .collect();

        ServiceGen {
            proto: proto,
            methods: methods,
        }
    }

    fn service_name(&self) -> &str {
        self.proto.get_name()
    }

    fn client_name(&self) -> String {
        format!("{}Client", self.service_name())
    }

    fn write_client(&self, w: &mut CodeWriter) {
        w.pub_struct(&self.client_name(),
                     |w| { w.field_decl("client", "::grpc::Client"); });

        w.write_line("");

        w.impl_self_block(&self.client_name(), |w| {
            w.pub_fn("new(channel: ::grpc::Channel) -> Self", |w| {
                w.expr_block(&self.client_name(),
                             |w| { w.field_entry("client", "::grpc::Client::new(channel)"); });
            });

            for method in &self.methods {
                w.write_line("");
                method.write_client(w);
            }
        });
    }

    fn write_method_definitions(&self, w: &mut CodeWriter) {
        for (i, method) in self.methods.iter().enumerate() {
            if i != 0 {
                w.write_line("");
            }

            method.write_definition(w);
        }
    }

    fn write(&self, w: &mut CodeWriter) {
        self.write_method_definitions(w);
        w.write_line("");
        self.write_client(w);
    }
}

fn gen_file(file: &FileDescriptorProto,
            root_scope: &RootScope)
            -> Option<compiler_plugin::GenResult> {
    if file.get_service().is_empty() {
        return None;
    }

    let base = protobuf::descriptorx::proto_path_to_rust_mod(file.get_name());

    let mut v = Vec::new();
    {
        let mut w = CodeWriter::new(&mut v);
        w.write_generated();

        for service in file.get_service() {
            w.write_line("");
            ServiceGen::new(service, file, root_scope).write(&mut w);
        }
    }

    Some(compiler_plugin::GenResult {
             name: base + "_grpc.rs",
             content: v,
         })
}

pub fn gen(file_descriptors: &[FileDescriptorProto],
           files_to_generate: &[String])
           -> Vec<compiler_plugin::GenResult> {
    let files_map: HashMap<&str, &FileDescriptorProto> = file_descriptors
        .iter()
        .map(|f| (f.get_name(), f))
        .collect();

    let root_scope = RootScope { file_descriptors: file_descriptors };

    let mut results = Vec::new();

    for file_name in files_to_generate {
        let file = files_map[&file_name[..]];

        if file.get_service().is_empty() {
            continue;
        }

        results.extend(gen_file(file, &root_scope).into_iter());
    }

    results
}

pub fn protoc_gen_grpc_rust_main() {
    compiler_plugin::plugin_main(gen);
}
