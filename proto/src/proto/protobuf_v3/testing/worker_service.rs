// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `worker_service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14worker_service.proto\x12\x0cgrpc.testing\x1a\rcontrol.proto2\x97\
    \x02\n\rWorkerService\x12E\n\tRunServer\x12\x18.grpc.testing.ServerArgs\
    \x1a\x1a.grpc.testing.ServerStatus(\x010\x01\x12E\n\tRunClient\x12\x18.g\
    rpc.testing.ClientArgs\x1a\x1a.grpc.testing.ClientStatus(\x010\x01\x12B\
    \n\tCoreCount\x12\x19.grpc.testing.CoreRequest\x1a\x1a.grpc.testing.Core\
    Response\x124\n\nQuitWorker\x12\x12.grpc.testing.Void\x1a\x12.grpc.testi\
    ng.VoidJ\xed\r\n\x06\x12\x04\x10\0,\x01\n\xb8\x05\n\x01\x0c\x12\x03\x10\
    \0\x12\x1aw\x20An\x20integration\x20test\x20service\x20that\x20covers\
    \x20all\x20the\x20method\x20signature\x20permutations\n\x20of\x20unary/s\
    treaming\x20requests/responses.\n2\xb4\x04\x20Copyright\x202015\x20gRPC\
    \x20authors.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Ve\
    rsion\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20t\
    his\x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\
    \x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\
    \x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Un\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\x20\
    is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20\
    WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20expres\
    s\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\t\n\x02\x03\0\x12\x03\x12\0\x17\n\x08\n\x01\x02\
    \x12\x03\x14\0\x15\n\n\n\x02\x06\0\x12\x04\x16\0,\x01\n\n\n\x03\x06\0\
    \x01\x12\x03\x16\x08\x15\n\xdf\x02\n\x04\x06\0\x02\0\x12\x03\x1d\x02A\
    \x1a\xd1\x02\x20Start\x20server\x20with\x20specified\x20workload.\n\x20F\
    irst\x20request\x20sent\x20specifies\x20the\x20ServerConfig\x20followed\
    \x20by\x20ServerStatus\n\x20response.\x20After\x20that,\x20a\x20\"Mark\"\
    \x20can\x20be\x20sent\x20anytime\x20to\x20request\x20the\x20latest\n\x20\
    stats.\x20Closing\x20the\x20stream\x20will\x20initiate\x20shutdown\x20of\
    \x20the\x20test\x20server\n\x20and\x20once\x20the\x20shutdown\x20has\x20\
    finished,\x20the\x20OK\x20status\x20is\x20sent\x20to\x20terminate\n\x20t\
    his\x20RPC.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x1d\x06\x0f\n\x0c\n\
    \x05\x06\0\x02\0\x05\x12\x03\x1d\x10\x16\n\x0c\n\x05\x06\0\x02\0\x02\x12\
    \x03\x1d\x17!\n\x0c\n\x05\x06\0\x02\0\x06\x12\x03\x1d,2\n\x0c\n\x05\x06\
    \0\x02\0\x03\x12\x03\x1d3?\n\xdf\x02\n\x04\x06\0\x02\x01\x12\x03%\x02A\
    \x1a\xd1\x02\x20Start\x20client\x20with\x20specified\x20workload.\n\x20F\
    irst\x20request\x20sent\x20specifies\x20the\x20ClientConfig\x20followed\
    \x20by\x20ClientStatus\n\x20response.\x20After\x20that,\x20a\x20\"Mark\"\
    \x20can\x20be\x20sent\x20anytime\x20to\x20request\x20the\x20latest\n\x20\
    stats.\x20Closing\x20the\x20stream\x20will\x20initiate\x20shutdown\x20of\
    \x20the\x20test\x20client\n\x20and\x20once\x20the\x20shutdown\x20has\x20\
    finished,\x20the\x20OK\x20status\x20is\x20sent\x20to\x20terminate\n\x20t\
    his\x20RPC.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03%\x06\x0f\n\x0c\n\x05\
    \x06\0\x02\x01\x05\x12\x03%\x10\x16\n\x0c\n\x05\x06\0\x02\x01\x02\x12\
    \x03%\x17!\n\x0c\n\x05\x06\0\x02\x01\x06\x12\x03%,2\n\x0c\n\x05\x06\0\
    \x02\x01\x03\x12\x03%3?\n6\n\x04\x06\0\x02\x02\x12\x03(\x024\x1a)\x20Jus\
    t\x20return\x20the\x20core\x20count\x20-\x20unary\x20call\n\n\x0c\n\x05\
    \x06\0\x02\x02\x01\x12\x03(\x06\x0f\n\x0c\n\x05\x06\0\x02\x02\x02\x12\
    \x03(\x10\x1b\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03(&2\n\x1f\n\x04\x06\0\
    \x02\x03\x12\x03+\x02&\x1a\x12\x20Quit\x20this\x20worker\n\n\x0c\n\x05\
    \x06\0\x02\x03\x01\x12\x03+\x06\x10\n\x0c\n\x05\x06\0\x02\x03\x02\x12\
    \x03+\x11\x15\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03+\x20$b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::control::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
