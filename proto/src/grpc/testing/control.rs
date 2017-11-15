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
pub struct PoissonParams {
    // message fields
    pub offered_load: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoissonParams {}

impl PoissonParams {
    pub fn new() -> PoissonParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoissonParams {
        static mut instance: ::protobuf::lazy::Lazy<PoissonParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoissonParams,
        };
        unsafe {
            instance.get(PoissonParams::new)
        }
    }

    // double offered_load = 1;

    pub fn clear_offered_load(&mut self) {
        self.offered_load = 0.;
    }

    // Param is passed by value, moved
    pub fn set_offered_load(&mut self, v: f64) {
        self.offered_load = v;
    }

    pub fn get_offered_load(&self) -> f64 {
        self.offered_load
    }

    fn get_offered_load_for_reflect(&self) -> &f64 {
        &self.offered_load
    }

    fn mut_offered_load_for_reflect(&mut self) -> &mut f64 {
        &mut self.offered_load
    }
}

impl ::protobuf::Message for PoissonParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.offered_load = tmp;
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
        if self.offered_load != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.offered_load != 0. {
            os.write_double(1, self.offered_load)?;
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

impl ::protobuf::MessageStatic for PoissonParams {
    fn new() -> PoissonParams {
        PoissonParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoissonParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "offered_load",
                    PoissonParams::get_offered_load_for_reflect,
                    PoissonParams::mut_offered_load_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoissonParams>(
                    "PoissonParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoissonParams {
    fn clear(&mut self) {
        self.clear_offered_load();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoissonParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoissonParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClosedLoopParams {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClosedLoopParams {}

impl ClosedLoopParams {
    pub fn new() -> ClosedLoopParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClosedLoopParams {
        static mut instance: ::protobuf::lazy::Lazy<ClosedLoopParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClosedLoopParams,
        };
        unsafe {
            instance.get(ClosedLoopParams::new)
        }
    }
}

impl ::protobuf::Message for ClosedLoopParams {
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

impl ::protobuf::MessageStatic for ClosedLoopParams {
    fn new() -> ClosedLoopParams {
        ClosedLoopParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClosedLoopParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ClosedLoopParams>(
                    "ClosedLoopParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClosedLoopParams {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClosedLoopParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClosedLoopParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoadParams {
    // message oneof groups
    load: ::std::option::Option<LoadParams_oneof_load>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoadParams {}

#[derive(Clone,PartialEq)]
pub enum LoadParams_oneof_load {
    closed_loop(ClosedLoopParams),
    poisson(PoissonParams),
}

impl LoadParams {
    pub fn new() -> LoadParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoadParams {
        static mut instance: ::protobuf::lazy::Lazy<LoadParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoadParams,
        };
        unsafe {
            instance.get(LoadParams::new)
        }
    }

    // .grpc.testing.ClosedLoopParams closed_loop = 1;

    pub fn clear_closed_loop(&mut self) {
        self.load = ::std::option::Option::None;
    }

    pub fn has_closed_loop(&self) -> bool {
        match self.load {
            ::std::option::Option::Some(LoadParams_oneof_load::closed_loop(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_closed_loop(&mut self, v: ClosedLoopParams) {
        self.load = ::std::option::Option::Some(LoadParams_oneof_load::closed_loop(v))
    }

    // Mutable pointer to the field.
    pub fn mut_closed_loop(&mut self) -> &mut ClosedLoopParams {
        if let ::std::option::Option::Some(LoadParams_oneof_load::closed_loop(_)) = self.load {
        } else {
            self.load = ::std::option::Option::Some(LoadParams_oneof_load::closed_loop(ClosedLoopParams::new()));
        }
        match self.load {
            ::std::option::Option::Some(LoadParams_oneof_load::closed_loop(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_closed_loop(&mut self) -> ClosedLoopParams {
        if self.has_closed_loop() {
            match self.load.take() {
                ::std::option::Option::Some(LoadParams_oneof_load::closed_loop(v)) => v,
                _ => panic!(),
            }
        } else {
            ClosedLoopParams::new()
        }
    }

    pub fn get_closed_loop(&self) -> &ClosedLoopParams {
        match self.load {
            ::std::option::Option::Some(LoadParams_oneof_load::closed_loop(ref v)) => v,
            _ => ClosedLoopParams::default_instance(),
        }
    }

    // .grpc.testing.PoissonParams poisson = 2;

    pub fn clear_poisson(&mut self) {
        self.load = ::std::option::Option::None;
    }

    pub fn has_poisson(&self) -> bool {
        match self.load {
            ::std::option::Option::Some(LoadParams_oneof_load::poisson(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_poisson(&mut self, v: PoissonParams) {
        self.load = ::std::option::Option::Some(LoadParams_oneof_load::poisson(v))
    }

    // Mutable pointer to the field.
    pub fn mut_poisson(&mut self) -> &mut PoissonParams {
        if let ::std::option::Option::Some(LoadParams_oneof_load::poisson(_)) = self.load {
        } else {
            self.load = ::std::option::Option::Some(LoadParams_oneof_load::poisson(PoissonParams::new()));
        }
        match self.load {
            ::std::option::Option::Some(LoadParams_oneof_load::poisson(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_poisson(&mut self) -> PoissonParams {
        if self.has_poisson() {
            match self.load.take() {
                ::std::option::Option::Some(LoadParams_oneof_load::poisson(v)) => v,
                _ => panic!(),
            }
        } else {
            PoissonParams::new()
        }
    }

    pub fn get_poisson(&self) -> &PoissonParams {
        match self.load {
            ::std::option::Option::Some(LoadParams_oneof_load::poisson(ref v)) => v,
            _ => PoissonParams::default_instance(),
        }
    }
}

impl ::protobuf::Message for LoadParams {
    fn is_initialized(&self) -> bool {
        if let Some(LoadParams_oneof_load::closed_loop(ref v)) = self.load {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(LoadParams_oneof_load::poisson(ref v)) = self.load {
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
                    self.load = ::std::option::Option::Some(LoadParams_oneof_load::closed_loop(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.load = ::std::option::Option::Some(LoadParams_oneof_load::poisson(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.load {
            match v {
                &LoadParams_oneof_load::closed_loop(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &LoadParams_oneof_load::poisson(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.load {
            match v {
                &LoadParams_oneof_load::closed_loop(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &LoadParams_oneof_load::poisson(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LoadParams {
    fn new() -> LoadParams {
        LoadParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoadParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ClosedLoopParams>(
                    "closed_loop",
                    LoadParams::has_closed_loop,
                    LoadParams::get_closed_loop,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PoissonParams>(
                    "poisson",
                    LoadParams::has_poisson,
                    LoadParams::get_poisson,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoadParams>(
                    "LoadParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoadParams {
    fn clear(&mut self) {
        self.clear_closed_loop();
        self.clear_poisson();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoadParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoadParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SecurityParams {
    // message fields
    pub use_test_ca: bool,
    pub server_host_override: ::std::string::String,
    pub cred_type: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SecurityParams {}

impl SecurityParams {
    pub fn new() -> SecurityParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SecurityParams {
        static mut instance: ::protobuf::lazy::Lazy<SecurityParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SecurityParams,
        };
        unsafe {
            instance.get(SecurityParams::new)
        }
    }

    // bool use_test_ca = 1;

    pub fn clear_use_test_ca(&mut self) {
        self.use_test_ca = false;
    }

    // Param is passed by value, moved
    pub fn set_use_test_ca(&mut self, v: bool) {
        self.use_test_ca = v;
    }

    pub fn get_use_test_ca(&self) -> bool {
        self.use_test_ca
    }

    fn get_use_test_ca_for_reflect(&self) -> &bool {
        &self.use_test_ca
    }

    fn mut_use_test_ca_for_reflect(&mut self) -> &mut bool {
        &mut self.use_test_ca
    }

    // string server_host_override = 2;

    pub fn clear_server_host_override(&mut self) {
        self.server_host_override.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_host_override(&mut self, v: ::std::string::String) {
        self.server_host_override = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_host_override(&mut self) -> &mut ::std::string::String {
        &mut self.server_host_override
    }

    // Take field
    pub fn take_server_host_override(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.server_host_override, ::std::string::String::new())
    }

    pub fn get_server_host_override(&self) -> &str {
        &self.server_host_override
    }

    fn get_server_host_override_for_reflect(&self) -> &::std::string::String {
        &self.server_host_override
    }

    fn mut_server_host_override_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.server_host_override
    }

    // string cred_type = 3;

    pub fn clear_cred_type(&mut self) {
        self.cred_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_cred_type(&mut self, v: ::std::string::String) {
        self.cred_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cred_type(&mut self) -> &mut ::std::string::String {
        &mut self.cred_type
    }

    // Take field
    pub fn take_cred_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cred_type, ::std::string::String::new())
    }

    pub fn get_cred_type(&self) -> &str {
        &self.cred_type
    }

    fn get_cred_type_for_reflect(&self) -> &::std::string::String {
        &self.cred_type
    }

    fn mut_cred_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cred_type
    }
}

impl ::protobuf::Message for SecurityParams {
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
                    self.use_test_ca = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.server_host_override)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cred_type)?;
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
        if self.use_test_ca != false {
            my_size += 2;
        }
        if !self.server_host_override.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.server_host_override);
        }
        if !self.cred_type.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.cred_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.use_test_ca != false {
            os.write_bool(1, self.use_test_ca)?;
        }
        if !self.server_host_override.is_empty() {
            os.write_string(2, &self.server_host_override)?;
        }
        if !self.cred_type.is_empty() {
            os.write_string(3, &self.cred_type)?;
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

impl ::protobuf::MessageStatic for SecurityParams {
    fn new() -> SecurityParams {
        SecurityParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<SecurityParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "use_test_ca",
                    SecurityParams::get_use_test_ca_for_reflect,
                    SecurityParams::mut_use_test_ca_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "server_host_override",
                    SecurityParams::get_server_host_override_for_reflect,
                    SecurityParams::mut_server_host_override_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cred_type",
                    SecurityParams::get_cred_type_for_reflect,
                    SecurityParams::mut_cred_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SecurityParams>(
                    "SecurityParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SecurityParams {
    fn clear(&mut self) {
        self.clear_use_test_ca();
        self.clear_server_host_override();
        self.clear_cred_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SecurityParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SecurityParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChannelArg {
    // message fields
    pub name: ::std::string::String,
    // message oneof groups
    value: ::std::option::Option<ChannelArg_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChannelArg {}

#[derive(Clone,PartialEq)]
pub enum ChannelArg_oneof_value {
    str_value(::std::string::String),
    int_value(i32),
}

impl ChannelArg {
    pub fn new() -> ChannelArg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChannelArg {
        static mut instance: ::protobuf::lazy::Lazy<ChannelArg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChannelArg,
        };
        unsafe {
            instance.get(ChannelArg::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string str_value = 2;

    pub fn clear_str_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_str_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ChannelArg_oneof_value::str_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_str_value(&mut self, v: ::std::string::String) {
        self.value = ::std::option::Option::Some(ChannelArg_oneof_value::str_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_str_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ChannelArg_oneof_value::str_value(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ChannelArg_oneof_value::str_value(::std::string::String::new()));
        }
        match self.value {
            ::std::option::Option::Some(ChannelArg_oneof_value::str_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_str_value(&mut self) -> ::std::string::String {
        if self.has_str_value() {
            match self.value.take() {
                ::std::option::Option::Some(ChannelArg_oneof_value::str_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_str_value(&self) -> &str {
        match self.value {
            ::std::option::Option::Some(ChannelArg_oneof_value::str_value(ref v)) => v,
            _ => "",
        }
    }

    // int32 int_value = 3;

    pub fn clear_int_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ChannelArg_oneof_value::int_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(ChannelArg_oneof_value::int_value(v))
    }

    pub fn get_int_value(&self) -> i32 {
        match self.value {
            ::std::option::Option::Some(ChannelArg_oneof_value::int_value(v)) => v,
            _ => 0,
        }
    }
}

impl ::protobuf::Message for ChannelArg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(ChannelArg_oneof_value::str_value(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(ChannelArg_oneof_value::int_value(is.read_int32()?));
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &ChannelArg_oneof_value::str_value(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
                &ChannelArg_oneof_value::int_value(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &ChannelArg_oneof_value::str_value(ref v) => {
                    os.write_string(2, v)?;
                },
                &ChannelArg_oneof_value::int_value(v) => {
                    os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for ChannelArg {
    fn new() -> ChannelArg {
        ChannelArg::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChannelArg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ChannelArg::get_name_for_reflect,
                    ChannelArg::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "str_value",
                    ChannelArg::has_str_value,
                    ChannelArg::get_str_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                    "int_value",
                    ChannelArg::has_int_value,
                    ChannelArg::get_int_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChannelArg>(
                    "ChannelArg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChannelArg {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_str_value();
        self.clear_int_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChannelArg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelArg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientConfig {
    // message fields
    pub server_targets: ::protobuf::RepeatedField<::std::string::String>,
    pub client_type: ClientType,
    pub security_params: ::protobuf::SingularPtrField<SecurityParams>,
    pub outstanding_rpcs_per_channel: i32,
    pub client_channels: i32,
    pub async_client_threads: i32,
    pub rpc_type: RpcType,
    pub load_params: ::protobuf::SingularPtrField<LoadParams>,
    pub payload_config: ::protobuf::SingularPtrField<super::payloads::PayloadConfig>,
    pub histogram_params: ::protobuf::SingularPtrField<super::stats::HistogramParams>,
    pub core_list: ::std::vec::Vec<i32>,
    pub core_limit: i32,
    pub other_client_api: ::std::string::String,
    pub channel_args: ::protobuf::RepeatedField<ChannelArg>,
    pub threads_per_cq: i32,
    pub messages_per_stream: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientConfig {}

impl ClientConfig {
    pub fn new() -> ClientConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientConfig {
        static mut instance: ::protobuf::lazy::Lazy<ClientConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientConfig,
        };
        unsafe {
            instance.get(ClientConfig::new)
        }
    }

    // repeated string server_targets = 1;

    pub fn clear_server_targets(&mut self) {
        self.server_targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_targets(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.server_targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_server_targets(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.server_targets
    }

    // Take field
    pub fn take_server_targets(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.server_targets, ::protobuf::RepeatedField::new())
    }

    pub fn get_server_targets(&self) -> &[::std::string::String] {
        &self.server_targets
    }

    fn get_server_targets_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.server_targets
    }

    fn mut_server_targets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.server_targets
    }

    // .grpc.testing.ClientType client_type = 2;

    pub fn clear_client_type(&mut self) {
        self.client_type = ClientType::SYNC_CLIENT;
    }

    // Param is passed by value, moved
    pub fn set_client_type(&mut self, v: ClientType) {
        self.client_type = v;
    }

    pub fn get_client_type(&self) -> ClientType {
        self.client_type
    }

    fn get_client_type_for_reflect(&self) -> &ClientType {
        &self.client_type
    }

    fn mut_client_type_for_reflect(&mut self) -> &mut ClientType {
        &mut self.client_type
    }

    // .grpc.testing.SecurityParams security_params = 3;

    pub fn clear_security_params(&mut self) {
        self.security_params.clear();
    }

    pub fn has_security_params(&self) -> bool {
        self.security_params.is_some()
    }

    // Param is passed by value, moved
    pub fn set_security_params(&mut self, v: SecurityParams) {
        self.security_params = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_security_params(&mut self) -> &mut SecurityParams {
        if self.security_params.is_none() {
            self.security_params.set_default();
        }
        self.security_params.as_mut().unwrap()
    }

    // Take field
    pub fn take_security_params(&mut self) -> SecurityParams {
        self.security_params.take().unwrap_or_else(|| SecurityParams::new())
    }

    pub fn get_security_params(&self) -> &SecurityParams {
        self.security_params.as_ref().unwrap_or_else(|| SecurityParams::default_instance())
    }

    fn get_security_params_for_reflect(&self) -> &::protobuf::SingularPtrField<SecurityParams> {
        &self.security_params
    }

    fn mut_security_params_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SecurityParams> {
        &mut self.security_params
    }

    // int32 outstanding_rpcs_per_channel = 4;

    pub fn clear_outstanding_rpcs_per_channel(&mut self) {
        self.outstanding_rpcs_per_channel = 0;
    }

    // Param is passed by value, moved
    pub fn set_outstanding_rpcs_per_channel(&mut self, v: i32) {
        self.outstanding_rpcs_per_channel = v;
    }

    pub fn get_outstanding_rpcs_per_channel(&self) -> i32 {
        self.outstanding_rpcs_per_channel
    }

    fn get_outstanding_rpcs_per_channel_for_reflect(&self) -> &i32 {
        &self.outstanding_rpcs_per_channel
    }

    fn mut_outstanding_rpcs_per_channel_for_reflect(&mut self) -> &mut i32 {
        &mut self.outstanding_rpcs_per_channel
    }

    // int32 client_channels = 5;

    pub fn clear_client_channels(&mut self) {
        self.client_channels = 0;
    }

    // Param is passed by value, moved
    pub fn set_client_channels(&mut self, v: i32) {
        self.client_channels = v;
    }

    pub fn get_client_channels(&self) -> i32 {
        self.client_channels
    }

    fn get_client_channels_for_reflect(&self) -> &i32 {
        &self.client_channels
    }

    fn mut_client_channels_for_reflect(&mut self) -> &mut i32 {
        &mut self.client_channels
    }

    // int32 async_client_threads = 7;

    pub fn clear_async_client_threads(&mut self) {
        self.async_client_threads = 0;
    }

    // Param is passed by value, moved
    pub fn set_async_client_threads(&mut self, v: i32) {
        self.async_client_threads = v;
    }

    pub fn get_async_client_threads(&self) -> i32 {
        self.async_client_threads
    }

    fn get_async_client_threads_for_reflect(&self) -> &i32 {
        &self.async_client_threads
    }

    fn mut_async_client_threads_for_reflect(&mut self) -> &mut i32 {
        &mut self.async_client_threads
    }

    // .grpc.testing.RpcType rpc_type = 8;

    pub fn clear_rpc_type(&mut self) {
        self.rpc_type = RpcType::UNARY;
    }

    // Param is passed by value, moved
    pub fn set_rpc_type(&mut self, v: RpcType) {
        self.rpc_type = v;
    }

    pub fn get_rpc_type(&self) -> RpcType {
        self.rpc_type
    }

    fn get_rpc_type_for_reflect(&self) -> &RpcType {
        &self.rpc_type
    }

    fn mut_rpc_type_for_reflect(&mut self) -> &mut RpcType {
        &mut self.rpc_type
    }

    // .grpc.testing.LoadParams load_params = 10;

    pub fn clear_load_params(&mut self) {
        self.load_params.clear();
    }

    pub fn has_load_params(&self) -> bool {
        self.load_params.is_some()
    }

    // Param is passed by value, moved
    pub fn set_load_params(&mut self, v: LoadParams) {
        self.load_params = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_load_params(&mut self) -> &mut LoadParams {
        if self.load_params.is_none() {
            self.load_params.set_default();
        }
        self.load_params.as_mut().unwrap()
    }

    // Take field
    pub fn take_load_params(&mut self) -> LoadParams {
        self.load_params.take().unwrap_or_else(|| LoadParams::new())
    }

    pub fn get_load_params(&self) -> &LoadParams {
        self.load_params.as_ref().unwrap_or_else(|| LoadParams::default_instance())
    }

    fn get_load_params_for_reflect(&self) -> &::protobuf::SingularPtrField<LoadParams> {
        &self.load_params
    }

    fn mut_load_params_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoadParams> {
        &mut self.load_params
    }

    // .grpc.testing.PayloadConfig payload_config = 11;

    pub fn clear_payload_config(&mut self) {
        self.payload_config.clear();
    }

    pub fn has_payload_config(&self) -> bool {
        self.payload_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload_config(&mut self, v: super::payloads::PayloadConfig) {
        self.payload_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload_config(&mut self) -> &mut super::payloads::PayloadConfig {
        if self.payload_config.is_none() {
            self.payload_config.set_default();
        }
        self.payload_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload_config(&mut self) -> super::payloads::PayloadConfig {
        self.payload_config.take().unwrap_or_else(|| super::payloads::PayloadConfig::new())
    }

    pub fn get_payload_config(&self) -> &super::payloads::PayloadConfig {
        self.payload_config.as_ref().unwrap_or_else(|| super::payloads::PayloadConfig::default_instance())
    }

    fn get_payload_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::payloads::PayloadConfig> {
        &self.payload_config
    }

    fn mut_payload_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::payloads::PayloadConfig> {
        &mut self.payload_config
    }

    // .grpc.testing.HistogramParams histogram_params = 12;

    pub fn clear_histogram_params(&mut self) {
        self.histogram_params.clear();
    }

    pub fn has_histogram_params(&self) -> bool {
        self.histogram_params.is_some()
    }

    // Param is passed by value, moved
    pub fn set_histogram_params(&mut self, v: super::stats::HistogramParams) {
        self.histogram_params = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_histogram_params(&mut self) -> &mut super::stats::HistogramParams {
        if self.histogram_params.is_none() {
            self.histogram_params.set_default();
        }
        self.histogram_params.as_mut().unwrap()
    }

    // Take field
    pub fn take_histogram_params(&mut self) -> super::stats::HistogramParams {
        self.histogram_params.take().unwrap_or_else(|| super::stats::HistogramParams::new())
    }

    pub fn get_histogram_params(&self) -> &super::stats::HistogramParams {
        self.histogram_params.as_ref().unwrap_or_else(|| super::stats::HistogramParams::default_instance())
    }

    fn get_histogram_params_for_reflect(&self) -> &::protobuf::SingularPtrField<super::stats::HistogramParams> {
        &self.histogram_params
    }

    fn mut_histogram_params_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::stats::HistogramParams> {
        &mut self.histogram_params
    }

    // repeated int32 core_list = 13;

    pub fn clear_core_list(&mut self) {
        self.core_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_core_list(&mut self, v: ::std::vec::Vec<i32>) {
        self.core_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_core_list(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.core_list
    }

    // Take field
    pub fn take_core_list(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.core_list, ::std::vec::Vec::new())
    }

    pub fn get_core_list(&self) -> &[i32] {
        &self.core_list
    }

    fn get_core_list_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.core_list
    }

    fn mut_core_list_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.core_list
    }

    // int32 core_limit = 14;

    pub fn clear_core_limit(&mut self) {
        self.core_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_core_limit(&mut self, v: i32) {
        self.core_limit = v;
    }

    pub fn get_core_limit(&self) -> i32 {
        self.core_limit
    }

    fn get_core_limit_for_reflect(&self) -> &i32 {
        &self.core_limit
    }

    fn mut_core_limit_for_reflect(&mut self) -> &mut i32 {
        &mut self.core_limit
    }

    // string other_client_api = 15;

    pub fn clear_other_client_api(&mut self) {
        self.other_client_api.clear();
    }

    // Param is passed by value, moved
    pub fn set_other_client_api(&mut self, v: ::std::string::String) {
        self.other_client_api = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_other_client_api(&mut self) -> &mut ::std::string::String {
        &mut self.other_client_api
    }

    // Take field
    pub fn take_other_client_api(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.other_client_api, ::std::string::String::new())
    }

    pub fn get_other_client_api(&self) -> &str {
        &self.other_client_api
    }

    fn get_other_client_api_for_reflect(&self) -> &::std::string::String {
        &self.other_client_api
    }

    fn mut_other_client_api_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.other_client_api
    }

    // repeated .grpc.testing.ChannelArg channel_args = 16;

    pub fn clear_channel_args(&mut self) {
        self.channel_args.clear();
    }

    // Param is passed by value, moved
    pub fn set_channel_args(&mut self, v: ::protobuf::RepeatedField<ChannelArg>) {
        self.channel_args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_channel_args(&mut self) -> &mut ::protobuf::RepeatedField<ChannelArg> {
        &mut self.channel_args
    }

    // Take field
    pub fn take_channel_args(&mut self) -> ::protobuf::RepeatedField<ChannelArg> {
        ::std::mem::replace(&mut self.channel_args, ::protobuf::RepeatedField::new())
    }

    pub fn get_channel_args(&self) -> &[ChannelArg] {
        &self.channel_args
    }

    fn get_channel_args_for_reflect(&self) -> &::protobuf::RepeatedField<ChannelArg> {
        &self.channel_args
    }

    fn mut_channel_args_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ChannelArg> {
        &mut self.channel_args
    }

    // int32 threads_per_cq = 17;

    pub fn clear_threads_per_cq(&mut self) {
        self.threads_per_cq = 0;
    }

    // Param is passed by value, moved
    pub fn set_threads_per_cq(&mut self, v: i32) {
        self.threads_per_cq = v;
    }

    pub fn get_threads_per_cq(&self) -> i32 {
        self.threads_per_cq
    }

    fn get_threads_per_cq_for_reflect(&self) -> &i32 {
        &self.threads_per_cq
    }

    fn mut_threads_per_cq_for_reflect(&mut self) -> &mut i32 {
        &mut self.threads_per_cq
    }

    // int32 messages_per_stream = 18;

    pub fn clear_messages_per_stream(&mut self) {
        self.messages_per_stream = 0;
    }

    // Param is passed by value, moved
    pub fn set_messages_per_stream(&mut self, v: i32) {
        self.messages_per_stream = v;
    }

    pub fn get_messages_per_stream(&self) -> i32 {
        self.messages_per_stream
    }

    fn get_messages_per_stream_for_reflect(&self) -> &i32 {
        &self.messages_per_stream
    }

    fn mut_messages_per_stream_for_reflect(&mut self) -> &mut i32 {
        &mut self.messages_per_stream
    }
}

impl ::protobuf::Message for ClientConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.security_params {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.load_params {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.payload_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.histogram_params {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.channel_args {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.server_targets)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.client_type = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.security_params)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.outstanding_rpcs_per_channel = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.client_channels = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.async_client_threads = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.rpc_type = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.load_params)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payload_config)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.histogram_params)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.core_list)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.core_limit = tmp;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.other_client_api)?;
                },
                16 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.channel_args)?;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.threads_per_cq = tmp;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.messages_per_stream = tmp;
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
        for value in &self.server_targets {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.client_type != ClientType::SYNC_CLIENT {
            my_size += ::protobuf::rt::enum_size(2, self.client_type);
        }
        if let Some(ref v) = self.security_params.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.outstanding_rpcs_per_channel != 0 {
            my_size += ::protobuf::rt::value_size(4, self.outstanding_rpcs_per_channel, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.client_channels != 0 {
            my_size += ::protobuf::rt::value_size(5, self.client_channels, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.async_client_threads != 0 {
            my_size += ::protobuf::rt::value_size(7, self.async_client_threads, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.rpc_type != RpcType::UNARY {
            my_size += ::protobuf::rt::enum_size(8, self.rpc_type);
        }
        if let Some(ref v) = self.load_params.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.payload_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.histogram_params.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.core_list {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.core_limit != 0 {
            my_size += ::protobuf::rt::value_size(14, self.core_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.other_client_api.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.other_client_api);
        }
        for value in &self.channel_args {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.threads_per_cq != 0 {
            my_size += ::protobuf::rt::value_size(17, self.threads_per_cq, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.messages_per_stream != 0 {
            my_size += ::protobuf::rt::value_size(18, self.messages_per_stream, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.server_targets {
            os.write_string(1, &v)?;
        };
        if self.client_type != ClientType::SYNC_CLIENT {
            os.write_enum(2, self.client_type.value())?;
        }
        if let Some(ref v) = self.security_params.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.outstanding_rpcs_per_channel != 0 {
            os.write_int32(4, self.outstanding_rpcs_per_channel)?;
        }
        if self.client_channels != 0 {
            os.write_int32(5, self.client_channels)?;
        }
        if self.async_client_threads != 0 {
            os.write_int32(7, self.async_client_threads)?;
        }
        if self.rpc_type != RpcType::UNARY {
            os.write_enum(8, self.rpc_type.value())?;
        }
        if let Some(ref v) = self.load_params.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.payload_config.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.histogram_params.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.core_list {
            os.write_int32(13, *v)?;
        };
        if self.core_limit != 0 {
            os.write_int32(14, self.core_limit)?;
        }
        if !self.other_client_api.is_empty() {
            os.write_string(15, &self.other_client_api)?;
        }
        for v in &self.channel_args {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.threads_per_cq != 0 {
            os.write_int32(17, self.threads_per_cq)?;
        }
        if self.messages_per_stream != 0 {
            os.write_int32(18, self.messages_per_stream)?;
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

impl ::protobuf::MessageStatic for ClientConfig {
    fn new() -> ClientConfig {
        ClientConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "server_targets",
                    ClientConfig::get_server_targets_for_reflect,
                    ClientConfig::mut_server_targets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ClientType>>(
                    "client_type",
                    ClientConfig::get_client_type_for_reflect,
                    ClientConfig::mut_client_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SecurityParams>>(
                    "security_params",
                    ClientConfig::get_security_params_for_reflect,
                    ClientConfig::mut_security_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "outstanding_rpcs_per_channel",
                    ClientConfig::get_outstanding_rpcs_per_channel_for_reflect,
                    ClientConfig::mut_outstanding_rpcs_per_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "client_channels",
                    ClientConfig::get_client_channels_for_reflect,
                    ClientConfig::mut_client_channels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "async_client_threads",
                    ClientConfig::get_async_client_threads_for_reflect,
                    ClientConfig::mut_async_client_threads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RpcType>>(
                    "rpc_type",
                    ClientConfig::get_rpc_type_for_reflect,
                    ClientConfig::mut_rpc_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoadParams>>(
                    "load_params",
                    ClientConfig::get_load_params_for_reflect,
                    ClientConfig::mut_load_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::payloads::PayloadConfig>>(
                    "payload_config",
                    ClientConfig::get_payload_config_for_reflect,
                    ClientConfig::mut_payload_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::stats::HistogramParams>>(
                    "histogram_params",
                    ClientConfig::get_histogram_params_for_reflect,
                    ClientConfig::mut_histogram_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "core_list",
                    ClientConfig::get_core_list_for_reflect,
                    ClientConfig::mut_core_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "core_limit",
                    ClientConfig::get_core_limit_for_reflect,
                    ClientConfig::mut_core_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "other_client_api",
                    ClientConfig::get_other_client_api_for_reflect,
                    ClientConfig::mut_other_client_api_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelArg>>(
                    "channel_args",
                    ClientConfig::get_channel_args_for_reflect,
                    ClientConfig::mut_channel_args_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "threads_per_cq",
                    ClientConfig::get_threads_per_cq_for_reflect,
                    ClientConfig::mut_threads_per_cq_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "messages_per_stream",
                    ClientConfig::get_messages_per_stream_for_reflect,
                    ClientConfig::mut_messages_per_stream_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientConfig>(
                    "ClientConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientConfig {
    fn clear(&mut self) {
        self.clear_server_targets();
        self.clear_client_type();
        self.clear_security_params();
        self.clear_outstanding_rpcs_per_channel();
        self.clear_client_channels();
        self.clear_async_client_threads();
        self.clear_rpc_type();
        self.clear_load_params();
        self.clear_payload_config();
        self.clear_histogram_params();
        self.clear_core_list();
        self.clear_core_limit();
        self.clear_other_client_api();
        self.clear_channel_args();
        self.clear_threads_per_cq();
        self.clear_messages_per_stream();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientStatus {
    // message fields
    pub stats: ::protobuf::SingularPtrField<super::stats::ClientStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientStatus {}

impl ClientStatus {
    pub fn new() -> ClientStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientStatus {
        static mut instance: ::protobuf::lazy::Lazy<ClientStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientStatus,
        };
        unsafe {
            instance.get(ClientStatus::new)
        }
    }

    // .grpc.testing.ClientStats stats = 1;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: super::stats::ClientStats) {
        self.stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats(&mut self) -> &mut super::stats::ClientStats {
        if self.stats.is_none() {
            self.stats.set_default();
        }
        self.stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats(&mut self) -> super::stats::ClientStats {
        self.stats.take().unwrap_or_else(|| super::stats::ClientStats::new())
    }

    pub fn get_stats(&self) -> &super::stats::ClientStats {
        self.stats.as_ref().unwrap_or_else(|| super::stats::ClientStats::default_instance())
    }

    fn get_stats_for_reflect(&self) -> &::protobuf::SingularPtrField<super::stats::ClientStats> {
        &self.stats
    }

    fn mut_stats_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::stats::ClientStats> {
        &mut self.stats
    }
}

impl ::protobuf::Message for ClientStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.stats {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stats)?;
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
        if let Some(ref v) = self.stats.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.stats.as_ref() {
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

impl ::protobuf::MessageStatic for ClientStatus {
    fn new() -> ClientStatus {
        ClientStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::stats::ClientStats>>(
                    "stats",
                    ClientStatus::get_stats_for_reflect,
                    ClientStatus::mut_stats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientStatus>(
                    "ClientStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientStatus {
    fn clear(&mut self) {
        self.clear_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mark {
    // message fields
    pub reset: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mark {}

impl Mark {
    pub fn new() -> Mark {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mark {
        static mut instance: ::protobuf::lazy::Lazy<Mark> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mark,
        };
        unsafe {
            instance.get(Mark::new)
        }
    }

    // bool reset = 1;

    pub fn clear_reset(&mut self) {
        self.reset = false;
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: bool) {
        self.reset = v;
    }

    pub fn get_reset(&self) -> bool {
        self.reset
    }

    fn get_reset_for_reflect(&self) -> &bool {
        &self.reset
    }

    fn mut_reset_for_reflect(&mut self) -> &mut bool {
        &mut self.reset
    }
}

impl ::protobuf::Message for Mark {
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
                    self.reset = tmp;
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
        if self.reset != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.reset != false {
            os.write_bool(1, self.reset)?;
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

impl ::protobuf::MessageStatic for Mark {
    fn new() -> Mark {
        Mark::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mark>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reset",
                    Mark::get_reset_for_reflect,
                    Mark::mut_reset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mark>(
                    "Mark",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mark {
    fn clear(&mut self) {
        self.clear_reset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mark {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mark {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientArgs {
    // message oneof groups
    argtype: ::std::option::Option<ClientArgs_oneof_argtype>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientArgs {}

#[derive(Clone,PartialEq)]
pub enum ClientArgs_oneof_argtype {
    setup(ClientConfig),
    mark(Mark),
}

impl ClientArgs {
    pub fn new() -> ClientArgs {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientArgs {
        static mut instance: ::protobuf::lazy::Lazy<ClientArgs> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientArgs,
        };
        unsafe {
            instance.get(ClientArgs::new)
        }
    }

    // .grpc.testing.ClientConfig setup = 1;

    pub fn clear_setup(&mut self) {
        self.argtype = ::std::option::Option::None;
    }

    pub fn has_setup(&self) -> bool {
        match self.argtype {
            ::std::option::Option::Some(ClientArgs_oneof_argtype::setup(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setup(&mut self, v: ClientConfig) {
        self.argtype = ::std::option::Option::Some(ClientArgs_oneof_argtype::setup(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setup(&mut self) -> &mut ClientConfig {
        if let ::std::option::Option::Some(ClientArgs_oneof_argtype::setup(_)) = self.argtype {
        } else {
            self.argtype = ::std::option::Option::Some(ClientArgs_oneof_argtype::setup(ClientConfig::new()));
        }
        match self.argtype {
            ::std::option::Option::Some(ClientArgs_oneof_argtype::setup(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setup(&mut self) -> ClientConfig {
        if self.has_setup() {
            match self.argtype.take() {
                ::std::option::Option::Some(ClientArgs_oneof_argtype::setup(v)) => v,
                _ => panic!(),
            }
        } else {
            ClientConfig::new()
        }
    }

    pub fn get_setup(&self) -> &ClientConfig {
        match self.argtype {
            ::std::option::Option::Some(ClientArgs_oneof_argtype::setup(ref v)) => v,
            _ => ClientConfig::default_instance(),
        }
    }

    // .grpc.testing.Mark mark = 2;

    pub fn clear_mark(&mut self) {
        self.argtype = ::std::option::Option::None;
    }

    pub fn has_mark(&self) -> bool {
        match self.argtype {
            ::std::option::Option::Some(ClientArgs_oneof_argtype::mark(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_mark(&mut self, v: Mark) {
        self.argtype = ::std::option::Option::Some(ClientArgs_oneof_argtype::mark(v))
    }

    // Mutable pointer to the field.
    pub fn mut_mark(&mut self) -> &mut Mark {
        if let ::std::option::Option::Some(ClientArgs_oneof_argtype::mark(_)) = self.argtype {
        } else {
            self.argtype = ::std::option::Option::Some(ClientArgs_oneof_argtype::mark(Mark::new()));
        }
        match self.argtype {
            ::std::option::Option::Some(ClientArgs_oneof_argtype::mark(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_mark(&mut self) -> Mark {
        if self.has_mark() {
            match self.argtype.take() {
                ::std::option::Option::Some(ClientArgs_oneof_argtype::mark(v)) => v,
                _ => panic!(),
            }
        } else {
            Mark::new()
        }
    }

    pub fn get_mark(&self) -> &Mark {
        match self.argtype {
            ::std::option::Option::Some(ClientArgs_oneof_argtype::mark(ref v)) => v,
            _ => Mark::default_instance(),
        }
    }
}

impl ::protobuf::Message for ClientArgs {
    fn is_initialized(&self) -> bool {
        if let Some(ClientArgs_oneof_argtype::setup(ref v)) = self.argtype {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ClientArgs_oneof_argtype::mark(ref v)) = self.argtype {
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
                    self.argtype = ::std::option::Option::Some(ClientArgs_oneof_argtype::setup(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.argtype = ::std::option::Option::Some(ClientArgs_oneof_argtype::mark(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.argtype {
            match v {
                &ClientArgs_oneof_argtype::setup(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ClientArgs_oneof_argtype::mark(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.argtype {
            match v {
                &ClientArgs_oneof_argtype::setup(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ClientArgs_oneof_argtype::mark(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ClientArgs {
    fn new() -> ClientArgs {
        ClientArgs::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientArgs>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ClientConfig>(
                    "setup",
                    ClientArgs::has_setup,
                    ClientArgs::get_setup,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Mark>(
                    "mark",
                    ClientArgs::has_mark,
                    ClientArgs::get_mark,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientArgs>(
                    "ClientArgs",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientArgs {
    fn clear(&mut self) {
        self.clear_setup();
        self.clear_mark();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientArgs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientArgs {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServerConfig {
    // message fields
    pub server_type: ServerType,
    pub security_params: ::protobuf::SingularPtrField<SecurityParams>,
    pub port: i32,
    pub async_server_threads: i32,
    pub core_limit: i32,
    pub payload_config: ::protobuf::SingularPtrField<super::payloads::PayloadConfig>,
    pub core_list: ::std::vec::Vec<i32>,
    pub other_server_api: ::std::string::String,
    pub threads_per_cq: i32,
    pub resource_quota_size: i32,
    pub channel_args: ::protobuf::RepeatedField<ChannelArg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerConfig {}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerConfig {
        static mut instance: ::protobuf::lazy::Lazy<ServerConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerConfig,
        };
        unsafe {
            instance.get(ServerConfig::new)
        }
    }

    // .grpc.testing.ServerType server_type = 1;

    pub fn clear_server_type(&mut self) {
        self.server_type = ServerType::SYNC_SERVER;
    }

    // Param is passed by value, moved
    pub fn set_server_type(&mut self, v: ServerType) {
        self.server_type = v;
    }

    pub fn get_server_type(&self) -> ServerType {
        self.server_type
    }

    fn get_server_type_for_reflect(&self) -> &ServerType {
        &self.server_type
    }

    fn mut_server_type_for_reflect(&mut self) -> &mut ServerType {
        &mut self.server_type
    }

    // .grpc.testing.SecurityParams security_params = 2;

    pub fn clear_security_params(&mut self) {
        self.security_params.clear();
    }

    pub fn has_security_params(&self) -> bool {
        self.security_params.is_some()
    }

    // Param is passed by value, moved
    pub fn set_security_params(&mut self, v: SecurityParams) {
        self.security_params = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_security_params(&mut self) -> &mut SecurityParams {
        if self.security_params.is_none() {
            self.security_params.set_default();
        }
        self.security_params.as_mut().unwrap()
    }

    // Take field
    pub fn take_security_params(&mut self) -> SecurityParams {
        self.security_params.take().unwrap_or_else(|| SecurityParams::new())
    }

    pub fn get_security_params(&self) -> &SecurityParams {
        self.security_params.as_ref().unwrap_or_else(|| SecurityParams::default_instance())
    }

    fn get_security_params_for_reflect(&self) -> &::protobuf::SingularPtrField<SecurityParams> {
        &self.security_params
    }

    fn mut_security_params_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SecurityParams> {
        &mut self.security_params
    }

    // int32 port = 4;

    pub fn clear_port(&mut self) {
        self.port = 0;
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: i32) {
        self.port = v;
    }

    pub fn get_port(&self) -> i32 {
        self.port
    }

    fn get_port_for_reflect(&self) -> &i32 {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut i32 {
        &mut self.port
    }

    // int32 async_server_threads = 7;

    pub fn clear_async_server_threads(&mut self) {
        self.async_server_threads = 0;
    }

    // Param is passed by value, moved
    pub fn set_async_server_threads(&mut self, v: i32) {
        self.async_server_threads = v;
    }

    pub fn get_async_server_threads(&self) -> i32 {
        self.async_server_threads
    }

    fn get_async_server_threads_for_reflect(&self) -> &i32 {
        &self.async_server_threads
    }

    fn mut_async_server_threads_for_reflect(&mut self) -> &mut i32 {
        &mut self.async_server_threads
    }

    // int32 core_limit = 8;

    pub fn clear_core_limit(&mut self) {
        self.core_limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_core_limit(&mut self, v: i32) {
        self.core_limit = v;
    }

    pub fn get_core_limit(&self) -> i32 {
        self.core_limit
    }

    fn get_core_limit_for_reflect(&self) -> &i32 {
        &self.core_limit
    }

    fn mut_core_limit_for_reflect(&mut self) -> &mut i32 {
        &mut self.core_limit
    }

    // .grpc.testing.PayloadConfig payload_config = 9;

    pub fn clear_payload_config(&mut self) {
        self.payload_config.clear();
    }

    pub fn has_payload_config(&self) -> bool {
        self.payload_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload_config(&mut self, v: super::payloads::PayloadConfig) {
        self.payload_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload_config(&mut self) -> &mut super::payloads::PayloadConfig {
        if self.payload_config.is_none() {
            self.payload_config.set_default();
        }
        self.payload_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload_config(&mut self) -> super::payloads::PayloadConfig {
        self.payload_config.take().unwrap_or_else(|| super::payloads::PayloadConfig::new())
    }

    pub fn get_payload_config(&self) -> &super::payloads::PayloadConfig {
        self.payload_config.as_ref().unwrap_or_else(|| super::payloads::PayloadConfig::default_instance())
    }

    fn get_payload_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::payloads::PayloadConfig> {
        &self.payload_config
    }

    fn mut_payload_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::payloads::PayloadConfig> {
        &mut self.payload_config
    }

    // repeated int32 core_list = 10;

    pub fn clear_core_list(&mut self) {
        self.core_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_core_list(&mut self, v: ::std::vec::Vec<i32>) {
        self.core_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_core_list(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.core_list
    }

    // Take field
    pub fn take_core_list(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.core_list, ::std::vec::Vec::new())
    }

    pub fn get_core_list(&self) -> &[i32] {
        &self.core_list
    }

    fn get_core_list_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.core_list
    }

    fn mut_core_list_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.core_list
    }

    // string other_server_api = 11;

    pub fn clear_other_server_api(&mut self) {
        self.other_server_api.clear();
    }

    // Param is passed by value, moved
    pub fn set_other_server_api(&mut self, v: ::std::string::String) {
        self.other_server_api = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_other_server_api(&mut self) -> &mut ::std::string::String {
        &mut self.other_server_api
    }

    // Take field
    pub fn take_other_server_api(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.other_server_api, ::std::string::String::new())
    }

    pub fn get_other_server_api(&self) -> &str {
        &self.other_server_api
    }

    fn get_other_server_api_for_reflect(&self) -> &::std::string::String {
        &self.other_server_api
    }

    fn mut_other_server_api_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.other_server_api
    }

    // int32 threads_per_cq = 12;

    pub fn clear_threads_per_cq(&mut self) {
        self.threads_per_cq = 0;
    }

    // Param is passed by value, moved
    pub fn set_threads_per_cq(&mut self, v: i32) {
        self.threads_per_cq = v;
    }

    pub fn get_threads_per_cq(&self) -> i32 {
        self.threads_per_cq
    }

    fn get_threads_per_cq_for_reflect(&self) -> &i32 {
        &self.threads_per_cq
    }

    fn mut_threads_per_cq_for_reflect(&mut self) -> &mut i32 {
        &mut self.threads_per_cq
    }

    // int32 resource_quota_size = 1001;

    pub fn clear_resource_quota_size(&mut self) {
        self.resource_quota_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_resource_quota_size(&mut self, v: i32) {
        self.resource_quota_size = v;
    }

    pub fn get_resource_quota_size(&self) -> i32 {
        self.resource_quota_size
    }

    fn get_resource_quota_size_for_reflect(&self) -> &i32 {
        &self.resource_quota_size
    }

    fn mut_resource_quota_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.resource_quota_size
    }

    // repeated .grpc.testing.ChannelArg channel_args = 1002;

    pub fn clear_channel_args(&mut self) {
        self.channel_args.clear();
    }

    // Param is passed by value, moved
    pub fn set_channel_args(&mut self, v: ::protobuf::RepeatedField<ChannelArg>) {
        self.channel_args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_channel_args(&mut self) -> &mut ::protobuf::RepeatedField<ChannelArg> {
        &mut self.channel_args
    }

    // Take field
    pub fn take_channel_args(&mut self) -> ::protobuf::RepeatedField<ChannelArg> {
        ::std::mem::replace(&mut self.channel_args, ::protobuf::RepeatedField::new())
    }

    pub fn get_channel_args(&self) -> &[ChannelArg] {
        &self.channel_args
    }

    fn get_channel_args_for_reflect(&self) -> &::protobuf::RepeatedField<ChannelArg> {
        &self.channel_args
    }

    fn mut_channel_args_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ChannelArg> {
        &mut self.channel_args
    }
}

impl ::protobuf::Message for ServerConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.security_params {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.payload_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.channel_args {
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
                    self.server_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.security_params)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.port = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.async_server_threads = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.core_limit = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payload_config)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.core_list)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.other_server_api)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.threads_per_cq = tmp;
                },
                1001 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.resource_quota_size = tmp;
                },
                1002 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.channel_args)?;
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
        if self.server_type != ServerType::SYNC_SERVER {
            my_size += ::protobuf::rt::enum_size(1, self.server_type);
        }
        if let Some(ref v) = self.security_params.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.port != 0 {
            my_size += ::protobuf::rt::value_size(4, self.port, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.async_server_threads != 0 {
            my_size += ::protobuf::rt::value_size(7, self.async_server_threads, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.core_limit != 0 {
            my_size += ::protobuf::rt::value_size(8, self.core_limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.payload_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.core_list {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.other_server_api.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.other_server_api);
        }
        if self.threads_per_cq != 0 {
            my_size += ::protobuf::rt::value_size(12, self.threads_per_cq, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.resource_quota_size != 0 {
            my_size += ::protobuf::rt::value_size(1001, self.resource_quota_size, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.channel_args {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.server_type != ServerType::SYNC_SERVER {
            os.write_enum(1, self.server_type.value())?;
        }
        if let Some(ref v) = self.security_params.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.port != 0 {
            os.write_int32(4, self.port)?;
        }
        if self.async_server_threads != 0 {
            os.write_int32(7, self.async_server_threads)?;
        }
        if self.core_limit != 0 {
            os.write_int32(8, self.core_limit)?;
        }
        if let Some(ref v) = self.payload_config.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.core_list {
            os.write_int32(10, *v)?;
        };
        if !self.other_server_api.is_empty() {
            os.write_string(11, &self.other_server_api)?;
        }
        if self.threads_per_cq != 0 {
            os.write_int32(12, self.threads_per_cq)?;
        }
        if self.resource_quota_size != 0 {
            os.write_int32(1001, self.resource_quota_size)?;
        }
        for v in &self.channel_args {
            os.write_tag(1002, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ServerConfig {
    fn new() -> ServerConfig {
        ServerConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ServerType>>(
                    "server_type",
                    ServerConfig::get_server_type_for_reflect,
                    ServerConfig::mut_server_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SecurityParams>>(
                    "security_params",
                    ServerConfig::get_security_params_for_reflect,
                    ServerConfig::mut_security_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "port",
                    ServerConfig::get_port_for_reflect,
                    ServerConfig::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "async_server_threads",
                    ServerConfig::get_async_server_threads_for_reflect,
                    ServerConfig::mut_async_server_threads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "core_limit",
                    ServerConfig::get_core_limit_for_reflect,
                    ServerConfig::mut_core_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::payloads::PayloadConfig>>(
                    "payload_config",
                    ServerConfig::get_payload_config_for_reflect,
                    ServerConfig::mut_payload_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "core_list",
                    ServerConfig::get_core_list_for_reflect,
                    ServerConfig::mut_core_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "other_server_api",
                    ServerConfig::get_other_server_api_for_reflect,
                    ServerConfig::mut_other_server_api_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "threads_per_cq",
                    ServerConfig::get_threads_per_cq_for_reflect,
                    ServerConfig::mut_threads_per_cq_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "resource_quota_size",
                    ServerConfig::get_resource_quota_size_for_reflect,
                    ServerConfig::mut_resource_quota_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChannelArg>>(
                    "channel_args",
                    ServerConfig::get_channel_args_for_reflect,
                    ServerConfig::mut_channel_args_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerConfig>(
                    "ServerConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerConfig {
    fn clear(&mut self) {
        self.clear_server_type();
        self.clear_security_params();
        self.clear_port();
        self.clear_async_server_threads();
        self.clear_core_limit();
        self.clear_payload_config();
        self.clear_core_list();
        self.clear_other_server_api();
        self.clear_threads_per_cq();
        self.clear_resource_quota_size();
        self.clear_channel_args();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServerArgs {
    // message oneof groups
    argtype: ::std::option::Option<ServerArgs_oneof_argtype>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerArgs {}

#[derive(Clone,PartialEq)]
pub enum ServerArgs_oneof_argtype {
    setup(ServerConfig),
    mark(Mark),
}

impl ServerArgs {
    pub fn new() -> ServerArgs {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerArgs {
        static mut instance: ::protobuf::lazy::Lazy<ServerArgs> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerArgs,
        };
        unsafe {
            instance.get(ServerArgs::new)
        }
    }

    // .grpc.testing.ServerConfig setup = 1;

    pub fn clear_setup(&mut self) {
        self.argtype = ::std::option::Option::None;
    }

    pub fn has_setup(&self) -> bool {
        match self.argtype {
            ::std::option::Option::Some(ServerArgs_oneof_argtype::setup(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setup(&mut self, v: ServerConfig) {
        self.argtype = ::std::option::Option::Some(ServerArgs_oneof_argtype::setup(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setup(&mut self) -> &mut ServerConfig {
        if let ::std::option::Option::Some(ServerArgs_oneof_argtype::setup(_)) = self.argtype {
        } else {
            self.argtype = ::std::option::Option::Some(ServerArgs_oneof_argtype::setup(ServerConfig::new()));
        }
        match self.argtype {
            ::std::option::Option::Some(ServerArgs_oneof_argtype::setup(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setup(&mut self) -> ServerConfig {
        if self.has_setup() {
            match self.argtype.take() {
                ::std::option::Option::Some(ServerArgs_oneof_argtype::setup(v)) => v,
                _ => panic!(),
            }
        } else {
            ServerConfig::new()
        }
    }

    pub fn get_setup(&self) -> &ServerConfig {
        match self.argtype {
            ::std::option::Option::Some(ServerArgs_oneof_argtype::setup(ref v)) => v,
            _ => ServerConfig::default_instance(),
        }
    }

    // .grpc.testing.Mark mark = 2;

    pub fn clear_mark(&mut self) {
        self.argtype = ::std::option::Option::None;
    }

    pub fn has_mark(&self) -> bool {
        match self.argtype {
            ::std::option::Option::Some(ServerArgs_oneof_argtype::mark(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_mark(&mut self, v: Mark) {
        self.argtype = ::std::option::Option::Some(ServerArgs_oneof_argtype::mark(v))
    }

    // Mutable pointer to the field.
    pub fn mut_mark(&mut self) -> &mut Mark {
        if let ::std::option::Option::Some(ServerArgs_oneof_argtype::mark(_)) = self.argtype {
        } else {
            self.argtype = ::std::option::Option::Some(ServerArgs_oneof_argtype::mark(Mark::new()));
        }
        match self.argtype {
            ::std::option::Option::Some(ServerArgs_oneof_argtype::mark(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_mark(&mut self) -> Mark {
        if self.has_mark() {
            match self.argtype.take() {
                ::std::option::Option::Some(ServerArgs_oneof_argtype::mark(v)) => v,
                _ => panic!(),
            }
        } else {
            Mark::new()
        }
    }

    pub fn get_mark(&self) -> &Mark {
        match self.argtype {
            ::std::option::Option::Some(ServerArgs_oneof_argtype::mark(ref v)) => v,
            _ => Mark::default_instance(),
        }
    }
}

impl ::protobuf::Message for ServerArgs {
    fn is_initialized(&self) -> bool {
        if let Some(ServerArgs_oneof_argtype::setup(ref v)) = self.argtype {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ServerArgs_oneof_argtype::mark(ref v)) = self.argtype {
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
                    self.argtype = ::std::option::Option::Some(ServerArgs_oneof_argtype::setup(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.argtype = ::std::option::Option::Some(ServerArgs_oneof_argtype::mark(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.argtype {
            match v {
                &ServerArgs_oneof_argtype::setup(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ServerArgs_oneof_argtype::mark(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.argtype {
            match v {
                &ServerArgs_oneof_argtype::setup(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ServerArgs_oneof_argtype::mark(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ServerArgs {
    fn new() -> ServerArgs {
        ServerArgs::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerArgs>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ServerConfig>(
                    "setup",
                    ServerArgs::has_setup,
                    ServerArgs::get_setup,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Mark>(
                    "mark",
                    ServerArgs::has_mark,
                    ServerArgs::get_mark,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerArgs>(
                    "ServerArgs",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerArgs {
    fn clear(&mut self) {
        self.clear_setup();
        self.clear_mark();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerArgs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerArgs {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServerStatus {
    // message fields
    pub stats: ::protobuf::SingularPtrField<super::stats::ServerStats>,
    pub port: i32,
    pub cores: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerStatus {}

impl ServerStatus {
    pub fn new() -> ServerStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerStatus {
        static mut instance: ::protobuf::lazy::Lazy<ServerStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerStatus,
        };
        unsafe {
            instance.get(ServerStatus::new)
        }
    }

    // .grpc.testing.ServerStats stats = 1;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: super::stats::ServerStats) {
        self.stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats(&mut self) -> &mut super::stats::ServerStats {
        if self.stats.is_none() {
            self.stats.set_default();
        }
        self.stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats(&mut self) -> super::stats::ServerStats {
        self.stats.take().unwrap_or_else(|| super::stats::ServerStats::new())
    }

    pub fn get_stats(&self) -> &super::stats::ServerStats {
        self.stats.as_ref().unwrap_or_else(|| super::stats::ServerStats::default_instance())
    }

    fn get_stats_for_reflect(&self) -> &::protobuf::SingularPtrField<super::stats::ServerStats> {
        &self.stats
    }

    fn mut_stats_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::stats::ServerStats> {
        &mut self.stats
    }

    // int32 port = 2;

    pub fn clear_port(&mut self) {
        self.port = 0;
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: i32) {
        self.port = v;
    }

    pub fn get_port(&self) -> i32 {
        self.port
    }

    fn get_port_for_reflect(&self) -> &i32 {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut i32 {
        &mut self.port
    }

    // int32 cores = 3;

    pub fn clear_cores(&mut self) {
        self.cores = 0;
    }

    // Param is passed by value, moved
    pub fn set_cores(&mut self, v: i32) {
        self.cores = v;
    }

    pub fn get_cores(&self) -> i32 {
        self.cores
    }

    fn get_cores_for_reflect(&self) -> &i32 {
        &self.cores
    }

    fn mut_cores_for_reflect(&mut self) -> &mut i32 {
        &mut self.cores
    }
}

impl ::protobuf::Message for ServerStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.stats {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stats)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.port = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.cores = tmp;
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
        if let Some(ref v) = self.stats.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.port != 0 {
            my_size += ::protobuf::rt::value_size(2, self.port, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.cores != 0 {
            my_size += ::protobuf::rt::value_size(3, self.cores, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.stats.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.port != 0 {
            os.write_int32(2, self.port)?;
        }
        if self.cores != 0 {
            os.write_int32(3, self.cores)?;
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

impl ::protobuf::MessageStatic for ServerStatus {
    fn new() -> ServerStatus {
        ServerStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::stats::ServerStats>>(
                    "stats",
                    ServerStatus::get_stats_for_reflect,
                    ServerStatus::mut_stats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "port",
                    ServerStatus::get_port_for_reflect,
                    ServerStatus::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cores",
                    ServerStatus::get_cores_for_reflect,
                    ServerStatus::mut_cores_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerStatus>(
                    "ServerStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerStatus {
    fn clear(&mut self) {
        self.clear_stats();
        self.clear_port();
        self.clear_cores();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CoreRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CoreRequest {}

impl CoreRequest {
    pub fn new() -> CoreRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CoreRequest {
        static mut instance: ::protobuf::lazy::Lazy<CoreRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CoreRequest,
        };
        unsafe {
            instance.get(CoreRequest::new)
        }
    }
}

impl ::protobuf::Message for CoreRequest {
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

impl ::protobuf::MessageStatic for CoreRequest {
    fn new() -> CoreRequest {
        CoreRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CoreRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CoreRequest>(
                    "CoreRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CoreRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CoreRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CoreRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CoreResponse {
    // message fields
    pub cores: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CoreResponse {}

impl CoreResponse {
    pub fn new() -> CoreResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CoreResponse {
        static mut instance: ::protobuf::lazy::Lazy<CoreResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CoreResponse,
        };
        unsafe {
            instance.get(CoreResponse::new)
        }
    }

    // int32 cores = 1;

    pub fn clear_cores(&mut self) {
        self.cores = 0;
    }

    // Param is passed by value, moved
    pub fn set_cores(&mut self, v: i32) {
        self.cores = v;
    }

    pub fn get_cores(&self) -> i32 {
        self.cores
    }

    fn get_cores_for_reflect(&self) -> &i32 {
        &self.cores
    }

    fn mut_cores_for_reflect(&mut self) -> &mut i32 {
        &mut self.cores
    }
}

impl ::protobuf::Message for CoreResponse {
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
                    self.cores = tmp;
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
        if self.cores != 0 {
            my_size += ::protobuf::rt::value_size(1, self.cores, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cores != 0 {
            os.write_int32(1, self.cores)?;
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

impl ::protobuf::MessageStatic for CoreResponse {
    fn new() -> CoreResponse {
        CoreResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CoreResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cores",
                    CoreResponse::get_cores_for_reflect,
                    CoreResponse::mut_cores_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CoreResponse>(
                    "CoreResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CoreResponse {
    fn clear(&mut self) {
        self.clear_cores();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CoreResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CoreResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Void {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Void {}

impl Void {
    pub fn new() -> Void {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Void {
        static mut instance: ::protobuf::lazy::Lazy<Void> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Void,
        };
        unsafe {
            instance.get(Void::new)
        }
    }
}

impl ::protobuf::Message for Void {
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

impl ::protobuf::MessageStatic for Void {
    fn new() -> Void {
        Void::new()
    }

    fn descriptor_static(_: ::std::option::Option<Void>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Void>(
                    "Void",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Void {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Void {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Void {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Scenario {
    // message fields
    pub name: ::std::string::String,
    pub client_config: ::protobuf::SingularPtrField<ClientConfig>,
    pub num_clients: i32,
    pub server_config: ::protobuf::SingularPtrField<ServerConfig>,
    pub num_servers: i32,
    pub warmup_seconds: i32,
    pub benchmark_seconds: i32,
    pub spawn_local_worker_count: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Scenario {}

impl Scenario {
    pub fn new() -> Scenario {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Scenario {
        static mut instance: ::protobuf::lazy::Lazy<Scenario> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Scenario,
        };
        unsafe {
            instance.get(Scenario::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .grpc.testing.ClientConfig client_config = 2;

    pub fn clear_client_config(&mut self) {
        self.client_config.clear();
    }

    pub fn has_client_config(&self) -> bool {
        self.client_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_config(&mut self, v: ClientConfig) {
        self.client_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_config(&mut self) -> &mut ClientConfig {
        if self.client_config.is_none() {
            self.client_config.set_default();
        }
        self.client_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_config(&mut self) -> ClientConfig {
        self.client_config.take().unwrap_or_else(|| ClientConfig::new())
    }

    pub fn get_client_config(&self) -> &ClientConfig {
        self.client_config.as_ref().unwrap_or_else(|| ClientConfig::default_instance())
    }

    fn get_client_config_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientConfig> {
        &self.client_config
    }

    fn mut_client_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientConfig> {
        &mut self.client_config
    }

    // int32 num_clients = 3;

    pub fn clear_num_clients(&mut self) {
        self.num_clients = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_clients(&mut self, v: i32) {
        self.num_clients = v;
    }

    pub fn get_num_clients(&self) -> i32 {
        self.num_clients
    }

    fn get_num_clients_for_reflect(&self) -> &i32 {
        &self.num_clients
    }

    fn mut_num_clients_for_reflect(&mut self) -> &mut i32 {
        &mut self.num_clients
    }

    // .grpc.testing.ServerConfig server_config = 4;

    pub fn clear_server_config(&mut self) {
        self.server_config.clear();
    }

    pub fn has_server_config(&self) -> bool {
        self.server_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_config(&mut self, v: ServerConfig) {
        self.server_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_config(&mut self) -> &mut ServerConfig {
        if self.server_config.is_none() {
            self.server_config.set_default();
        }
        self.server_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_config(&mut self) -> ServerConfig {
        self.server_config.take().unwrap_or_else(|| ServerConfig::new())
    }

    pub fn get_server_config(&self) -> &ServerConfig {
        self.server_config.as_ref().unwrap_or_else(|| ServerConfig::default_instance())
    }

    fn get_server_config_for_reflect(&self) -> &::protobuf::SingularPtrField<ServerConfig> {
        &self.server_config
    }

    fn mut_server_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ServerConfig> {
        &mut self.server_config
    }

    // int32 num_servers = 5;

    pub fn clear_num_servers(&mut self) {
        self.num_servers = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_servers(&mut self, v: i32) {
        self.num_servers = v;
    }

    pub fn get_num_servers(&self) -> i32 {
        self.num_servers
    }

    fn get_num_servers_for_reflect(&self) -> &i32 {
        &self.num_servers
    }

    fn mut_num_servers_for_reflect(&mut self) -> &mut i32 {
        &mut self.num_servers
    }

    // int32 warmup_seconds = 6;

    pub fn clear_warmup_seconds(&mut self) {
        self.warmup_seconds = 0;
    }

    // Param is passed by value, moved
    pub fn set_warmup_seconds(&mut self, v: i32) {
        self.warmup_seconds = v;
    }

    pub fn get_warmup_seconds(&self) -> i32 {
        self.warmup_seconds
    }

    fn get_warmup_seconds_for_reflect(&self) -> &i32 {
        &self.warmup_seconds
    }

    fn mut_warmup_seconds_for_reflect(&mut self) -> &mut i32 {
        &mut self.warmup_seconds
    }

    // int32 benchmark_seconds = 7;

    pub fn clear_benchmark_seconds(&mut self) {
        self.benchmark_seconds = 0;
    }

    // Param is passed by value, moved
    pub fn set_benchmark_seconds(&mut self, v: i32) {
        self.benchmark_seconds = v;
    }

    pub fn get_benchmark_seconds(&self) -> i32 {
        self.benchmark_seconds
    }

    fn get_benchmark_seconds_for_reflect(&self) -> &i32 {
        &self.benchmark_seconds
    }

    fn mut_benchmark_seconds_for_reflect(&mut self) -> &mut i32 {
        &mut self.benchmark_seconds
    }

    // int32 spawn_local_worker_count = 8;

    pub fn clear_spawn_local_worker_count(&mut self) {
        self.spawn_local_worker_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_spawn_local_worker_count(&mut self, v: i32) {
        self.spawn_local_worker_count = v;
    }

    pub fn get_spawn_local_worker_count(&self) -> i32 {
        self.spawn_local_worker_count
    }

    fn get_spawn_local_worker_count_for_reflect(&self) -> &i32 {
        &self.spawn_local_worker_count
    }

    fn mut_spawn_local_worker_count_for_reflect(&mut self) -> &mut i32 {
        &mut self.spawn_local_worker_count
    }
}

impl ::protobuf::Message for Scenario {
    fn is_initialized(&self) -> bool {
        for v in &self.client_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.server_config {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client_config)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_clients = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.server_config)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_servers = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.warmup_seconds = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.benchmark_seconds = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.spawn_local_worker_count = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.client_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.num_clients != 0 {
            my_size += ::protobuf::rt::value_size(3, self.num_clients, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.server_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.num_servers != 0 {
            my_size += ::protobuf::rt::value_size(5, self.num_servers, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.warmup_seconds != 0 {
            my_size += ::protobuf::rt::value_size(6, self.warmup_seconds, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.benchmark_seconds != 0 {
            my_size += ::protobuf::rt::value_size(7, self.benchmark_seconds, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.spawn_local_worker_count != 0 {
            my_size += ::protobuf::rt::value_size(8, self.spawn_local_worker_count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.client_config.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.num_clients != 0 {
            os.write_int32(3, self.num_clients)?;
        }
        if let Some(ref v) = self.server_config.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.num_servers != 0 {
            os.write_int32(5, self.num_servers)?;
        }
        if self.warmup_seconds != 0 {
            os.write_int32(6, self.warmup_seconds)?;
        }
        if self.benchmark_seconds != 0 {
            os.write_int32(7, self.benchmark_seconds)?;
        }
        if self.spawn_local_worker_count != 0 {
            os.write_int32(8, self.spawn_local_worker_count)?;
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

impl ::protobuf::MessageStatic for Scenario {
    fn new() -> Scenario {
        Scenario::new()
    }

    fn descriptor_static(_: ::std::option::Option<Scenario>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Scenario::get_name_for_reflect,
                    Scenario::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientConfig>>(
                    "client_config",
                    Scenario::get_client_config_for_reflect,
                    Scenario::mut_client_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_clients",
                    Scenario::get_num_clients_for_reflect,
                    Scenario::mut_num_clients_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ServerConfig>>(
                    "server_config",
                    Scenario::get_server_config_for_reflect,
                    Scenario::mut_server_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_servers",
                    Scenario::get_num_servers_for_reflect,
                    Scenario::mut_num_servers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "warmup_seconds",
                    Scenario::get_warmup_seconds_for_reflect,
                    Scenario::mut_warmup_seconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "benchmark_seconds",
                    Scenario::get_benchmark_seconds_for_reflect,
                    Scenario::mut_benchmark_seconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "spawn_local_worker_count",
                    Scenario::get_spawn_local_worker_count_for_reflect,
                    Scenario::mut_spawn_local_worker_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Scenario>(
                    "Scenario",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Scenario {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_client_config();
        self.clear_num_clients();
        self.clear_server_config();
        self.clear_num_servers();
        self.clear_warmup_seconds();
        self.clear_benchmark_seconds();
        self.clear_spawn_local_worker_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Scenario {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Scenario {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Scenarios {
    // message fields
    pub scenarios: ::protobuf::RepeatedField<Scenario>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Scenarios {}

impl Scenarios {
    pub fn new() -> Scenarios {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Scenarios {
        static mut instance: ::protobuf::lazy::Lazy<Scenarios> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Scenarios,
        };
        unsafe {
            instance.get(Scenarios::new)
        }
    }

    // repeated .grpc.testing.Scenario scenarios = 1;

    pub fn clear_scenarios(&mut self) {
        self.scenarios.clear();
    }

    // Param is passed by value, moved
    pub fn set_scenarios(&mut self, v: ::protobuf::RepeatedField<Scenario>) {
        self.scenarios = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scenarios(&mut self) -> &mut ::protobuf::RepeatedField<Scenario> {
        &mut self.scenarios
    }

    // Take field
    pub fn take_scenarios(&mut self) -> ::protobuf::RepeatedField<Scenario> {
        ::std::mem::replace(&mut self.scenarios, ::protobuf::RepeatedField::new())
    }

    pub fn get_scenarios(&self) -> &[Scenario] {
        &self.scenarios
    }

    fn get_scenarios_for_reflect(&self) -> &::protobuf::RepeatedField<Scenario> {
        &self.scenarios
    }

    fn mut_scenarios_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Scenario> {
        &mut self.scenarios
    }
}

impl ::protobuf::Message for Scenarios {
    fn is_initialized(&self) -> bool {
        for v in &self.scenarios {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.scenarios)?;
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
        for value in &self.scenarios {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.scenarios {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Scenarios {
    fn new() -> Scenarios {
        Scenarios::new()
    }

    fn descriptor_static(_: ::std::option::Option<Scenarios>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Scenario>>(
                    "scenarios",
                    Scenarios::get_scenarios_for_reflect,
                    Scenarios::mut_scenarios_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Scenarios>(
                    "Scenarios",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Scenarios {
    fn clear(&mut self) {
        self.clear_scenarios();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Scenarios {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Scenarios {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScenarioResultSummary {
    // message fields
    pub qps: f64,
    pub qps_per_server_core: f64,
    pub server_system_time: f64,
    pub server_user_time: f64,
    pub client_system_time: f64,
    pub client_user_time: f64,
    pub latency_50: f64,
    pub latency_90: f64,
    pub latency_95: f64,
    pub latency_99: f64,
    pub latency_999: f64,
    pub server_cpu_usage: f64,
    pub successful_requests_per_second: f64,
    pub failed_requests_per_second: f64,
    pub client_polls_per_request: f64,
    pub server_polls_per_request: f64,
    pub server_queries_per_cpu_sec: f64,
    pub client_queries_per_cpu_sec: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScenarioResultSummary {}

impl ScenarioResultSummary {
    pub fn new() -> ScenarioResultSummary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScenarioResultSummary {
        static mut instance: ::protobuf::lazy::Lazy<ScenarioResultSummary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScenarioResultSummary,
        };
        unsafe {
            instance.get(ScenarioResultSummary::new)
        }
    }

    // double qps = 1;

    pub fn clear_qps(&mut self) {
        self.qps = 0.;
    }

    // Param is passed by value, moved
    pub fn set_qps(&mut self, v: f64) {
        self.qps = v;
    }

    pub fn get_qps(&self) -> f64 {
        self.qps
    }

    fn get_qps_for_reflect(&self) -> &f64 {
        &self.qps
    }

    fn mut_qps_for_reflect(&mut self) -> &mut f64 {
        &mut self.qps
    }

    // double qps_per_server_core = 2;

    pub fn clear_qps_per_server_core(&mut self) {
        self.qps_per_server_core = 0.;
    }

    // Param is passed by value, moved
    pub fn set_qps_per_server_core(&mut self, v: f64) {
        self.qps_per_server_core = v;
    }

    pub fn get_qps_per_server_core(&self) -> f64 {
        self.qps_per_server_core
    }

    fn get_qps_per_server_core_for_reflect(&self) -> &f64 {
        &self.qps_per_server_core
    }

    fn mut_qps_per_server_core_for_reflect(&mut self) -> &mut f64 {
        &mut self.qps_per_server_core
    }

    // double server_system_time = 3;

    pub fn clear_server_system_time(&mut self) {
        self.server_system_time = 0.;
    }

    // Param is passed by value, moved
    pub fn set_server_system_time(&mut self, v: f64) {
        self.server_system_time = v;
    }

    pub fn get_server_system_time(&self) -> f64 {
        self.server_system_time
    }

    fn get_server_system_time_for_reflect(&self) -> &f64 {
        &self.server_system_time
    }

    fn mut_server_system_time_for_reflect(&mut self) -> &mut f64 {
        &mut self.server_system_time
    }

    // double server_user_time = 4;

    pub fn clear_server_user_time(&mut self) {
        self.server_user_time = 0.;
    }

    // Param is passed by value, moved
    pub fn set_server_user_time(&mut self, v: f64) {
        self.server_user_time = v;
    }

    pub fn get_server_user_time(&self) -> f64 {
        self.server_user_time
    }

    fn get_server_user_time_for_reflect(&self) -> &f64 {
        &self.server_user_time
    }

    fn mut_server_user_time_for_reflect(&mut self) -> &mut f64 {
        &mut self.server_user_time
    }

    // double client_system_time = 5;

    pub fn clear_client_system_time(&mut self) {
        self.client_system_time = 0.;
    }

    // Param is passed by value, moved
    pub fn set_client_system_time(&mut self, v: f64) {
        self.client_system_time = v;
    }

    pub fn get_client_system_time(&self) -> f64 {
        self.client_system_time
    }

    fn get_client_system_time_for_reflect(&self) -> &f64 {
        &self.client_system_time
    }

    fn mut_client_system_time_for_reflect(&mut self) -> &mut f64 {
        &mut self.client_system_time
    }

    // double client_user_time = 6;

    pub fn clear_client_user_time(&mut self) {
        self.client_user_time = 0.;
    }

    // Param is passed by value, moved
    pub fn set_client_user_time(&mut self, v: f64) {
        self.client_user_time = v;
    }

    pub fn get_client_user_time(&self) -> f64 {
        self.client_user_time
    }

    fn get_client_user_time_for_reflect(&self) -> &f64 {
        &self.client_user_time
    }

    fn mut_client_user_time_for_reflect(&mut self) -> &mut f64 {
        &mut self.client_user_time
    }

    // double latency_50 = 7;

    pub fn clear_latency_50(&mut self) {
        self.latency_50 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_latency_50(&mut self, v: f64) {
        self.latency_50 = v;
    }

    pub fn get_latency_50(&self) -> f64 {
        self.latency_50
    }

    fn get_latency_50_for_reflect(&self) -> &f64 {
        &self.latency_50
    }

    fn mut_latency_50_for_reflect(&mut self) -> &mut f64 {
        &mut self.latency_50
    }

    // double latency_90 = 8;

    pub fn clear_latency_90(&mut self) {
        self.latency_90 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_latency_90(&mut self, v: f64) {
        self.latency_90 = v;
    }

    pub fn get_latency_90(&self) -> f64 {
        self.latency_90
    }

    fn get_latency_90_for_reflect(&self) -> &f64 {
        &self.latency_90
    }

    fn mut_latency_90_for_reflect(&mut self) -> &mut f64 {
        &mut self.latency_90
    }

    // double latency_95 = 9;

    pub fn clear_latency_95(&mut self) {
        self.latency_95 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_latency_95(&mut self, v: f64) {
        self.latency_95 = v;
    }

    pub fn get_latency_95(&self) -> f64 {
        self.latency_95
    }

    fn get_latency_95_for_reflect(&self) -> &f64 {
        &self.latency_95
    }

    fn mut_latency_95_for_reflect(&mut self) -> &mut f64 {
        &mut self.latency_95
    }

    // double latency_99 = 10;

    pub fn clear_latency_99(&mut self) {
        self.latency_99 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_latency_99(&mut self, v: f64) {
        self.latency_99 = v;
    }

    pub fn get_latency_99(&self) -> f64 {
        self.latency_99
    }

    fn get_latency_99_for_reflect(&self) -> &f64 {
        &self.latency_99
    }

    fn mut_latency_99_for_reflect(&mut self) -> &mut f64 {
        &mut self.latency_99
    }

    // double latency_999 = 11;

    pub fn clear_latency_999(&mut self) {
        self.latency_999 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_latency_999(&mut self, v: f64) {
        self.latency_999 = v;
    }

    pub fn get_latency_999(&self) -> f64 {
        self.latency_999
    }

    fn get_latency_999_for_reflect(&self) -> &f64 {
        &self.latency_999
    }

    fn mut_latency_999_for_reflect(&mut self) -> &mut f64 {
        &mut self.latency_999
    }

    // double server_cpu_usage = 12;

    pub fn clear_server_cpu_usage(&mut self) {
        self.server_cpu_usage = 0.;
    }

    // Param is passed by value, moved
    pub fn set_server_cpu_usage(&mut self, v: f64) {
        self.server_cpu_usage = v;
    }

    pub fn get_server_cpu_usage(&self) -> f64 {
        self.server_cpu_usage
    }

    fn get_server_cpu_usage_for_reflect(&self) -> &f64 {
        &self.server_cpu_usage
    }

    fn mut_server_cpu_usage_for_reflect(&mut self) -> &mut f64 {
        &mut self.server_cpu_usage
    }

    // double successful_requests_per_second = 13;

    pub fn clear_successful_requests_per_second(&mut self) {
        self.successful_requests_per_second = 0.;
    }

    // Param is passed by value, moved
    pub fn set_successful_requests_per_second(&mut self, v: f64) {
        self.successful_requests_per_second = v;
    }

    pub fn get_successful_requests_per_second(&self) -> f64 {
        self.successful_requests_per_second
    }

    fn get_successful_requests_per_second_for_reflect(&self) -> &f64 {
        &self.successful_requests_per_second
    }

    fn mut_successful_requests_per_second_for_reflect(&mut self) -> &mut f64 {
        &mut self.successful_requests_per_second
    }

    // double failed_requests_per_second = 14;

    pub fn clear_failed_requests_per_second(&mut self) {
        self.failed_requests_per_second = 0.;
    }

    // Param is passed by value, moved
    pub fn set_failed_requests_per_second(&mut self, v: f64) {
        self.failed_requests_per_second = v;
    }

    pub fn get_failed_requests_per_second(&self) -> f64 {
        self.failed_requests_per_second
    }

    fn get_failed_requests_per_second_for_reflect(&self) -> &f64 {
        &self.failed_requests_per_second
    }

    fn mut_failed_requests_per_second_for_reflect(&mut self) -> &mut f64 {
        &mut self.failed_requests_per_second
    }

    // double client_polls_per_request = 15;

    pub fn clear_client_polls_per_request(&mut self) {
        self.client_polls_per_request = 0.;
    }

    // Param is passed by value, moved
    pub fn set_client_polls_per_request(&mut self, v: f64) {
        self.client_polls_per_request = v;
    }

    pub fn get_client_polls_per_request(&self) -> f64 {
        self.client_polls_per_request
    }

    fn get_client_polls_per_request_for_reflect(&self) -> &f64 {
        &self.client_polls_per_request
    }

    fn mut_client_polls_per_request_for_reflect(&mut self) -> &mut f64 {
        &mut self.client_polls_per_request
    }

    // double server_polls_per_request = 16;

    pub fn clear_server_polls_per_request(&mut self) {
        self.server_polls_per_request = 0.;
    }

    // Param is passed by value, moved
    pub fn set_server_polls_per_request(&mut self, v: f64) {
        self.server_polls_per_request = v;
    }

    pub fn get_server_polls_per_request(&self) -> f64 {
        self.server_polls_per_request
    }

    fn get_server_polls_per_request_for_reflect(&self) -> &f64 {
        &self.server_polls_per_request
    }

    fn mut_server_polls_per_request_for_reflect(&mut self) -> &mut f64 {
        &mut self.server_polls_per_request
    }

    // double server_queries_per_cpu_sec = 17;

    pub fn clear_server_queries_per_cpu_sec(&mut self) {
        self.server_queries_per_cpu_sec = 0.;
    }

    // Param is passed by value, moved
    pub fn set_server_queries_per_cpu_sec(&mut self, v: f64) {
        self.server_queries_per_cpu_sec = v;
    }

    pub fn get_server_queries_per_cpu_sec(&self) -> f64 {
        self.server_queries_per_cpu_sec
    }

    fn get_server_queries_per_cpu_sec_for_reflect(&self) -> &f64 {
        &self.server_queries_per_cpu_sec
    }

    fn mut_server_queries_per_cpu_sec_for_reflect(&mut self) -> &mut f64 {
        &mut self.server_queries_per_cpu_sec
    }

    // double client_queries_per_cpu_sec = 18;

    pub fn clear_client_queries_per_cpu_sec(&mut self) {
        self.client_queries_per_cpu_sec = 0.;
    }

    // Param is passed by value, moved
    pub fn set_client_queries_per_cpu_sec(&mut self, v: f64) {
        self.client_queries_per_cpu_sec = v;
    }

    pub fn get_client_queries_per_cpu_sec(&self) -> f64 {
        self.client_queries_per_cpu_sec
    }

    fn get_client_queries_per_cpu_sec_for_reflect(&self) -> &f64 {
        &self.client_queries_per_cpu_sec
    }

    fn mut_client_queries_per_cpu_sec_for_reflect(&mut self) -> &mut f64 {
        &mut self.client_queries_per_cpu_sec
    }
}

impl ::protobuf::Message for ScenarioResultSummary {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.qps = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.qps_per_server_core = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.server_system_time = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.server_user_time = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.client_system_time = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.client_user_time = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.latency_50 = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.latency_90 = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.latency_95 = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.latency_99 = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.latency_999 = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.server_cpu_usage = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.successful_requests_per_second = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.failed_requests_per_second = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.client_polls_per_request = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.server_polls_per_request = tmp;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.server_queries_per_cpu_sec = tmp;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.client_queries_per_cpu_sec = tmp;
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
        if self.qps != 0. {
            my_size += 9;
        }
        if self.qps_per_server_core != 0. {
            my_size += 9;
        }
        if self.server_system_time != 0. {
            my_size += 9;
        }
        if self.server_user_time != 0. {
            my_size += 9;
        }
        if self.client_system_time != 0. {
            my_size += 9;
        }
        if self.client_user_time != 0. {
            my_size += 9;
        }
        if self.latency_50 != 0. {
            my_size += 9;
        }
        if self.latency_90 != 0. {
            my_size += 9;
        }
        if self.latency_95 != 0. {
            my_size += 9;
        }
        if self.latency_99 != 0. {
            my_size += 9;
        }
        if self.latency_999 != 0. {
            my_size += 9;
        }
        if self.server_cpu_usage != 0. {
            my_size += 9;
        }
        if self.successful_requests_per_second != 0. {
            my_size += 9;
        }
        if self.failed_requests_per_second != 0. {
            my_size += 9;
        }
        if self.client_polls_per_request != 0. {
            my_size += 9;
        }
        if self.server_polls_per_request != 0. {
            my_size += 10;
        }
        if self.server_queries_per_cpu_sec != 0. {
            my_size += 10;
        }
        if self.client_queries_per_cpu_sec != 0. {
            my_size += 10;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.qps != 0. {
            os.write_double(1, self.qps)?;
        }
        if self.qps_per_server_core != 0. {
            os.write_double(2, self.qps_per_server_core)?;
        }
        if self.server_system_time != 0. {
            os.write_double(3, self.server_system_time)?;
        }
        if self.server_user_time != 0. {
            os.write_double(4, self.server_user_time)?;
        }
        if self.client_system_time != 0. {
            os.write_double(5, self.client_system_time)?;
        }
        if self.client_user_time != 0. {
            os.write_double(6, self.client_user_time)?;
        }
        if self.latency_50 != 0. {
            os.write_double(7, self.latency_50)?;
        }
        if self.latency_90 != 0. {
            os.write_double(8, self.latency_90)?;
        }
        if self.latency_95 != 0. {
            os.write_double(9, self.latency_95)?;
        }
        if self.latency_99 != 0. {
            os.write_double(10, self.latency_99)?;
        }
        if self.latency_999 != 0. {
            os.write_double(11, self.latency_999)?;
        }
        if self.server_cpu_usage != 0. {
            os.write_double(12, self.server_cpu_usage)?;
        }
        if self.successful_requests_per_second != 0. {
            os.write_double(13, self.successful_requests_per_second)?;
        }
        if self.failed_requests_per_second != 0. {
            os.write_double(14, self.failed_requests_per_second)?;
        }
        if self.client_polls_per_request != 0. {
            os.write_double(15, self.client_polls_per_request)?;
        }
        if self.server_polls_per_request != 0. {
            os.write_double(16, self.server_polls_per_request)?;
        }
        if self.server_queries_per_cpu_sec != 0. {
            os.write_double(17, self.server_queries_per_cpu_sec)?;
        }
        if self.client_queries_per_cpu_sec != 0. {
            os.write_double(18, self.client_queries_per_cpu_sec)?;
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

impl ::protobuf::MessageStatic for ScenarioResultSummary {
    fn new() -> ScenarioResultSummary {
        ScenarioResultSummary::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScenarioResultSummary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "qps",
                    ScenarioResultSummary::get_qps_for_reflect,
                    ScenarioResultSummary::mut_qps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "qps_per_server_core",
                    ScenarioResultSummary::get_qps_per_server_core_for_reflect,
                    ScenarioResultSummary::mut_qps_per_server_core_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "server_system_time",
                    ScenarioResultSummary::get_server_system_time_for_reflect,
                    ScenarioResultSummary::mut_server_system_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "server_user_time",
                    ScenarioResultSummary::get_server_user_time_for_reflect,
                    ScenarioResultSummary::mut_server_user_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "client_system_time",
                    ScenarioResultSummary::get_client_system_time_for_reflect,
                    ScenarioResultSummary::mut_client_system_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "client_user_time",
                    ScenarioResultSummary::get_client_user_time_for_reflect,
                    ScenarioResultSummary::mut_client_user_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "latency_50",
                    ScenarioResultSummary::get_latency_50_for_reflect,
                    ScenarioResultSummary::mut_latency_50_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "latency_90",
                    ScenarioResultSummary::get_latency_90_for_reflect,
                    ScenarioResultSummary::mut_latency_90_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "latency_95",
                    ScenarioResultSummary::get_latency_95_for_reflect,
                    ScenarioResultSummary::mut_latency_95_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "latency_99",
                    ScenarioResultSummary::get_latency_99_for_reflect,
                    ScenarioResultSummary::mut_latency_99_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "latency_999",
                    ScenarioResultSummary::get_latency_999_for_reflect,
                    ScenarioResultSummary::mut_latency_999_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "server_cpu_usage",
                    ScenarioResultSummary::get_server_cpu_usage_for_reflect,
                    ScenarioResultSummary::mut_server_cpu_usage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "successful_requests_per_second",
                    ScenarioResultSummary::get_successful_requests_per_second_for_reflect,
                    ScenarioResultSummary::mut_successful_requests_per_second_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "failed_requests_per_second",
                    ScenarioResultSummary::get_failed_requests_per_second_for_reflect,
                    ScenarioResultSummary::mut_failed_requests_per_second_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "client_polls_per_request",
                    ScenarioResultSummary::get_client_polls_per_request_for_reflect,
                    ScenarioResultSummary::mut_client_polls_per_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "server_polls_per_request",
                    ScenarioResultSummary::get_server_polls_per_request_for_reflect,
                    ScenarioResultSummary::mut_server_polls_per_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "server_queries_per_cpu_sec",
                    ScenarioResultSummary::get_server_queries_per_cpu_sec_for_reflect,
                    ScenarioResultSummary::mut_server_queries_per_cpu_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "client_queries_per_cpu_sec",
                    ScenarioResultSummary::get_client_queries_per_cpu_sec_for_reflect,
                    ScenarioResultSummary::mut_client_queries_per_cpu_sec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScenarioResultSummary>(
                    "ScenarioResultSummary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScenarioResultSummary {
    fn clear(&mut self) {
        self.clear_qps();
        self.clear_qps_per_server_core();
        self.clear_server_system_time();
        self.clear_server_user_time();
        self.clear_client_system_time();
        self.clear_client_user_time();
        self.clear_latency_50();
        self.clear_latency_90();
        self.clear_latency_95();
        self.clear_latency_99();
        self.clear_latency_999();
        self.clear_server_cpu_usage();
        self.clear_successful_requests_per_second();
        self.clear_failed_requests_per_second();
        self.clear_client_polls_per_request();
        self.clear_server_polls_per_request();
        self.clear_server_queries_per_cpu_sec();
        self.clear_client_queries_per_cpu_sec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScenarioResultSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScenarioResultSummary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScenarioResult {
    // message fields
    pub scenario: ::protobuf::SingularPtrField<Scenario>,
    pub latencies: ::protobuf::SingularPtrField<super::stats::HistogramData>,
    pub client_stats: ::protobuf::RepeatedField<super::stats::ClientStats>,
    pub server_stats: ::protobuf::RepeatedField<super::stats::ServerStats>,
    pub server_cores: ::std::vec::Vec<i32>,
    pub summary: ::protobuf::SingularPtrField<ScenarioResultSummary>,
    pub client_success: ::std::vec::Vec<bool>,
    pub server_success: ::std::vec::Vec<bool>,
    pub request_results: ::protobuf::RepeatedField<super::stats::RequestResultCount>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScenarioResult {}

impl ScenarioResult {
    pub fn new() -> ScenarioResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScenarioResult {
        static mut instance: ::protobuf::lazy::Lazy<ScenarioResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScenarioResult,
        };
        unsafe {
            instance.get(ScenarioResult::new)
        }
    }

    // .grpc.testing.Scenario scenario = 1;

    pub fn clear_scenario(&mut self) {
        self.scenario.clear();
    }

    pub fn has_scenario(&self) -> bool {
        self.scenario.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scenario(&mut self, v: Scenario) {
        self.scenario = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scenario(&mut self) -> &mut Scenario {
        if self.scenario.is_none() {
            self.scenario.set_default();
        }
        self.scenario.as_mut().unwrap()
    }

    // Take field
    pub fn take_scenario(&mut self) -> Scenario {
        self.scenario.take().unwrap_or_else(|| Scenario::new())
    }

    pub fn get_scenario(&self) -> &Scenario {
        self.scenario.as_ref().unwrap_or_else(|| Scenario::default_instance())
    }

    fn get_scenario_for_reflect(&self) -> &::protobuf::SingularPtrField<Scenario> {
        &self.scenario
    }

    fn mut_scenario_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Scenario> {
        &mut self.scenario
    }

    // .grpc.testing.HistogramData latencies = 2;

    pub fn clear_latencies(&mut self) {
        self.latencies.clear();
    }

    pub fn has_latencies(&self) -> bool {
        self.latencies.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latencies(&mut self, v: super::stats::HistogramData) {
        self.latencies = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_latencies(&mut self) -> &mut super::stats::HistogramData {
        if self.latencies.is_none() {
            self.latencies.set_default();
        }
        self.latencies.as_mut().unwrap()
    }

    // Take field
    pub fn take_latencies(&mut self) -> super::stats::HistogramData {
        self.latencies.take().unwrap_or_else(|| super::stats::HistogramData::new())
    }

    pub fn get_latencies(&self) -> &super::stats::HistogramData {
        self.latencies.as_ref().unwrap_or_else(|| super::stats::HistogramData::default_instance())
    }

    fn get_latencies_for_reflect(&self) -> &::protobuf::SingularPtrField<super::stats::HistogramData> {
        &self.latencies
    }

    fn mut_latencies_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::stats::HistogramData> {
        &mut self.latencies
    }

    // repeated .grpc.testing.ClientStats client_stats = 3;

    pub fn clear_client_stats(&mut self) {
        self.client_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_client_stats(&mut self, v: ::protobuf::RepeatedField<super::stats::ClientStats>) {
        self.client_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_client_stats(&mut self) -> &mut ::protobuf::RepeatedField<super::stats::ClientStats> {
        &mut self.client_stats
    }

    // Take field
    pub fn take_client_stats(&mut self) -> ::protobuf::RepeatedField<super::stats::ClientStats> {
        ::std::mem::replace(&mut self.client_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_client_stats(&self) -> &[super::stats::ClientStats] {
        &self.client_stats
    }

    fn get_client_stats_for_reflect(&self) -> &::protobuf::RepeatedField<super::stats::ClientStats> {
        &self.client_stats
    }

    fn mut_client_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::stats::ClientStats> {
        &mut self.client_stats
    }

    // repeated .grpc.testing.ServerStats server_stats = 4;

    pub fn clear_server_stats(&mut self) {
        self.server_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_stats(&mut self, v: ::protobuf::RepeatedField<super::stats::ServerStats>) {
        self.server_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_server_stats(&mut self) -> &mut ::protobuf::RepeatedField<super::stats::ServerStats> {
        &mut self.server_stats
    }

    // Take field
    pub fn take_server_stats(&mut self) -> ::protobuf::RepeatedField<super::stats::ServerStats> {
        ::std::mem::replace(&mut self.server_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_server_stats(&self) -> &[super::stats::ServerStats] {
        &self.server_stats
    }

    fn get_server_stats_for_reflect(&self) -> &::protobuf::RepeatedField<super::stats::ServerStats> {
        &self.server_stats
    }

    fn mut_server_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::stats::ServerStats> {
        &mut self.server_stats
    }

    // repeated int32 server_cores = 5;

    pub fn clear_server_cores(&mut self) {
        self.server_cores.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_cores(&mut self, v: ::std::vec::Vec<i32>) {
        self.server_cores = v;
    }

    // Mutable pointer to the field.
    pub fn mut_server_cores(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.server_cores
    }

    // Take field
    pub fn take_server_cores(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.server_cores, ::std::vec::Vec::new())
    }

    pub fn get_server_cores(&self) -> &[i32] {
        &self.server_cores
    }

    fn get_server_cores_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.server_cores
    }

    fn mut_server_cores_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.server_cores
    }

    // .grpc.testing.ScenarioResultSummary summary = 6;

    pub fn clear_summary(&mut self) {
        self.summary.clear();
    }

    pub fn has_summary(&self) -> bool {
        self.summary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_summary(&mut self, v: ScenarioResultSummary) {
        self.summary = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_summary(&mut self) -> &mut ScenarioResultSummary {
        if self.summary.is_none() {
            self.summary.set_default();
        }
        self.summary.as_mut().unwrap()
    }

    // Take field
    pub fn take_summary(&mut self) -> ScenarioResultSummary {
        self.summary.take().unwrap_or_else(|| ScenarioResultSummary::new())
    }

    pub fn get_summary(&self) -> &ScenarioResultSummary {
        self.summary.as_ref().unwrap_or_else(|| ScenarioResultSummary::default_instance())
    }

    fn get_summary_for_reflect(&self) -> &::protobuf::SingularPtrField<ScenarioResultSummary> {
        &self.summary
    }

    fn mut_summary_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ScenarioResultSummary> {
        &mut self.summary
    }

    // repeated bool client_success = 7;

    pub fn clear_client_success(&mut self) {
        self.client_success.clear();
    }

    // Param is passed by value, moved
    pub fn set_client_success(&mut self, v: ::std::vec::Vec<bool>) {
        self.client_success = v;
    }

    // Mutable pointer to the field.
    pub fn mut_client_success(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.client_success
    }

    // Take field
    pub fn take_client_success(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.client_success, ::std::vec::Vec::new())
    }

    pub fn get_client_success(&self) -> &[bool] {
        &self.client_success
    }

    fn get_client_success_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.client_success
    }

    fn mut_client_success_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.client_success
    }

    // repeated bool server_success = 8;

    pub fn clear_server_success(&mut self) {
        self.server_success.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_success(&mut self, v: ::std::vec::Vec<bool>) {
        self.server_success = v;
    }

    // Mutable pointer to the field.
    pub fn mut_server_success(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.server_success
    }

    // Take field
    pub fn take_server_success(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.server_success, ::std::vec::Vec::new())
    }

    pub fn get_server_success(&self) -> &[bool] {
        &self.server_success
    }

    fn get_server_success_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.server_success
    }

    fn mut_server_success_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.server_success
    }

    // repeated .grpc.testing.RequestResultCount request_results = 9;

    pub fn clear_request_results(&mut self) {
        self.request_results.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_results(&mut self, v: ::protobuf::RepeatedField<super::stats::RequestResultCount>) {
        self.request_results = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_results(&mut self) -> &mut ::protobuf::RepeatedField<super::stats::RequestResultCount> {
        &mut self.request_results
    }

    // Take field
    pub fn take_request_results(&mut self) -> ::protobuf::RepeatedField<super::stats::RequestResultCount> {
        ::std::mem::replace(&mut self.request_results, ::protobuf::RepeatedField::new())
    }

    pub fn get_request_results(&self) -> &[super::stats::RequestResultCount] {
        &self.request_results
    }

    fn get_request_results_for_reflect(&self) -> &::protobuf::RepeatedField<super::stats::RequestResultCount> {
        &self.request_results
    }

    fn mut_request_results_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::stats::RequestResultCount> {
        &mut self.request_results
    }
}

impl ::protobuf::Message for ScenarioResult {
    fn is_initialized(&self) -> bool {
        for v in &self.scenario {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.latencies {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.client_stats {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.server_stats {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.summary {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_results {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scenario)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.latencies)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.client_stats)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.server_stats)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.server_cores)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.summary)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.client_success)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.server_success)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request_results)?;
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
        if let Some(ref v) = self.scenario.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.latencies.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.client_stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.server_stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.server_cores {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(ref v) = self.summary.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += 2 * self.client_success.len() as u32;
        my_size += 2 * self.server_success.len() as u32;
        for value in &self.request_results {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.scenario.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.latencies.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.client_stats {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.server_stats {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.server_cores {
            os.write_int32(5, *v)?;
        };
        if let Some(ref v) = self.summary.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.client_success {
            os.write_bool(7, *v)?;
        };
        for v in &self.server_success {
            os.write_bool(8, *v)?;
        };
        for v in &self.request_results {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ScenarioResult {
    fn new() -> ScenarioResult {
        ScenarioResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScenarioResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Scenario>>(
                    "scenario",
                    ScenarioResult::get_scenario_for_reflect,
                    ScenarioResult::mut_scenario_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::stats::HistogramData>>(
                    "latencies",
                    ScenarioResult::get_latencies_for_reflect,
                    ScenarioResult::mut_latencies_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::stats::ClientStats>>(
                    "client_stats",
                    ScenarioResult::get_client_stats_for_reflect,
                    ScenarioResult::mut_client_stats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::stats::ServerStats>>(
                    "server_stats",
                    ScenarioResult::get_server_stats_for_reflect,
                    ScenarioResult::mut_server_stats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "server_cores",
                    ScenarioResult::get_server_cores_for_reflect,
                    ScenarioResult::mut_server_cores_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScenarioResultSummary>>(
                    "summary",
                    ScenarioResult::get_summary_for_reflect,
                    ScenarioResult::mut_summary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "client_success",
                    ScenarioResult::get_client_success_for_reflect,
                    ScenarioResult::mut_client_success_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "server_success",
                    ScenarioResult::get_server_success_for_reflect,
                    ScenarioResult::mut_server_success_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::stats::RequestResultCount>>(
                    "request_results",
                    ScenarioResult::get_request_results_for_reflect,
                    ScenarioResult::mut_request_results_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScenarioResult>(
                    "ScenarioResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScenarioResult {
    fn clear(&mut self) {
        self.clear_scenario();
        self.clear_latencies();
        self.clear_client_stats();
        self.clear_server_stats();
        self.clear_server_cores();
        self.clear_summary();
        self.clear_client_success();
        self.clear_server_success();
        self.clear_request_results();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScenarioResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScenarioResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ClientType {
    SYNC_CLIENT = 0,
    ASYNC_CLIENT = 1,
    OTHER_CLIENT = 2,
}

impl ::protobuf::ProtobufEnum for ClientType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ClientType> {
        match value {
            0 => ::std::option::Option::Some(ClientType::SYNC_CLIENT),
            1 => ::std::option::Option::Some(ClientType::ASYNC_CLIENT),
            2 => ::std::option::Option::Some(ClientType::OTHER_CLIENT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ClientType] = &[
            ClientType::SYNC_CLIENT,
            ClientType::ASYNC_CLIENT,
            ClientType::OTHER_CLIENT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ClientType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ClientType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ClientType {
}

impl ::std::default::Default for ClientType {
    fn default() -> Self {
        ClientType::SYNC_CLIENT
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ServerType {
    SYNC_SERVER = 0,
    ASYNC_SERVER = 1,
    ASYNC_GENERIC_SERVER = 2,
    OTHER_SERVER = 3,
}

impl ::protobuf::ProtobufEnum for ServerType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ServerType> {
        match value {
            0 => ::std::option::Option::Some(ServerType::SYNC_SERVER),
            1 => ::std::option::Option::Some(ServerType::ASYNC_SERVER),
            2 => ::std::option::Option::Some(ServerType::ASYNC_GENERIC_SERVER),
            3 => ::std::option::Option::Some(ServerType::OTHER_SERVER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ServerType] = &[
            ServerType::SYNC_SERVER,
            ServerType::ASYNC_SERVER,
            ServerType::ASYNC_GENERIC_SERVER,
            ServerType::OTHER_SERVER,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ServerType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ServerType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ServerType {
}

impl ::std::default::Default for ServerType {
    fn default() -> Self {
        ServerType::SYNC_SERVER
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpcType {
    UNARY = 0,
    STREAMING = 1,
    STREAMING_FROM_CLIENT = 2,
    STREAMING_FROM_SERVER = 3,
    STREAMING_BOTH_WAYS = 4,
}

impl ::protobuf::ProtobufEnum for RpcType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpcType> {
        match value {
            0 => ::std::option::Option::Some(RpcType::UNARY),
            1 => ::std::option::Option::Some(RpcType::STREAMING),
            2 => ::std::option::Option::Some(RpcType::STREAMING_FROM_CLIENT),
            3 => ::std::option::Option::Some(RpcType::STREAMING_FROM_SERVER),
            4 => ::std::option::Option::Some(RpcType::STREAMING_BOTH_WAYS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpcType] = &[
            RpcType::UNARY,
            RpcType::STREAMING,
            RpcType::STREAMING_FROM_CLIENT,
            RpcType::STREAMING_FROM_SERVER,
            RpcType::STREAMING_BOTH_WAYS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RpcType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpcType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpcType {
}

impl ::std::default::Default for RpcType {
    fn default() -> Self {
        RpcType::UNARY
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1agrpc/testing/control.proto\x12\x0cgrpc.testing\x1a\x1bgrpc/testing\
    /payloads.proto\x1a\x18grpc/testing/stats.proto\"2\n\rPoissonParams\x12!\
    \n\x0coffered_load\x18\x01\x20\x01(\x01R\x0bofferedLoad\"\x12\n\x10Close\
    dLoopParams\"\x90\x01\n\nLoadParams\x12A\n\x0bclosed_loop\x18\x01\x20\
    \x01(\x0b2\x1e.grpc.testing.ClosedLoopParamsH\0R\nclosedLoop\x127\n\x07p\
    oisson\x18\x02\x20\x01(\x0b2\x1b.grpc.testing.PoissonParamsH\0R\x07poiss\
    onB\x06\n\x04load\"\x7f\n\x0eSecurityParams\x12\x1e\n\x0buse_test_ca\x18\
    \x01\x20\x01(\x08R\tuseTestCa\x120\n\x14server_host_override\x18\x02\x20\
    \x01(\tR\x12serverHostOverride\x12\x1b\n\tcred_type\x18\x03\x20\x01(\tR\
    \x08credType\"g\n\nChannelArg\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04n\
    ame\x12\x1d\n\tstr_value\x18\x02\x20\x01(\tH\0R\x08strValue\x12\x1d\n\ti\
    nt_value\x18\x03\x20\x01(\x05H\0R\x08intValueB\x07\n\x05value\"\xc7\x06\
    \n\x0cClientConfig\x12%\n\x0eserver_targets\x18\x01\x20\x03(\tR\rserverT\
    argets\x129\n\x0bclient_type\x18\x02\x20\x01(\x0e2\x18.grpc.testing.Clie\
    ntTypeR\nclientType\x12E\n\x0fsecurity_params\x18\x03\x20\x01(\x0b2\x1c.\
    grpc.testing.SecurityParamsR\x0esecurityParams\x12?\n\x1coutstanding_rpc\
    s_per_channel\x18\x04\x20\x01(\x05R\x19outstandingRpcsPerChannel\x12'\n\
    \x0fclient_channels\x18\x05\x20\x01(\x05R\x0eclientChannels\x120\n\x14as\
    ync_client_threads\x18\x07\x20\x01(\x05R\x12asyncClientThreads\x120\n\
    \x08rpc_type\x18\x08\x20\x01(\x0e2\x15.grpc.testing.RpcTypeR\x07rpcType\
    \x129\n\x0bload_params\x18\n\x20\x01(\x0b2\x18.grpc.testing.LoadParamsR\
    \nloadParams\x12B\n\x0epayload_config\x18\x0b\x20\x01(\x0b2\x1b.grpc.tes\
    ting.PayloadConfigR\rpayloadConfig\x12H\n\x10histogram_params\x18\x0c\
    \x20\x01(\x0b2\x1d.grpc.testing.HistogramParamsR\x0fhistogramParams\x12\
    \x1b\n\tcore_list\x18\r\x20\x03(\x05R\x08coreList\x12\x1d\n\ncore_limit\
    \x18\x0e\x20\x01(\x05R\tcoreLimit\x12(\n\x10other_client_api\x18\x0f\x20\
    \x01(\tR\x0eotherClientApi\x12;\n\x0cchannel_args\x18\x10\x20\x03(\x0b2\
    \x18.grpc.testing.ChannelArgR\x0bchannelArgs\x12$\n\x0ethreads_per_cq\
    \x18\x11\x20\x01(\x05R\x0cthreadsPerCq\x12.\n\x13messages_per_stream\x18\
    \x12\x20\x01(\x05R\x11messagesPerStream\"?\n\x0cClientStatus\x12/\n\x05s\
    tats\x18\x01\x20\x01(\x0b2\x19.grpc.testing.ClientStatsR\x05stats\"\x1c\
    \n\x04Mark\x12\x14\n\x05reset\x18\x01\x20\x01(\x08R\x05reset\"u\n\nClien\
    tArgs\x122\n\x05setup\x18\x01\x20\x01(\x0b2\x1a.grpc.testing.ClientConfi\
    gH\0R\x05setup\x12(\n\x04mark\x18\x02\x20\x01(\x0b2\x12.grpc.testing.Mar\
    kH\0R\x04markB\t\n\x07argtype\"\x95\x04\n\x0cServerConfig\x129\n\x0bserv\
    er_type\x18\x01\x20\x01(\x0e2\x18.grpc.testing.ServerTypeR\nserverType\
    \x12E\n\x0fsecurity_params\x18\x02\x20\x01(\x0b2\x1c.grpc.testing.Securi\
    tyParamsR\x0esecurityParams\x12\x12\n\x04port\x18\x04\x20\x01(\x05R\x04p\
    ort\x120\n\x14async_server_threads\x18\x07\x20\x01(\x05R\x12asyncServerT\
    hreads\x12\x1d\n\ncore_limit\x18\x08\x20\x01(\x05R\tcoreLimit\x12B\n\x0e\
    payload_config\x18\t\x20\x01(\x0b2\x1b.grpc.testing.PayloadConfigR\rpayl\
    oadConfig\x12\x1b\n\tcore_list\x18\n\x20\x03(\x05R\x08coreList\x12(\n\
    \x10other_server_api\x18\x0b\x20\x01(\tR\x0eotherServerApi\x12$\n\x0ethr\
    eads_per_cq\x18\x0c\x20\x01(\x05R\x0cthreadsPerCq\x12/\n\x13resource_quo\
    ta_size\x18\xe9\x07\x20\x01(\x05R\x11resourceQuotaSize\x12<\n\x0cchannel\
    _args\x18\xea\x07\x20\x03(\x0b2\x18.grpc.testing.ChannelArgR\x0bchannelA\
    rgs\"u\n\nServerArgs\x122\n\x05setup\x18\x01\x20\x01(\x0b2\x1a.grpc.test\
    ing.ServerConfigH\0R\x05setup\x12(\n\x04mark\x18\x02\x20\x01(\x0b2\x12.g\
    rpc.testing.MarkH\0R\x04markB\t\n\x07argtype\"i\n\x0cServerStatus\x12/\n\
    \x05stats\x18\x01\x20\x01(\x0b2\x19.grpc.testing.ServerStatsR\x05stats\
    \x12\x12\n\x04port\x18\x02\x20\x01(\x05R\x04port\x12\x14\n\x05cores\x18\
    \x03\x20\x01(\x05R\x05cores\"\r\n\x0bCoreRequest\"$\n\x0cCoreResponse\
    \x12\x14\n\x05cores\x18\x01\x20\x01(\x05R\x05cores\"\x06\n\x04Void\"\xef\
    \x02\n\x08Scenario\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12?\n\
    \rclient_config\x18\x02\x20\x01(\x0b2\x1a.grpc.testing.ClientConfigR\x0c\
    clientConfig\x12\x1f\n\x0bnum_clients\x18\x03\x20\x01(\x05R\nnumClients\
    \x12?\n\rserver_config\x18\x04\x20\x01(\x0b2\x1a.grpc.testing.ServerConf\
    igR\x0cserverConfig\x12\x1f\n\x0bnum_servers\x18\x05\x20\x01(\x05R\nnumS\
    ervers\x12%\n\x0ewarmup_seconds\x18\x06\x20\x01(\x05R\rwarmupSeconds\x12\
    +\n\x11benchmark_seconds\x18\x07\x20\x01(\x05R\x10benchmarkSeconds\x127\
    \n\x18spawn_local_worker_count\x18\x08\x20\x01(\x05R\x15spawnLocalWorker\
    Count\"A\n\tScenarios\x124\n\tscenarios\x18\x01\x20\x03(\x0b2\x16.grpc.t\
    esting.ScenarioR\tscenarios\"\xbb\x06\n\x15ScenarioResultSummary\x12\x10\
    \n\x03qps\x18\x01\x20\x01(\x01R\x03qps\x12-\n\x13qps_per_server_core\x18\
    \x02\x20\x01(\x01R\x10qpsPerServerCore\x12,\n\x12server_system_time\x18\
    \x03\x20\x01(\x01R\x10serverSystemTime\x12(\n\x10server_user_time\x18\
    \x04\x20\x01(\x01R\x0eserverUserTime\x12,\n\x12client_system_time\x18\
    \x05\x20\x01(\x01R\x10clientSystemTime\x12(\n\x10client_user_time\x18\
    \x06\x20\x01(\x01R\x0eclientUserTime\x12\x1d\n\nlatency_50\x18\x07\x20\
    \x01(\x01R\tlatency50\x12\x1d\n\nlatency_90\x18\x08\x20\x01(\x01R\tlaten\
    cy90\x12\x1d\n\nlatency_95\x18\t\x20\x01(\x01R\tlatency95\x12\x1d\n\nlat\
    ency_99\x18\n\x20\x01(\x01R\tlatency99\x12\x1f\n\x0blatency_999\x18\x0b\
    \x20\x01(\x01R\nlatency999\x12(\n\x10server_cpu_usage\x18\x0c\x20\x01(\
    \x01R\x0eserverCpuUsage\x12C\n\x1esuccessful_requests_per_second\x18\r\
    \x20\x01(\x01R\x1bsuccessfulRequestsPerSecond\x12;\n\x1afailed_requests_\
    per_second\x18\x0e\x20\x01(\x01R\x17failedRequestsPerSecond\x127\n\x18cl\
    ient_polls_per_request\x18\x0f\x20\x01(\x01R\x15clientPollsPerRequest\
    \x127\n\x18server_polls_per_request\x18\x10\x20\x01(\x01R\x15serverPolls\
    PerRequest\x12:\n\x1aserver_queries_per_cpu_sec\x18\x11\x20\x01(\x01R\
    \x16serverQueriesPerCpuSec\x12:\n\x1aclient_queries_per_cpu_sec\x18\x12\
    \x20\x01(\x01R\x16clientQueriesPerCpuSec\"\xf6\x03\n\x0eScenarioResult\
    \x122\n\x08scenario\x18\x01\x20\x01(\x0b2\x16.grpc.testing.ScenarioR\x08\
    scenario\x129\n\tlatencies\x18\x02\x20\x01(\x0b2\x1b.grpc.testing.Histog\
    ramDataR\tlatencies\x12<\n\x0cclient_stats\x18\x03\x20\x03(\x0b2\x19.grp\
    c.testing.ClientStatsR\x0bclientStats\x12<\n\x0cserver_stats\x18\x04\x20\
    \x03(\x0b2\x19.grpc.testing.ServerStatsR\x0bserverStats\x12!\n\x0cserver\
    _cores\x18\x05\x20\x03(\x05R\x0bserverCores\x12=\n\x07summary\x18\x06\
    \x20\x01(\x0b2#.grpc.testing.ScenarioResultSummaryR\x07summary\x12%\n\
    \x0eclient_success\x18\x07\x20\x03(\x08R\rclientSuccess\x12%\n\x0eserver\
    _success\x18\x08\x20\x03(\x08R\rserverSuccess\x12I\n\x0frequest_results\
    \x18\t\x20\x03(\x0b2\x20.grpc.testing.RequestResultCountR\x0erequestResu\
    lts*A\n\nClientType\x12\x0f\n\x0bSYNC_CLIENT\x10\0\x12\x10\n\x0cASYNC_CL\
    IENT\x10\x01\x12\x10\n\x0cOTHER_CLIENT\x10\x02*[\n\nServerType\x12\x0f\n\
    \x0bSYNC_SERVER\x10\0\x12\x10\n\x0cASYNC_SERVER\x10\x01\x12\x18\n\x14ASY\
    NC_GENERIC_SERVER\x10\x02\x12\x10\n\x0cOTHER_SERVER\x10\x03*r\n\x07RpcTy\
    pe\x12\t\n\x05UNARY\x10\0\x12\r\n\tSTREAMING\x10\x01\x12\x19\n\x15STREAM\
    ING_FROM_CLIENT\x10\x02\x12\x19\n\x15STREAMING_FROM_SERVER\x10\x03\x12\
    \x17\n\x13STREAMING_BOTH_WAYS\x10\x04J\xdeX\n\x07\x12\x05\x0e\0\x8c\x02\
    \x01\n\xbf\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb4\x04\x20Copyright\x202015\
    \x20gRPC\x20authors.\n\n\x20Licensed\x20under\x20the\x20Apache\x20Licens\
    e,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20\
    use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20Lice\
    nse.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20a\
    t\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\
    \x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\
    \x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\x20Licen\
    se\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHO\
    UT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20\
    express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20sp\
    ecific\x20language\x20governing\x20permissions\x20and\n\x20limitations\
    \x20under\x20the\x20License.\n\n\t\n\x02\x03\0\x12\x03\x10\x07$\n\t\n\
    \x02\x03\x01\x12\x03\x11\x07!\n\x08\n\x01\x02\x12\x03\x13\x08\x14\n\n\n\
    \x02\x05\0\x12\x04\x15\0\x1b\x01\n\n\n\x03\x05\0\x01\x12\x03\x15\x05\x0f\
    \n\x80\x01\n\x04\x05\0\x02\0\x12\x03\x18\x02\x12\x1as\x20Many\x20languag\
    es\x20support\x20a\x20basic\x20distinction\x20between\x20using\n\x20sync\
    \x20or\x20async\x20client,\x20and\x20this\x20allows\x20the\x20specificat\
    ion\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x18\x02\r\n\x0c\n\x05\x05\0\
    \x02\0\x02\x12\x03\x18\x10\x11\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x19\x02\
    \x13\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x19\x02\x0e\n\x0c\n\x05\x05\0\
    \x02\x01\x02\x12\x03\x19\x11\x12\n7\n\x04\x05\0\x02\x02\x12\x03\x1a\x02\
    \x13\"*\x20used\x20for\x20some\x20language-specific\x20variants\n\n\x0c\
    \n\x05\x05\0\x02\x02\x01\x12\x03\x1a\x02\x0e\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x03\x1a\x11\x12\n\n\n\x02\x05\x01\x12\x04\x1d\0\"\x01\n\n\n\x03\
    \x05\x01\x01\x12\x03\x1d\x05\x0f\n\x0b\n\x04\x05\x01\x02\0\x12\x03\x1e\
    \x02\x12\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03\x1e\x02\r\n\x0c\n\x05\x05\
    \x01\x02\0\x02\x12\x03\x1e\x10\x11\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\
    \x1f\x02\x13\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03\x1f\x02\x0e\n\x0c\n\
    \x05\x05\x01\x02\x01\x02\x12\x03\x1f\x11\x12\n\x0b\n\x04\x05\x01\x02\x02\
    \x12\x03\x20\x02\x1b\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03\x20\x02\x16\
    \n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03\x20\x19\x1a\n7\n\x04\x05\x01\
    \x02\x03\x12\x03!\x02\x13\"*\x20used\x20for\x20some\x20language-specific\
    \x20variants\n\n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x03!\x02\x0e\n\x0c\n\
    \x05\x05\x01\x02\x03\x02\x12\x03!\x11\x12\n\n\n\x02\x05\x02\x12\x04$\0*\
    \x01\n\n\n\x03\x05\x02\x01\x12\x03$\x05\x0c\n\x0b\n\x04\x05\x02\x02\0\
    \x12\x03%\x02\x0c\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03%\x02\x07\n\x0c\n\
    \x05\x05\x02\x02\0\x02\x12\x03%\n\x0b\n\x0b\n\x04\x05\x02\x02\x01\x12\
    \x03&\x02\x10\n\x0c\n\x05\x05\x02\x02\x01\x01\x12\x03&\x02\x0b\n\x0c\n\
    \x05\x05\x02\x02\x01\x02\x12\x03&\x0e\x0f\n\x0b\n\x04\x05\x02\x02\x02\
    \x12\x03'\x02\x1c\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\x03'\x02\x17\n\x0c\
    \n\x05\x05\x02\x02\x02\x02\x12\x03'\x1a\x1b\n\x0b\n\x04\x05\x02\x02\x03\
    \x12\x03(\x02\x1c\n\x0c\n\x05\x05\x02\x02\x03\x01\x12\x03(\x02\x17\n\x0c\
    \n\x05\x05\x02\x02\x03\x02\x12\x03(\x1a\x1b\n\x0b\n\x04\x05\x02\x02\x04\
    \x12\x03)\x02\x1a\n\x0c\n\x05\x05\x02\x02\x04\x01\x12\x03)\x02\x15\n\x0c\
    \n\x05\x05\x02\x02\x04\x02\x12\x03)\x18\x19\n\x9f\x01\n\x02\x04\0\x12\
    \x04.\01\x01\x1a\x92\x01\x20Parameters\x20of\x20poisson\x20process\x20di\
    stribution,\x20which\x20is\x20a\x20good\x20representation\n\x20of\x20act\
    ivity\x20coming\x20in\x20from\x20independent\x20identical\x20stationary\
    \x20sources.\n\n\n\n\x03\x04\0\x01\x12\x03.\x08\x15\nV\n\x04\x04\0\x02\0\
    \x12\x030\x02\x1a\x1aI\x20The\x20rate\x20of\x20arrivals\x20(a.k.a.\x20la\
    mbda\x20parameter\x20of\x20the\x20exp\x20distribution).\n\n\r\n\x05\x04\
    \0\x02\0\x04\x12\x040\x02.\x17\n\x0c\n\x05\x04\0\x02\0\x05\x12\x030\x02\
    \x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x030\t\x15\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x030\x18\x19\nd\n\x02\x04\x01\x12\x035\0\x1b\x1aY\x20Once\x20an\
    \x20RPC\x20finishes,\x20immediately\x20start\x20a\x20new\x20one.\n\x20No\
    \x20configuration\x20parameters\x20needed.\n\n\n\n\x03\x04\x01\x01\x12\
    \x035\x08\x18\n\n\n\x02\x04\x02\x12\x047\0<\x01\n\n\n\x03\x04\x02\x01\
    \x12\x037\x08\x12\n\x0c\n\x04\x04\x02\x08\0\x12\x048\x02;\x03\n\x0c\n\
    \x05\x04\x02\x08\0\x01\x12\x038\x08\x0c\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x039\x04%\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x039\x04\x14\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x039\x15\x20\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x039#$\n\x0b\n\x04\x04\x02\x02\x01\x12\x03:\x04\x1e\n\x0c\n\x05\x04\x02\
    \x02\x01\x06\x12\x03:\x04\x11\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03:\
    \x12\x19\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03:\x1c\x1d\n;\n\x02\x04\
    \x03\x12\x04?\0C\x01\x1a/\x20presence\x20of\x20SecurityParams\x20implies\
    \x20use\x20of\x20TLS\n\n\n\n\x03\x04\x03\x01\x12\x03?\x08\x16\n\x0b\n\
    \x04\x04\x03\x02\0\x12\x03@\x02\x17\n\r\n\x05\x04\x03\x02\0\x04\x12\x04@\
    \x02?\x18\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03@\x02\x06\n\x0c\n\x05\x04\
    \x03\x02\0\x01\x12\x03@\x07\x12\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03@\
    \x15\x16\n\x0b\n\x04\x04\x03\x02\x01\x12\x03A\x02\"\n\r\n\x05\x04\x03\
    \x02\x01\x04\x12\x04A\x02@\x17\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03A\
    \x02\x08\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03A\t\x1d\n\x0c\n\x05\x04\
    \x03\x02\x01\x03\x12\x03A\x20!\n\x0b\n\x04\x04\x03\x02\x02\x12\x03B\x02\
    \x17\n\r\n\x05\x04\x03\x02\x02\x04\x12\x04B\x02A\"\n\x0c\n\x05\x04\x03\
    \x02\x02\x05\x12\x03B\x02\x08\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03B\t\
    \x12\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03B\x15\x16\n\n\n\x02\x04\x04\
    \x12\x04E\0K\x01\n\n\n\x03\x04\x04\x01\x12\x03E\x08\x12\n\x0b\n\x04\x04\
    \x04\x02\0\x12\x03F\x02\x12\n\r\n\x05\x04\x04\x02\0\x04\x12\x04F\x02E\
    \x14\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03F\x02\x08\n\x0c\n\x05\x04\x04\
    \x02\0\x01\x12\x03F\t\r\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03F\x10\x11\n\
    \x0c\n\x04\x04\x04\x08\0\x12\x04G\x02J\x03\n\x0c\n\x05\x04\x04\x08\0\x01\
    \x12\x03G\x08\r\n\x0b\n\x04\x04\x04\x02\x01\x12\x03H\x04\x19\n\x0c\n\x05\
    \x04\x04\x02\x01\x05\x12\x03H\x04\n\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\
    \x03H\x0b\x14\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03H\x17\x18\n\x0b\n\
    \x04\x04\x04\x02\x02\x12\x03I\x04\x18\n\x0c\n\x05\x04\x04\x02\x02\x05\
    \x12\x03I\x04\t\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03I\n\x13\n\x0c\n\
    \x05\x04\x04\x02\x02\x03\x12\x03I\x16\x17\n\n\n\x02\x04\x05\x12\x04M\0n\
    \x01\n\n\n\x03\x04\x05\x01\x12\x03M\x08\x14\nX\n\x04\x04\x05\x02\0\x12\
    \x03O\x02%\x1aK\x20List\x20of\x20targets\x20to\x20connect\x20to.\x20At\
    \x20least\x20one\x20target\x20needs\x20to\x20be\x20specified.\n\n\x0c\n\
    \x05\x04\x05\x02\0\x04\x12\x03O\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\
    \x03O\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03O\x12\x20\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x03O#$\n\x0b\n\x04\x04\x05\x02\x01\x12\x03P\x02\
    \x1d\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04P\x02O%\n\x0c\n\x05\x04\x05\
    \x02\x01\x06\x12\x03P\x02\x0c\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03P\r\
    \x18\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03P\x1b\x1c\n\x0b\n\x04\x04\
    \x05\x02\x02\x12\x03Q\x02%\n\r\n\x05\x04\x05\x02\x02\x04\x12\x04Q\x02P\
    \x1d\n\x0c\n\x05\x04\x05\x02\x02\x06\x12\x03Q\x02\x10\n\x0c\n\x05\x04\
    \x05\x02\x02\x01\x12\x03Q\x11\x20\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\
    \x03Q#$\n\x8b\x01\n\x04\x04\x05\x02\x03\x12\x03T\x02)\x1a~\x20How\x20man\
    y\x20concurrent\x20RPCs\x20to\x20start\x20for\x20each\x20channel.\n\x20F\
    or\x20synchronous\x20client,\x20use\x20a\x20separate\x20thread\x20for\
    \x20each\x20outstanding\x20RPC.\n\n\r\n\x05\x04\x05\x02\x03\x04\x12\x04T\
    \x02Q%\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\x03T\x02\x07\n\x0c\n\x05\x04\
    \x05\x02\x03\x01\x12\x03T\x08$\n\x0c\n\x05\x04\x05\x02\x03\x03\x12\x03T'\
    (\n\x86\x01\n\x04\x04\x05\x02\x04\x12\x03W\x02\x1c\x1ay\x20Number\x20of\
    \x20independent\x20client\x20channels\x20to\x20create.\n\x20i-th\x20chan\
    nel\x20will\x20connect\x20to\x20server_target[i\x20%\x20server_targets.s\
    ize()]\n\n\r\n\x05\x04\x05\x02\x04\x04\x12\x04W\x02T)\n\x0c\n\x05\x04\
    \x05\x02\x04\x05\x12\x03W\x02\x07\n\x0c\n\x05\x04\x05\x02\x04\x01\x12\
    \x03W\x08\x17\n\x0c\n\x05\x04\x05\x02\x04\x03\x12\x03W\x1a\x1b\nT\n\x04\
    \x04\x05\x02\x05\x12\x03Y\x02!\x1aG\x20Only\x20for\x20async\x20client.\
    \x20Number\x20of\x20threads\x20to\x20use\x20to\x20start/manage\x20RPCs.\
    \n\n\r\n\x05\x04\x05\x02\x05\x04\x12\x04Y\x02W\x1c\n\x0c\n\x05\x04\x05\
    \x02\x05\x05\x12\x03Y\x02\x07\n\x0c\n\x05\x04\x05\x02\x05\x01\x12\x03Y\
    \x08\x1c\n\x0c\n\x05\x04\x05\x02\x05\x03\x12\x03Y\x1f\x20\n\x0b\n\x04\
    \x04\x05\x02\x06\x12\x03Z\x02\x17\n\r\n\x05\x04\x05\x02\x06\x04\x12\x04Z\
    \x02Y!\n\x0c\n\x05\x04\x05\x02\x06\x06\x12\x03Z\x02\t\n\x0c\n\x05\x04\
    \x05\x02\x06\x01\x12\x03Z\n\x12\n\x0c\n\x05\x04\x05\x02\x06\x03\x12\x03Z\
    \x15\x16\nZ\n\x04\x04\x05\x02\x07\x12\x03\\\x02\x1e\x1aM\x20The\x20reque\
    sted\x20load\x20for\x20the\x20entire\x20client\x20(aggregated\x20over\
    \x20all\x20the\x20threads).\n\n\r\n\x05\x04\x05\x02\x07\x04\x12\x04\\\
    \x02Z\x17\n\x0c\n\x05\x04\x05\x02\x07\x06\x12\x03\\\x02\x0c\n\x0c\n\x05\
    \x04\x05\x02\x07\x01\x12\x03\\\r\x18\n\x0c\n\x05\x04\x05\x02\x07\x03\x12\
    \x03\\\x1b\x1d\n\x0b\n\x04\x04\x05\x02\x08\x12\x03]\x02$\n\r\n\x05\x04\
    \x05\x02\x08\x04\x12\x04]\x02\\\x1e\n\x0c\n\x05\x04\x05\x02\x08\x06\x12\
    \x03]\x02\x0f\n\x0c\n\x05\x04\x05\x02\x08\x01\x12\x03]\x10\x1e\n\x0c\n\
    \x05\x04\x05\x02\x08\x03\x12\x03]!#\n\x0b\n\x04\x04\x05\x02\t\x12\x03^\
    \x02(\n\r\n\x05\x04\x05\x02\t\x04\x12\x04^\x02]$\n\x0c\n\x05\x04\x05\x02\
    \t\x06\x12\x03^\x02\x11\n\x0c\n\x05\x04\x05\x02\t\x01\x12\x03^\x12\"\n\
    \x0c\n\x05\x04\x05\x02\t\x03\x12\x03^%'\nH\n\x04\x04\x05\x02\n\x12\x03a\
    \x02\x20\x1a;\x20Specify\x20the\x20cores\x20we\x20should\x20run\x20the\
    \x20client\x20on,\x20if\x20desired\n\n\x0c\n\x05\x04\x05\x02\n\x04\x12\
    \x03a\x02\n\n\x0c\n\x05\x04\x05\x02\n\x05\x12\x03a\x0b\x10\n\x0c\n\x05\
    \x04\x05\x02\n\x01\x12\x03a\x11\x1a\n\x0c\n\x05\x04\x05\x02\n\x03\x12\
    \x03a\x1d\x1f\n\x0b\n\x04\x04\x05\x02\x0b\x12\x03b\x02\x18\n\r\n\x05\x04\
    \x05\x02\x0b\x04\x12\x04b\x02a\x20\n\x0c\n\x05\x04\x05\x02\x0b\x05\x12\
    \x03b\x02\x07\n\x0c\n\x05\x04\x05\x02\x0b\x01\x12\x03b\x08\x12\n\x0c\n\
    \x05\x04\x05\x02\x0b\x03\x12\x03b\x15\x17\nS\n\x04\x04\x05\x02\x0c\x12\
    \x03e\x02\x1f\x1aF\x20If\x20we\x20use\x20an\x20OTHER_CLIENT\x20client_ty\
    pe,\x20this\x20string\x20gives\x20more\x20detail\n\n\r\n\x05\x04\x05\x02\
    \x0c\x04\x12\x04e\x02b\x18\n\x0c\n\x05\x04\x05\x02\x0c\x05\x12\x03e\x02\
    \x08\n\x0c\n\x05\x04\x05\x02\x0c\x01\x12\x03e\t\x19\n\x0c\n\x05\x04\x05\
    \x02\x0c\x03\x12\x03e\x1c\x1e\n\x0b\n\x04\x04\x05\x02\r\x12\x03g\x02(\n\
    \x0c\n\x05\x04\x05\x02\r\x04\x12\x03g\x02\n\n\x0c\n\x05\x04\x05\x02\r\
    \x06\x12\x03g\x0b\x15\n\x0c\n\x05\x04\x05\x02\r\x01\x12\x03g\x16\"\n\x0c\
    \n\x05\x04\x05\x02\r\x03\x12\x03g%'\nA\n\x04\x04\x05\x02\x0e\x12\x03j\
    \x02\x1c\x1a4\x20Number\x20of\x20threads\x20that\x20share\x20each\x20com\
    pletion\x20queue\n\n\r\n\x05\x04\x05\x02\x0e\x04\x12\x04j\x02g(\n\x0c\n\
    \x05\x04\x05\x02\x0e\x05\x12\x03j\x02\x07\n\x0c\n\x05\x04\x05\x02\x0e\
    \x01\x12\x03j\x08\x16\n\x0c\n\x05\x04\x05\x02\x0e\x03\x12\x03j\x19\x1b\n\
    O\n\x04\x04\x05\x02\x0f\x12\x03m\x02!\x1aB\x20Number\x20of\x20messages\
    \x20on\x20a\x20stream\x20before\x20it\x20gets\x20finished/restarted\n\n\
    \r\n\x05\x04\x05\x02\x0f\x04\x12\x04m\x02j\x1c\n\x0c\n\x05\x04\x05\x02\
    \x0f\x05\x12\x03m\x02\x07\n\x0c\n\x05\x04\x05\x02\x0f\x01\x12\x03m\x08\
    \x1b\n\x0c\n\x05\x04\x05\x02\x0f\x03\x12\x03m\x1e\x20\n\t\n\x02\x04\x06\
    \x12\x03p\0/\n\n\n\x03\x04\x06\x01\x12\x03p\x08\x14\n\x0b\n\x04\x04\x06\
    \x02\0\x12\x03p\x17-\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03p\x17\x16\n\
    \x0c\n\x05\x04\x06\x02\0\x06\x12\x03p\x17\"\n\x0c\n\x05\x04\x06\x02\0\
    \x01\x12\x03p#(\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03p+,\n#\n\x02\x04\
    \x07\x12\x04s\0v\x01\x1a\x17\x20Request\x20current\x20stats\n\n\n\n\x03\
    \x04\x07\x01\x12\x03s\x08\x0c\nL\n\x04\x04\x07\x02\0\x12\x03u\x02\x11\
    \x1a?\x20if\x20true,\x20the\x20stats\x20will\x20be\x20reset\x20after\x20\
    taking\x20their\x20snapshot.\n\n\r\n\x05\x04\x07\x02\0\x04\x12\x04u\x02s\
    \x0e\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03u\x02\x06\n\x0c\n\x05\x04\x07\
    \x02\0\x01\x12\x03u\x07\x0c\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03u\x0f\
    \x10\n\n\n\x02\x04\x08\x12\x04x\0}\x01\n\n\n\x03\x04\x08\x01\x12\x03x\
    \x08\x12\n\x0c\n\x04\x04\x08\x08\0\x12\x04y\x02|\x03\n\x0c\n\x05\x04\x08\
    \x08\0\x01\x12\x03y\x08\x0f\n\x0b\n\x04\x04\x08\x02\0\x12\x03z\x04\x1b\n\
    \x0c\n\x05\x04\x08\x02\0\x06\x12\x03z\x04\x10\n\x0c\n\x05\x04\x08\x02\0\
    \x01\x12\x03z\x11\x16\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03z\x19\x1a\n\
    \x0b\n\x04\x04\x08\x02\x01\x12\x03{\x04\x12\n\x0c\n\x05\x04\x08\x02\x01\
    \x06\x12\x03{\x04\x08\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03{\t\r\n\x0c\
    \n\x05\x04\x08\x02\x01\x03\x12\x03{\x10\x11\n\x0b\n\x02\x04\t\x12\x05\
    \x7f\0\x9c\x01\x01\n\n\n\x03\x04\t\x01\x12\x03\x7f\x08\x14\n\x0c\n\x04\
    \x04\t\x02\0\x12\x04\x80\x01\x02\x1d\n\x0e\n\x05\x04\t\x02\0\x04\x12\x05\
    \x80\x01\x02\x7f\x16\n\r\n\x05\x04\t\x02\0\x06\x12\x04\x80\x01\x02\x0c\n\
    \r\n\x05\x04\t\x02\0\x01\x12\x04\x80\x01\r\x18\n\r\n\x05\x04\t\x02\0\x03\
    \x12\x04\x80\x01\x1b\x1c\n\x0c\n\x04\x04\t\x02\x01\x12\x04\x81\x01\x02%\
    \n\x0f\n\x05\x04\t\x02\x01\x04\x12\x06\x81\x01\x02\x80\x01\x1d\n\r\n\x05\
    \x04\t\x02\x01\x06\x12\x04\x81\x01\x02\x10\n\r\n\x05\x04\t\x02\x01\x01\
    \x12\x04\x81\x01\x11\x20\n\r\n\x05\x04\t\x02\x01\x03\x12\x04\x81\x01#$\n\
    E\n\x04\x04\t\x02\x02\x12\x04\x83\x01\x02\x11\x1a7\x20Port\x20on\x20whic\
    h\x20to\x20listen.\x20Zero\x20means\x20pick\x20unused\x20port.\n\n\x0f\n\
    \x05\x04\t\x02\x02\x04\x12\x06\x83\x01\x02\x81\x01%\n\r\n\x05\x04\t\x02\
    \x02\x05\x12\x04\x83\x01\x02\x07\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\x83\
    \x01\x08\x0c\n\r\n\x05\x04\t\x02\x02\x03\x12\x04\x83\x01\x0f\x10\nT\n\
    \x04\x04\t\x02\x03\x12\x04\x85\x01\x02!\x1aF\x20Only\x20for\x20async\x20\
    server.\x20Number\x20of\x20threads\x20used\x20to\x20serve\x20the\x20requ\
    ests.\n\n\x0f\n\x05\x04\t\x02\x03\x04\x12\x06\x85\x01\x02\x83\x01\x11\n\
    \r\n\x05\x04\t\x02\x03\x05\x12\x04\x85\x01\x02\x07\n\r\n\x05\x04\t\x02\
    \x03\x01\x12\x04\x85\x01\x08\x1c\n\r\n\x05\x04\t\x02\x03\x03\x12\x04\x85\
    \x01\x1f\x20\nJ\n\x04\x04\t\x02\x04\x12\x04\x87\x01\x02\x17\x1a<\x20Spec\
    ify\x20the\x20number\x20of\x20cores\x20to\x20limit\x20server\x20to,\x20i\
    f\x20desired\n\n\x0f\n\x05\x04\t\x02\x04\x04\x12\x06\x87\x01\x02\x85\x01\
    !\n\r\n\x05\x04\t\x02\x04\x05\x12\x04\x87\x01\x02\x07\n\r\n\x05\x04\t\
    \x02\x04\x01\x12\x04\x87\x01\x08\x12\n\r\n\x05\x04\t\x02\x04\x03\x12\x04\
    \x87\x01\x15\x16\n\xfc\x01\n\x04\x04\t\x02\x05\x12\x04\x8c\x01\x02#\x1a\
    \xed\x01\x20payload\x20config,\x20used\x20in\x20generic\x20server.\n\x20\
    Note\x20this\x20must\x20NOT\x20be\x20used\x20in\x20proto\x20(non-generic\
    )\x20servers.\x20For\x20proto\x20servers,\n\x20'response\x20sizes'\x20mu\
    st\x20be\x20configured\x20from\x20the\x20'response_size'\x20field\x20of\
    \x20the\n\x20'SimpleRequest'\x20objects\x20in\x20RPC\x20requests.\n\n\
    \x0f\n\x05\x04\t\x02\x05\x04\x12\x06\x8c\x01\x02\x87\x01\x17\n\r\n\x05\
    \x04\t\x02\x05\x06\x12\x04\x8c\x01\x02\x0f\n\r\n\x05\x04\t\x02\x05\x01\
    \x12\x04\x8c\x01\x10\x1e\n\r\n\x05\x04\t\x02\x05\x03\x12\x04\x8c\x01!\"\
    \nI\n\x04\x04\t\x02\x06\x12\x04\x8f\x01\x02\x20\x1a;\x20Specify\x20the\
    \x20cores\x20we\x20should\x20run\x20the\x20server\x20on,\x20if\x20desire\
    d\n\n\r\n\x05\x04\t\x02\x06\x04\x12\x04\x8f\x01\x02\n\n\r\n\x05\x04\t\
    \x02\x06\x05\x12\x04\x8f\x01\x0b\x10\n\r\n\x05\x04\t\x02\x06\x01\x12\x04\
    \x8f\x01\x11\x1a\n\r\n\x05\x04\t\x02\x06\x03\x12\x04\x8f\x01\x1d\x1f\nT\
    \n\x04\x04\t\x02\x07\x12\x04\x92\x01\x02\x1f\x1aF\x20If\x20we\x20use\x20\
    an\x20OTHER_SERVER\x20client_type,\x20this\x20string\x20gives\x20more\
    \x20detail\n\n\x0f\n\x05\x04\t\x02\x07\x04\x12\x06\x92\x01\x02\x8f\x01\
    \x20\n\r\n\x05\x04\t\x02\x07\x05\x12\x04\x92\x01\x02\x08\n\r\n\x05\x04\t\
    \x02\x07\x01\x12\x04\x92\x01\t\x19\n\r\n\x05\x04\t\x02\x07\x03\x12\x04\
    \x92\x01\x1c\x1e\nB\n\x04\x04\t\x02\x08\x12\x04\x95\x01\x02\x1c\x1a4\x20\
    Number\x20of\x20threads\x20that\x20share\x20each\x20completion\x20queue\
    \n\n\x0f\n\x05\x04\t\x02\x08\x04\x12\x06\x95\x01\x02\x92\x01\x1f\n\r\n\
    \x05\x04\t\x02\x08\x05\x12\x04\x95\x01\x02\x07\n\r\n\x05\x04\t\x02\x08\
    \x01\x12\x04\x95\x01\x08\x16\n\r\n\x05\x04\t\x02\x08\x03\x12\x04\x95\x01\
    \x19\x1b\n\x83\x01\n\x04\x04\t\x02\t\x12\x04\x9a\x01\x02#\x1a6\x20Buffer\
    \x20pool\x20size\x20(no\x20buffer\x20pool\x20specified\x20if\x20unset)\n\
    2=\x20c++-only\x20options\x20(for\x20now)\x20---------------------------\
    -----\n\n\x0f\n\x05\x04\t\x02\t\x04\x12\x06\x9a\x01\x02\x95\x01\x1c\n\r\
    \n\x05\x04\t\x02\t\x05\x12\x04\x9a\x01\x02\x07\n\r\n\x05\x04\t\x02\t\x01\
    \x12\x04\x9a\x01\x08\x1b\n\r\n\x05\x04\t\x02\t\x03\x12\x04\x9a\x01\x1e\"\
    \n\x0c\n\x04\x04\t\x02\n\x12\x04\x9b\x01\x02*\n\r\n\x05\x04\t\x02\n\x04\
    \x12\x04\x9b\x01\x02\n\n\r\n\x05\x04\t\x02\n\x06\x12\x04\x9b\x01\x0b\x15\
    \n\r\n\x05\x04\t\x02\n\x01\x12\x04\x9b\x01\x16\"\n\r\n\x05\x04\t\x02\n\
    \x03\x12\x04\x9b\x01%)\n\x0c\n\x02\x04\n\x12\x06\x9e\x01\0\xa3\x01\x01\n\
    \x0b\n\x03\x04\n\x01\x12\x04\x9e\x01\x08\x12\n\x0e\n\x04\x04\n\x08\0\x12\
    \x06\x9f\x01\x02\xa2\x01\x03\n\r\n\x05\x04\n\x08\0\x01\x12\x04\x9f\x01\
    \x08\x0f\n\x0c\n\x04\x04\n\x02\0\x12\x04\xa0\x01\x04\x1b\n\r\n\x05\x04\n\
    \x02\0\x06\x12\x04\xa0\x01\x04\x10\n\r\n\x05\x04\n\x02\0\x01\x12\x04\xa0\
    \x01\x11\x16\n\r\n\x05\x04\n\x02\0\x03\x12\x04\xa0\x01\x19\x1a\n\x0c\n\
    \x04\x04\n\x02\x01\x12\x04\xa1\x01\x04\x12\n\r\n\x05\x04\n\x02\x01\x06\
    \x12\x04\xa1\x01\x04\x08\n\r\n\x05\x04\n\x02\x01\x01\x12\x04\xa1\x01\t\r\
    \n\r\n\x05\x04\n\x02\x01\x03\x12\x04\xa1\x01\x10\x11\n\x0c\n\x02\x04\x0b\
    \x12\x06\xa5\x01\0\xab\x01\x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\xa5\x01\
    \x08\x14\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\xa6\x01\x02\x18\n\x0f\n\x05\
    \x04\x0b\x02\0\x04\x12\x06\xa6\x01\x02\xa5\x01\x16\n\r\n\x05\x04\x0b\x02\
    \0\x06\x12\x04\xa6\x01\x02\r\n\r\n\x05\x04\x0b\x02\0\x01\x12\x04\xa6\x01\
    \x0e\x13\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\xa6\x01\x16\x17\n,\n\x04\
    \x04\x0b\x02\x01\x12\x04\xa8\x01\x02\x11\x1a\x1e\x20the\x20port\x20bound\
    \x20by\x20the\x20server\n\n\x0f\n\x05\x04\x0b\x02\x01\x04\x12\x06\xa8\
    \x01\x02\xa6\x01\x18\n\r\n\x05\x04\x0b\x02\x01\x05\x12\x04\xa8\x01\x02\
    \x07\n\r\n\x05\x04\x0b\x02\x01\x01\x12\x04\xa8\x01\x08\x0c\n\r\n\x05\x04\
    \x0b\x02\x01\x03\x12\x04\xa8\x01\x0f\x10\n7\n\x04\x04\x0b\x02\x02\x12\
    \x04\xaa\x01\x02\x12\x1a)\x20Number\x20of\x20cores\x20available\x20to\
    \x20the\x20server\n\n\x0f\n\x05\x04\x0b\x02\x02\x04\x12\x06\xaa\x01\x02\
    \xa8\x01\x11\n\r\n\x05\x04\x0b\x02\x02\x05\x12\x04\xaa\x01\x02\x07\n\r\n\
    \x05\x04\x0b\x02\x02\x01\x12\x04\xaa\x01\x08\r\n\r\n\x05\x04\x0b\x02\x02\
    \x03\x12\x04\xaa\x01\x10\x11\n\x0c\n\x02\x04\x0c\x12\x06\xad\x01\0\xae\
    \x01\x01\n\x0b\n\x03\x04\x0c\x01\x12\x04\xad\x01\x08\x13\n\x0c\n\x02\x04\
    \r\x12\x06\xb0\x01\0\xb3\x01\x01\n\x0b\n\x03\x04\r\x01\x12\x04\xb0\x01\
    \x08\x14\n7\n\x04\x04\r\x02\0\x12\x04\xb2\x01\x02\x12\x1a)\x20Number\x20\
    of\x20cores\x20available\x20on\x20the\x20server\n\n\x0f\n\x05\x04\r\x02\
    \0\x04\x12\x06\xb2\x01\x02\xb0\x01\x16\n\r\n\x05\x04\r\x02\0\x05\x12\x04\
    \xb2\x01\x02\x07\n\r\n\x05\x04\r\x02\0\x01\x12\x04\xb2\x01\x08\r\n\r\n\
    \x05\x04\r\x02\0\x03\x12\x04\xb2\x01\x10\x11\n\x0c\n\x02\x04\x0e\x12\x06\
    \xb5\x01\0\xb6\x01\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\xb5\x01\x08\x0c\n\
    G\n\x02\x04\x0f\x12\x06\xb9\x01\0\xca\x01\x01\x1a9\x20A\x20single\x20per\
    formance\x20scenario:\x20input\x20to\x20qps_json_driver\n\n\x0b\n\x03\
    \x04\x0f\x01\x12\x04\xb9\x01\x08\x10\n5\n\x04\x04\x0f\x02\0\x12\x04\xbb\
    \x01\x02\x12\x1a'\x20Human\x20readable\x20name\x20for\x20this\x20scenari\
    o\n\n\x0f\n\x05\x04\x0f\x02\0\x04\x12\x06\xbb\x01\x02\xb9\x01\x12\n\r\n\
    \x05\x04\x0f\x02\0\x05\x12\x04\xbb\x01\x02\x08\n\r\n\x05\x04\x0f\x02\0\
    \x01\x12\x04\xbb\x01\t\r\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\xbb\x01\x10\
    \x11\n$\n\x04\x04\x0f\x02\x01\x12\x04\xbd\x01\x02!\x1a\x16\x20Client\x20\
    configuration\n\n\x0f\n\x05\x04\x0f\x02\x01\x04\x12\x06\xbd\x01\x02\xbb\
    \x01\x12\n\r\n\x05\x04\x0f\x02\x01\x06\x12\x04\xbd\x01\x02\x0e\n\r\n\x05\
    \x04\x0f\x02\x01\x01\x12\x04\xbd\x01\x0f\x1c\n\r\n\x05\x04\x0f\x02\x01\
    \x03\x12\x04\xbd\x01\x1f\x20\n7\n\x04\x04\x0f\x02\x02\x12\x04\xbf\x01\
    \x02\x18\x1a)\x20Number\x20of\x20clients\x20to\x20start\x20for\x20the\
    \x20test\n\n\x0f\n\x05\x04\x0f\x02\x02\x04\x12\x06\xbf\x01\x02\xbd\x01!\
    \n\r\n\x05\x04\x0f\x02\x02\x05\x12\x04\xbf\x01\x02\x07\n\r\n\x05\x04\x0f\
    \x02\x02\x01\x12\x04\xbf\x01\x08\x13\n\r\n\x05\x04\x0f\x02\x02\x03\x12\
    \x04\xbf\x01\x16\x17\n$\n\x04\x04\x0f\x02\x03\x12\x04\xc1\x01\x02!\x1a\
    \x16\x20Server\x20configuration\n\n\x0f\n\x05\x04\x0f\x02\x03\x04\x12\
    \x06\xc1\x01\x02\xbf\x01\x18\n\r\n\x05\x04\x0f\x02\x03\x06\x12\x04\xc1\
    \x01\x02\x0e\n\r\n\x05\x04\x0f\x02\x03\x01\x12\x04\xc1\x01\x0f\x1c\n\r\n\
    \x05\x04\x0f\x02\x03\x03\x12\x04\xc1\x01\x1f\x20\n7\n\x04\x04\x0f\x02\
    \x04\x12\x04\xc3\x01\x02\x18\x1a)\x20Number\x20of\x20servers\x20to\x20st\
    art\x20for\x20the\x20test\n\n\x0f\n\x05\x04\x0f\x02\x04\x04\x12\x06\xc3\
    \x01\x02\xc1\x01!\n\r\n\x05\x04\x0f\x02\x04\x05\x12\x04\xc3\x01\x02\x07\
    \n\r\n\x05\x04\x0f\x02\x04\x01\x12\x04\xc3\x01\x08\x13\n\r\n\x05\x04\x0f\
    \x02\x04\x03\x12\x04\xc3\x01\x16\x17\n)\n\x04\x04\x0f\x02\x05\x12\x04\
    \xc5\x01\x02\x1b\x1a\x1b\x20Warmup\x20period,\x20in\x20seconds\n\n\x0f\n\
    \x05\x04\x0f\x02\x05\x04\x12\x06\xc5\x01\x02\xc3\x01\x18\n\r\n\x05\x04\
    \x0f\x02\x05\x05\x12\x04\xc5\x01\x02\x07\n\r\n\x05\x04\x0f\x02\x05\x01\
    \x12\x04\xc5\x01\x08\x16\n\r\n\x05\x04\x0f\x02\x05\x03\x12\x04\xc5\x01\
    \x19\x1a\n*\n\x04\x04\x0f\x02\x06\x12\x04\xc7\x01\x02\x1e\x1a\x1c\x20Ben\
    chmark\x20time,\x20in\x20seconds\n\n\x0f\n\x05\x04\x0f\x02\x06\x04\x12\
    \x06\xc7\x01\x02\xc5\x01\x1b\n\r\n\x05\x04\x0f\x02\x06\x05\x12\x04\xc7\
    \x01\x02\x07\n\r\n\x05\x04\x0f\x02\x06\x01\x12\x04\xc7\x01\x08\x19\n\r\n\
    \x05\x04\x0f\x02\x06\x03\x12\x04\xc7\x01\x1c\x1d\nA\n\x04\x04\x0f\x02\
    \x07\x12\x04\xc9\x01\x02%\x1a3\x20Number\x20of\x20workers\x20to\x20spawn\
    \x20locally\x20(usually\x20zero)\n\n\x0f\n\x05\x04\x0f\x02\x07\x04\x12\
    \x06\xc9\x01\x02\xc7\x01\x1e\n\r\n\x05\x04\x0f\x02\x07\x05\x12\x04\xc9\
    \x01\x02\x07\n\r\n\x05\x04\x0f\x02\x07\x01\x12\x04\xc9\x01\x08\x20\n\r\n\
    \x05\x04\x0f\x02\x07\x03\x12\x04\xc9\x01#$\nA\n\x02\x04\x10\x12\x06\xcd\
    \x01\0\xcf\x01\x01\x1a3\x20A\x20set\x20of\x20scenarios\x20to\x20be\x20ru\
    n\x20with\x20qps_json_driver\n\n\x0b\n\x03\x04\x10\x01\x12\x04\xcd\x01\
    \x08\x11\n\x0c\n\x04\x04\x10\x02\0\x12\x04\xce\x01\x02\"\n\r\n\x05\x04\
    \x10\x02\0\x04\x12\x04\xce\x01\x02\n\n\r\n\x05\x04\x10\x02\0\x06\x12\x04\
    \xce\x01\x0b\x13\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\xce\x01\x14\x1d\n\r\
    \n\x05\x04\x10\x02\0\x03\x12\x04\xce\x01\x20!\nt\n\x02\x04\x11\x12\x06\
    \xd3\x01\0\xf7\x01\x01\x1af\x20Basic\x20summary\x20that\x20can\x20be\x20\
    computed\x20from\x20ClientStats\x20and\x20ServerStats\n\x20once\x20the\
    \x20scenario\x20has\x20finished.\n\n\x0b\n\x03\x04\x11\x01\x12\x04\xd3\
    \x01\x08\x1d\nG\n\x04\x04\x11\x02\0\x12\x04\xd6\x01\x02\x11\x1a9\x20Tota\
    l\x20number\x20of\x20operations\x20per\x20second\x20over\x20all\x20clien\
    ts.\n\n\x0f\n\x05\x04\x11\x02\0\x04\x12\x06\xd6\x01\x02\xd4\x01\x01\n\r\
    \n\x05\x04\x11\x02\0\x05\x12\x04\xd6\x01\x02\x08\n\r\n\x05\x04\x11\x02\0\
    \x01\x12\x04\xd6\x01\t\x0c\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xd6\x01\
    \x0f\x10\n(\n\x04\x04\x11\x02\x01\x12\x04\xd8\x01\x02!\x1a\x1a\x20QPS\
    \x20per\x20one\x20server\x20core.\n\n\x0f\n\x05\x04\x11\x02\x01\x04\x12\
    \x06\xd8\x01\x02\xd6\x01\x11\n\r\n\x05\x04\x11\x02\x01\x05\x12\x04\xd8\
    \x01\x02\x08\n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\xd8\x01\t\x1c\n\r\n\
    \x05\x04\x11\x02\x01\x03\x12\x04\xd8\x01\x1f\x20\n>\n\x04\x04\x11\x02\
    \x02\x12\x04\xda\x01\x02\x20\x1a0\x20server\x20load\x20based\x20on\x20sy\
    stem_time\x20(0.85\x20=>\x2085%)\n\n\x0f\n\x05\x04\x11\x02\x02\x04\x12\
    \x06\xda\x01\x02\xd8\x01!\n\r\n\x05\x04\x11\x02\x02\x05\x12\x04\xda\x01\
    \x02\x08\n\r\n\x05\x04\x11\x02\x02\x01\x12\x04\xda\x01\t\x1b\n\r\n\x05\
    \x04\x11\x02\x02\x03\x12\x04\xda\x01\x1e\x1f\n<\n\x04\x04\x11\x02\x03\
    \x12\x04\xdc\x01\x02\x1e\x1a.\x20server\x20load\x20based\x20on\x20user_t\
    ime\x20(0.85\x20=>\x2085%)\n\n\x0f\n\x05\x04\x11\x02\x03\x04\x12\x06\xdc\
    \x01\x02\xda\x01\x20\n\r\n\x05\x04\x11\x02\x03\x05\x12\x04\xdc\x01\x02\
    \x08\n\r\n\x05\x04\x11\x02\x03\x01\x12\x04\xdc\x01\t\x19\n\r\n\x05\x04\
    \x11\x02\x03\x03\x12\x04\xdc\x01\x1c\x1d\n>\n\x04\x04\x11\x02\x04\x12\
    \x04\xde\x01\x02\x20\x1a0\x20client\x20load\x20based\x20on\x20system_tim\
    e\x20(0.85\x20=>\x2085%)\n\n\x0f\n\x05\x04\x11\x02\x04\x04\x12\x06\xde\
    \x01\x02\xdc\x01\x1e\n\r\n\x05\x04\x11\x02\x04\x05\x12\x04\xde\x01\x02\
    \x08\n\r\n\x05\x04\x11\x02\x04\x01\x12\x04\xde\x01\t\x1b\n\r\n\x05\x04\
    \x11\x02\x04\x03\x12\x04\xde\x01\x1e\x1f\n<\n\x04\x04\x11\x02\x05\x12\
    \x04\xe0\x01\x02\x1e\x1a.\x20client\x20load\x20based\x20on\x20user_time\
    \x20(0.85\x20=>\x2085%)\n\n\x0f\n\x05\x04\x11\x02\x05\x04\x12\x06\xe0\
    \x01\x02\xde\x01\x20\n\r\n\x05\x04\x11\x02\x05\x05\x12\x04\xe0\x01\x02\
    \x08\n\r\n\x05\x04\x11\x02\x05\x01\x12\x04\xe0\x01\t\x19\n\r\n\x05\x04\
    \x11\x02\x05\x03\x12\x04\xe0\x01\x1c\x1d\n7\n\x04\x04\x11\x02\x06\x12\
    \x04\xe3\x01\x02\x18\x1a)\x20X%\x20latency\x20percentiles\x20(in\x20nano\
    seconds)\n\n\x0f\n\x05\x04\x11\x02\x06\x04\x12\x06\xe3\x01\x02\xe0\x01\
    \x1e\n\r\n\x05\x04\x11\x02\x06\x05\x12\x04\xe3\x01\x02\x08\n\r\n\x05\x04\
    \x11\x02\x06\x01\x12\x04\xe3\x01\t\x13\n\r\n\x05\x04\x11\x02\x06\x03\x12\
    \x04\xe3\x01\x16\x17\n\x0c\n\x04\x04\x11\x02\x07\x12\x04\xe4\x01\x02\x18\
    \n\x0f\n\x05\x04\x11\x02\x07\x04\x12\x06\xe4\x01\x02\xe3\x01\x18\n\r\n\
    \x05\x04\x11\x02\x07\x05\x12\x04\xe4\x01\x02\x08\n\r\n\x05\x04\x11\x02\
    \x07\x01\x12\x04\xe4\x01\t\x13\n\r\n\x05\x04\x11\x02\x07\x03\x12\x04\xe4\
    \x01\x16\x17\n\x0c\n\x04\x04\x11\x02\x08\x12\x04\xe5\x01\x02\x18\n\x0f\n\
    \x05\x04\x11\x02\x08\x04\x12\x06\xe5\x01\x02\xe4\x01\x18\n\r\n\x05\x04\
    \x11\x02\x08\x05\x12\x04\xe5\x01\x02\x08\n\r\n\x05\x04\x11\x02\x08\x01\
    \x12\x04\xe5\x01\t\x13\n\r\n\x05\x04\x11\x02\x08\x03\x12\x04\xe5\x01\x16\
    \x17\n\x0c\n\x04\x04\x11\x02\t\x12\x04\xe6\x01\x02\x19\n\x0f\n\x05\x04\
    \x11\x02\t\x04\x12\x06\xe6\x01\x02\xe5\x01\x18\n\r\n\x05\x04\x11\x02\t\
    \x05\x12\x04\xe6\x01\x02\x08\n\r\n\x05\x04\x11\x02\t\x01\x12\x04\xe6\x01\
    \t\x13\n\r\n\x05\x04\x11\x02\t\x03\x12\x04\xe6\x01\x16\x18\n\x0c\n\x04\
    \x04\x11\x02\n\x12\x04\xe7\x01\x02\x1a\n\x0f\n\x05\x04\x11\x02\n\x04\x12\
    \x06\xe7\x01\x02\xe6\x01\x19\n\r\n\x05\x04\x11\x02\n\x05\x12\x04\xe7\x01\
    \x02\x08\n\r\n\x05\x04\x11\x02\n\x01\x12\x04\xe7\x01\t\x14\n\r\n\x05\x04\
    \x11\x02\n\x03\x12\x04\xe7\x01\x17\x19\n+\n\x04\x04\x11\x02\x0b\x12\x04\
    \xea\x01\x02\x1f\x1a\x1d\x20server\x20cpu\x20usage\x20percentage\n\n\x0f\
    \n\x05\x04\x11\x02\x0b\x04\x12\x06\xea\x01\x02\xe7\x01\x1a\n\r\n\x05\x04\
    \x11\x02\x0b\x05\x12\x04\xea\x01\x02\x08\n\r\n\x05\x04\x11\x02\x0b\x01\
    \x12\x04\xea\x01\t\x19\n\r\n\x05\x04\x11\x02\x0b\x03\x12\x04\xea\x01\x1c\
    \x1e\n8\n\x04\x04\x11\x02\x0c\x12\x04\xed\x01\x02-\x1a*\x20Number\x20of\
    \x20requests\x20that\x20succeeded/failed\n\n\x0f\n\x05\x04\x11\x02\x0c\
    \x04\x12\x06\xed\x01\x02\xea\x01\x1f\n\r\n\x05\x04\x11\x02\x0c\x05\x12\
    \x04\xed\x01\x02\x08\n\r\n\x05\x04\x11\x02\x0c\x01\x12\x04\xed\x01\t'\n\
    \r\n\x05\x04\x11\x02\x0c\x03\x12\x04\xed\x01*,\n\x0c\n\x04\x04\x11\x02\r\
    \x12\x04\xee\x01\x02)\n\x0f\n\x05\x04\x11\x02\r\x04\x12\x06\xee\x01\x02\
    \xed\x01-\n\r\n\x05\x04\x11\x02\r\x05\x12\x04\xee\x01\x02\x08\n\r\n\x05\
    \x04\x11\x02\r\x01\x12\x04\xee\x01\t#\n\r\n\x05\x04\x11\x02\r\x03\x12\
    \x04\xee\x01&(\nJ\n\x04\x04\x11\x02\x0e\x12\x04\xf1\x01\x02'\x1a<\x20Num\
    ber\x20of\x20polls\x20called\x20inside\x20completion\x20queue\x20per\x20\
    request\n\n\x0f\n\x05\x04\x11\x02\x0e\x04\x12\x06\xf1\x01\x02\xee\x01)\n\
    \r\n\x05\x04\x11\x02\x0e\x05\x12\x04\xf1\x01\x02\x08\n\r\n\x05\x04\x11\
    \x02\x0e\x01\x12\x04\xf1\x01\t!\n\r\n\x05\x04\x11\x02\x0e\x03\x12\x04\
    \xf1\x01$&\n\x0c\n\x04\x04\x11\x02\x0f\x12\x04\xf2\x01\x02'\n\x0f\n\x05\
    \x04\x11\x02\x0f\x04\x12\x06\xf2\x01\x02\xf1\x01'\n\r\n\x05\x04\x11\x02\
    \x0f\x05\x12\x04\xf2\x01\x02\x08\n\r\n\x05\x04\x11\x02\x0f\x01\x12\x04\
    \xf2\x01\t!\n\r\n\x05\x04\x11\x02\x0f\x03\x12\x04\xf2\x01$&\n?\n\x04\x04\
    \x11\x02\x10\x12\x04\xf5\x01\x02)\x1a1\x20Queries\x20per\x20CPU-sec\x20o\
    ver\x20all\x20servers\x20or\x20clients\n\n\x0f\n\x05\x04\x11\x02\x10\x04\
    \x12\x06\xf5\x01\x02\xf2\x01'\n\r\n\x05\x04\x11\x02\x10\x05\x12\x04\xf5\
    \x01\x02\x08\n\r\n\x05\x04\x11\x02\x10\x01\x12\x04\xf5\x01\t#\n\r\n\x05\
    \x04\x11\x02\x10\x03\x12\x04\xf5\x01&(\n\x0c\n\x04\x04\x11\x02\x11\x12\
    \x04\xf6\x01\x02)\n\x0f\n\x05\x04\x11\x02\x11\x04\x12\x06\xf6\x01\x02\
    \xf5\x01)\n\r\n\x05\x04\x11\x02\x11\x05\x12\x04\xf6\x01\x02\x08\n\r\n\
    \x05\x04\x11\x02\x11\x01\x12\x04\xf6\x01\t#\n\r\n\x05\x04\x11\x02\x11\
    \x03\x12\x04\xf6\x01&(\n7\n\x02\x04\x12\x12\x06\xfa\x01\0\x8c\x02\x01\
    \x1a)\x20Results\x20of\x20a\x20single\x20benchmark\x20scenario.\n\n\x0b\
    \n\x03\x04\x12\x01\x12\x04\xfa\x01\x08\x16\n0\n\x04\x04\x12\x02\0\x12\
    \x04\xfc\x01\x02\x18\x1a\"\x20Inputs\x20used\x20to\x20run\x20the\x20scen\
    ario.\n\n\x0f\n\x05\x04\x12\x02\0\x04\x12\x06\xfc\x01\x02\xfa\x01\x18\n\
    \r\n\x05\x04\x12\x02\0\x06\x12\x04\xfc\x01\x02\n\n\r\n\x05\x04\x12\x02\0\
    \x01\x12\x04\xfc\x01\x0b\x13\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xfc\x01\
    \x16\x17\nF\n\x04\x04\x12\x02\x01\x12\x04\xfe\x01\x02\x1e\x1a8\x20Histog\
    rams\x20from\x20all\x20clients\x20merged\x20into\x20one\x20histogram.\n\
    \n\x0f\n\x05\x04\x12\x02\x01\x04\x12\x06\xfe\x01\x02\xfc\x01\x18\n\r\n\
    \x05\x04\x12\x02\x01\x06\x12\x04\xfe\x01\x02\x0f\n\r\n\x05\x04\x12\x02\
    \x01\x01\x12\x04\xfe\x01\x10\x19\n\r\n\x05\x04\x12\x02\x01\x03\x12\x04\
    \xfe\x01\x1c\x1d\n,\n\x04\x04\x12\x02\x02\x12\x04\x80\x02\x02(\x1a\x1e\
    \x20Client\x20stats\x20for\x20each\x20client\n\n\r\n\x05\x04\x12\x02\x02\
    \x04\x12\x04\x80\x02\x02\n\n\r\n\x05\x04\x12\x02\x02\x06\x12\x04\x80\x02\
    \x0b\x16\n\r\n\x05\x04\x12\x02\x02\x01\x12\x04\x80\x02\x17#\n\r\n\x05\
    \x04\x12\x02\x02\x03\x12\x04\x80\x02&'\n,\n\x04\x04\x12\x02\x03\x12\x04\
    \x82\x02\x02(\x1a\x1e\x20Server\x20stats\x20for\x20each\x20server\n\n\r\
    \n\x05\x04\x12\x02\x03\x04\x12\x04\x82\x02\x02\n\n\r\n\x05\x04\x12\x02\
    \x03\x06\x12\x04\x82\x02\x0b\x16\n\r\n\x05\x04\x12\x02\x03\x01\x12\x04\
    \x82\x02\x17#\n\r\n\x05\x04\x12\x02\x03\x03\x12\x04\x82\x02&'\n8\n\x04\
    \x04\x12\x02\x04\x12\x04\x84\x02\x02\"\x1a*\x20Number\x20of\x20cores\x20\
    available\x20to\x20each\x20server\n\n\r\n\x05\x04\x12\x02\x04\x04\x12\
    \x04\x84\x02\x02\n\n\r\n\x05\x04\x12\x02\x04\x05\x12\x04\x84\x02\x0b\x10\
    \n\r\n\x05\x04\x12\x02\x04\x01\x12\x04\x84\x02\x11\x1d\n\r\n\x05\x04\x12\
    \x02\x04\x03\x12\x04\x84\x02\x20!\n2\n\x04\x04\x12\x02\x05\x12\x04\x86\
    \x02\x02$\x1a$\x20An\x20after-the-fact\x20computed\x20summary\n\n\x0f\n\
    \x05\x04\x12\x02\x05\x04\x12\x06\x86\x02\x02\x84\x02\"\n\r\n\x05\x04\x12\
    \x02\x05\x06\x12\x04\x86\x02\x02\x17\n\r\n\x05\x04\x12\x02\x05\x01\x12\
    \x04\x86\x02\x18\x1f\n\r\n\x05\x04\x12\x02\x05\x03\x12\x04\x86\x02\"#\n@\
    \n\x04\x04\x12\x02\x06\x12\x04\x88\x02\x02#\x1a2\x20Information\x20on\
    \x20success\x20or\x20failure\x20of\x20each\x20worker\n\n\r\n\x05\x04\x12\
    \x02\x06\x04\x12\x04\x88\x02\x02\n\n\r\n\x05\x04\x12\x02\x06\x05\x12\x04\
    \x88\x02\x0b\x0f\n\r\n\x05\x04\x12\x02\x06\x01\x12\x04\x88\x02\x10\x1e\n\
    \r\n\x05\x04\x12\x02\x06\x03\x12\x04\x88\x02!\"\n\x0c\n\x04\x04\x12\x02\
    \x07\x12\x04\x89\x02\x02#\n\r\n\x05\x04\x12\x02\x07\x04\x12\x04\x89\x02\
    \x02\n\n\r\n\x05\x04\x12\x02\x07\x05\x12\x04\x89\x02\x0b\x0f\n\r\n\x05\
    \x04\x12\x02\x07\x01\x12\x04\x89\x02\x10\x1e\n\r\n\x05\x04\x12\x02\x07\
    \x03\x12\x04\x89\x02!\"\nH\n\x04\x04\x12\x02\x08\x12\x04\x8b\x02\x022\
    \x1a:\x20Number\x20of\x20failed\x20requests\x20(one\x20row\x20per\x20sta\
    tus\x20code\x20seen)\n\n\r\n\x05\x04\x12\x02\x08\x04\x12\x04\x8b\x02\x02\
    \n\n\r\n\x05\x04\x12\x02\x08\x06\x12\x04\x8b\x02\x0b\x1d\n\r\n\x05\x04\
    \x12\x02\x08\x01\x12\x04\x8b\x02\x1e-\n\r\n\x05\x04\x12\x02\x08\x03\x12\
    \x04\x8b\x0201b\x06proto3\
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
