use grpc_sys::{self, GrpcMetadataArray};
use std::ffi::CString;
use libc::c_char;
use std::ascii::AsciiExt;
use std::slice;

/// Builder used to construct a MetadataArray value.
pub struct MetadataArrayBuilder {
    entries: Vec<(*mut c_char, *mut c_char, usize)>
}

impl MetadataArrayBuilder {

    /// Create a new empty builder.
    pub fn new() -> MetadataArrayBuilder {
        MetadataArrayBuilder { entries: vec![] }
    }

    /// Add a new key-value pair to the metadata being built.
    pub fn add(mut self, key: Vec<u8>, value: Vec<u8>) -> MetadataArrayBuilder {
        assert!(key.iter()
            .all(|b|
                     ((*b as u32 >= 0x30) && (*b as u32 <= 0x39))        // digits
                         || ((*b as u32 >= 0x41) && (*b as u32 <= 0x5a)) // uppercase
                         || ((*b as u32 >= 0x61) && (*b as u32 <= 0x7a)) // lowercase
                         || (*b as u32 == 0x2e)                          // .
                         || (*b as u32 == 0x2d)                          // -
                         || (*b as u32 == 0x5f)                          // _
            ));

        let key_normalized = key.to_ascii_lowercase();

        let value_size = value.len();
        let pair = (
            CString::new(key_normalized).unwrap().into_raw(),
            CString::new(value).unwrap().into_raw(),
            value_size
        );
        self.entries.push(pair);
        self
    }

    /// Build the metadata array data type that can be used in CallOption
    pub fn build(self) -> MetadataArray {
        let array_size = self.entries.len();
        let array = unsafe { grpc_sys::grpcwrap_metadata_array_create(array_size) };

        for (key, value, value_size) in self.entries {
            unsafe { grpc_sys::grpcwrap_metadata_array_add(array, key, value, value_size) };
        }

        MetadataArray { array }
    }
}

/// Metadata data type used in CallOption data type.
pub struct MetadataArray {
    array: *mut GrpcMetadataArray,
}

impl MetadataArray {
    pub fn as_mut_ptr(&mut self) -> *mut GrpcMetadataArray {
        self.array
    }
}

impl Drop for MetadataArray {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpcwrap_metadata_array_destroy_full(self.array) }
    }
}

/// Immutable handle to the core MetadataArray data type with accessor methods.
pub struct MetadataArrayView {
    array: *const GrpcMetadataArray,
}

impl MetadataArrayView {
    /// Create a new value for the underlying MetadataArray.
    pub fn new(array: *const GrpcMetadataArray) -> MetadataArrayView {
        MetadataArrayView { array }
    }

    /// Number of elements in the MetadataArray.
    pub fn count(&self) -> usize {
        unsafe {
            grpc_sys::grpcwrap_metadata_array_count(self.array)
        }
    }

    /// Return a reference to a slice for the key of the indexed metadata element.
    pub fn key(&self, index: usize) -> &[u8] {
        let mut key_size = 0;
        unsafe {
            let key = grpc_sys::grpcwrap_metadata_array_get_key(self.array, index, &mut key_size);
            slice::from_raw_parts(key as *const u8, key_size)
        }
    }

    /// Return a reference to a slice for the value of the indexed metadata element.
    pub fn value(&self, index: usize) -> &[u8] {
        let mut value_size: usize = 0;
        unsafe {
            let value = grpc_sys::grpcwrap_metadata_array_get_value(self.array, index, &mut value_size);
            slice::from_raw_parts(value as *const u8, value_size)
        }
    }
}
