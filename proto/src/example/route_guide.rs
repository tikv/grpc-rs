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
pub struct Point {
    // message fields
    pub latitude: i32,
    pub longitude: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Point {}

impl Point {
    pub fn new() -> Point {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Point {
        static mut instance: ::protobuf::lazy::Lazy<Point> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Point,
        };
        unsafe {
            instance.get(Point::new)
        }
    }

    // int32 latitude = 1;

    pub fn clear_latitude(&mut self) {
        self.latitude = 0;
    }

    // Param is passed by value, moved
    pub fn set_latitude(&mut self, v: i32) {
        self.latitude = v;
    }

    pub fn get_latitude(&self) -> i32 {
        self.latitude
    }

    fn get_latitude_for_reflect(&self) -> &i32 {
        &self.latitude
    }

    fn mut_latitude_for_reflect(&mut self) -> &mut i32 {
        &mut self.latitude
    }

    // int32 longitude = 2;

    pub fn clear_longitude(&mut self) {
        self.longitude = 0;
    }

    // Param is passed by value, moved
    pub fn set_longitude(&mut self, v: i32) {
        self.longitude = v;
    }

    pub fn get_longitude(&self) -> i32 {
        self.longitude
    }

    fn get_longitude_for_reflect(&self) -> &i32 {
        &self.longitude
    }

