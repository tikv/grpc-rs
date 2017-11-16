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
pub struct ServerStats {
    // message fields
    pub time_elapsed: f64,
    pub time_user: f64,
    pub time_system: f64,
    pub total_cpu_time: u64,
    pub idle_cpu_time: u64,
    pub cq_poll_count: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerStats {}

impl ServerStats {
    pub fn new() -> ServerStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerStats {
        static mut instance: ::protobuf::lazy::Lazy<ServerStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerStats,
        };
        unsafe {
            instance.get(ServerStats::new)
        }
    }

    // double time_elapsed = 1;

    pub fn clear_time_elapsed(&mut self) {
        self.time_elapsed = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time_elapsed(&mut self, v: f64) {
        self.time_elapsed = v;
    }

    pub fn get_time_elapsed(&self) -> f64 {
        self.time_elapsed
    }

    fn get_time_elapsed_for_reflect(&self) -> &f64 {
        &self.time_elapsed
    }

    fn mut_time_elapsed_for_reflect(&mut self) -> &mut f64 {
        &mut self.time_elapsed
    }

    // double time_user = 2;

    pub fn clear_time_user(&mut self) {
        self.time_user = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time_user(&mut self, v: f64) {
        self.time_user = v;
    }

    pub fn get_time_user(&self) -> f64 {
        self.time_user
    }

    fn get_time_user_for_reflect(&self) -> &f64 {
        &self.time_user
    }

    fn mut_time_user_for_reflect(&mut self) -> &mut f64 {
        &mut self.time_user
    }

    // double time_system = 3;

    pub fn clear_time_system(&mut self) {
        self.time_system = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time_system(&mut self, v: f64) {
        self.time_system = v;
    }

    pub fn get_time_system(&self) -> f64 {
        self.time_system
    }

    fn get_time_system_for_reflect(&self) -> &f64 {
        &self.time_system
    }

    fn mut_time_system_for_reflect(&mut self) -> &mut f64 {
        &mut self.time_system
    }

    // uint64 total_cpu_time = 4;

    pub fn clear_total_cpu_time(&mut self) {
        self.total_cpu_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_cpu_time(&mut self, v: u64) {
        self.total_cpu_time = v;
    }

    pub fn get_total_cpu_time(&self) -> u64 {
        self.total_cpu_time
    }

    fn get_total_cpu_time_for_reflect(&self) -> &u64 {
        &self.total_cpu_time
    }

    fn mut_total_cpu_time_for_reflect(&mut self) -> &mut u64 {
        &mut self.total_cpu_time
    }

    // uint64 idle_cpu_time = 5;

    pub fn clear_idle_cpu_time(&mut self) {
        self.idle_cpu_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_idle_cpu_time(&mut self, v: u64) {
        self.idle_cpu_time = v;
    }

    pub fn get_idle_cpu_time(&self) -> u64 {
        self.idle_cpu_time
    }

    fn get_idle_cpu_time_for_reflect(&self) -> &u64 {
        &self.idle_cpu_time
    }

    fn mut_idle_cpu_time_for_reflect(&mut self) -> &mut u64 {
        &mut self.idle_cpu_time
    }

    // uint64 cq_poll_count = 6;

    pub fn clear_cq_poll_count(&mut self) {
        self.cq_poll_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_cq_poll_count(&mut self, v: u64) {
        self.cq_poll_count = v;
    }

    pub fn get_cq_poll_count(&self) -> u64 {
        self.cq_poll_count
    }

    fn get_cq_poll_count_for_reflect(&self) -> &u64 {
        &self.cq_poll_count
    }

    fn mut_cq_poll_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.cq_poll_count
    }
}

impl ::protobuf::Message for ServerStats {
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
                    self.time_elapsed = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.time_user = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.time_system = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_cpu_time = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.idle_cpu_time = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cq_poll_count = tmp;
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
        if self.time_elapsed != 0. {
            my_size += 9;
        }
        if self.time_user != 0. {
            my_size += 9;
        }
        if self.time_system != 0. {
            my_size += 9;
        }
        if self.total_cpu_time != 0 {
            my_size += ::protobuf::rt::value_size(4, self.total_cpu_time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.idle_cpu_time != 0 {
            my_size += ::protobuf::rt::value_size(5, self.idle_cpu_time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.cq_poll_count != 0 {
            my_size += ::protobuf::rt::value_size(6, self.cq_poll_count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.time_elapsed != 0. {
            os.write_double(1, self.time_elapsed)?;
        }
        if self.time_user != 0. {
            os.write_double(2, self.time_user)?;
        }
        if self.time_system != 0. {
            os.write_double(3, self.time_system)?;
        }
        if self.total_cpu_time != 0 {
            os.write_uint64(4, self.total_cpu_time)?;
        }
        if self.idle_cpu_time != 0 {
            os.write_uint64(5, self.idle_cpu_time)?;
        }
        if self.cq_poll_count != 0 {
            os.write_uint64(6, self.cq_poll_count)?;
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

impl ::protobuf::MessageStatic for ServerStats {
    fn new() -> ServerStats {
        ServerStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "time_elapsed",
                    ServerStats::get_time_elapsed_for_reflect,
                    ServerStats::mut_time_elapsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "time_user",
                    ServerStats::get_time_user_for_reflect,
                    ServerStats::mut_time_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "time_system",
                    ServerStats::get_time_system_for_reflect,
                    ServerStats::mut_time_system_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_cpu_time",
                    ServerStats::get_total_cpu_time_for_reflect,
                    ServerStats::mut_total_cpu_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "idle_cpu_time",
                    ServerStats::get_idle_cpu_time_for_reflect,
                    ServerStats::mut_idle_cpu_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cq_poll_count",
                    ServerStats::get_cq_poll_count_for_reflect,
                    ServerStats::mut_cq_poll_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerStats>(
                    "ServerStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerStats {
    fn clear(&mut self) {
        self.clear_time_elapsed();
        self.clear_time_user();
        self.clear_time_system();
        self.clear_total_cpu_time();
        self.clear_idle_cpu_time();
        self.clear_cq_poll_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HistogramParams {
    // message fields
    pub resolution: f64,
    pub max_possible: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HistogramParams {}

impl HistogramParams {
    pub fn new() -> HistogramParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HistogramParams {
        static mut instance: ::protobuf::lazy::Lazy<HistogramParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HistogramParams,
        };
        unsafe {
            instance.get(HistogramParams::new)
        }
    }

    // double resolution = 1;

    pub fn clear_resolution(&mut self) {
        self.resolution = 0.;
    }

    // Param is passed by value, moved
    pub fn set_resolution(&mut self, v: f64) {
        self.resolution = v;
    }

    pub fn get_resolution(&self) -> f64 {
        self.resolution
    }

    fn get_resolution_for_reflect(&self) -> &f64 {
        &self.resolution
    }

    fn mut_resolution_for_reflect(&mut self) -> &mut f64 {
        &mut self.resolution
    }

    // double max_possible = 2;

    pub fn clear_max_possible(&mut self) {
        self.max_possible = 0.;
    }

    // Param is passed by value, moved
    pub fn set_max_possible(&mut self, v: f64) {
        self.max_possible = v;
    }

    pub fn get_max_possible(&self) -> f64 {
        self.max_possible
    }

    fn get_max_possible_for_reflect(&self) -> &f64 {
        &self.max_possible
    }

    fn mut_max_possible_for_reflect(&mut self) -> &mut f64 {
        &mut self.max_possible
    }
}

impl ::protobuf::Message for HistogramParams {
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
                    self.resolution = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.max_possible = tmp;
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
        if self.resolution != 0. {
            my_size += 9;
        }
        if self.max_possible != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.resolution != 0. {
            os.write_double(1, self.resolution)?;
        }
        if self.max_possible != 0. {
            os.write_double(2, self.max_possible)?;
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

impl ::protobuf::MessageStatic for HistogramParams {
    fn new() -> HistogramParams {
        HistogramParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<HistogramParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "resolution",
                    HistogramParams::get_resolution_for_reflect,
                    HistogramParams::mut_resolution_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "max_possible",
                    HistogramParams::get_max_possible_for_reflect,
                    HistogramParams::mut_max_possible_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HistogramParams>(
                    "HistogramParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HistogramParams {
    fn clear(&mut self) {
        self.clear_resolution();
        self.clear_max_possible();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HistogramParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HistogramParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HistogramData {
    // message fields
    pub bucket: ::std::vec::Vec<u32>,
    pub min_seen: f64,
    pub max_seen: f64,
    pub sum: f64,
    pub sum_of_squares: f64,
    pub count: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HistogramData {}

impl HistogramData {
    pub fn new() -> HistogramData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HistogramData {
        static mut instance: ::protobuf::lazy::Lazy<HistogramData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HistogramData,
        };
        unsafe {
            instance.get(HistogramData::new)
        }
    }

    // repeated uint32 bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u32>) {
        self.bucket = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.bucket
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.bucket, ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u32] {
        &self.bucket
    }

    fn get_bucket_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.bucket
    }

    fn mut_bucket_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.bucket
    }

    // double min_seen = 2;

    pub fn clear_min_seen(&mut self) {
        self.min_seen = 0.;
    }

    // Param is passed by value, moved
    pub fn set_min_seen(&mut self, v: f64) {
        self.min_seen = v;
    }

    pub fn get_min_seen(&self) -> f64 {
        self.min_seen
    }

    fn get_min_seen_for_reflect(&self) -> &f64 {
        &self.min_seen
    }

    fn mut_min_seen_for_reflect(&mut self) -> &mut f64 {
        &mut self.min_seen
    }

    // double max_seen = 3;

    pub fn clear_max_seen(&mut self) {
        self.max_seen = 0.;
    }

    // Param is passed by value, moved
    pub fn set_max_seen(&mut self, v: f64) {
        self.max_seen = v;
    }

    pub fn get_max_seen(&self) -> f64 {
        self.max_seen
    }

    fn get_max_seen_for_reflect(&self) -> &f64 {
        &self.max_seen
    }

    fn mut_max_seen_for_reflect(&mut self) -> &mut f64 {
        &mut self.max_seen
    }

    // double sum = 4;

    pub fn clear_sum(&mut self) {
        self.sum = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: f64) {
        self.sum = v;
    }

    pub fn get_sum(&self) -> f64 {
        self.sum
    }

    fn get_sum_for_reflect(&self) -> &f64 {
        &self.sum
    }

    fn mut_sum_for_reflect(&mut self) -> &mut f64 {
        &mut self.sum
    }

    // double sum_of_squares = 5;

    pub fn clear_sum_of_squares(&mut self) {
        self.sum_of_squares = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sum_of_squares(&mut self, v: f64) {
        self.sum_of_squares = v;
    }

    pub fn get_sum_of_squares(&self) -> f64 {
        self.sum_of_squares
    }

    fn get_sum_of_squares_for_reflect(&self) -> &f64 {
        &self.sum_of_squares
    }

    fn mut_sum_of_squares_for_reflect(&mut self) -> &mut f64 {
        &mut self.sum_of_squares
    }

    // double count = 6;

    pub fn clear_count(&mut self) {
        self.count = 0.;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: f64) {
        self.count = v;
    }

    pub fn get_count(&self) -> f64 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &f64 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut f64 {
        &mut self.count
    }
}

impl ::protobuf::Message for HistogramData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.bucket)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.min_seen = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.max_seen = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sum = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sum_of_squares = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
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
        for value in &self.bucket {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.min_seen != 0. {
            my_size += 9;
        }
        if self.max_seen != 0. {
            my_size += 9;
        }
        if self.sum != 0. {
            my_size += 9;
        }
        if self.sum_of_squares != 0. {
            my_size += 9;
        }
        if self.count != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.bucket {
            os.write_uint32(1, *v)?;
        };
        if self.min_seen != 0. {
            os.write_double(2, self.min_seen)?;
        }
        if self.max_seen != 0. {
            os.write_double(3, self.max_seen)?;
        }
        if self.sum != 0. {
            os.write_double(4, self.sum)?;
        }
        if self.sum_of_squares != 0. {
            os.write_double(5, self.sum_of_squares)?;
        }
        if self.count != 0. {
            os.write_double(6, self.count)?;
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

impl ::protobuf::MessageStatic for HistogramData {
    fn new() -> HistogramData {
        HistogramData::new()
    }

    fn descriptor_static(_: ::std::option::Option<HistogramData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bucket",
                    HistogramData::get_bucket_for_reflect,
                    HistogramData::mut_bucket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "min_seen",
                    HistogramData::get_min_seen_for_reflect,
                    HistogramData::mut_min_seen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "max_seen",
                    HistogramData::get_max_seen_for_reflect,
                    HistogramData::mut_max_seen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "sum",
                    HistogramData::get_sum_for_reflect,
                    HistogramData::mut_sum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "sum_of_squares",
                    HistogramData::get_sum_of_squares_for_reflect,
                    HistogramData::mut_sum_of_squares_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "count",
                    HistogramData::get_count_for_reflect,
                    HistogramData::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HistogramData>(
                    "HistogramData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HistogramData {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_min_seen();
        self.clear_max_seen();
        self.clear_sum();
        self.clear_sum_of_squares();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HistogramData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HistogramData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestResultCount {
    // message fields
    pub status_code: i32,
    pub count: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestResultCount {}

impl RequestResultCount {
    pub fn new() -> RequestResultCount {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestResultCount {
        static mut instance: ::protobuf::lazy::Lazy<RequestResultCount> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestResultCount,
        };
        unsafe {
            instance.get(RequestResultCount::new)
        }
    }

    // int32 status_code = 1;

    pub fn clear_status_code(&mut self) {
        self.status_code = 0;
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = v;
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code
    }

    fn get_status_code_for_reflect(&self) -> &i32 {
        &self.status_code
    }

    fn mut_status_code_for_reflect(&mut self) -> &mut i32 {
        &mut self.status_code
    }

    // int64 count = 2;

    pub fn clear_count(&mut self) {
        self.count = 0;
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i64) {
        self.count = v;
    }

    pub fn get_count(&self) -> i64 {
        self.count
    }

    fn get_count_for_reflect(&self) -> &i64 {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut i64 {
        &mut self.count
    }
}

impl ::protobuf::Message for RequestResultCount {
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
                    self.status_code = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
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
        if self.status_code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.status_code, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::value_size(2, self.count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status_code != 0 {
            os.write_int32(1, self.status_code)?;
        }
        if self.count != 0 {
            os.write_int64(2, self.count)?;
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

impl ::protobuf::MessageStatic for RequestResultCount {
    fn new() -> RequestResultCount {
        RequestResultCount::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestResultCount>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "status_code",
                    RequestResultCount::get_status_code_for_reflect,
                    RequestResultCount::mut_status_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "count",
                    RequestResultCount::get_count_for_reflect,
                    RequestResultCount::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestResultCount>(
                    "RequestResultCount",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestResultCount {
    fn clear(&mut self) {
        self.clear_status_code();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestResultCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestResultCount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientStats {
    // message fields
    pub latencies: ::protobuf::SingularPtrField<HistogramData>,
    pub time_elapsed: f64,
    pub time_user: f64,
    pub time_system: f64,
    pub request_results: ::protobuf::RepeatedField<RequestResultCount>,
    pub cq_poll_count: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientStats {}

impl ClientStats {
    pub fn new() -> ClientStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientStats {
        static mut instance: ::protobuf::lazy::Lazy<ClientStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientStats,
        };
        unsafe {
            instance.get(ClientStats::new)
        }
    }

    // .grpc.testing.HistogramData latencies = 1;

    pub fn clear_latencies(&mut self) {
        self.latencies.clear();
    }

    pub fn has_latencies(&self) -> bool {
        self.latencies.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latencies(&mut self, v: HistogramData) {
        self.latencies = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_latencies(&mut self) -> &mut HistogramData {
        if self.latencies.is_none() {
            self.latencies.set_default();
        }
        self.latencies.as_mut().unwrap()
    }

    // Take field
    pub fn take_latencies(&mut self) -> HistogramData {
        self.latencies.take().unwrap_or_else(|| HistogramData::new())
    }

    pub fn get_latencies(&self) -> &HistogramData {
        self.latencies.as_ref().unwrap_or_else(|| HistogramData::default_instance())
    }

    fn get_latencies_for_reflect(&self) -> &::protobuf::SingularPtrField<HistogramData> {
        &self.latencies
    }

    fn mut_latencies_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HistogramData> {
        &mut self.latencies
    }

    // double time_elapsed = 2;

    pub fn clear_time_elapsed(&mut self) {
        self.time_elapsed = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time_elapsed(&mut self, v: f64) {
        self.time_elapsed = v;
    }

    pub fn get_time_elapsed(&self) -> f64 {
        self.time_elapsed
    }

    fn get_time_elapsed_for_reflect(&self) -> &f64 {
        &self.time_elapsed
    }

    fn mut_time_elapsed_for_reflect(&mut self) -> &mut f64 {
        &mut self.time_elapsed
    }

    // double time_user = 3;

    pub fn clear_time_user(&mut self) {
        self.time_user = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time_user(&mut self, v: f64) {
        self.time_user = v;
    }

    pub fn get_time_user(&self) -> f64 {
        self.time_user
    }

    fn get_time_user_for_reflect(&self) -> &f64 {
        &self.time_user
    }

    fn mut_time_user_for_reflect(&mut self) -> &mut f64 {
        &mut self.time_user
    }

    // double time_system = 4;

    pub fn clear_time_system(&mut self) {
        self.time_system = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time_system(&mut self, v: f64) {
        self.time_system = v;
    }

    pub fn get_time_system(&self) -> f64 {
        self.time_system
    }

    fn get_time_system_for_reflect(&self) -> &f64 {
        &self.time_system
    }

    fn mut_time_system_for_reflect(&mut self) -> &mut f64 {
        &mut self.time_system
    }

    // repeated .grpc.testing.RequestResultCount request_results = 5;

    pub fn clear_request_results(&mut self) {
        self.request_results.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_results(&mut self, v: ::protobuf::RepeatedField<RequestResultCount>) {
        self.request_results = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_results(&mut self) -> &mut ::protobuf::RepeatedField<RequestResultCount> {
        &mut self.request_results
    }

    // Take field
    pub fn take_request_results(&mut self) -> ::protobuf::RepeatedField<RequestResultCount> {
        ::std::mem::replace(&mut self.request_results, ::protobuf::RepeatedField::new())
    }

    pub fn get_request_results(&self) -> &[RequestResultCount] {
        &self.request_results
    }

    fn get_request_results_for_reflect(&self) -> &::protobuf::RepeatedField<RequestResultCount> {
        &self.request_results
    }

    fn mut_request_results_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RequestResultCount> {
        &mut self.request_results
    }

    // uint64 cq_poll_count = 6;

    pub fn clear_cq_poll_count(&mut self) {
        self.cq_poll_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_cq_poll_count(&mut self, v: u64) {
        self.cq_poll_count = v;
    }

    pub fn get_cq_poll_count(&self) -> u64 {
        self.cq_poll_count
    }

    fn get_cq_poll_count_for_reflect(&self) -> &u64 {
        &self.cq_poll_count
    }

    fn mut_cq_poll_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.cq_poll_count
    }
}

impl ::protobuf::Message for ClientStats {
    fn is_initialized(&self) -> bool {
        for v in &self.latencies {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.latencies)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.time_elapsed = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.time_user = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.time_system = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request_results)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cq_poll_count = tmp;
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
        if let Some(ref v) = self.latencies.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.time_elapsed != 0. {
            my_size += 9;
        }
        if self.time_user != 0. {
            my_size += 9;
        }
        if self.time_system != 0. {
            my_size += 9;
        }
        for value in &self.request_results {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.cq_poll_count != 0 {
            my_size += ::protobuf::rt::value_size(6, self.cq_poll_count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.latencies.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.time_elapsed != 0. {
            os.write_double(2, self.time_elapsed)?;
        }
        if self.time_user != 0. {
            os.write_double(3, self.time_user)?;
        }
        if self.time_system != 0. {
            os.write_double(4, self.time_system)?;
        }
        for v in &self.request_results {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.cq_poll_count != 0 {
            os.write_uint64(6, self.cq_poll_count)?;
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

impl ::protobuf::MessageStatic for ClientStats {
    fn new() -> ClientStats {
        ClientStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HistogramData>>(
                    "latencies",
                    ClientStats::get_latencies_for_reflect,
                    ClientStats::mut_latencies_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "time_elapsed",
                    ClientStats::get_time_elapsed_for_reflect,
                    ClientStats::mut_time_elapsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "time_user",
                    ClientStats::get_time_user_for_reflect,
                    ClientStats::mut_time_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "time_system",
                    ClientStats::get_time_system_for_reflect,
                    ClientStats::mut_time_system_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestResultCount>>(
                    "request_results",
                    ClientStats::get_request_results_for_reflect,
                    ClientStats::mut_request_results_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cq_poll_count",
                    ClientStats::get_cq_poll_count_for_reflect,
                    ClientStats::mut_cq_poll_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientStats>(
                    "ClientStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientStats {
    fn clear(&mut self) {
        self.clear_latencies();
        self.clear_time_elapsed();
        self.clear_time_user();
        self.clear_time_system();
        self.clear_request_results();
        self.clear_cq_poll_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18grpc/testing/stats.proto\x12\x0cgrpc.testing\"\xdc\x01\n\x0bServer\
    Stats\x12!\n\x0ctime_elapsed\x18\x01\x20\x01(\x01R\x0btimeElapsed\x12\
    \x1b\n\ttime_user\x18\x02\x20\x01(\x01R\x08timeUser\x12\x1f\n\x0btime_sy\
    stem\x18\x03\x20\x01(\x01R\ntimeSystem\x12$\n\x0etotal_cpu_time\x18\x04\
    \x20\x01(\x04R\x0ctotalCpuTime\x12\"\n\ridle_cpu_time\x18\x05\x20\x01(\
    \x04R\x0bidleCpuTime\x12\"\n\rcq_poll_count\x18\x06\x20\x01(\x04R\x0bcqP\
    ollCount\"T\n\x0fHistogramParams\x12\x1e\n\nresolution\x18\x01\x20\x01(\
    \x01R\nresolution\x12!\n\x0cmax_possible\x18\x02\x20\x01(\x01R\x0bmaxPos\
    sible\"\xab\x01\n\rHistogramData\x12\x16\n\x06bucket\x18\x01\x20\x03(\rR\
    \x06bucket\x12\x19\n\x08min_seen\x18\x02\x20\x01(\x01R\x07minSeen\x12\
    \x19\n\x08max_seen\x18\x03\x20\x01(\x01R\x07maxSeen\x12\x10\n\x03sum\x18\
    \x04\x20\x01(\x01R\x03sum\x12$\n\x0esum_of_squares\x18\x05\x20\x01(\x01R\
    \x0csumOfSquares\x12\x14\n\x05count\x18\x06\x20\x01(\x01R\x05count\"K\n\
    \x12RequestResultCount\x12\x1f\n\x0bstatus_code\x18\x01\x20\x01(\x05R\ns\
    tatusCode\x12\x14\n\x05count\x18\x02\x20\x01(\x03R\x05count\"\x98\x02\n\
    \x0bClientStats\x129\n\tlatencies\x18\x01\x20\x01(\x0b2\x1b.grpc.testing\
    .HistogramDataR\tlatencies\x12!\n\x0ctime_elapsed\x18\x02\x20\x01(\x01R\
    \x0btimeElapsed\x12\x1b\n\ttime_user\x18\x03\x20\x01(\x01R\x08timeUser\
    \x12\x1f\n\x0btime_system\x18\x04\x20\x01(\x01R\ntimeSystem\x12I\n\x0fre\
    quest_results\x18\x05\x20\x03(\x0b2\x20.grpc.testing.RequestResultCountR\
    \x0erequestResults\x12\"\n\rcq_poll_count\x18\x06\x20\x01(\x04R\x0bcqPol\
    lCountJ\xdb\x17\n\x06\x12\x04\x0e\0J\x01\n\xbf\x04\n\x01\x0c\x12\x03\x0e\
    \0\x122\xb4\x04\x20Copyright\x202015\x20gRPC\x20authors.\n\n\x20Licensed\
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
    \x08\n\x01\x02\x12\x03\x10\x08\x14\n\n\n\x02\x04\0\x12\x04\x12\0%\x01\n\
    \n\n\x03\x04\0\x01\x12\x03\x12\x08\x13\nA\n\x04\x04\0\x02\0\x12\x03\x14\
    \x02\x1a\x1a4\x20wall\x20clock\x20time\x20change\x20in\x20seconds\x20sin\
    ce\x20last\x20reset\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x14\x02\x12\x15\
    \n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x14\x02\x08\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x14\t\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x14\x18\x19\n\
    S\n\x04\x04\0\x02\x01\x12\x03\x17\x02\x17\x1aF\x20change\x20in\x20user\
    \x20time\x20(in\x20seconds)\x20used\x20by\x20the\x20server\x20since\x20l\
    ast\x20reset\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x17\x02\x14\x1a\n\x0c\
    \n\x05\x04\0\x02\x01\x05\x12\x03\x17\x02\x08\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x17\t\x12\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x17\x15\x16\
    \nn\n\x04\x04\0\x02\x02\x12\x03\x1b\x02\x19\x1aa\x20change\x20in\x20serv\
    er\x20time\x20(in\x20seconds)\x20used\x20by\x20the\x20server\x20process\
    \x20and\x20all\n\x20threads\x20since\x20last\x20reset\n\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04\x1b\x02\x17\x17\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03\x1b\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1b\t\x14\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\x1b\x17\x18\nK\n\x04\x04\0\x02\x03\x12\
    \x03\x1e\x02\x1c\x1a>\x20change\x20in\x20total\x20cpu\x20time\x20of\x20t\
    he\x20server\x20(data\x20from\x20proc/stat)\n\n\r\n\x05\x04\0\x02\x03\
    \x04\x12\x04\x1e\x02\x1b\x19\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x1e\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x1e\t\x17\n\x0c\n\x05\x04\
    \0\x02\x03\x03\x12\x03\x1e\x1a\x1b\nF\n\x04\x04\0\x02\x04\x12\x03!\x02\
    \x1b\x1a9\x20change\x20in\x20idle\x20time\x20of\x20the\x20server\x20(dat\
    a\x20from\x20proc/stat)\n\n\r\n\x05\x04\0\x02\x04\x04\x12\x04!\x02\x1e\
    \x1c\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03!\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x04\x01\x12\x03!\t\x16\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03!\x19\
    \x1a\n=\n\x04\x04\0\x02\x05\x12\x03$\x02\x1b\x1a0\x20Number\x20of\x20pol\
    ls\x20called\x20inside\x20completion\x20queue\n\n\r\n\x05\x04\0\x02\x05\
    \x04\x12\x04$\x02!\x1b\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03$\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x05\x01\x12\x03$\t\x16\n\x0c\n\x05\x04\0\x02\x05\
    \x03\x12\x03$\x19\x1a\n@\n\x02\x04\x01\x12\x04(\0+\x01\x1a4\x20Histogram\
    \x20params\x20based\x20on\x20grpc/support/histogram.c\n\n\n\n\x03\x04\
    \x01\x01\x12\x03(\x08\x17\n2\n\x04\x04\x01\x02\0\x12\x03)\x02\x18\"%\x20\
    first\x20bucket\x20is\x20[0,\x201\x20+\x20resolution)\n\n\r\n\x05\x04\
    \x01\x02\0\x04\x12\x04)\x02(\x19\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03)\
    \x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03)\t\x13\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03)\x16\x17\n5\n\x04\x04\x01\x02\x01\x12\x03*\x02\
    \x1a\"(\x20use\x20enough\x20buckets\x20to\x20allow\x20this\x20value\n\n\
    \r\n\x05\x04\x01\x02\x01\x04\x12\x04*\x02)\x18\n\x0c\n\x05\x04\x01\x02\
    \x01\x05\x12\x03*\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03*\t\x15\
    \n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03*\x18\x19\n>\n\x02\x04\x02\x12\
    \x04.\05\x01\x1a2\x20Histogram\x20data\x20based\x20on\x20grpc/support/hi\
    stogram.c\n\n\n\n\x03\x04\x02\x01\x12\x03.\x08\x15\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03/\x02\x1d\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03/\x02\n\n\
    \x0c\n\x05\x04\x02\x02\0\x05\x12\x03/\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\
    \x01\x12\x03/\x12\x18\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03/\x1b\x1c\n\
    \x0b\n\x04\x04\x02\x02\x01\x12\x030\x02\x16\n\r\n\x05\x04\x02\x02\x01\
    \x04\x12\x040\x02/\x1d\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x030\x02\x08\
    \n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x030\t\x11\n\x0c\n\x05\x04\x02\x02\
    \x01\x03\x12\x030\x14\x15\n\x0b\n\x04\x04\x02\x02\x02\x12\x031\x02\x16\n\
    \r\n\x05\x04\x02\x02\x02\x04\x12\x041\x020\x16\n\x0c\n\x05\x04\x02\x02\
    \x02\x05\x12\x031\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x031\t\x11\
    \n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x031\x14\x15\n\x0b\n\x04\x04\x02\
    \x02\x03\x12\x032\x02\x11\n\r\n\x05\x04\x02\x02\x03\x04\x12\x042\x021\
    \x16\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x032\x02\x08\n\x0c\n\x05\x04\
    \x02\x02\x03\x01\x12\x032\t\x0c\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x032\
    \x0f\x10\n\x0b\n\x04\x04\x02\x02\x04\x12\x033\x02\x1c\n\r\n\x05\x04\x02\
    \x02\x04\x04\x12\x043\x022\x11\n\x0c\n\x05\x04\x02\x02\x04\x05\x12\x033\
    \x02\x08\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x033\t\x17\n\x0c\n\x05\x04\
    \x02\x02\x04\x03\x12\x033\x1a\x1b\n\x0b\n\x04\x04\x02\x02\x05\x12\x034\
    \x02\x13\n\r\n\x05\x04\x02\x02\x05\x04\x12\x044\x023\x1c\n\x0c\n\x05\x04\
    \x02\x02\x05\x05\x12\x034\x02\x08\n\x0c\n\x05\x04\x02\x02\x05\x01\x12\
    \x034\t\x0e\n\x0c\n\x05\x04\x02\x02\x05\x03\x12\x034\x11\x12\n\n\n\x02\
    \x04\x03\x12\x047\0:\x01\n\n\n\x03\x04\x03\x01\x12\x037\x08\x1a\n\x0b\n\
    \x04\x04\x03\x02\0\x12\x038\x02\x18\n\r\n\x05\x04\x03\x02\0\x04\x12\x048\
    \x027\x1c\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x038\x02\x07\n\x0c\n\x05\x04\
    \x03\x02\0\x01\x12\x038\x08\x13\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x038\
    \x16\x17\n\x0b\n\x04\x04\x03\x02\x01\x12\x039\x02\x12\n\r\n\x05\x04\x03\
    \x02\x01\x04\x12\x049\x028\x18\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x039\
    \x02\x07\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x039\x08\r\n\x0c\n\x05\x04\
    \x03\x02\x01\x03\x12\x039\x10\x11\n\n\n\x02\x04\x04\x12\x04<\0J\x01\n\n\
    \n\x03\x04\x04\x01\x12\x03<\x08\x13\nA\n\x04\x04\x04\x02\0\x12\x03>\x02\
    \x1e\x1a4\x20Latency\x20histogram.\x20Data\x20points\x20are\x20in\x20nan\
    oseconds.\n\n\r\n\x05\x04\x04\x02\0\x04\x12\x04>\x02<\x15\n\x0c\n\x05\
    \x04\x04\x02\0\x06\x12\x03>\x02\x0f\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03>\x10\x19\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03>\x1c\x1d\n+\n\x04\
    \x04\x04\x02\x01\x12\x03A\x02\x1a\x1a\x1e\x20See\x20ServerStats\x20for\
    \x20details.\n\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04A\x02>\x1e\n\x0c\n\
    \x05\x04\x04\x02\x01\x05\x12\x03A\x02\x08\n\x0c\n\x05\x04\x04\x02\x01\
    \x01\x12\x03A\t\x15\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03A\x18\x19\n\
    \x0b\n\x04\x04\x04\x02\x02\x12\x03B\x02\x17\n\r\n\x05\x04\x04\x02\x02\
    \x04\x12\x04B\x02A\x1a\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03B\x02\x08\
    \n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03B\t\x12\n\x0c\n\x05\x04\x04\x02\
    \x02\x03\x12\x03B\x15\x16\n\x0b\n\x04\x04\x04\x02\x03\x12\x03C\x02\x19\n\
    \r\n\x05\x04\x04\x02\x03\x04\x12\x04C\x02B\x17\n\x0c\n\x05\x04\x04\x02\
    \x03\x05\x12\x03C\x02\x08\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03C\t\x14\
    \n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03C\x17\x18\nG\n\x04\x04\x04\x02\
    \x04\x12\x03F\x022\x1a:\x20Number\x20of\x20failed\x20requests\x20(one\
    \x20row\x20per\x20status\x20code\x20seen)\n\n\x0c\n\x05\x04\x04\x02\x04\
    \x04\x12\x03F\x02\n\n\x0c\n\x05\x04\x04\x02\x04\x06\x12\x03F\x0b\x1d\n\
    \x0c\n\x05\x04\x04\x02\x04\x01\x12\x03F\x1e-\n\x0c\n\x05\x04\x04\x02\x04\
    \x03\x12\x03F01\n=\n\x04\x04\x04\x02\x05\x12\x03I\x02\x1b\x1a0\x20Number\
    \x20of\x20polls\x20called\x20inside\x20completion\x20queue\n\n\r\n\x05\
    \x04\x04\x02\x05\x04\x12\x04I\x02F2\n\x0c\n\x05\x04\x04\x02\x05\x05\x12\
    \x03I\x02\x08\n\x0c\n\x05\x04\x04\x02\x05\x01\x12\x03I\t\x16\n\x0c\n\x05\
    \x04\x04\x02\x05\x03\x12\x03I\x19\x1ab\x06proto3\
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
