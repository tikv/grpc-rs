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
pub struct BoolValue {
    // message fields
    pub value: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BoolValue {}

impl BoolValue {
    pub fn new() -> BoolValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BoolValue {
        static mut instance: ::protobuf::lazy::Lazy<BoolValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BoolValue,
        };
        unsafe {
            instance.get(BoolValue::new)
        }
    }

    // bool value = 1;

    pub fn clear_value(&mut self) {
        self.value = false;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: bool) {
        self.value = v;
    }

    pub fn get_value(&self) -> bool {
        self.value
    }

    fn get_value_for_reflect(&self) -> &bool {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl ::protobuf::Message for BoolValue {
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
                    let tmp = is.read_bool()?;
                    self.value = tmp;
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
        if self.value != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != false {
            os.write_bool(1, self.value)?;
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

impl ::protobuf::MessageStatic for BoolValue {
    fn new() -> BoolValue {
        BoolValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<BoolValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "value",
                    BoolValue::get_value_for_reflect,
                    BoolValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BoolValue>(
                    "BoolValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BoolValue {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BoolValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BoolValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Payload {
    // message fields
    pub field_type: PayloadType,
    pub body: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Payload {}

impl Payload {
    pub fn new() -> Payload {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Payload {
        static mut instance: ::protobuf::lazy::Lazy<Payload> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Payload,
        };
        unsafe {
            instance.get(Payload::new)
        }
    }

    // .grpc.testing.PayloadType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = PayloadType::COMPRESSABLE;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: PayloadType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> PayloadType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &PayloadType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut PayloadType {
        &mut self.field_type
    }

    // bytes body = 2;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.body
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.body, ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body
    }

    fn get_body_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.body
    }
}

impl ::protobuf::Message for Payload {
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
                    self.field_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.body)?;
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
        if self.field_type != PayloadType::COMPRESSABLE {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if !self.body.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.body);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != PayloadType::COMPRESSABLE {
            os.write_enum(1, self.field_type.value())?;
        }
        if !self.body.is_empty() {
            os.write_bytes(2, &self.body)?;
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

impl ::protobuf::MessageStatic for Payload {
    fn new() -> Payload {
        Payload::new()
    }

    fn descriptor_static(_: ::std::option::Option<Payload>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PayloadType>>(
                    "type",
                    Payload::get_field_type_for_reflect,
                    Payload::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    Payload::get_body_for_reflect,
                    Payload::mut_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Payload>(
                    "Payload",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Payload {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Payload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Payload {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EchoStatus {
    // message fields
    pub code: i32,
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EchoStatus {}

impl EchoStatus {
    pub fn new() -> EchoStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EchoStatus {
        static mut instance: ::protobuf::lazy::Lazy<EchoStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EchoStatus,
        };
        unsafe {
            instance.get(EchoStatus::new)
        }
    }

    // int32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: i32) {
        self.code = v;
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &i32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut i32 {
        &mut self.code
    }

    // string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for EchoStatus {
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
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_int32(1, self.code)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
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

impl ::protobuf::MessageStatic for EchoStatus {
    fn new() -> EchoStatus {
        EchoStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<EchoStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code",
                    EchoStatus::get_code_for_reflect,
                    EchoStatus::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    EchoStatus::get_message_for_reflect,
                    EchoStatus::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EchoStatus>(
                    "EchoStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EchoStatus {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EchoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EchoStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SimpleRequest {
    // message fields
    pub response_type: PayloadType,
    pub response_size: i32,
    pub payload: ::protobuf::SingularPtrField<Payload>,
    pub fill_username: bool,
    pub fill_oauth_scope: bool,
    pub response_compressed: ::protobuf::SingularPtrField<BoolValue>,
    pub response_status: ::protobuf::SingularPtrField<EchoStatus>,
    pub expect_compressed: ::protobuf::SingularPtrField<BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SimpleRequest {}

impl SimpleRequest {
    pub fn new() -> SimpleRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SimpleRequest {
        static mut instance: ::protobuf::lazy::Lazy<SimpleRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SimpleRequest,
        };
        unsafe {
            instance.get(SimpleRequest::new)
        }
    }

    // .grpc.testing.PayloadType response_type = 1;

    pub fn clear_response_type(&mut self) {
        self.response_type = PayloadType::COMPRESSABLE;
    }

    // Param is passed by value, moved
    pub fn set_response_type(&mut self, v: PayloadType) {
        self.response_type = v;
    }

    pub fn get_response_type(&self) -> PayloadType {
        self.response_type
    }

    fn get_response_type_for_reflect(&self) -> &PayloadType {
        &self.response_type
    }

    fn mut_response_type_for_reflect(&mut self) -> &mut PayloadType {
        &mut self.response_type
    }

    // int32 response_size = 2;

    pub fn clear_response_size(&mut self) {
        self.response_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_response_size(&mut self, v: i32) {
        self.response_size = v;
    }

    pub fn get_response_size(&self) -> i32 {
        self.response_size
    }

    fn get_response_size_for_reflect(&self) -> &i32 {
        &self.response_size
    }

    fn mut_response_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.response_size
    }

    // .grpc.testing.Payload payload = 3;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: Payload) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut Payload {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> Payload {
        self.payload.take().unwrap_or_else(|| Payload::new())
    }

    pub fn get_payload(&self) -> &Payload {
        self.payload.as_ref().unwrap_or_else(|| Payload::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<Payload> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Payload> {
        &mut self.payload
    }

    // bool fill_username = 4;

    pub fn clear_fill_username(&mut self) {
        self.fill_username = false;
    }

    // Param is passed by value, moved
    pub fn set_fill_username(&mut self, v: bool) {
        self.fill_username = v;
    }

    pub fn get_fill_username(&self) -> bool {
        self.fill_username
    }

    fn get_fill_username_for_reflect(&self) -> &bool {
        &self.fill_username
    }

    fn mut_fill_username_for_reflect(&mut self) -> &mut bool {
        &mut self.fill_username
    }

    // bool fill_oauth_scope = 5;

    pub fn clear_fill_oauth_scope(&mut self) {
        self.fill_oauth_scope = false;
    }

    // Param is passed by value, moved
    pub fn set_fill_oauth_scope(&mut self, v: bool) {
        self.fill_oauth_scope = v;
    }

    pub fn get_fill_oauth_scope(&self) -> bool {
        self.fill_oauth_scope
    }

    fn get_fill_oauth_scope_for_reflect(&self) -> &bool {
        &self.fill_oauth_scope
    }

    fn mut_fill_oauth_scope_for_reflect(&mut self) -> &mut bool {
        &mut self.fill_oauth_scope
    }

    // .grpc.testing.BoolValue response_compressed = 6;

    pub fn clear_response_compressed(&mut self) {
        self.response_compressed.clear();
    }

    pub fn has_response_compressed(&self) -> bool {
        self.response_compressed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_compressed(&mut self, v: BoolValue) {
        self.response_compressed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_compressed(&mut self) -> &mut BoolValue {
        if self.response_compressed.is_none() {
            self.response_compressed.set_default();
        }
        self.response_compressed.as_mut().unwrap()
    }

    // Take field
    pub fn take_response_compressed(&mut self) -> BoolValue {
        self.response_compressed.take().unwrap_or_else(|| BoolValue::new())
    }

    pub fn get_response_compressed(&self) -> &BoolValue {
        self.response_compressed.as_ref().unwrap_or_else(|| BoolValue::default_instance())
    }

    fn get_response_compressed_for_reflect(&self) -> &::protobuf::SingularPtrField<BoolValue> {
        &self.response_compressed
    }

    fn mut_response_compressed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BoolValue> {
        &mut self.response_compressed
    }

    // .grpc.testing.EchoStatus response_status = 7;

    pub fn clear_response_status(&mut self) {
        self.response_status.clear();
    }

    pub fn has_response_status(&self) -> bool {
        self.response_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_status(&mut self, v: EchoStatus) {
        self.response_status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_status(&mut self) -> &mut EchoStatus {
        if self.response_status.is_none() {
            self.response_status.set_default();
        }
        self.response_status.as_mut().unwrap()
    }

    // Take field
    pub fn take_response_status(&mut self) -> EchoStatus {
        self.response_status.take().unwrap_or_else(|| EchoStatus::new())
    }

    pub fn get_response_status(&self) -> &EchoStatus {
        self.response_status.as_ref().unwrap_or_else(|| EchoStatus::default_instance())
    }

    fn get_response_status_for_reflect(&self) -> &::protobuf::SingularPtrField<EchoStatus> {
        &self.response_status
    }

    fn mut_response_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EchoStatus> {
        &mut self.response_status
    }

    // .grpc.testing.BoolValue expect_compressed = 8;

    pub fn clear_expect_compressed(&mut self) {
        self.expect_compressed.clear();
    }

    pub fn has_expect_compressed(&self) -> bool {
        self.expect_compressed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expect_compressed(&mut self, v: BoolValue) {
        self.expect_compressed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expect_compressed(&mut self) -> &mut BoolValue {
        if self.expect_compressed.is_none() {
            self.expect_compressed.set_default();
        }
        self.expect_compressed.as_mut().unwrap()
    }

    // Take field
    pub fn take_expect_compressed(&mut self) -> BoolValue {
        self.expect_compressed.take().unwrap_or_else(|| BoolValue::new())
    }

    pub fn get_expect_compressed(&self) -> &BoolValue {
        self.expect_compressed.as_ref().unwrap_or_else(|| BoolValue::default_instance())
    }

    fn get_expect_compressed_for_reflect(&self) -> &::protobuf::SingularPtrField<BoolValue> {
        &self.expect_compressed
    }

    fn mut_expect_compressed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BoolValue> {
        &mut self.expect_compressed
    }
}

impl ::protobuf::Message for SimpleRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.payload {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_compressed {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_status {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.expect_compressed {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    self.response_type = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.response_size = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payload)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.fill_username = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.fill_oauth_scope = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response_compressed)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response_status)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expect_compressed)?;
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
        if self.response_type != PayloadType::COMPRESSABLE {
            my_size += ::protobuf::rt::enum_size(1, self.response_type);
        }
        if self.response_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.response_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.payload.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.fill_username != false {
            my_size += 2;
        }
        if self.fill_oauth_scope != false {
            my_size += 2;
        }
        if let Some(ref v) = self.response_compressed.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.response_status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.expect_compressed.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.response_type != PayloadType::COMPRESSABLE {
            os.write_enum(1, self.response_type.value())?;
        }
        if self.response_size != 0 {
            os.write_int32(2, self.response_size)?;
        }
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.fill_username != false {
            os.write_bool(4, self.fill_username)?;
        }
        if self.fill_oauth_scope != false {
            os.write_bool(5, self.fill_oauth_scope)?;
        }
        if let Some(ref v) = self.response_compressed.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.response_status.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.expect_compressed.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for SimpleRequest {
    fn new() -> SimpleRequest {
        SimpleRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SimpleRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PayloadType>>(
                    "response_type",
                    SimpleRequest::get_response_type_for_reflect,
                    SimpleRequest::mut_response_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "response_size",
                    SimpleRequest::get_response_size_for_reflect,
                    SimpleRequest::mut_response_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Payload>>(
                    "payload",
                    SimpleRequest::get_payload_for_reflect,
                    SimpleRequest::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "fill_username",
                    SimpleRequest::get_fill_username_for_reflect,
                    SimpleRequest::mut_fill_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "fill_oauth_scope",
                    SimpleRequest::get_fill_oauth_scope_for_reflect,
                    SimpleRequest::mut_fill_oauth_scope_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BoolValue>>(
                    "response_compressed",
                    SimpleRequest::get_response_compressed_for_reflect,
                    SimpleRequest::mut_response_compressed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EchoStatus>>(
                    "response_status",
                    SimpleRequest::get_response_status_for_reflect,
                    SimpleRequest::mut_response_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BoolValue>>(
                    "expect_compressed",
                    SimpleRequest::get_expect_compressed_for_reflect,
                    SimpleRequest::mut_expect_compressed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SimpleRequest>(
                    "SimpleRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SimpleRequest {
    fn clear(&mut self) {
        self.clear_response_type();
        self.clear_response_size();
        self.clear_payload();
        self.clear_fill_username();
        self.clear_fill_oauth_scope();
        self.clear_response_compressed();
        self.clear_response_status();
        self.clear_expect_compressed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SimpleRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SimpleRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SimpleResponse {
    // message fields
    pub payload: ::protobuf::SingularPtrField<Payload>,
    pub username: ::std::string::String,
    pub oauth_scope: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SimpleResponse {}

impl SimpleResponse {
    pub fn new() -> SimpleResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SimpleResponse {
        static mut instance: ::protobuf::lazy::Lazy<SimpleResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SimpleResponse,
        };
        unsafe {
            instance.get(SimpleResponse::new)
        }
    }

    // .grpc.testing.Payload payload = 1;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: Payload) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut Payload {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> Payload {
        self.payload.take().unwrap_or_else(|| Payload::new())
    }

    pub fn get_payload(&self) -> &Payload {
        self.payload.as_ref().unwrap_or_else(|| Payload::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<Payload> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Payload> {
        &mut self.payload
    }

    // string username = 2;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // string oauth_scope = 3;

    pub fn clear_oauth_scope(&mut self) {
        self.oauth_scope.clear();
    }

    // Param is passed by value, moved
    pub fn set_oauth_scope(&mut self, v: ::std::string::String) {
        self.oauth_scope = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oauth_scope(&mut self) -> &mut ::std::string::String {
        &mut self.oauth_scope
    }

    // Take field
    pub fn take_oauth_scope(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.oauth_scope, ::std::string::String::new())
    }

    pub fn get_oauth_scope(&self) -> &str {
        &self.oauth_scope
    }

    fn get_oauth_scope_for_reflect(&self) -> &::std::string::String {
        &self.oauth_scope
    }

    fn mut_oauth_scope_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.oauth_scope
    }
}

impl ::protobuf::Message for SimpleResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.payload {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payload)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.oauth_scope)?;
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
        if let Some(ref v) = self.payload.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.username);
        }
        if !self.oauth_scope.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.oauth_scope);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.username.is_empty() {
            os.write_string(2, &self.username)?;
        }
        if !self.oauth_scope.is_empty() {
            os.write_string(3, &self.oauth_scope)?;
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

impl ::protobuf::MessageStatic for SimpleResponse {
    fn new() -> SimpleResponse {
        SimpleResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SimpleResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Payload>>(
                    "payload",
                    SimpleResponse::get_payload_for_reflect,
                    SimpleResponse::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    SimpleResponse::get_username_for_reflect,
                    SimpleResponse::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "oauth_scope",
                    SimpleResponse::get_oauth_scope_for_reflect,
                    SimpleResponse::mut_oauth_scope_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SimpleResponse>(
                    "SimpleResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SimpleResponse {
    fn clear(&mut self) {
        self.clear_payload();
        self.clear_username();
        self.clear_oauth_scope();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SimpleResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SimpleResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamingInputCallRequest {
    // message fields
    pub payload: ::protobuf::SingularPtrField<Payload>,
    pub expect_compressed: ::protobuf::SingularPtrField<BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreamingInputCallRequest {}

impl StreamingInputCallRequest {
    pub fn new() -> StreamingInputCallRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreamingInputCallRequest {
        static mut instance: ::protobuf::lazy::Lazy<StreamingInputCallRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreamingInputCallRequest,
        };
        unsafe {
            instance.get(StreamingInputCallRequest::new)
        }
    }

    // .grpc.testing.Payload payload = 1;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: Payload) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut Payload {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> Payload {
        self.payload.take().unwrap_or_else(|| Payload::new())
    }

    pub fn get_payload(&self) -> &Payload {
        self.payload.as_ref().unwrap_or_else(|| Payload::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<Payload> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Payload> {
        &mut self.payload
    }

    // .grpc.testing.BoolValue expect_compressed = 2;

    pub fn clear_expect_compressed(&mut self) {
        self.expect_compressed.clear();
    }

    pub fn has_expect_compressed(&self) -> bool {
        self.expect_compressed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expect_compressed(&mut self, v: BoolValue) {
        self.expect_compressed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expect_compressed(&mut self) -> &mut BoolValue {
        if self.expect_compressed.is_none() {
            self.expect_compressed.set_default();
        }
        self.expect_compressed.as_mut().unwrap()
    }

    // Take field
    pub fn take_expect_compressed(&mut self) -> BoolValue {
        self.expect_compressed.take().unwrap_or_else(|| BoolValue::new())
    }

    pub fn get_expect_compressed(&self) -> &BoolValue {
        self.expect_compressed.as_ref().unwrap_or_else(|| BoolValue::default_instance())
    }

    fn get_expect_compressed_for_reflect(&self) -> &::protobuf::SingularPtrField<BoolValue> {
        &self.expect_compressed
    }

    fn mut_expect_compressed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BoolValue> {
        &mut self.expect_compressed
    }
}

impl ::protobuf::Message for StreamingInputCallRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.payload {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.expect_compressed {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payload)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expect_compressed)?;
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
        if let Some(ref v) = self.payload.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.expect_compressed.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.expect_compressed.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for StreamingInputCallRequest {
    fn new() -> StreamingInputCallRequest {
        StreamingInputCallRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreamingInputCallRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Payload>>(
                    "payload",
                    StreamingInputCallRequest::get_payload_for_reflect,
                    StreamingInputCallRequest::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BoolValue>>(
                    "expect_compressed",
                    StreamingInputCallRequest::get_expect_compressed_for_reflect,
                    StreamingInputCallRequest::mut_expect_compressed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreamingInputCallRequest>(
                    "StreamingInputCallRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreamingInputCallRequest {
    fn clear(&mut self) {
        self.clear_payload();
        self.clear_expect_compressed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamingInputCallRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamingInputCallRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamingInputCallResponse {
    // message fields
    pub aggregated_payload_size: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreamingInputCallResponse {}

impl StreamingInputCallResponse {
    pub fn new() -> StreamingInputCallResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreamingInputCallResponse {
        static mut instance: ::protobuf::lazy::Lazy<StreamingInputCallResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreamingInputCallResponse,
        };
        unsafe {
            instance.get(StreamingInputCallResponse::new)
        }
    }

    // int32 aggregated_payload_size = 1;

    pub fn clear_aggregated_payload_size(&mut self) {
        self.aggregated_payload_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_aggregated_payload_size(&mut self, v: i32) {
        self.aggregated_payload_size = v;
    }

    pub fn get_aggregated_payload_size(&self) -> i32 {
        self.aggregated_payload_size
    }

    fn get_aggregated_payload_size_for_reflect(&self) -> &i32 {
        &self.aggregated_payload_size
    }

    fn mut_aggregated_payload_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.aggregated_payload_size
    }
}

impl ::protobuf::Message for StreamingInputCallResponse {
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
                    self.aggregated_payload_size = tmp;
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
        if self.aggregated_payload_size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.aggregated_payload_size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.aggregated_payload_size != 0 {
            os.write_int32(1, self.aggregated_payload_size)?;
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

impl ::protobuf::MessageStatic for StreamingInputCallResponse {
    fn new() -> StreamingInputCallResponse {
        StreamingInputCallResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreamingInputCallResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "aggregated_payload_size",
                    StreamingInputCallResponse::get_aggregated_payload_size_for_reflect,
                    StreamingInputCallResponse::mut_aggregated_payload_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreamingInputCallResponse>(
                    "StreamingInputCallResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreamingInputCallResponse {
    fn clear(&mut self) {
        self.clear_aggregated_payload_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamingInputCallResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamingInputCallResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseParameters {
    // message fields
    pub size: i32,
    pub interval_us: i32,
    pub compressed: ::protobuf::SingularPtrField<BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseParameters {}

impl ResponseParameters {
    pub fn new() -> ResponseParameters {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseParameters {
        static mut instance: ::protobuf::lazy::Lazy<ResponseParameters> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseParameters,
        };
        unsafe {
            instance.get(ResponseParameters::new)
        }
    }

    // int32 size = 1;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i32) {
        self.size = v;
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &i32 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.size
    }

    // int32 interval_us = 2;

    pub fn clear_interval_us(&mut self) {
        self.interval_us = 0;
    }

    // Param is passed by value, moved
    pub fn set_interval_us(&mut self, v: i32) {
        self.interval_us = v;
    }

    pub fn get_interval_us(&self) -> i32 {
        self.interval_us
    }

    fn get_interval_us_for_reflect(&self) -> &i32 {
        &self.interval_us
    }

    fn mut_interval_us_for_reflect(&mut self) -> &mut i32 {
        &mut self.interval_us
    }

    // .grpc.testing.BoolValue compressed = 3;

    pub fn clear_compressed(&mut self) {
        self.compressed.clear();
    }

    pub fn has_compressed(&self) -> bool {
        self.compressed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compressed(&mut self, v: BoolValue) {
        self.compressed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_compressed(&mut self) -> &mut BoolValue {
        if self.compressed.is_none() {
            self.compressed.set_default();
        }
        self.compressed.as_mut().unwrap()
    }

    // Take field
    pub fn take_compressed(&mut self) -> BoolValue {
        self.compressed.take().unwrap_or_else(|| BoolValue::new())
    }

    pub fn get_compressed(&self) -> &BoolValue {
        self.compressed.as_ref().unwrap_or_else(|| BoolValue::default_instance())
    }

    fn get_compressed_for_reflect(&self) -> &::protobuf::SingularPtrField<BoolValue> {
        &self.compressed
    }

    fn mut_compressed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BoolValue> {
        &mut self.compressed
    }
}

impl ::protobuf::Message for ResponseParameters {
    fn is_initialized(&self) -> bool {
        for v in &self.compressed {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    self.size = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.interval_us = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.compressed)?;
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
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.interval_us != 0 {
            my_size += ::protobuf::rt::value_size(2, self.interval_us, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.compressed.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.size != 0 {
            os.write_int32(1, self.size)?;
        }
        if self.interval_us != 0 {
            os.write_int32(2, self.interval_us)?;
        }
        if let Some(ref v) = self.compressed.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ResponseParameters {
    fn new() -> ResponseParameters {
        ResponseParameters::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseParameters>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "size",
                    ResponseParameters::get_size_for_reflect,
                    ResponseParameters::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "interval_us",
                    ResponseParameters::get_interval_us_for_reflect,
                    ResponseParameters::mut_interval_us_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BoolValue>>(
                    "compressed",
                    ResponseParameters::get_compressed_for_reflect,
                    ResponseParameters::mut_compressed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseParameters>(
                    "ResponseParameters",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseParameters {
    fn clear(&mut self) {
        self.clear_size();
        self.clear_interval_us();
        self.clear_compressed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseParameters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseParameters {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamingOutputCallRequest {
    // message fields
    pub response_type: PayloadType,
    pub response_parameters: ::protobuf::RepeatedField<ResponseParameters>,
    pub payload: ::protobuf::SingularPtrField<Payload>,
    pub response_status: ::protobuf::SingularPtrField<EchoStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreamingOutputCallRequest {}

impl StreamingOutputCallRequest {
    pub fn new() -> StreamingOutputCallRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreamingOutputCallRequest {
        static mut instance: ::protobuf::lazy::Lazy<StreamingOutputCallRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreamingOutputCallRequest,
        };
        unsafe {
            instance.get(StreamingOutputCallRequest::new)
        }
    }

    // .grpc.testing.PayloadType response_type = 1;

    pub fn clear_response_type(&mut self) {
        self.response_type = PayloadType::COMPRESSABLE;
    }

    // Param is passed by value, moved
    pub fn set_response_type(&mut self, v: PayloadType) {
        self.response_type = v;
    }

    pub fn get_response_type(&self) -> PayloadType {
        self.response_type
    }

    fn get_response_type_for_reflect(&self) -> &PayloadType {
        &self.response_type
    }

    fn mut_response_type_for_reflect(&mut self) -> &mut PayloadType {
        &mut self.response_type
    }

    // repeated .grpc.testing.ResponseParameters response_parameters = 2;

    pub fn clear_response_parameters(&mut self) {
        self.response_parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_parameters(&mut self, v: ::protobuf::RepeatedField<ResponseParameters>) {
        self.response_parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_response_parameters(&mut self) -> &mut ::protobuf::RepeatedField<ResponseParameters> {
        &mut self.response_parameters
    }

    // Take field
    pub fn take_response_parameters(&mut self) -> ::protobuf::RepeatedField<ResponseParameters> {
        ::std::mem::replace(&mut self.response_parameters, ::protobuf::RepeatedField::new())
    }

    pub fn get_response_parameters(&self) -> &[ResponseParameters] {
        &self.response_parameters
    }

    fn get_response_parameters_for_reflect(&self) -> &::protobuf::RepeatedField<ResponseParameters> {
        &self.response_parameters
    }

    fn mut_response_parameters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ResponseParameters> {
        &mut self.response_parameters
    }

    // .grpc.testing.Payload payload = 3;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: Payload) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut Payload {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> Payload {
        self.payload.take().unwrap_or_else(|| Payload::new())
    }

    pub fn get_payload(&self) -> &Payload {
        self.payload.as_ref().unwrap_or_else(|| Payload::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<Payload> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Payload> {
        &mut self.payload
    }

    // .grpc.testing.EchoStatus response_status = 7;

    pub fn clear_response_status(&mut self) {
        self.response_status.clear();
    }

    pub fn has_response_status(&self) -> bool {
        self.response_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_status(&mut self, v: EchoStatus) {
        self.response_status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_status(&mut self) -> &mut EchoStatus {
        if self.response_status.is_none() {
            self.response_status.set_default();
        }
        self.response_status.as_mut().unwrap()
    }

    // Take field
    pub fn take_response_status(&mut self) -> EchoStatus {
        self.response_status.take().unwrap_or_else(|| EchoStatus::new())
    }

    pub fn get_response_status(&self) -> &EchoStatus {
        self.response_status.as_ref().unwrap_or_else(|| EchoStatus::default_instance())
    }

    fn get_response_status_for_reflect(&self) -> &::protobuf::SingularPtrField<EchoStatus> {
        &self.response_status
    }

    fn mut_response_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EchoStatus> {
        &mut self.response_status
    }
}

impl ::protobuf::Message for StreamingOutputCallRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.response_parameters {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.payload {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_status {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    self.response_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.response_parameters)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payload)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response_status)?;
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
        if self.response_type != PayloadType::COMPRESSABLE {
            my_size += ::protobuf::rt::enum_size(1, self.response_type);
        }
        for value in &self.response_parameters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.payload.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.response_status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.response_type != PayloadType::COMPRESSABLE {
            os.write_enum(1, self.response_type.value())?;
        }
        for v in &self.response_parameters {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.response_status.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for StreamingOutputCallRequest {
    fn new() -> StreamingOutputCallRequest {
        StreamingOutputCallRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreamingOutputCallRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PayloadType>>(
                    "response_type",
                    StreamingOutputCallRequest::get_response_type_for_reflect,
                    StreamingOutputCallRequest::mut_response_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseParameters>>(
                    "response_parameters",
                    StreamingOutputCallRequest::get_response_parameters_for_reflect,
                    StreamingOutputCallRequest::mut_response_parameters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Payload>>(
                    "payload",
                    StreamingOutputCallRequest::get_payload_for_reflect,
                    StreamingOutputCallRequest::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EchoStatus>>(
                    "response_status",
                    StreamingOutputCallRequest::get_response_status_for_reflect,
                    StreamingOutputCallRequest::mut_response_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreamingOutputCallRequest>(
                    "StreamingOutputCallRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreamingOutputCallRequest {
    fn clear(&mut self) {
        self.clear_response_type();
        self.clear_response_parameters();
        self.clear_payload();
        self.clear_response_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamingOutputCallRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamingOutputCallRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamingOutputCallResponse {
    // message fields
    pub payload: ::protobuf::SingularPtrField<Payload>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StreamingOutputCallResponse {}

impl StreamingOutputCallResponse {
    pub fn new() -> StreamingOutputCallResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StreamingOutputCallResponse {
        static mut instance: ::protobuf::lazy::Lazy<StreamingOutputCallResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StreamingOutputCallResponse,
        };
        unsafe {
            instance.get(StreamingOutputCallResponse::new)
        }
    }

    // .grpc.testing.Payload payload = 1;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: Payload) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut Payload {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> Payload {
        self.payload.take().unwrap_or_else(|| Payload::new())
    }

    pub fn get_payload(&self) -> &Payload {
        self.payload.as_ref().unwrap_or_else(|| Payload::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<Payload> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Payload> {
        &mut self.payload
    }
}

impl ::protobuf::Message for StreamingOutputCallResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.payload {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payload)?;
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
        if let Some(ref v) = self.payload.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for StreamingOutputCallResponse {
    fn new() -> StreamingOutputCallResponse {
        StreamingOutputCallResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StreamingOutputCallResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Payload>>(
                    "payload",
                    StreamingOutputCallResponse::get_payload_for_reflect,
                    StreamingOutputCallResponse::mut_payload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StreamingOutputCallResponse>(
                    "StreamingOutputCallResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StreamingOutputCallResponse {
    fn clear(&mut self) {
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamingOutputCallResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamingOutputCallResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReconnectParams {
    // message fields
    pub max_reconnect_backoff_ms: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReconnectParams {}

impl ReconnectParams {
    pub fn new() -> ReconnectParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReconnectParams {
        static mut instance: ::protobuf::lazy::Lazy<ReconnectParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReconnectParams,
        };
        unsafe {
            instance.get(ReconnectParams::new)
        }
    }

    // int32 max_reconnect_backoff_ms = 1;

    pub fn clear_max_reconnect_backoff_ms(&mut self) {
        self.max_reconnect_backoff_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_reconnect_backoff_ms(&mut self, v: i32) {
        self.max_reconnect_backoff_ms = v;
    }

    pub fn get_max_reconnect_backoff_ms(&self) -> i32 {
        self.max_reconnect_backoff_ms
    }

    fn get_max_reconnect_backoff_ms_for_reflect(&self) -> &i32 {
        &self.max_reconnect_backoff_ms
    }

    fn mut_max_reconnect_backoff_ms_for_reflect(&mut self) -> &mut i32 {
        &mut self.max_reconnect_backoff_ms
    }
}

impl ::protobuf::Message for ReconnectParams {
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
                    self.max_reconnect_backoff_ms = tmp;
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
        if self.max_reconnect_backoff_ms != 0 {
            my_size += ::protobuf::rt::value_size(1, self.max_reconnect_backoff_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.max_reconnect_backoff_ms != 0 {
            os.write_int32(1, self.max_reconnect_backoff_ms)?;
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

impl ::protobuf::MessageStatic for ReconnectParams {
    fn new() -> ReconnectParams {
        ReconnectParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReconnectParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_reconnect_backoff_ms",
                    ReconnectParams::get_max_reconnect_backoff_ms_for_reflect,
                    ReconnectParams::mut_max_reconnect_backoff_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReconnectParams>(
                    "ReconnectParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReconnectParams {
    fn clear(&mut self) {
        self.clear_max_reconnect_backoff_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReconnectParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReconnectParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReconnectInfo {
    // message fields
    pub passed: bool,
    pub backoff_ms: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReconnectInfo {}

impl ReconnectInfo {
    pub fn new() -> ReconnectInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReconnectInfo {
        static mut instance: ::protobuf::lazy::Lazy<ReconnectInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReconnectInfo,
        };
        unsafe {
            instance.get(ReconnectInfo::new)
        }
    }

    // bool passed = 1;

    pub fn clear_passed(&mut self) {
        self.passed = false;
    }

    // Param is passed by value, moved
    pub fn set_passed(&mut self, v: bool) {
        self.passed = v;
    }

    pub fn get_passed(&self) -> bool {
        self.passed
    }

    fn get_passed_for_reflect(&self) -> &bool {
        &self.passed
    }

    fn mut_passed_for_reflect(&mut self) -> &mut bool {
        &mut self.passed
    }

    // repeated int32 backoff_ms = 2;

    pub fn clear_backoff_ms(&mut self) {
        self.backoff_ms.clear();
    }

    // Param is passed by value, moved
    pub fn set_backoff_ms(&mut self, v: ::std::vec::Vec<i32>) {
        self.backoff_ms = v;
    }

    // Mutable pointer to the field.
    pub fn mut_backoff_ms(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.backoff_ms
    }

    // Take field
    pub fn take_backoff_ms(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.backoff_ms, ::std::vec::Vec::new())
    }

    pub fn get_backoff_ms(&self) -> &[i32] {
        &self.backoff_ms
    }

    fn get_backoff_ms_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.backoff_ms
    }

    fn mut_backoff_ms_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.backoff_ms
    }
}

impl ::protobuf::Message for ReconnectInfo {
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
                    let tmp = is.read_bool()?;
                    self.passed = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.backoff_ms)?;
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
        if self.passed != false {
            my_size += 2;
        }
        for value in &self.backoff_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.passed != false {
            os.write_bool(1, self.passed)?;
        }
        for v in &self.backoff_ms {
            os.write_int32(2, *v)?;
        };
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

impl ::protobuf::MessageStatic for ReconnectInfo {
    fn new() -> ReconnectInfo {
        ReconnectInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReconnectInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "passed",
                    ReconnectInfo::get_passed_for_reflect,
                    ReconnectInfo::mut_passed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "backoff_ms",
                    ReconnectInfo::get_backoff_ms_for_reflect,
                    ReconnectInfo::mut_backoff_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReconnectInfo>(
                    "ReconnectInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReconnectInfo {
    fn clear(&mut self) {
        self.clear_passed();
        self.clear_backoff_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReconnectInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReconnectInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PayloadType {
    COMPRESSABLE = 0,
}

impl ::protobuf::ProtobufEnum for PayloadType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PayloadType> {
        match value {
            0 => ::std::option::Option::Some(PayloadType::COMPRESSABLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PayloadType] = &[
            PayloadType::COMPRESSABLE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<PayloadType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PayloadType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PayloadType {
}

impl ::std::default::Default for PayloadType {
    fn default() -> Self {
        PayloadType::COMPRESSABLE
    }
}

impl ::protobuf::reflect::ProtobufValue for PayloadType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgrpc/testing/messages.proto\x12\x0cgrpc.testing\"!\n\tBoolValue\
    \x12\x14\n\x05value\x18\x01\x20\x01(\x08R\x05value\"L\n\x07Payload\x12-\
    \n\x04type\x18\x01\x20\x01(\x0e2\x19.grpc.testing.PayloadTypeR\x04type\
    \x12\x12\n\x04body\x18\x02\x20\x01(\x0cR\x04body\":\n\nEchoStatus\x12\
    \x12\n\x04code\x18\x01\x20\x01(\x05R\x04code\x12\x18\n\x07message\x18\
    \x02\x20\x01(\tR\x07message\"\xc7\x03\n\rSimpleRequest\x12>\n\rresponse_\
    type\x18\x01\x20\x01(\x0e2\x19.grpc.testing.PayloadTypeR\x0cresponseType\
    \x12#\n\rresponse_size\x18\x02\x20\x01(\x05R\x0cresponseSize\x12/\n\x07p\
    ayload\x18\x03\x20\x01(\x0b2\x15.grpc.testing.PayloadR\x07payload\x12#\n\
    \rfill_username\x18\x04\x20\x01(\x08R\x0cfillUsername\x12(\n\x10fill_oau\
    th_scope\x18\x05\x20\x01(\x08R\x0efillOauthScope\x12H\n\x13response_comp\
    ressed\x18\x06\x20\x01(\x0b2\x17.grpc.testing.BoolValueR\x12responseComp\
    ressed\x12A\n\x0fresponse_status\x18\x07\x20\x01(\x0b2\x18.grpc.testing.\
    EchoStatusR\x0eresponseStatus\x12D\n\x11expect_compressed\x18\x08\x20\
    \x01(\x0b2\x17.grpc.testing.BoolValueR\x10expectCompressed\"~\n\x0eSimpl\
    eResponse\x12/\n\x07payload\x18\x01\x20\x01(\x0b2\x15.grpc.testing.Paylo\
    adR\x07payload\x12\x1a\n\x08username\x18\x02\x20\x01(\tR\x08username\x12\
    \x1f\n\x0boauth_scope\x18\x03\x20\x01(\tR\noauthScope\"\x92\x01\n\x19Str\
    eamingInputCallRequest\x12/\n\x07payload\x18\x01\x20\x01(\x0b2\x15.grpc.\
    testing.PayloadR\x07payload\x12D\n\x11expect_compressed\x18\x02\x20\x01(\
    \x0b2\x17.grpc.testing.BoolValueR\x10expectCompressed\"T\n\x1aStreamingI\
    nputCallResponse\x126\n\x17aggregated_payload_size\x18\x01\x20\x01(\x05R\
    \x15aggregatedPayloadSize\"\x82\x01\n\x12ResponseParameters\x12\x12\n\
    \x04size\x18\x01\x20\x01(\x05R\x04size\x12\x1f\n\x0binterval_us\x18\x02\
    \x20\x01(\x05R\nintervalUs\x127\n\ncompressed\x18\x03\x20\x01(\x0b2\x17.\
    grpc.testing.BoolValueR\ncompressed\"\xa3\x02\n\x1aStreamingOutputCallRe\
    quest\x12>\n\rresponse_type\x18\x01\x20\x01(\x0e2\x19.grpc.testing.Paylo\
    adTypeR\x0cresponseType\x12Q\n\x13response_parameters\x18\x02\x20\x03(\
    \x0b2\x20.grpc.testing.ResponseParametersR\x12responseParameters\x12/\n\
    \x07payload\x18\x03\x20\x01(\x0b2\x15.grpc.testing.PayloadR\x07payload\
    \x12A\n\x0fresponse_status\x18\x07\x20\x01(\x0b2\x18.grpc.testing.EchoSt\
    atusR\x0eresponseStatus\"N\n\x1bStreamingOutputCallResponse\x12/\n\x07pa\
    yload\x18\x01\x20\x01(\x0b2\x15.grpc.testing.PayloadR\x07payload\"J\n\
    \x0fReconnectParams\x127\n\x18max_reconnect_backoff_ms\x18\x01\x20\x01(\
    \x05R\x15maxReconnectBackoffMs\"F\n\rReconnectInfo\x12\x16\n\x06passed\
    \x18\x01\x20\x01(\x08R\x06passed\x12\x1d\n\nbackoff_ms\x18\x02\x20\x03(\
    \x05R\tbackoffMs*\x1f\n\x0bPayloadType\x12\x10\n\x0cCOMPRESSABLE\x10\0J\
    \xcd2\n\x07\x12\x05\x11\0\xa8\x01\x01\n\x8f\x05\n\x01\x0c\x12\x03\x11\0\
    \x122\xb9\x04\x20Copyright\x202015-2016\x20gRPC\x20authors.\n\n\x20Licen\
    sed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\
    \"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\
    \x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20obtai\
    n\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http\
    ://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\
    \x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20softwar\
    e\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\x20on\
    \x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CON\
    DITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\
    \x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20gover\
    ning\x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\
    \n2I\x20Message\x20definitions\x20to\x20be\x20used\x20by\x20integration\
    \x20test\x20service\x20definitions.\n\n\x08\n\x01\x02\x12\x03\x13\x08\
    \x14\n\xa4\x01\n\x02\x04\0\x12\x04\x18\0\x1b\x01\x1a\x97\x01\x20TODO(dgq\
    ):\x20Go\x20back\x20to\x20using\x20well-known\x20types\x20once\n\x20http\
    s://github.com/grpc/grpc/issues/6980\x20has\x20been\x20fixed.\n\x20impor\
    t\x20\"google/protobuf/wrappers.proto\";\n\n\n\n\x03\x04\0\x01\x12\x03\
    \x18\x08\x11\n\x1e\n\x04\x04\0\x02\0\x12\x03\x1a\x02\x11\x1a\x11\x20The\
    \x20bool\x20value.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x1a\x02\x18\x13\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1a\x02\x06\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x1a\x07\x0c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1a\x0f\x10\
    \ni\n\x02\x05\0\x12\x04\x1f\0\"\x01\x1a]\x20DEPRECATED,\x20don't\x20use.\
    \x20To\x20be\x20removed\x20shortly.\n\x20The\x20type\x20of\x20payload\
    \x20that\x20should\x20be\x20returned.\n\n\n\n\x03\x05\0\x01\x12\x03\x1f\
    \x05\x10\n(\n\x04\x05\0\x02\0\x12\x03!\x02\x13\x1a\x1b\x20Compressable\
    \x20text\x20format.\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03!\x02\x0e\n\x0c\
    \n\x05\x05\0\x02\0\x02\x12\x03!\x11\x12\nD\n\x02\x04\x01\x12\x04%\0+\x01\
    \x1a8\x20A\x20block\x20of\x20data,\x20to\x20simply\x20increase\x20gRPC\
    \x20message\x20size.\n\n\n\n\x03\x04\x01\x01\x12\x03%\x08\x0f\nW\n\x04\
    \x04\x01\x02\0\x12\x03(\x02\x17\x1aJ\x20DEPRECATED,\x20don't\x20use.\x20\
    To\x20be\x20removed\x20shortly.\n\x20The\x20type\x20of\x20data\x20in\x20\
    body.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04(\x02%\x11\n\x0c\n\x05\x04\
    \x01\x02\0\x06\x12\x03(\x02\r\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03(\x0e\
    \x12\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03(\x15\x16\n+\n\x04\x04\x01\x02\
    \x01\x12\x03*\x02\x11\x1a\x1e\x20Primary\x20contents\x20of\x20payload.\n\
    \n\r\n\x05\x04\x01\x02\x01\x04\x12\x04*\x02(\x17\n\x0c\n\x05\x04\x01\x02\
    \x01\x05\x12\x03*\x02\x07\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03*\x08\
    \x0c\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03*\x0f\x10\n\x95\x01\n\x02\
    \x04\x02\x12\x04/\02\x01\x1a\x88\x01\x20A\x20protobuf\x20representation\
    \x20for\x20grpc\x20status.\x20This\x20is\x20used\x20by\x20test\n\x20clie\
    nts\x20to\x20specify\x20a\x20status\x20that\x20the\x20server\x20should\
    \x20attempt\x20to\x20return.\n\n\n\n\x03\x04\x02\x01\x12\x03/\x08\x12\n\
    \x0b\n\x04\x04\x02\x02\0\x12\x030\x02\x11\n\r\n\x05\x04\x02\x02\0\x04\
    \x12\x040\x02/\x14\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x030\x02\x07\n\x0c\
    \n\x05\x04\x02\x02\0\x01\x12\x030\x08\x0c\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x030\x0f\x10\n\x0b\n\x04\x04\x02\x02\x01\x12\x031\x02\x15\n\r\n\x05\
    \x04\x02\x02\x01\x04\x12\x041\x020\x11\n\x0c\n\x05\x04\x02\x02\x01\x05\
    \x12\x031\x02\x08\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x031\t\x10\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x031\x13\x14\n\x1c\n\x02\x04\x03\x12\x045\0\
    R\x01\x1a\x10\x20Unary\x20request.\n\n\n\n\x03\x04\x03\x01\x12\x035\x08\
    \x15\n\xc1\x01\n\x04\x04\x03\x02\0\x12\x039\x02\x20\x1a\xb3\x01\x20DEPRE\
    CATED,\x20don't\x20use.\x20To\x20be\x20removed\x20shortly.\n\x20Desired\
    \x20payload\x20type\x20in\x20the\x20response\x20from\x20the\x20server.\n\
    \x20If\x20response_type\x20is\x20RANDOM,\x20server\x20randomly\x20choose\
    s\x20one\x20from\x20other\x20formats.\n\n\r\n\x05\x04\x03\x02\0\x04\x12\
    \x049\x025\x17\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x039\x02\r\n\x0c\n\x05\
    \x04\x03\x02\0\x01\x12\x039\x0e\x1b\n\x0c\n\x05\x04\x03\x02\0\x03\x12\
    \x039\x1e\x1f\nD\n\x04\x04\x03\x02\x01\x12\x03<\x02\x1a\x1a7\x20Desired\
    \x20payload\x20size\x20in\x20the\x20response\x20from\x20the\x20server.\n\
    \n\r\n\x05\x04\x03\x02\x01\x04\x12\x04<\x029\x20\n\x0c\n\x05\x04\x03\x02\
    \x01\x05\x12\x03<\x02\x07\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03<\x08\
    \x15\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03<\x18\x19\nB\n\x04\x04\x03\
    \x02\x02\x12\x03?\x02\x16\x1a5\x20Optional\x20input\x20payload\x20sent\
    \x20along\x20with\x20the\x20request.\n\n\r\n\x05\x04\x03\x02\x02\x04\x12\
    \x04?\x02<\x1a\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\x03?\x02\t\n\x0c\n\
    \x05\x04\x03\x02\x02\x01\x12\x03?\n\x11\n\x0c\n\x05\x04\x03\x02\x02\x03\
    \x12\x03?\x14\x15\n>\n\x04\x04\x03\x02\x03\x12\x03B\x02\x19\x1a1\x20Whet\
    her\x20SimpleResponse\x20should\x20include\x20username.\n\n\r\n\x05\x04\
    \x03\x02\x03\x04\x12\x04B\x02?\x16\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\
    \x03B\x02\x06\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03B\x07\x14\n\x0c\n\
    \x05\x04\x03\x02\x03\x03\x12\x03B\x17\x18\nA\n\x04\x04\x03\x02\x04\x12\
    \x03E\x02\x1c\x1a4\x20Whether\x20SimpleResponse\x20should\x20include\x20\
    OAuth\x20scope.\n\n\r\n\x05\x04\x03\x02\x04\x04\x12\x04E\x02B\x19\n\x0c\
    \n\x05\x04\x03\x02\x04\x05\x12\x03E\x02\x06\n\x0c\n\x05\x04\x03\x02\x04\
    \x01\x12\x03E\x07\x17\n\x0c\n\x05\x04\x03\x02\x04\x03\x12\x03E\x1a\x1b\n\
    \x8c\x02\n\x04\x04\x03\x02\x05\x12\x03K\x02$\x1a\xfe\x01\x20Whether\x20t\
    o\x20request\x20the\x20server\x20to\x20compress\x20the\x20response.\x20T\
    his\x20field\x20is\n\x20\"nullable\"\x20in\x20order\x20to\x20interoperat\
    e\x20seamlessly\x20with\x20clients\x20not\x20able\x20to\n\x20implement\
    \x20the\x20full\x20compression\x20tests\x20by\x20introspecting\x20the\
    \x20call\x20to\x20verify\n\x20the\x20response's\x20compression\x20status\
    .\n\n\r\n\x05\x04\x03\x02\x05\x04\x12\x04K\x02E\x1c\n\x0c\n\x05\x04\x03\
    \x02\x05\x06\x12\x03K\x02\x0b\n\x0c\n\x05\x04\x03\x02\x05\x01\x12\x03K\
    \x0c\x1f\n\x0c\n\x05\x04\x03\x02\x05\x03\x12\x03K\"#\n:\n\x04\x04\x03\
    \x02\x06\x12\x03N\x02!\x1a-\x20Whether\x20server\x20should\x20return\x20\
    a\x20given\x20status\n\n\r\n\x05\x04\x03\x02\x06\x04\x12\x04N\x02K$\n\
    \x0c\n\x05\x04\x03\x02\x06\x06\x12\x03N\x02\x0c\n\x0c\n\x05\x04\x03\x02\
    \x06\x01\x12\x03N\r\x1c\n\x0c\n\x05\x04\x03\x02\x06\x03\x12\x03N\x1f\x20\
    \nN\n\x04\x04\x03\x02\x07\x12\x03Q\x02\"\x1aA\x20Whether\x20the\x20serve\
    r\x20should\x20expect\x20this\x20request\x20to\x20be\x20compressed.\n\n\
    \r\n\x05\x04\x03\x02\x07\x04\x12\x04Q\x02N!\n\x0c\n\x05\x04\x03\x02\x07\
    \x06\x12\x03Q\x02\x0b\n\x0c\n\x05\x04\x03\x02\x07\x01\x12\x03Q\x0c\x1d\n\
    \x0c\n\x05\x04\x03\x02\x07\x03\x12\x03Q\x20!\n;\n\x02\x04\x04\x12\x04U\0\
    ]\x01\x1a/\x20Unary\x20response,\x20as\x20configured\x20by\x20the\x20req\
    uest.\n\n\n\n\x03\x04\x04\x01\x12\x03U\x08\x16\n0\n\x04\x04\x04\x02\0\
    \x12\x03W\x02\x16\x1a#\x20Payload\x20to\x20increase\x20message\x20size.\
    \n\n\r\n\x05\x04\x04\x02\0\x04\x12\x04W\x02U\x18\n\x0c\n\x05\x04\x04\x02\
    \0\x06\x12\x03W\x02\t\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03W\n\x11\n\x0c\
    \n\x05\x04\x04\x02\0\x03\x12\x03W\x14\x15\nx\n\x04\x04\x04\x02\x01\x12\
    \x03Z\x02\x16\x1ak\x20The\x20user\x20the\x20request\x20came\x20from,\x20\
    for\x20verifying\x20authentication\x20was\n\x20successful\x20when\x20the\
    \x20client\x20expected\x20it.\n\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04Z\
    \x02W\x16\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03Z\x02\x08\n\x0c\n\x05\
    \x04\x04\x02\x01\x01\x12\x03Z\t\x11\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\
    \x03Z\x14\x15\n\x1b\n\x04\x04\x04\x02\x02\x12\x03\\\x02\x19\x1a\x0e\x20O\
    Auth\x20scope.\n\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04\\\x02Z\x16\n\x0c\
    \n\x05\x04\x04\x02\x02\x05\x12\x03\\\x02\x08\n\x0c\n\x05\x04\x04\x02\x02\
    \x01\x12\x03\\\t\x14\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03\\\x17\x18\n\
    '\n\x02\x04\x05\x12\x04`\0k\x01\x1a\x1b\x20Client-streaming\x20request.\
    \n\n\n\n\x03\x04\x05\x01\x12\x03`\x08!\nB\n\x04\x04\x05\x02\0\x12\x03b\
    \x02\x16\x1a5\x20Optional\x20input\x20payload\x20sent\x20along\x20with\
    \x20the\x20request.\n\n\r\n\x05\x04\x05\x02\0\x04\x12\x04b\x02`#\n\x0c\n\
    \x05\x04\x05\x02\0\x06\x12\x03b\x02\t\n\x0c\n\x05\x04\x05\x02\0\x01\x12\
    \x03b\n\x11\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03b\x14\x15\n\x93\x02\n\
    \x04\x04\x05\x02\x01\x12\x03h\x02\"\x1a\x85\x02\x20Whether\x20the\x20ser\
    ver\x20should\x20expect\x20this\x20request\x20to\x20be\x20compressed.\
    \x20This\x20field\n\x20is\x20\"nullable\"\x20in\x20order\x20to\x20intero\
    perate\x20seamlessly\x20with\x20servers\x20not\x20able\x20to\n\x20implem\
    ent\x20the\x20full\x20compression\x20tests\x20by\x20introspecting\x20the\
    \x20call\x20to\x20verify\n\x20the\x20request's\x20compression\x20status.\
    \n\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04h\x02b\x16\n\x0c\n\x05\x04\x05\
    \x02\x01\x06\x12\x03h\x02\x0b\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03h\
    \x0c\x1d\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03h\x20!\n(\n\x02\x04\x06\
    \x12\x04n\0q\x01\x1a\x1c\x20Client-streaming\x20response.\n\n\n\n\x03\
    \x04\x06\x01\x12\x03n\x08\"\nD\n\x04\x04\x06\x02\0\x12\x03p\x02$\x1a7\
    \x20Aggregated\x20size\x20of\x20payloads\x20received\x20from\x20the\x20c\
    lient.\n\n\r\n\x05\x04\x06\x02\0\x04\x12\x04p\x02n$\n\x0c\n\x05\x04\x06\
    \x02\0\x05\x12\x03p\x02\x07\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03p\x08\
    \x1f\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03p\"#\n7\n\x02\x04\x07\x12\x05t\
    \0\x81\x01\x01\x1a*\x20Configuration\x20for\x20a\x20particular\x20respon\
    se.\n\n\n\n\x03\x04\x07\x01\x12\x03t\x08\x1a\nB\n\x04\x04\x07\x02\0\x12\
    \x03v\x02\x11\x1a5\x20Desired\x20payload\x20sizes\x20in\x20responses\x20\
    from\x20the\x20server.\n\n\r\n\x05\x04\x07\x02\0\x04\x12\x04v\x02t\x1c\n\
    \x0c\n\x05\x04\x07\x02\0\x05\x12\x03v\x02\x07\n\x0c\n\x05\x04\x07\x02\0\
    \x01\x12\x03v\x08\x0c\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03v\x0f\x10\nf\
    \n\x04\x04\x07\x02\x01\x12\x03z\x02\x18\x1aY\x20Desired\x20interval\x20b\
    etween\x20consecutive\x20responses\x20in\x20the\x20response\x20stream\
    \x20in\n\x20microseconds.\n\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04z\x02v\
    \x11\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x03z\x02\x07\n\x0c\n\x05\x04\
    \x07\x02\x01\x01\x12\x03z\x08\x13\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\
    \x03z\x16\x17\n\x8d\x02\n\x04\x04\x07\x02\x02\x12\x04\x80\x01\x02\x1b\
    \x1a\xfe\x01\x20Whether\x20to\x20request\x20the\x20server\x20to\x20compr\
    ess\x20the\x20response.\x20This\x20field\x20is\n\x20\"nullable\"\x20in\
    \x20order\x20to\x20interoperate\x20seamlessly\x20with\x20clients\x20not\
    \x20able\x20to\n\x20implement\x20the\x20full\x20compression\x20tests\x20\
    by\x20introspecting\x20the\x20call\x20to\x20verify\n\x20the\x20response'\
    s\x20compression\x20status.\n\n\x0e\n\x05\x04\x07\x02\x02\x04\x12\x05\
    \x80\x01\x02z\x18\n\r\n\x05\x04\x07\x02\x02\x06\x12\x04\x80\x01\x02\x0b\
    \n\r\n\x05\x04\x07\x02\x02\x01\x12\x04\x80\x01\x0c\x16\n\r\n\x05\x04\x07\
    \x02\x02\x03\x12\x04\x80\x01\x19\x1a\n)\n\x02\x04\x08\x12\x06\x84\x01\0\
    \x94\x01\x01\x1a\x1b\x20Server-streaming\x20request.\n\n\x0b\n\x03\x04\
    \x08\x01\x12\x04\x84\x01\x08\"\n\x92\x02\n\x04\x04\x08\x02\0\x12\x04\x8a\
    \x01\x02\x20\x1a\x83\x02\x20DEPRECATED,\x20don't\x20use.\x20To\x20be\x20\
    removed\x20shortly.\n\x20Desired\x20payload\x20type\x20in\x20the\x20resp\
    onse\x20from\x20the\x20server.\n\x20If\x20response_type\x20is\x20RANDOM,\
    \x20the\x20payload\x20from\x20each\x20response\x20in\x20the\x20stream\n\
    \x20might\x20be\x20of\x20different\x20types.\x20This\x20is\x20to\x20simu\
    late\x20a\x20mixed\x20type\x20of\x20payload\n\x20stream.\n\n\x0f\n\x05\
    \x04\x08\x02\0\x04\x12\x06\x8a\x01\x02\x84\x01$\n\r\n\x05\x04\x08\x02\0\
    \x06\x12\x04\x8a\x01\x02\r\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\x8a\x01\
    \x0e\x1b\n\r\n\x05\x04\x08\x02\0\x03\x12\x04\x8a\x01\x1e\x1f\nA\n\x04\
    \x04\x08\x02\x01\x12\x04\x8d\x01\x026\x1a3\x20Configuration\x20for\x20ea\
    ch\x20expected\x20response\x20message.\n\n\r\n\x05\x04\x08\x02\x01\x04\
    \x12\x04\x8d\x01\x02\n\n\r\n\x05\x04\x08\x02\x01\x06\x12\x04\x8d\x01\x0b\
    \x1d\n\r\n\x05\x04\x08\x02\x01\x01\x12\x04\x8d\x01\x1e1\n\r\n\x05\x04\
    \x08\x02\x01\x03\x12\x04\x8d\x0145\nC\n\x04\x04\x08\x02\x02\x12\x04\x90\
    \x01\x02\x16\x1a5\x20Optional\x20input\x20payload\x20sent\x20along\x20wi\
    th\x20the\x20request.\n\n\x0f\n\x05\x04\x08\x02\x02\x04\x12\x06\x90\x01\
    \x02\x8d\x016\n\r\n\x05\x04\x08\x02\x02\x06\x12\x04\x90\x01\x02\t\n\r\n\
    \x05\x04\x08\x02\x02\x01\x12\x04\x90\x01\n\x11\n\r\n\x05\x04\x08\x02\x02\
    \x03\x12\x04\x90\x01\x14\x15\n;\n\x04\x04\x08\x02\x03\x12\x04\x93\x01\
    \x02!\x1a-\x20Whether\x20server\x20should\x20return\x20a\x20given\x20sta\
    tus\n\n\x0f\n\x05\x04\x08\x02\x03\x04\x12\x06\x93\x01\x02\x90\x01\x16\n\
    \r\n\x05\x04\x08\x02\x03\x06\x12\x04\x93\x01\x02\x0c\n\r\n\x05\x04\x08\
    \x02\x03\x01\x12\x04\x93\x01\r\x1c\n\r\n\x05\x04\x08\x02\x03\x03\x12\x04\
    \x93\x01\x1f\x20\nW\n\x02\x04\t\x12\x06\x97\x01\0\x9a\x01\x01\x1aI\x20Se\
    rver-streaming\x20response,\x20as\x20configured\x20by\x20the\x20request\
    \x20and\x20parameters.\n\n\x0b\n\x03\x04\t\x01\x12\x04\x97\x01\x08#\n2\n\
    \x04\x04\t\x02\0\x12\x04\x99\x01\x02\x16\x1a$\x20Payload\x20to\x20increa\
    se\x20response\x20size.\n\n\x0f\n\x05\x04\t\x02\0\x04\x12\x06\x99\x01\
    \x02\x97\x01%\n\r\n\x05\x04\t\x02\0\x06\x12\x04\x99\x01\x02\t\n\r\n\x05\
    \x04\t\x02\0\x01\x12\x04\x99\x01\n\x11\n\r\n\x05\x04\t\x02\0\x03\x12\x04\
    \x99\x01\x14\x15\nk\n\x02\x04\n\x12\x06\x9e\x01\0\xa0\x01\x01\x1a]\x20Fo\
    r\x20reconnect\x20interop\x20test\x20only.\n\x20Client\x20tells\x20serve\
    r\x20what\x20reconnection\x20parameters\x20it\x20used.\n\n\x0b\n\x03\x04\
    \n\x01\x12\x04\x9e\x01\x08\x17\n\x0c\n\x04\x04\n\x02\0\x12\x04\x9f\x01\
    \x02%\n\x0f\n\x05\x04\n\x02\0\x04\x12\x06\x9f\x01\x02\x9e\x01\x19\n\r\n\
    \x05\x04\n\x02\0\x05\x12\x04\x9f\x01\x02\x07\n\r\n\x05\x04\n\x02\0\x01\
    \x12\x04\x9f\x01\x08\x20\n\r\n\x05\x04\n\x02\0\x03\x12\x04\x9f\x01#$\n\
    \x98\x01\n\x02\x04\x0b\x12\x06\xa5\x01\0\xa8\x01\x01\x1a\x89\x01\x20For\
    \x20reconnect\x20interop\x20test\x20only.\n\x20Server\x20tells\x20client\
    \x20whether\x20its\x20reconnects\x20are\x20following\x20the\x20spec\x20a\
    nd\x20the\n\x20reconnect\x20backoffs\x20it\x20saw.\n\n\x0b\n\x03\x04\x0b\
    \x01\x12\x04\xa5\x01\x08\x15\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\xa6\x01\
    \x02\x12\n\x0f\n\x05\x04\x0b\x02\0\x04\x12\x06\xa6\x01\x02\xa5\x01\x17\n\
    \r\n\x05\x04\x0b\x02\0\x05\x12\x04\xa6\x01\x02\x06\n\r\n\x05\x04\x0b\x02\
    \0\x01\x12\x04\xa6\x01\x07\r\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\xa6\x01\
    \x10\x11\n\x0c\n\x04\x04\x0b\x02\x01\x12\x04\xa7\x01\x02\x20\n\r\n\x05\
    \x04\x0b\x02\x01\x04\x12\x04\xa7\x01\x02\n\n\r\n\x05\x04\x0b\x02\x01\x05\
    \x12\x04\xa7\x01\x0b\x10\n\r\n\x05\x04\x0b\x02\x01\x01\x12\x04\xa7\x01\
    \x11\x1b\n\r\n\x05\x04\x0b\x02\x01\x03\x12\x04\xa7\x01\x1e\x1fb\x06proto\
    3\
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
