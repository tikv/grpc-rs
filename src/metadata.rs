use grpc_sys::{self, GrpcMetadataArray};
use std::ffi::CString;
#[allow(unused)]
use std::ascii::AsciiExt;
use std::slice;

/// Builder used to construct a MetadataArray value.
///   Internally entries are represented as CString, Vec<u8> tuple.
///   CString because key is expected to be a null terminated c-style string,
///   while value is just opaque memory buffer.
pub struct MetadataArrayBuilder {
    entries: Vec<(CString, Vec<u8>)>
}

impl MetadataArrayBuilder {

    /// Create a new empty builder.
    pub fn new() -> MetadataArrayBuilder {
        MetadataArrayBuilder { entries: vec![] }
    }

    /// Create a new builder with entries copied from the supplied MetadataArray.
    pub fn from_metadata_array(array: &MetadataArray) -> MetadataArrayBuilder {
        let mut builder = MetadataArrayBuilder::new();
        let view = MetadataArrayView::new(array.array);

        for index in 0..view.count() {
            let key = view.key(index);
            let value = view.value(index);
            builder = builder.add(key.to_vec(), value.to_vec());
        }
        builder
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
        let pair = (CString::new(key_normalized).unwrap(), value);
        self.entries.push(pair);
        self
    }

    /// Build the metadata array data type that can be used in CallOption
    pub fn build(self) -> MetadataArray {
        let array_size = self.entries.len();
        let array = unsafe { grpc_sys::grpcwrap_metadata_array_create(array_size) };

        for (key, value) in self.entries {
            let value_len = value.len();
            unsafe {
                grpc_sys::grpcwrap_metadata_array_add(
                    array,
                    key.into_raw(),
                    value.as_ptr() as *const _,
                    value_len
                )
            };
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

impl Clone for MetadataArray {
    fn clone(&self) -> Self {
        // Note: For performance reasons this could be re-implemented without using the builder.
        MetadataArrayBuilder::from_metadata_array(&self).build()
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
