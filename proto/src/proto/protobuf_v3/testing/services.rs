// This file is generated by rust-protobuf 3.3.0. Do not edit
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

//! Generated file from `grpc/testing/services.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobufv3::VERSION_3_3_0;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgrpc/testing/services.proto\x12\x0cgrpc.testing\x1a\x1bgrpc/testin\
    g/messages.proto\x1a\x1agrpc/testing/control.proto\x1a\x18grpc/testing/s\
    tats.proto2\xa6\x03\n\x10BenchmarkService\x12F\n\tUnaryCall\x12\x1b.grpc\
    .testing.SimpleRequest\x1a\x1c.grpc.testing.SimpleResponse\x12N\n\rStrea\
    mingCall\x12\x1b.grpc.testing.SimpleRequest\x1a\x1c.grpc.testing.SimpleR\
    esponse(\x010\x01\x12R\n\x13StreamingFromClient\x12\x1b.grpc.testing.Sim\
    pleRequest\x1a\x1c.grpc.testing.SimpleResponse(\x01\x12R\n\x13StreamingF\
    romServer\x12\x1b.grpc.testing.SimpleRequest\x1a\x1c.grpc.testing.Simple\
    Response0\x01\x12R\n\x11StreamingBothWays\x12\x1b.grpc.testing.SimpleReq\
    uest\x1a\x1c.grpc.testing.SimpleResponse(\x010\x012\x97\x02\n\rWorkerSer\
    vice\x12E\n\tRunServer\x12\x18.grpc.testing.ServerArgs\x1a\x1a.grpc.test\
    ing.ServerStatus(\x010\x01\x12E\n\tRunClient\x12\x18.grpc.testing.Client\
    Args\x1a\x1a.grpc.testing.ClientStatus(\x010\x01\x12B\n\tCoreCount\x12\
    \x19.grpc.testing.CoreRequest\x1a\x1a.grpc.testing.CoreResponse\x124\n\n\
    QuitWorker\x12\x12.grpc.testing.Void\x1a\x12.grpc.testing.Void2^\n\x18Re\
    portQpsScenarioService\x12B\n\x0eReportScenario\x12\x1c.grpc.testing.Sce\
    narioResult\x1a\x12.grpc.testing.Voidb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobufv3::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobufv3::rt::Lazy<::protobufv3::descriptor::FileDescriptorProto> = ::protobufv3::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobufv3::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobufv3::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobufv3::rt::Lazy<::protobufv3::reflect::GeneratedFileDescriptor> = ::protobufv3::rt::Lazy::new();
    static file_descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::FileDescriptor> = ::protobufv3::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::messages::file_descriptor().clone());
            deps.push(super::control::file_descriptor().clone());
            deps.push(super::stats::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobufv3::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobufv3::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}

pub use super::services_grpc::*;
