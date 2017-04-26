use std::collections::HashMap;

use protobuf;
use protobuf::compiler_plugin;
use protobuf::code_writer::CodeWriter;
use protobuf::descriptor::*;
use protobuf::descriptorx::*;

use super::util::{snake_name, fq_grpc, MethodType};

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
            (false, false) => (MethodType::Unary, fq_grpc("MethodType::Unary")),
            (true, false) => (MethodType::ClientStreaming, fq_grpc("MethodType::ClientStreaming")),
            (false, true) => (MethodType::ServerStreaming, fq_grpc("MethodType::ServerStreaming")),
            (true, true) => (MethodType::Dulex, fq_grpc("MethodType::Dulex")),
        }
    }

    fn name(&self) -> String {
        snake_name(self.proto.get_name())
    }

    fn fq_name(&self) -> String {
        format!("\"{}/{}\"", self.service_path, &self.proto.get_name())
    }

    fn const_method_name(&self) -> String {
        format!("METHOD_{}", self.name().to_uppercase())
    }

    fn write_definition(&self, w: &mut CodeWriter) {
        w.block(&format!("const {}: {} = {} {{",
                         self.const_method_name(),
                         fq_grpc("Method"),
                         fq_grpc("Method")),
                &format!("}};"),
                |w| {
                    w.field_entry("ty", &self.method_type().1);
                    w.field_entry("name", &self.fq_name());
                });
    }

    // Method signatures
    fn unary(&self, method_name: &str) -> String {
        format!("{}(&self, req: {}) -> {}<{}>",
                method_name,
                self.input(),
                fq_grpc("Result"),
                self.output())
    }

    fn unary_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, req: {}, opt: {}) -> {}<{}>",
                method_name,
                self.input(),
                fq_grpc("CallOption"),
                fq_grpc("Result"),
                self.output())
    }

    fn unary_async(&self, method_name: &str) -> String {
        format!("{}_async(&self, req: {}) -> {}<{}<{}>>",
                method_name,
                self.input(),
                fq_grpc("Result"),
                fq_grpc("UnaryCallHandler"),
                self.output())
    }

    fn unary_async_opt(&self, method_name: &str) -> String {
        format!("{}_async_opt(&self, req: {}, opt: {}) -> {}<{}<{}>>",
                method_name,
                self.input(),
                fq_grpc("CallOption"),
                fq_grpc("Result"),
                fq_grpc("UnaryCallHandler"),
                self.output())
    }

    fn client_streaming(&self, method_name: &str) -> String {
        format!("{}(&self) -> {}<{}<{}, {}>>",
                method_name,
                fq_grpc("Result"),
                fq_grpc("ClientStreamingCallHandler"),
                self.input(),
                self.output())
    }

    fn client_streaming_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, opt: {}) -> {}<{}<{}, {}>>",
                method_name,
                fq_grpc("CallOption"),
                fq_grpc("Result"),
                fq_grpc("ClientStreamingCallHandler"),
                self.input(),
                self.output())
    }

    fn server_streaming(&self, method_name: &str) -> String {
        format!("{}(&self, req: {}) -> {}<{}<{}>>",
                method_name,
                self.input(),
                fq_grpc("Result"),
                fq_grpc("ServerStreamingCallHandler"),
                self.output())
    }

    fn server_streaming_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, req: {}, opt: {}) -> {}<{}<{}>>",
                method_name,
                self.input(),
                fq_grpc("CallOption"),
                fq_grpc("Result"),
                fq_grpc("ServerStreamingCallHandler"),
                self.output())
    }

    fn duplex_streaming(&self, method_name: &str) -> String {
        format!("{}(&self) -> {}<{}<{}, {}>>",
                method_name,
                fq_grpc("Result"),
                fq_grpc("DuplexStreamingCallHandler"),
                self.input(),
                self.output())
    }

    fn duplex_streaming_opt(&self, method_name: &str) -> String {
        format!("{}_opt(&self, opt: {}) -> {}<{}<{}, {}>>",
                method_name,
                fq_grpc("CallOption"),
                fq_grpc("Result"),
                fq_grpc("DuplexStreamingCallHandler"),
                self.input(),
                self.output())
    }

    fn write_client(&self, w: &mut CodeWriter) {
        let method_name = self.name();
        match self.method_type().0 {
            // Unary
            MethodType::Unary => {
                w.pub_fn(&self.unary_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.unary_call(&{}, req, opt)",
                                          self.const_method_name()));
                });
                w.write_line("");

                w.pub_fn(&self.unary(&method_name), |w| {
                    w.write_line(&format!("self.{}_opt(req, {})",
                                          method_name,
                                          fq_grpc("CallOption::default()")));
                });
                w.write_line("");

                w.pub_fn(&self.unary_async_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.unary_call_async(&{}, req, opt)",
                                          self.const_method_name()));
                });
                w.write_line("");

                w.pub_fn(&self.unary_async(&method_name), |w| {
                    w.write_line(&format!("self.{}_async_opt(req, {})",
                                          method_name,
                                          fq_grpc("CallOption::default()")));
                });
            }

            // Client streaming
            MethodType::ClientStreaming => {
                w.pub_fn(&self.client_streaming_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.client_streaming(&{}, opt)",
                                          self.const_method_name()));
                });
                w.write_line("");

                w.pub_fn(&self.client_streaming(&method_name), |w| {
                    w.write_line(&format!("self.{}_opt({})",
                                          method_name,
                                          fq_grpc("CallOption::default()")));
                });
            }

            // Server streaming
            MethodType::ServerStreaming => {
                w.pub_fn(&self.server_streaming_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.server_streaming(&{}, req, opt)",
                                          self.const_method_name()));
                });
                w.write_line("");

                w.pub_fn(&self.server_streaming(&method_name), |w| {
                    w.write_line(&format!("self.{}_opt(req, {})",
                                          method_name,
                                          fq_grpc("CallOption::default()")));
                });
            }

            // Duplex streaming
            MethodType::Dulex => {
                w.pub_fn(&self.duplex_streaming_opt(&method_name), |w| {
                    w.write_line(&format!("self.client.duplex_streaming(&{}, opt)",
                                          self.const_method_name()));
                });
                w.write_line("");

                w.pub_fn(&self.duplex_streaming(&method_name), |w| {
                    w.write_line(&format!("self.{}_opt({})",
                                          method_name,
                                          fq_grpc("CallOption::default()")));
                });
            }
        };
    }

    fn write_service(&self, w: &mut CodeWriter) {
        let (req, resp) = match self.method_type().0 {
            MethodType::Unary => ("UnaryRequest", "UnaryResponseSink"),
            MethodType::ClientStreaming => ("RequestStream", "ClientStreamingResponseSink"),
            MethodType::ServerStreaming => ("UnaryRequest", "ResponseSink"),
            MethodType::Dulex => ("RequestStream", "ResponseSink"),
        };
        let sig = format!("{}(&self, ctx: {}, req: {}<{}>, resp: {}<{}>)",
                          self.name(),
                          fq_grpc("RpcContext"),
                          fq_grpc(req),
                          self.input(),
                          fq_grpc(resp),
                          self.output());
        w.fn_def(&*sig);
    }

    fn write_bind(&self, w: &mut CodeWriter) {
        let add = match self.method_type().0 {
            MethodType::Unary => "add_unary_handler",
            MethodType::ClientStreaming => "add_client_streaming_handler",
            MethodType::ServerStreaming => "add_server_streaming_handler",
            MethodType::Dulex => "add_duplex_streaming_handler",
        };
        w.block(&*format!("builder = builder.{}(&{}, move |ctx, req, resp| {{",
                          add,
                          self.const_method_name()),
                "});",
                |w| {
                    w.write_line(&*format!("instance.{}(ctx, req, resp)", self.name()));
                });
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

    fn write_server(&self, w: &mut CodeWriter) {
        w.pub_trait(&self.service_name(), |w| {
            for method in &self.methods {
                method.write_service(w);
            }
        });

        w.write_line("");

        w.pub_fn(&*format!("bind_{}<S: {} + Send + 'static>(mut builder: {}, s: S) -> {2}",
                           snake_name(self.service_name()),
                           self.service_name(),
                           fq_grpc("ServerBuilder")),
                 |w| {
            w.write_line("let service = ::std::sync::Arc::new(s);");
            for method in &self.methods {
                w.write_line("let instance = service.clone();");
                method.write_bind(w);
            }
            w.write_line("builder");
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
        w.write_line("");
        self.write_server(w);
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
