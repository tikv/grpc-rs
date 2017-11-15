// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheckRequest {
    // message fields
    pub service: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheckRequest {}

impl HealthCheckRequest {
    pub fn new() -> HealthCheckRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheckRequest {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheckRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheckRequest,
        };
        unsafe {
            instance.get(HealthCheckRequest::new)
        }
    }

    // string service = 1;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        &mut self.service
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service, ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        &self.service
    }

    fn get_service_for_reflect(&self) -> &::std::string::String {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.service
    }
}

impl ::protobuf::Message for HealthCheckRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.service.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.service);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.service.is_empty() {
            os.write_string(1, &self.service)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HealthCheckRequest {
    fn new() -> HealthCheckRequest {
        HealthCheckRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheckRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    HealthCheckRequest::get_service_for_reflect,
                    HealthCheckRequest::mut_service_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheckRequest>(
                    "HealthCheckRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheckRequest {
    fn clear(&mut self) {
        self.clear_service();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheckRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheckResponse {
    // message fields
    pub status: HealthCheckResponse_ServingStatus,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheckResponse {}

impl HealthCheckResponse {
    pub fn new() -> HealthCheckResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheckResponse {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheckResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheckResponse,
        };
        unsafe {
            instance.get(HealthCheckResponse::new)
        }
    }

    // .grpc.health.v1.HealthCheckResponse.ServingStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = HealthCheckResponse_ServingStatus::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: HealthCheckResponse_ServingStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> HealthCheckResponse_ServingStatus {
        self.status
    }

    fn get_status_for_reflect(&self) -> &HealthCheckResponse_ServingStatus {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut HealthCheckResponse_ServingStatus {
        &mut self.status
    }
}

impl ::protobuf::Message for HealthCheckResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != HealthCheckResponse_ServingStatus::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != HealthCheckResponse_ServingStatus::UNKNOWN {
            os.write_enum(1, self.status.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HealthCheckResponse {
    fn new() -> HealthCheckResponse {
        HealthCheckResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheckResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<HealthCheckResponse_ServingStatus>>(
                    "status",
                    HealthCheckResponse::get_status_for_reflect,
                    HealthCheckResponse::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheckResponse>(
                    "HealthCheckResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheckResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheckResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HealthCheckResponse_ServingStatus {
    UNKNOWN = 0,
    SERVING = 1,
    NOT_SERVING = 2,
}

impl ::protobuf::ProtobufEnum for HealthCheckResponse_ServingStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HealthCheckResponse_ServingStatus> {
        match value {
            0 => ::std::option::Option::Some(HealthCheckResponse_ServingStatus::UNKNOWN),
            1 => ::std::option::Option::Some(HealthCheckResponse_ServingStatus::SERVING),
            2 => ::std::option::Option::Some(HealthCheckResponse_ServingStatus::NOT_SERVING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HealthCheckResponse_ServingStatus] = &[
            HealthCheckResponse_ServingStatus::UNKNOWN,
            HealthCheckResponse_ServingStatus::SERVING,
            HealthCheckResponse_ServingStatus::NOT_SERVING,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<HealthCheckResponse_ServingStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HealthCheckResponse_ServingStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HealthCheckResponse_ServingStatus {
}

impl ::std::default::Default for HealthCheckResponse_ServingStatus {
    fn default() -> Self {
        HealthCheckResponse_ServingStatus::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckResponse_ServingStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgrpc/health/v1/health.proto\x12\x0egrpc.health.v1\".\n\x12HealthCh\
    eckRequest\x12\x18\n\x07service\x18\x01\x20\x01(\tR\x07service\"\x9c\x01\
    \n\x13HealthCheckResponse\x12I\n\x06status\x18\x01\x20\x01(\x0e21.grpc.h\
    ealth.v1.HealthCheckResponse.ServingStatusR\x06status\":\n\rServingStatu\
    s\x12\x0b\n\x07UNKNOWN\x10\0\x12\x0b\n\x07SERVING\x10\x01\x12\x0f\n\x0bN\
    OT_SERVING\x10\x022Z\n\x06Health\x12P\n\x05Check\x12\".grpc.health.v1.He\
    althCheckRequest\x1a#.grpc.health.v1.HealthCheckResponseB\x11\xaa\x02\
    \x0eGrpc.Health.V1J\xda\x08\n\x06\x12\x04\x0e\0\"\x01\n\xbf\x04\n\x01\
    \x0c\x12\x03\x0e\0\x122\xb4\x04\x20Copyright\x202015\x20gRPC\x20authors.\
    \n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
    \x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\
    \x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20ma\
    y\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\x16\n\x08\n\x01\x08\x12\
    \x03\x11\0+\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x11\0+\n\x0c\n\x05\x08\xe7\
    \x07\0\x02\x12\x03\x11\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x11\
    \x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x11\x07\x17\n\x0c\n\
    \x05\x08\xe7\x07\0\x07\x12\x03\x11\x1a*\n\n\n\x02\x04\0\x12\x04\x13\0\
    \x15\x01\n\n\n\x03\x04\0\x01\x12\x03\x13\x08\x1a\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x14\x02\x15\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x14\x02\x13\x1c\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x14\x02\x08\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x14\t\x10\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x14\x13\x14\n\
    \n\n\x02\x04\x01\x12\x04\x17\0\x1e\x01\n\n\n\x03\x04\x01\x01\x12\x03\x17\
    \x08\x1b\n\x0c\n\x04\x04\x01\x04\0\x12\x04\x18\x02\x1c\x03\n\x0c\n\x05\
    \x04\x01\x04\0\x01\x12\x03\x18\x07\x14\n\r\n\x06\x04\x01\x04\0\x02\0\x12\
    \x03\x19\x04\x10\n\x0e\n\x07\x04\x01\x04\0\x02\0\x01\x12\x03\x19\x04\x0b\
    \n\x0e\n\x07\x04\x01\x04\0\x02\0\x02\x12\x03\x19\x0e\x0f\n\r\n\x06\x04\
    \x01\x04\0\x02\x01\x12\x03\x1a\x04\x10\n\x0e\n\x07\x04\x01\x04\0\x02\x01\
    \x01\x12\x03\x1a\x04\x0b\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x02\x12\x03\
    \x1a\x0e\x0f\n\r\n\x06\x04\x01\x04\0\x02\x02\x12\x03\x1b\x04\x14\n\x0e\n\
    \x07\x04\x01\x04\0\x02\x02\x01\x12\x03\x1b\x04\x0f\n\x0e\n\x07\x04\x01\
    \x04\0\x02\x02\x02\x12\x03\x1b\x12\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\
    \x1d\x02\x1b\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x1d\x02\x1c\x03\n\x0c\n\
    \x05\x04\x01\x02\0\x06\x12\x03\x1d\x02\x0f\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x1d\x10\x16\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1d\x19\x1a\n\
    \n\n\x02\x06\0\x12\x04\x20\0\"\x01\n\n\n\x03\x06\0\x01\x12\x03\x20\x08\
    \x0e\n\x0b\n\x04\x06\0\x02\0\x12\x03!\x02>\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x03!\x06\x0b\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03!\x0c\x1e\n\x0c\n\
    \x05\x06\0\x02\0\x03\x12\x03!)<b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
