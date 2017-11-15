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
pub struct ByteBufferParams {
    // message fields
    pub req_size: i32,
    pub resp_size: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ByteBufferParams {}

impl ByteBufferParams {
    pub fn new() -> ByteBufferParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ByteBufferParams {
        static mut instance: ::protobuf::lazy::Lazy<ByteBufferParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ByteBufferParams,
        };
        unsafe {
            instance.get(ByteBufferParams::new)
        }
    }

    // int32 req_size = 1;

    pub fn clear_req_size(&mut self) {
        self.req_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_req_size(&mut self, v: i32) {
        self.req_size = v;
    }

    pub fn get_req_size(&self) -> i32 {
        self.req_size
    }

    fn get_req_size_for_reflect(&self) -> &i32 {
        &self.req_size
    }

    fn mut_req_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.req_size
    }

    // int32 resp_size = 2;

    pub fn clear_resp_size(&mut self) {
        self.resp_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_resp_size(&mut self, v: i32) {
        self.resp_size = v;
    }

    pub fn get_resp_size(&self) -> i32 {
        self.resp_size
    }

    fn get_resp_size_for_reflect(&self) -> &i32 {
        &self.resp_size
    }

    fn mut_resp_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.resp_size
    }
}

impl ::protobuf::Message for ByteBufferParams {
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
                    let tmp = is.read_int32()?;
                    self.req_size = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.resp_size = tmp;
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
        if self.req_size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.req_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.resp_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.resp_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.req_size != 0 {
            os.write_int32(1, self.req_size)?;
        }
        if self.resp_size != 0 {
            os.write_int32(2, self.resp_size)?;
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

impl ::protobuf::MessageStatic for ByteBufferParams {
    fn new() -> ByteBufferParams {
        ByteBufferParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<ByteBufferParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "req_size",
                    ByteBufferParams::get_req_size_for_reflect,
                    ByteBufferParams::mut_req_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "resp_size",
                    ByteBufferParams::get_resp_size_for_reflect,
                    ByteBufferParams::mut_resp_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ByteBufferParams>(
                    "ByteBufferParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ByteBufferParams {
    fn clear(&mut self) {
        self.clear_req_size();
        self.clear_resp_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ByteBufferParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ByteBufferParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SimpleProtoParams {
    // message fields
    pub req_size: i32,
    pub resp_size: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SimpleProtoParams {}

impl SimpleProtoParams {
    pub fn new() -> SimpleProtoParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SimpleProtoParams {
        static mut instance: ::protobuf::lazy::Lazy<SimpleProtoParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SimpleProtoParams,
        };
        unsafe {
            instance.get(SimpleProtoParams::new)
        }
    }

    // int32 req_size = 1;

    pub fn clear_req_size(&mut self) {
        self.req_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_req_size(&mut self, v: i32) {
        self.req_size = v;
    }

    pub fn get_req_size(&self) -> i32 {
        self.req_size
    }

    fn get_req_size_for_reflect(&self) -> &i32 {
        &self.req_size
    }

    fn mut_req_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.req_size
    }

    // int32 resp_size = 2;

    pub fn clear_resp_size(&mut self) {
        self.resp_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_resp_size(&mut self, v: i32) {
        self.resp_size = v;
    }

    pub fn get_resp_size(&self) -> i32 {
        self.resp_size
    }

    fn get_resp_size_for_reflect(&self) -> &i32 {
        &self.resp_size
    }

    fn mut_resp_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.resp_size
    }
}

impl ::protobuf::Message for SimpleProtoParams {
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
                    let tmp = is.read_int32()?;
                    self.req_size = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.resp_size = tmp;
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
        if self.req_size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.req_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.resp_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.resp_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.req_size != 0 {
            os.write_int32(1, self.req_size)?;
        }
        if self.resp_size != 0 {
            os.write_int32(2, self.resp_size)?;
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

impl ::protobuf::MessageStatic for SimpleProtoParams {
    fn new() -> SimpleProtoParams {
        SimpleProtoParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<SimpleProtoParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "req_size",
                    SimpleProtoParams::get_req_size_for_reflect,
                    SimpleProtoParams::mut_req_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "resp_size",
                    SimpleProtoParams::get_resp_size_for_reflect,
                    SimpleProtoParams::mut_resp_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SimpleProtoParams>(
                    "SimpleProtoParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SimpleProtoParams {
    fn clear(&mut self) {
        self.clear_req_size();
        self.clear_resp_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SimpleProtoParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SimpleProtoParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ComplexProtoParams {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ComplexProtoParams {}

impl ComplexProtoParams {
    pub fn new() -> ComplexProtoParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ComplexProtoParams {
        static mut instance: ::protobuf::lazy::Lazy<ComplexProtoParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ComplexProtoParams,
        };
        unsafe {
            instance.get(ComplexProtoParams::new)
        }
    }
}

impl ::protobuf::Message for ComplexProtoParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ComplexProtoParams {
    fn new() -> ComplexProtoParams {
        ComplexProtoParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<ComplexProtoParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ComplexProtoParams>(
                    "ComplexProtoParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ComplexProtoParams {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ComplexProtoParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ComplexProtoParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PayloadConfig {
    // message oneof groups
    payload: ::std::option::Option<PayloadConfig_oneof_payload>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PayloadConfig {}

#[derive(Clone,PartialEq)]
pub enum PayloadConfig_oneof_payload {
    bytebuf_params(ByteBufferParams),
    simple_params(SimpleProtoParams),
    complex_params(ComplexProtoParams),
}

impl PayloadConfig {
    pub fn new() -> PayloadConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PayloadConfig {
        static mut instance: ::protobuf::lazy::Lazy<PayloadConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PayloadConfig,
        };
        unsafe {
            instance.get(PayloadConfig::new)
        }
    }

    // .grpc.testing.ByteBufferParams bytebuf_params = 1;

    pub fn clear_bytebuf_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_bytebuf_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytebuf_params(&mut self, v: ByteBufferParams) {
        self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bytebuf_params(&mut self) -> &mut ByteBufferParams {
        if let ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(ByteBufferParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytebuf_params(&mut self) -> ByteBufferParams {
        if self.has_bytebuf_params() {
            match self.payload.take() {
                ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(v)) => v,
                _ => panic!(),
            }
        } else {
            ByteBufferParams::new()
        }
    }

    pub fn get_bytebuf_params(&self) -> &ByteBufferParams {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(ref v)) => v,
            _ => ByteBufferParams::default_instance(),
        }
    }

    // .grpc.testing.SimpleProtoParams simple_params = 2;

    pub fn clear_simple_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_simple_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_simple_params(&mut self, v: SimpleProtoParams) {
        self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(v))
    }

    // Mutable pointer to the field.
    pub fn mut_simple_params(&mut self) -> &mut SimpleProtoParams {
        if let ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(SimpleProtoParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_simple_params(&mut self) -> SimpleProtoParams {
        if self.has_simple_params() {
            match self.payload.take() {
                ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(v)) => v,
                _ => panic!(),
            }
        } else {
            SimpleProtoParams::new()
        }
    }

    pub fn get_simple_params(&self) -> &SimpleProtoParams {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(ref v)) => v,
            _ => SimpleProtoParams::default_instance(),
        }
    }

    // .grpc.testing.ComplexProtoParams complex_params = 3;

    pub fn clear_complex_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_complex_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_complex_params(&mut self, v: ComplexProtoParams) {
        self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(v))
    }

    // Mutable pointer to the field.
    pub fn mut_complex_params(&mut self) -> &mut ComplexProtoParams {
        if let ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(ComplexProtoParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_complex_params(&mut self) -> ComplexProtoParams {
        if self.has_complex_params() {
            match self.payload.take() {
                ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(v)) => v,
                _ => panic!(),
            }
        } else {
            ComplexProtoParams::new()
        }
    }

    pub fn get_complex_params(&self) -> &ComplexProtoParams {
        match self.payload {
            ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(ref v)) => v,
            _ => ComplexProtoParams::default_instance(),
        }
    }
}

impl ::protobuf::Message for PayloadConfig {
    fn is_initialized(&self) -> bool {
        if let Some(PayloadConfig_oneof_payload::bytebuf_params(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(PayloadConfig_oneof_payload::simple_params(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(PayloadConfig_oneof_payload::complex_params(ref v)) = self.payload {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::bytebuf_params(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::simple_params(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(PayloadConfig_oneof_payload::complex_params(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &PayloadConfig_oneof_payload::bytebuf_params(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &PayloadConfig_oneof_payload::simple_params(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &PayloadConfig_oneof_payload::complex_params(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &PayloadConfig_oneof_payload::bytebuf_params(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &PayloadConfig_oneof_payload::simple_params(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &PayloadConfig_oneof_payload::complex_params(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for PayloadConfig {
    fn new() -> PayloadConfig {
        PayloadConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<PayloadConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ByteBufferParams>(
                    "bytebuf_params",
                    PayloadConfig::has_bytebuf_params,
                    PayloadConfig::get_bytebuf_params,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SimpleProtoParams>(
                    "simple_params",
                    PayloadConfig::has_simple_params,
                    PayloadConfig::get_simple_params,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ComplexProtoParams>(
                    "complex_params",
                    PayloadConfig::has_complex_params,
                    PayloadConfig::get_complex_params,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PayloadConfig>(
                    "PayloadConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PayloadConfig {
    fn clear(&mut self) {
        self.clear_bytebuf_params();
        self.clear_simple_params();
        self.clear_complex_params();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PayloadConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PayloadConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgrpc/testing/payloads.proto\x12\x0cgrpc.testing\"J\n\x10ByteBuffer\
    Params\x12\x19\n\x08req_size\x18\x01\x20\x01(\x05R\x07reqSize\x12\x1b\n\
    \tresp_size\x18\x02\x20\x01(\x05R\x08respSize\"K\n\x11SimpleProtoParams\
    \x12\x19\n\x08req_size\x18\x01\x20\x01(\x05R\x07reqSize\x12\x1b\n\tresp_\
    size\x18\x02\x20\x01(\x05R\x08respSize\"\x14\n\x12ComplexProtoParams\"\
    \xf6\x01\n\rPayloadConfig\x12G\n\x0ebytebuf_params\x18\x01\x20\x01(\x0b2\
    \x1e.grpc.testing.ByteBufferParamsH\0R\rbytebufParams\x12F\n\rsimple_par\
    ams\x18\x02\x20\x01(\x0b2\x1f.grpc.testing.SimpleProtoParamsH\0R\x0csimp\
    leParams\x12I\n\x0ecomplex_params\x18\x03\x20\x01(\x0b2\x20.grpc.testing\
    .ComplexProtoParamsH\0R\rcomplexParamsB\t\n\x07payloadJ\xf7\t\n\x06\x12\
    \x04\x0e\0'\x01\n\xbf\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb4\x04\x20Copyri\
    ght\x202015\x20gRPC\x20authors.\n\n\x20Licensed\x20under\x20the\x20Apach\
    e\x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\
    \x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20\
    the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20L\
    icense\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICEN\
    SE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agr\
    eed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\
    \x14\n\n\n\x02\x04\0\x12\x04\x12\0\x15\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x12\x08\x18\n\x0b\n\x04\x04\0\x02\0\x12\x03\x13\x02\x15\n\r\n\x05\x04\0\
    \x02\0\x04\x12\x04\x13\x02\x12\x1a\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x13\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x13\x08\x10\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x13\x13\x14\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x14\x02\x16\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x14\x02\x13\x15\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x14\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x14\x08\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x14\x14\x15\n\
    \n\n\x02\x04\x01\x12\x04\x17\0\x1a\x01\n\n\n\x03\x04\x01\x01\x12\x03\x17\
    \x08\x19\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x18\x02\x15\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04\x18\x02\x17\x1b\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \x18\x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x18\x08\x10\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x18\x13\x14\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03\x19\x02\x16\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x19\x02\x18\
    \x15\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x19\x02\x07\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03\x19\x08\x11\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03\x19\x14\x15\nt\n\x02\x04\x02\x12\x04\x1c\0\x1f\x01\"h\x20TODO\x20(v\
    pai):\x20Fill\x20this\x20in\x20once\x20the\x20details\x20of\x20complex,\
    \x20representative\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20protos\x20are\x20decided\n\n\n\n\x03\x04\x02\x01\x12\x03\x1c\x08\x1a\
    \n\n\n\x02\x04\x03\x12\x04!\0'\x01\n\n\n\x03\x04\x03\x01\x12\x03!\x08\
    \x15\n\x0c\n\x04\x04\x03\x08\0\x12\x04\"\x02&\x03\n\x0c\n\x05\x04\x03\
    \x08\0\x01\x12\x03\"\x08\x0f\n\x0b\n\x04\x04\x03\x02\0\x12\x03#\x04(\n\
    \x0c\n\x05\x04\x03\x02\0\x06\x12\x03#\x04\x14\n\x0c\n\x05\x04\x03\x02\0\
    \x01\x12\x03#\x15#\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03#&'\n\x0b\n\x04\
    \x04\x03\x02\x01\x12\x03$\x04(\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03$\
    \x04\x15\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03$\x16#\n\x0c\n\x05\x04\
    \x03\x02\x01\x03\x12\x03$&'\n\x0b\n\x04\x04\x03\x02\x02\x12\x03%\x04*\n\
    \x0c\n\x05\x04\x03\x02\x02\x06\x12\x03%\x04\x16\n\x0c\n\x05\x04\x03\x02\
    \x02\x01\x12\x03%\x17%\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03%()b\x06pr\
    oto3\
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