    fn mut_longitude_for_reflect(&mut self) -> &mut i32 {
        &mut self.longitude
    }
}

impl ::protobuf::Message for Point {
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
                    self.latitude = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.longitude = tmp;
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
        if self.latitude != 0 {
            my_size += ::protobuf::rt::value_size(1, self.latitude, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.longitude != 0 {
            my_size += ::protobuf::rt::value_size(2, self.longitude, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.latitude != 0 {
            os.write_int32(1, self.latitude)?;
        }
        if self.longitude != 0 {
            os.write_int32(2, self.longitude)?;
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

impl ::protobuf::MessageStatic for Point {
    fn new() -> Point {
        Point::new()
    }

    fn descriptor_static(_: ::std::option::Option<Point>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "latitude",
                    Point::get_latitude_for_reflect,
                    Point::mut_latitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "longitude",
                    Point::get_longitude_for_reflect,
                    Point::mut_longitude_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Point>(
                    "Point",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Point {
    fn clear(&mut self) {
        self.clear_latitude();
        self.clear_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Point {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Rectangle {
    // message fields
    pub lo: ::protobuf::SingularPtrField<Point>,
    pub hi: ::protobuf::SingularPtrField<Point>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Rectangle {}

impl Rectangle {
    pub fn new() -> Rectangle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Rectangle {
        static mut instance: ::protobuf::lazy::Lazy<Rectangle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Rectangle,
        };
        unsafe {
            instance.get(Rectangle::new)
        }
    }

    // .routeguide.Point lo = 1;

    pub fn clear_lo(&mut self) {
        self.lo.clear();
    }

    pub fn has_lo(&self) -> bool {
        self.lo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lo(&mut self, v: Point) {
        self.lo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lo(&mut self) -> &mut Point {
        if self.lo.is_none() {
            self.lo.set_default();
        }
        self.lo.as_mut().unwrap()
    }

    // Take field
    pub fn take_lo(&mut self) -> Point {
        self.lo.take().unwrap_or_else(|| Point::new())
    }

    pub fn get_lo(&self) -> &Point {
        self.lo.as_ref().unwrap_or_else(|| Point::default_instance())
    }

    fn get_lo_for_reflect(&self) -> &::protobuf::SingularPtrField<Point> {
        &self.lo
    }

    fn mut_lo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Point> {
        &mut self.lo
    }

    // .routeguide.Point hi = 2;

    pub fn clear_hi(&mut self) {
        self.hi.clear();
    }

    pub fn has_hi(&self) -> bool {
        self.hi.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hi(&mut self, v: Point) {
        self.hi = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hi(&mut self) -> &mut Point {
        if self.hi.is_none() {
            self.hi.set_default();
        }
        self.hi.as_mut().unwrap()
    }

    // Take field
    pub fn take_hi(&mut self) -> Point {
        self.hi.take().unwrap_or_else(|| Point::new())
    }

    pub fn get_hi(&self) -> &Point {
        self.hi.as_ref().unwrap_or_else(|| Point::default_instance())
    }

    fn get_hi_for_reflect(&self) -> &::protobuf::SingularPtrField<Point> {
        &self.hi
    }

    fn mut_hi_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Point> {
        &mut self.hi
    }
}

impl ::protobuf::Message for Rectangle {
    fn is_initialized(&self) -> bool {
        for v in &self.lo {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.hi {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lo)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hi)?;
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
        if let Some(ref v) = self.lo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.hi.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.lo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.hi.as_ref() {
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

impl ::protobuf::MessageStatic for Rectangle {
    fn new() -> Rectangle {
        Rectangle::new()
    }

    fn descriptor_static(_: ::std::option::Option<Rectangle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Point>>(
                    "lo",
                    Rectangle::get_lo_for_reflect,
                    Rectangle::mut_lo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Point>>(
                    "hi",
                    Rectangle::get_hi_for_reflect,
                    Rectangle::mut_hi_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Rectangle>(
                    "Rectangle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Rectangle {
    fn clear(&mut self) {
        self.clear_lo();
        self.clear_hi();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Rectangle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Feature {
    // message fields
    pub name: ::std::string::String,
    pub location: ::protobuf::SingularPtrField<Point>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Feature {}

impl Feature {
    pub fn new() -> Feature {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Feature {
        static mut instance: ::protobuf::lazy::Lazy<Feature> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Feature,
        };
        unsafe {
            instance.get(Feature::new)
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

    // .routeguide.Point location = 2;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: Point) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut Point {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> Point {
        self.location.take().unwrap_or_else(|| Point::new())
    }

    pub fn get_location(&self) -> &Point {
        self.location.as_ref().unwrap_or_else(|| Point::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<Point> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Point> {
        &mut self.location
    }
}

impl ::protobuf::Message for Feature {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
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
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.location.as_ref() {
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

impl ::protobuf::MessageStatic for Feature {
    fn new() -> Feature {
        Feature::new()
    }

    fn descriptor_static(_: ::std::option::Option<Feature>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Feature::get_name_for_reflect,
                    Feature::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Point>>(
                    "location",
                    Feature::get_location_for_reflect,
                    Feature::mut_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Feature>(
                    "Feature",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Feature {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Feature {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteNote {
    // message fields
    pub location: ::protobuf::SingularPtrField<Point>,
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteNote {}

impl RouteNote {
    pub fn new() -> RouteNote {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteNote {
        static mut instance: ::protobuf::lazy::Lazy<RouteNote> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteNote,
        };
        unsafe {
            instance.get(RouteNote::new)
        }
    }

    // .routeguide.Point location = 1;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: Point) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut Point {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> Point {
        self.location.take().unwrap_or_else(|| Point::new())
    }

    pub fn get_location(&self) -> &Point {
        self.location.as_ref().unwrap_or_else(|| Point::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<Point> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Point> {
        &mut self.location
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

impl ::protobuf::Message for RouteNote {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
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
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for RouteNote {
    fn new() -> RouteNote {
        RouteNote::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteNote>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Point>>(
                    "location",
                    RouteNote::get_location_for_reflect,
                    RouteNote::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    RouteNote::get_message_for_reflect,
                    RouteNote::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteNote>(
                    "RouteNote",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteNote {
    fn clear(&mut self) {
        self.clear_location();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteNote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteNote {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteSummary {
    // message fields
    pub point_count: i32,
    pub feature_count: i32,
    pub distance: i32,
    pub elapsed_time: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteSummary {}

impl RouteSummary {
    pub fn new() -> RouteSummary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteSummary {
        static mut instance: ::protobuf::lazy::Lazy<RouteSummary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteSummary,
        };
        unsafe {
            instance.get(RouteSummary::new)
        }
    }

    // int32 point_count = 1;

    pub fn clear_point_count(&mut self) {
        self.point_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_point_count(&mut self, v: i32) {
        self.point_count = v;
    }

    pub fn get_point_count(&self) -> i32 {
        self.point_count
    }

    fn get_point_count_for_reflect(&self) -> &i32 {
        &self.point_count
    }

    fn mut_point_count_for_reflect(&mut self) -> &mut i32 {
        &mut self.point_count
    }

    // int32 feature_count = 2;

    pub fn clear_feature_count(&mut self) {
        self.feature_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_feature_count(&mut self, v: i32) {
        self.feature_count = v;
    }

    pub fn get_feature_count(&self) -> i32 {
        self.feature_count
    }

    fn get_feature_count_for_reflect(&self) -> &i32 {
        &self.feature_count
    }

    fn mut_feature_count_for_reflect(&mut self) -> &mut i32 {
        &mut self.feature_count
    }

    // int32 distance = 3;

    pub fn clear_distance(&mut self) {
        self.distance = 0;
    }

    // Param is passed by value, moved
    pub fn set_distance(&mut self, v: i32) {
        self.distance = v;
    }

    pub fn get_distance(&self) -> i32 {
        self.distance
    }

    fn get_distance_for_reflect(&self) -> &i32 {
        &self.distance
    }

    fn mut_distance_for_reflect(&mut self) -> &mut i32 {
        &mut self.distance
    }

    // int32 elapsed_time = 4;

    pub fn clear_elapsed_time(&mut self) {
        self.elapsed_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_elapsed_time(&mut self, v: i32) {
        self.elapsed_time = v;
    }

    pub fn get_elapsed_time(&self) -> i32 {
        self.elapsed_time
    }

    fn get_elapsed_time_for_reflect(&self) -> &i32 {
        &self.elapsed_time
    }

    fn mut_elapsed_time_for_reflect(&mut self) -> &mut i32 {
        &mut self.elapsed_time
    }
}

impl ::protobuf::Message for RouteSummary {
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
                    self.point_count = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.feature_count = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.distance = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.elapsed_time = tmp;
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
        if self.point_count != 0 {
            my_size += ::protobuf::rt::value_size(1, self.point_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.feature_count != 0 {
            my_size += ::protobuf::rt::value_size(2, self.feature_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.distance != 0 {
            my_size += ::protobuf::rt::value_size(3, self.distance, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.elapsed_time != 0 {
            my_size += ::protobuf::rt::value_size(4, self.elapsed_time, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.point_count != 0 {
            os.write_int32(1, self.point_count)?;
        }
        if self.feature_count != 0 {
            os.write_int32(2, self.feature_count)?;
        }
        if self.distance != 0 {
            os.write_int32(3, self.distance)?;
        }
        if self.elapsed_time != 0 {
            os.write_int32(4, self.elapsed_time)?;
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

impl ::protobuf::MessageStatic for RouteSummary {
    fn new() -> RouteSummary {
        RouteSummary::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteSummary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "point_count",
                    RouteSummary::get_point_count_for_reflect,
                    RouteSummary::mut_point_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "feature_count",
                    RouteSummary::get_feature_count_for_reflect,
                    RouteSummary::mut_feature_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "distance",
                    RouteSummary::get_distance_for_reflect,
                    RouteSummary::mut_distance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "elapsed_time",
                    RouteSummary::get_elapsed_time_for_reflect,
                    RouteSummary::mut_elapsed_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteSummary>(
                    "RouteSummary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteSummary {
    fn clear(&mut self) {
        self.clear_point_count();
        self.clear_feature_count();
        self.clear_distance();
        self.clear_elapsed_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteSummary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11route_guide.proto\x12\nrouteguide\"A\n\x05Point\x12\x1a\n\x08latit\
    ude\x18\x01\x20\x01(\x05R\x08latitude\x12\x1c\n\tlongitude\x18\x02\x20\
    \x01(\x05R\tlongitude\"Q\n\tRectangle\x12!\n\x02lo\x18\x01\x20\x01(\x0b2\
    \x11.routeguide.PointR\x02lo\x12!\n\x02hi\x18\x02\x20\x01(\x0b2\x11.rout\
    eguide.PointR\x02hi\"L\n\x07Feature\x12\x12\n\x04name\x18\x01\x20\x01(\t\
    R\x04name\x12-\n\x08location\x18\x02\x20\x01(\x0b2\x11.routeguide.PointR\
    \x08location\"T\n\tRouteNote\x12-\n\x08location\x18\x01\x20\x01(\x0b2\
    \x11.routeguide.PointR\x08location\x12\x18\n\x07message\x18\x02\x20\x01(\
    \tR\x07message\"\x93\x01\n\x0cRouteSummary\x12\x1f\n\x0bpoint_count\x18\
    \x01\x20\x01(\x05R\npointCount\x12#\n\rfeature_count\x18\x02\x20\x01(\
    \x05R\x0cfeatureCount\x12\x1a\n\x08distance\x18\x03\x20\x01(\x05R\x08dis\
    tance\x12!\n\x0celapsed_time\x18\x04\x20\x01(\x05R\x0belapsedTime2\x85\
    \x02\n\nRouteGuide\x126\n\nGetFeature\x12\x11.routeguide.Point\x1a\x13.r\
    outeguide.Feature\"\0\x12>\n\x0cListFeatures\x12\x15.routeguide.Rectangl\
    e\x1a\x13.routeguide.Feature\"\00\x01\x12>\n\x0bRecordRoute\x12\x11.rout\
    eguide.Point\x1a\x18.routeguide.RouteSummary\"\0(\x01\x12?\n\tRouteChat\
    \x12\x15.routeguide.RouteNote\x1a\x15.routeguide.RouteNote\"\0(\x010\x01\
    B6\n\x1bio.grpc.examples.routeguideB\x0fRouteGuideProtoP\x01\xa2\x02\x03\
    RTGJ\xaa\x20\n\x06\x12\x04\x0e\0n\x01\n\xbf\x04\n\x01\x0c\x12\x03\x0e\0\
    \x122\xb4\x04\x20Copyright\x202015\x20gRPC\x20authors.\n\n\x20Licensed\
    \x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"Li\
    cense\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\
    \x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\
    \x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www\
    .apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20appl\
    icable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20d\
    istributed\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\
    \x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITION\
    S\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\
    \x20the\x20License\x20for\x20the\x20specific\x20language\x20governing\
    \x20permissions\x20and\n\x20limitations\x20under\x20the\x20License.\n\n\
    \x08\n\x01\x08\x12\x03\x10\0\"\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x10\0\"\
    \n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x10\x07\x1a\n\r\n\x06\x08\xe7\x07\
    \0\x02\0\x12\x03\x10\x07\x1a\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x10\x07\x1a\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x10\x1d!\n\x08\n\x01\
    \x08\x12\x03\x11\04\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x11\04\n\x0c\n\
    \x05\x08\xe7\x07\x01\x02\x12\x03\x11\x07\x13\n\r\n\x06\x08\xe7\x07\x01\
    \x02\0\x12\x03\x11\x07\x13\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\
    \x11\x07\x13\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x11\x163\n\x08\n\
    \x01\x08\x12\x03\x12\00\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x12\00\n\x0c\
    \n\x05\x08\xe7\x07\x02\x02\x12\x03\x12\x07\x1b\n\r\n\x06\x08\xe7\x07\x02\
    \x02\0\x12\x03\x12\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\
    \x12\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x02\x07\x12\x03\x12\x1e/\n\x08\n\
    \x01\x08\x12\x03\x13\0!\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x13\0!\n\x0c\
    \n\x05\x08\xe7\x07\x03\x02\x12\x03\x13\x07\x18\n\r\n\x06\x08\xe7\x07\x03\
    \x02\0\x12\x03\x13\x07\x18\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\
    \x13\x07\x18\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x13\x1b\x20\n\x08\n\
    \x01\x02\x12\x03\x15\x08\x12\n/\n\x02\x06\0\x12\x04\x18\04\x01\x1a#\x20I\
    nterface\x20exported\x20by\x20the\x20server.\n\n\n\n\x03\x06\0\x01\x12\
    \x03\x18\x08\x12\n\xa1\x01\n\x04\x06\0\x02\0\x12\x03\x1f\x02,\x1a\x93\
    \x01\x20A\x20simple\x20RPC.\n\n\x20Obtains\x20the\x20feature\x20at\x20a\
    \x20given\x20position.\n\n\x20A\x20feature\x20with\x20an\x20empty\x20nam\
    e\x20is\x20returned\x20if\x20there's\x20no\x20feature\x20at\x20the\x20gi\
    ven\n\x20position.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x1f\x06\x10\n\
    \x0c\n\x05\x06\0\x02\0\x02\x12\x03\x1f\x11\x16\n\x0c\n\x05\x06\0\x02\0\
    \x03\x12\x03\x1f!(\n\xa7\x02\n\x04\x06\0\x02\x01\x12\x03'\x029\x1a\x99\
    \x02\x20A\x20server-to-client\x20streaming\x20RPC.\n\n\x20Obtains\x20the\
    \x20Features\x20available\x20within\x20the\x20given\x20Rectangle.\x20\
    \x20Results\x20are\n\x20streamed\x20rather\x20than\x20returned\x20at\x20\
    once\x20(e.g.\x20in\x20a\x20response\x20message\x20with\x20a\n\x20repeat\
    ed\x20field),\x20as\x20the\x20rectangle\x20may\x20cover\x20a\x20large\
    \x20area\x20and\x20contain\x20a\n\x20huge\x20number\x20of\x20features.\n\
    \n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03'\x06\x12\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03'\x13\x1c\n\x0c\n\x05\x06\0\x02\x01\x06\x12\x03''-\n\x0c\
    \n\x05\x06\0\x02\x01\x03\x12\x03'.5\n\xa1\x01\n\x04\x06\0\x02\x02\x12\
    \x03-\x029\x1a\x93\x01\x20A\x20client-to-server\x20streaming\x20RPC.\n\n\
    \x20Accepts\x20a\x20stream\x20of\x20Points\x20on\x20a\x20route\x20being\
    \x20traversed,\x20returning\x20a\n\x20RouteSummary\x20when\x20traversal\
    \x20is\x20completed.\n\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03-\x06\x11\n\
    \x0c\n\x05\x06\0\x02\x02\x05\x12\x03-\x12\x18\n\x0c\n\x05\x06\0\x02\x02\
    \x02\x12\x03-\x19\x1e\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03-)5\n\xb1\x01\
    \n\x04\x06\0\x02\x03\x12\x033\x02?\x1a\xa3\x01\x20A\x20Bidirectional\x20\
    streaming\x20RPC.\n\n\x20Accepts\x20a\x20stream\x20of\x20RouteNotes\x20s\
    ent\x20while\x20a\x20route\x20is\x20being\x20traversed,\n\x20while\x20re\
    ceiving\x20other\x20RouteNotes\x20(e.g.\x20from\x20other\x20users).\n\n\
    \x0c\n\x05\x06\0\x02\x03\x01\x12\x033\x06\x0f\n\x0c\n\x05\x06\0\x02\x03\
    \x05\x12\x033\x10\x16\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x033\x17\x20\n\
    \x0c\n\x05\x06\0\x02\x03\x06\x12\x033+1\n\x0c\n\x05\x06\0\x02\x03\x03\
    \x12\x0332;\n\x91\x02\n\x02\x04\0\x12\x04:\0=\x01\x1a\x84\x02\x20Points\
    \x20are\x20represented\x20as\x20latitude-longitude\x20pairs\x20in\x20the\
    \x20E7\x20representation\n\x20(degrees\x20multiplied\x20by\x2010**7\x20a\
    nd\x20rounded\x20to\x20the\x20nearest\x20integer).\n\x20Latitudes\x20sho\
    uld\x20be\x20in\x20the\x20range\x20+/-\x2090\x20degrees\x20and\x20longit\
    ude\x20should\x20be\x20in\n\x20the\x20range\x20+/-\x20180\x20degrees\x20\
    (inclusive).\n\n\n\n\x03\x04\0\x01\x12\x03:\x08\r\n\x0b\n\x04\x04\0\x02\
    \0\x12\x03;\x02\x15\n\r\n\x05\x04\0\x02\0\x04\x12\x04;\x02:\x0f\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03;\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03;\x08\x10\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03;\x13\x14\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03<\x02\x16\n\r\n\x05\x04\0\x02\x01\x04\x12\x04<\x02\
    ;\x15\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03<\x02\x07\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03<\x08\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03<\x14\
    \x15\nk\n\x02\x04\x01\x12\x04A\0G\x01\x1a_\x20A\x20latitude-longitude\
    \x20rectangle,\x20represented\x20as\x20two\x20diagonally\x20opposite\n\
    \x20points\x20\"lo\"\x20and\x20\"hi\".\n\n\n\n\x03\x04\x01\x01\x12\x03A\
    \x08\x11\n+\n\x04\x04\x01\x02\0\x12\x03C\x02\x0f\x1a\x1e\x20One\x20corne\
    r\x20of\x20the\x20rectangle.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04C\x02A\
    \x13\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03C\x02\x07\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03C\x08\n\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03C\r\x0e\n\
    1\n\x04\x04\x01\x02\x01\x12\x03F\x02\x0f\x1a$\x20The\x20other\x20corner\
    \x20of\x20the\x20rectangle.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04F\x02\
    C\x0f\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03F\x02\x07\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03F\x08\n\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03F\
    \r\x0e\no\n\x02\x04\x02\x12\x04L\0R\x01\x1ac\x20A\x20feature\x20names\
    \x20something\x20at\x20a\x20given\x20point.\n\n\x20If\x20a\x20feature\
    \x20could\x20not\x20be\x20named,\x20the\x20name\x20is\x20empty.\n\n\n\n\
    \x03\x04\x02\x01\x12\x03L\x08\x0f\n'\n\x04\x04\x02\x02\0\x12\x03N\x02\
    \x12\x1a\x1a\x20The\x20name\x20of\x20the\x20feature.\n\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04N\x02L\x11\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03N\x02\
    \x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03N\t\r\n\x0c\n\x05\x04\x02\x02\
    \0\x03\x12\x03N\x10\x11\n7\n\x04\x04\x02\x02\x01\x12\x03Q\x02\x15\x1a*\
    \x20The\x20point\x20where\x20the\x20feature\x20is\x20detected.\n\n\r\n\
    \x05\x04\x02\x02\x01\x04\x12\x04Q\x02N\x12\n\x0c\n\x05\x04\x02\x02\x01\
    \x06\x12\x03Q\x02\x07\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03Q\x08\x10\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x03Q\x13\x14\nC\n\x02\x04\x03\x12\x04\
    U\0[\x01\x1a7\x20A\x20RouteNote\x20is\x20a\x20message\x20sent\x20while\
    \x20at\x20a\x20given\x20point.\n\n\n\n\x03\x04\x03\x01\x12\x03U\x08\x11\
    \n;\n\x04\x04\x03\x02\0\x12\x03W\x02\x15\x1a.\x20The\x20location\x20from\
    \x20which\x20the\x20message\x20is\x20sent.\n\n\r\n\x05\x04\x03\x02\0\x04\
    \x12\x04W\x02U\x13\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03W\x02\x07\n\x0c\
    \n\x05\x04\x03\x02\0\x01\x12\x03W\x08\x10\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03W\x13\x14\n&\n\x04\x04\x03\x02\x01\x12\x03Z\x02\x15\x1a\x19\x20T\
    he\x20message\x20to\x20be\x20sent.\n\n\r\n\x05\x04\x03\x02\x01\x04\x12\
    \x04Z\x02W\x15\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03Z\x02\x08\n\x0c\n\
    \x05\x04\x03\x02\x01\x01\x12\x03Z\t\x10\n\x0c\n\x05\x04\x03\x02\x01\x03\
    \x12\x03Z\x13\x14\n\xff\x01\n\x02\x04\x04\x12\x04b\0n\x01\x1a\xf2\x01\
    \x20A\x20RouteSummary\x20is\x20received\x20in\x20response\x20to\x20a\x20\
    RecordRoute\x20rpc.\n\n\x20It\x20contains\x20the\x20number\x20of\x20indi\
    vidual\x20points\x20received,\x20the\x20number\x20of\n\x20detected\x20fe\
    atures,\x20and\x20the\x20total\x20distance\x20covered\x20as\x20the\x20cu\
    mulative\x20sum\x20of\n\x20the\x20distance\x20between\x20each\x20point.\
    \n\n\n\n\x03\x04\x04\x01\x12\x03b\x08\x14\n-\n\x04\x04\x04\x02\0\x12\x03\
    d\x02\x18\x1a\x20\x20The\x20number\x20of\x20points\x20received.\n\n\r\n\
    \x05\x04\x04\x02\0\x04\x12\x04d\x02b\x16\n\x0c\n\x05\x04\x04\x02\0\x05\
    \x12\x03d\x02\x07\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03d\x08\x13\n\x0c\n\
    \x05\x04\x04\x02\0\x03\x12\x03d\x16\x17\nN\n\x04\x04\x04\x02\x01\x12\x03\
    g\x02\x1a\x1aA\x20The\x20number\x20of\x20known\x20features\x20passed\x20\
    while\x20traversing\x20the\x20route.\n\n\r\n\x05\x04\x04\x02\x01\x04\x12\
    \x04g\x02d\x18\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03g\x02\x07\n\x0c\n\
    \x05\x04\x04\x02\x01\x01\x12\x03g\x08\x15\n\x0c\n\x05\x04\x04\x02\x01\
    \x03\x12\x03g\x18\x19\n.\n\x04\x04\x04\x02\x02\x12\x03j\x02\x15\x1a!\x20\
    The\x20distance\x20covered\x20in\x20metres.\n\n\r\n\x05\x04\x04\x02\x02\
    \x04\x12\x04j\x02g\x1a\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03j\x02\x07\
    \n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03j\x08\x10\n\x0c\n\x05\x04\x04\
    \x02\x02\x03\x12\x03j\x13\x14\n8\n\x04\x04\x04\x02\x03\x12\x03m\x02\x19\
    \x1a+\x20The\x20duration\x20of\x20the\x20traversal\x20in\x20seconds.\n\n\
    \r\n\x05\x04\x04\x02\x03\x04\x12\x04m\x02j\x15\n\x0c\n\x05\x04\x04\x02\
    \x03\x05\x12\x03m\x02\x07\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03m\x08\
    \x14\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03m\x17\x18b\x06proto3\
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
