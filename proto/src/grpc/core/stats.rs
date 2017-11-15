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
pub struct Bucket {
    // message fields
    pub start: f64,
    pub count: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Bucket {}

impl Bucket {
    pub fn new() -> Bucket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Bucket {
        static mut instance: ::protobuf::lazy::Lazy<Bucket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bucket,
        };
        unsafe {
            instance.get(Bucket::new)
        }
    }

    // double start = 1;

    pub fn clear_start(&mut self) {
        self.start = 0.;
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: f64) {
        self.start = v;
    }

    pub fn get_start(&self) -> f64 {
        self.start
    }

    fn get_start_for_reflect(&self) -> &f64 {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut f64 {
        &mut self.start
    }

    // uint64 count = 2;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.count = v;
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &u64 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.count
    }
}

impl ::protobuf::Message for Bucket {
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
                    self.start = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.count = tmp;
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
        if self.start != 0. {
            my_size += 9;
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(2, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.start != 0. {
            os.write_double(1, self.start)?;
        }
        if self.count != 0 {
            os.write_uint64(2, self.count)?;
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

impl ::protobuf::MessageStatic for Bucket {
    fn new() -> Bucket {
        Bucket::new()
    }

    fn descriptor_static(_: ::std::option::Option<Bucket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "start",
                    Bucket::get_start_for_reflect,
                    Bucket::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "count",
                    Bucket::get_count_for_reflect,
                    Bucket::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bucket>(
                    "Bucket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Bucket {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bucket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bucket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Histogram {
    // message fields
    pub buckets: ::protobuf::RepeatedField<Bucket>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Histogram {}

impl Histogram {
    pub fn new() -> Histogram {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Histogram {
        static mut instance: ::protobuf::lazy::Lazy<Histogram> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Histogram,
        };
        unsafe {
            instance.get(Histogram::new)
        }
    }

    // repeated .grpc.core.Bucket buckets = 1;

    pub fn clear_buckets(&mut self) {
        self.buckets.clear();
    }

    // Param is passed by value, moved
    pub fn set_buckets(&mut self, v: ::protobuf::RepeatedField<Bucket>) {
        self.buckets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buckets(&mut self) -> &mut ::protobuf::RepeatedField<Bucket> {
        &mut self.buckets
    }

    // Take field
    pub fn take_buckets(&mut self) -> ::protobuf::RepeatedField<Bucket> {
        ::std::mem::replace(&mut self.buckets, ::protobuf::RepeatedField::new())
    }

    pub fn get_buckets(&self) -> &[Bucket] {
        &self.buckets
    }

    fn get_buckets_for_reflect(&self) -> &::protobuf::RepeatedField<Bucket> {
        &self.buckets
    }

    fn mut_buckets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Bucket> {
        &mut self.buckets
    }
}

impl ::protobuf::Message for Histogram {
    fn is_initialized(&self) -> bool {
        for v in &self.buckets {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.buckets)?;
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
        for value in &self.buckets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.buckets {
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

impl ::protobuf::MessageStatic for Histogram {
    fn new() -> Histogram {
        Histogram::new()
    }

    fn descriptor_static(_: ::std::option::Option<Histogram>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Bucket>>(
                    "buckets",
                    Histogram::get_buckets_for_reflect,
                    Histogram::mut_buckets_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Histogram>(
                    "Histogram",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Histogram {
    fn clear(&mut self) {
        self.clear_buckets();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Histogram {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Histogram {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metric {
    // message fields
    pub name: ::std::string::String,
    // message oneof groups
    value: ::std::option::Option<Metric_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metric {}

#[derive(Clone,PartialEq)]
pub enum Metric_oneof_value {
    count(u64),
    histogram(Histogram),
}

impl Metric {
    pub fn new() -> Metric {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metric {
        static mut instance: ::protobuf::lazy::Lazy<Metric> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metric,
        };
        unsafe {
            instance.get(Metric::new)
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

    // uint64 count = 10;

    pub fn clear_count(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Metric_oneof_value::count(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.value = ::std::option::Option::Some(Metric_oneof_value::count(v))
    }

    pub fn get_count(&self) -> u64 {
        match self.value {
            ::std::option::Option::Some(Metric_oneof_value::count(v)) => v,
            _ => 0,
        }
    }

    // .grpc.core.Histogram histogram = 11;

    pub fn clear_histogram(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_histogram(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Metric_oneof_value::histogram(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_histogram(&mut self, v: Histogram) {
        self.value = ::std::option::Option::Some(Metric_oneof_value::histogram(v))
    }

    // Mutable pointer to the field.
    pub fn mut_histogram(&mut self) -> &mut Histogram {
        if let ::std::option::Option::Some(Metric_oneof_value::histogram(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Metric_oneof_value::histogram(Histogram::new()));
        }
        match self.value {
            ::std::option::Option::Some(Metric_oneof_value::histogram(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_histogram(&mut self) -> Histogram {
        if self.has_histogram() {
            match self.value.take() {
                ::std::option::Option::Some(Metric_oneof_value::histogram(v)) => v,
                _ => panic!(),
            }
        } else {
            Histogram::new()
        }
    }

    pub fn get_histogram(&self) -> &Histogram {
        match self.value {
            ::std::option::Option::Some(Metric_oneof_value::histogram(ref v)) => v,
            _ => Histogram::default_instance(),
        }
    }
}

impl ::protobuf::Message for Metric {
    fn is_initialized(&self) -> bool {
        if let Some(Metric_oneof_value::histogram(ref v)) = self.value {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Metric_oneof_value::count(is.read_uint64()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Metric_oneof_value::histogram(is.read_message()?));
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
                &Metric_oneof_value::count(v) => {
                    my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &Metric_oneof_value::histogram(ref v) => {
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
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Metric_oneof_value::count(v) => {
                    os.write_uint64(10, v)?;
                },
                &Metric_oneof_value::histogram(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Metric {
    fn new() -> Metric {
        Metric::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metric>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Metric::get_name_for_reflect,
                    Metric::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "count",
                    Metric::has_count,
                    Metric::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Histogram>(
                    "histogram",
                    Metric::has_histogram,
                    Metric::get_histogram,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metric>(
                    "Metric",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metric {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_count();
        self.clear_histogram();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metric {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metric {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Stats {
    // message fields
    pub metrics: ::protobuf::RepeatedField<Metric>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Stats {}

impl Stats {
    pub fn new() -> Stats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Stats {
        static mut instance: ::protobuf::lazy::Lazy<Stats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Stats,
        };
        unsafe {
            instance.get(Stats::new)
        }
    }

    // repeated .grpc.core.Metric metrics = 1;

    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }

    // Param is passed by value, moved
    pub fn set_metrics(&mut self, v: ::protobuf::RepeatedField<Metric>) {
        self.metrics = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metrics(&mut self) -> &mut ::protobuf::RepeatedField<Metric> {
        &mut self.metrics
    }

    // Take field
    pub fn take_metrics(&mut self) -> ::protobuf::RepeatedField<Metric> {
        ::std::mem::replace(&mut self.metrics, ::protobuf::RepeatedField::new())
    }

    pub fn get_metrics(&self) -> &[Metric] {
        &self.metrics
    }

    fn get_metrics_for_reflect(&self) -> &::protobuf::RepeatedField<Metric> {
        &self.metrics
    }

    fn mut_metrics_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Metric> {
        &mut self.metrics
    }
}

impl ::protobuf::Message for Stats {
    fn is_initialized(&self) -> bool {
        for v in &self.metrics {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metrics)?;
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
        for value in &self.metrics {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.metrics {
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

impl ::protobuf::MessageStatic for Stats {
    fn new() -> Stats {
        Stats::new()
    }

    fn descriptor_static(_: ::std::option::Option<Stats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metric>>(
                    "metrics",
                    Stats::get_metrics_for_reflect,
                    Stats::mut_metrics_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Stats>(
                    "Stats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Stats {
    fn clear(&mut self) {
        self.clear_metrics();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Stats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15grpc/core/stats.proto\x12\tgrpc.core\"4\n\x06Bucket\x12\x14\n\x05s\
    tart\x18\x01\x20\x01(\x01R\x05start\x12\x14\n\x05count\x18\x02\x20\x01(\
    \x04R\x05count\"8\n\tHistogram\x12+\n\x07buckets\x18\x01\x20\x03(\x0b2\
    \x11.grpc.core.BucketR\x07buckets\"s\n\x06Metric\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12\x16\n\x05count\x18\n\x20\x01(\x04H\0R\x05co\
    unt\x124\n\thistogram\x18\x0b\x20\x01(\x0b2\x14.grpc.core.HistogramH\0R\
    \thistogramB\x07\n\x05value\"4\n\x05Stats\x12+\n\x07metrics\x18\x01\x20\
    \x03(\x0b2\x11.grpc.core.MetricR\x07metricsJ\x9a\t\n\x06\x12\x04\x0e\0%\
    \x01\n\xbf\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb4\x04\x20Copyright\x202017\
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
    \x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\x11\n\n\n\
    \x02\x04\0\x12\x04\x12\0\x15\x01\n\n\n\x03\x04\0\x01\x12\x03\x12\x08\x0e\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x13\x02\x13\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x04\x13\x02\x12\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x13\x02\x08\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x13\t\x0e\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x13\x11\x12\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x14\x02\x13\n\
    \r\n\x05\x04\0\x02\x01\x04\x12\x04\x14\x02\x13\x13\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03\x14\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x14\t\
    \x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x14\x11\x12\n\n\n\x02\x04\x01\
    \x12\x04\x17\0\x19\x01\n\n\n\x03\x04\x01\x01\x12\x03\x17\x08\x11\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\x18\x02\x1e\n\x0c\n\x05\x04\x01\x02\0\x04\x12\
    \x03\x18\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x18\x0b\x11\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x03\x18\x12\x19\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03\x18\x1c\x1d\n\n\n\x02\x04\x02\x12\x04\x1b\0!\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x1b\x08\x0e\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1c\x02\
    \x12\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x1c\x02\x1b\x10\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x1c\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x1c\t\r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1c\x10\x11\n\x0c\n\x04\
    \x04\x02\x08\0\x12\x04\x1d\x02\x20\x03\n\x0c\n\x05\x04\x02\x08\0\x01\x12\
    \x03\x1d\x08\r\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x1e\x04\x16\n\x0c\n\
    \x05\x04\x02\x02\x01\x05\x12\x03\x1e\x04\n\n\x0c\n\x05\x04\x02\x02\x01\
    \x01\x12\x03\x1e\x0b\x10\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x1e\x13\
    \x15\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x1f\x04\x1d\n\x0c\n\x05\x04\x02\
    \x02\x02\x06\x12\x03\x1f\x04\r\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\
    \x1f\x0e\x17\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x1f\x1a\x1c\n\n\n\
    \x02\x04\x03\x12\x04#\0%\x01\n\n\n\x03\x04\x03\x01\x12\x03#\x08\r\n\x0b\
    \n\x04\x04\x03\x02\0\x12\x03$\x02\x1e\n\x0c\n\x05\x04\x03\x02\0\x04\x12\
    \x03$\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03$\x0b\x11\n\x0c\n\x05\
    \x04\x03\x02\0\x01\x12\x03$\x12\x19\n\x0c\n\x05\x04\x03\x02\0\x03\x12\
    \x03$\x1c\x1db\x06proto3\
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
