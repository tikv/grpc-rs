use std::collections::HashMap;

use protobuf;
use protobuf::compiler_plugin;
use protobuf::code_writer::CodeWriter;
use protobuf::descriptor::*;
use protobuf::descriptorx::*;

use super::util::snake_name;

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

    fn input_async(&self) -> String {
        match self.proto.get_client_streaming() {
            false => self.input(),
            true => format!("::grpc::futures_grpc::GrpcStreamSend<{}>", self.input()),
        }
    }

    fn output_async(&self) -> String {
        match self.proto.get_server_streaming() {
            false => format!("::grpc::futures_grpc::GrpcFutureSend<{}>", self.output()),
            true => format!("::grpc::futures_grpc::GrpcStreamSend<{}>", self.output()),
        }
    }

    fn input_sync(&self) -> String {
        match self.proto.get_client_streaming() {
            false => self.input(),
            true => format!("::grpc::iter::GrpcIterator<{}>", self.input()),
        }
    }

    fn output_sync(&self) -> String {
        match self.proto.get_server_streaming() {
            false => format!("::grpc::Result<{}>", self.output()),
            true => {
                format!("::grpc::Result<ServerStreamingCallHandler<{}>>",
                        self.output())
            }
        }
    }

    // Method signatures
    fn unary(&self, method_name: &str) -> String {
        format!("{}(&self, req: {}) -> {}",
                method_name,
                self.input_sync(),
                self.output_sync())
    }

    fn unary_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, req: {}, opt: ::grpc::CallOption) -> {}",
                method_name,
                self.input_sync(),
                self.output_sync())
    }

    fn unary_async(&self, method_name: &str) -> String {
        format!("{}_async(&self, req: {}) -> ::grpc::Result<UnaryCallHandler<{}>>",
                method_name,
                self.input_sync(),
                self.output_sync())
    }

    fn unary_async_opt(&self, method_name: &str) -> String {
        format!("{}_async_opt(&self, req: {}, opt: ::grpc::CallOption) -> ::grpc::Result<UnaryCallHandler<{}>>",
                method_name,
                self.input_sync(),
                self.output_sync())
    }

    fn client_streaming(&self, method_name: &str) -> String {
        format!("{}(&self) -> ::grpc::Result<ClientStreamingCallHandler<{}, {}>>",
                method_name,
                self.input_sync(),
                self.output_sync())
    }

    fn client_streaming_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, opt: ::grpc::CallOption) -> ::grpc::Result<ClientStreamingCallHandler<{}, {}>>",
                method_name,
                self.input_sync(),
                self.output_sync())
    }

    fn server_stream(&self, method_name: &str) -> String {
        format!("{}(&self, req: {}) -> ::grpc::Result<ServerStreamingCallHandler<{}>>",
                method_name,
                self.input_sync(),
                self.output_sync())
    }

    fn server_stream_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, req: {}, opt: ::grpc::CallOption) -> ::grpc::Result<ServerStreamingCallHandler<{}>>",
                method_name,
                self.input_sync(),
                self.output_sync())
    }

    fn write_client(&self, w: &mut CodeWriter, method_name: &str) {
        match (self.proto.get_client_streaming(), self.proto.get_server_streaming()) {

            // Unary
            (false, false) => {
                w.def_fn(&self.unary_opt(method_name), |w| {
                    w.write_line(&format!("self.client.unary_call(METHOD_{}, req, opt)",
                                          method_name));
                });
                w.write_line("");

                w.def_fn(&self.unary(method_name), |w| {
                    w.write_line(&format!("self.{}_opt(req, CallOption::default())", method_name));
                });
                w.write_line("");

                w.def_fn(&self.unary_async_opt(method_name), |w| {
                    w.write_line(&format!("self.client.unary_call_async(METHOD_{}, req, opt)",
                                          method_name));
                });
                w.write_line("");

                w.def_fn(&self.unary_async(method_name), |w| {
                    w.write_line(&format!("self.{}_async_opt(req, CallOption::default())",
                                          method_name));
                });
            }

            // Client streaming
            (true, false) => {
                w.def_fn(&self.client_streaming_opt(method_name), |w| {
                    w.write_line(&format!("self.client.client_streaming(METHOD_{}, opt)",
                                          method_name));
                });
                w.write_line("");

                w.def_fn(&self.client_streaming(method_name), |w| {
                    w.write_line(&format!("self.{}_opt(CallOption::default())", method_name));
                });
            }

            // Server streaming
            (false, true) => {
                w.def_fn(&self.server_streaming_opt(method_name), |w| {
                    w.write_line(&format!("self.client.server_streaming(METHOD_{}, req, opt)",
                                          method_name));
                });
                w.write_line("");

                w.def_fn(&self.server_streaming(method_name), |w| {
                    w.write_line(&format!("self.{}_opt(req, CallOption::default())", method_name));
                });
            }

            // Bi-streaming
            (true, true) => {
                // println!("TODO");
            }
        };
    }
}

struct ServiceGen<'a> {
    proto: &'a ServiceDescriptorProto,
    _root_scope: &'a RootScope<'a>,
    methods: Vec<MethodGen<'a>>,
    service_path: String,
    _package: String,
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
            _root_scope: root_scope,
            methods: methods,
            service_path: service_path,
            _package: file.get_package().to_string(),
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
            w.pub_fn("new(channel: ::grpc::Client) -> Self", |w| {
                w.expr_block(&self.client_name(),
                             |w| { w.field_entry("client", "::grpc:Client::new(channel)"); });
            });

            for method in &self.methods {
                w.write_line("");

                let snake_method_name = snake_name(method.proto.get_name());
                method.write_client(w, &snake_method_name);
            }
        });
    }

    fn write_method_def(&self, w: &mut CodeWriter) {
        for (i, method) in self.methods.iter().enumerate() {
            if i != 0 {
                w.write_line("");
            }

            let method_name = snake_name(method.proto.get_name());
            // TODO: Better name.
            w.write_line(&format!("const METHOD_{}: Method = Method {{", method_name));
            w.indented(|w| {
                let ty = match (method.proto.get_client_streaming(),
                                method.proto.get_server_streaming()) {
                    (false, false) => "MethodType::Unary",
                    (false, true) => "MethodType::ClientStreaming",
                    (true, false) => "MethodType::ServerStreaming",
                    (true, true) => "MethodType::Dulex",
                };
                w.field_entry("ty", ty);

                let name = format!("\"{}/{}\"", self.service_path, method.proto.get_name());
                w.field_entry("name", &name);
            });
            w.write_line(&format!("}};"));
        }
    }

    fn write(&self, w: &mut CodeWriter) {
        self.write_method_def(w);
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
        w.write_line("");

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
