use grpc_sys::{self, GrpcMetadataArray};
use std::ffi::CString;
use libc::c_char;

pub struct MetadataArrayBuilder {
    entries: Vec<(*mut c_char, *mut c_char, usize)>
}

impl MetadataArrayBuilder {
    pub fn new() -> MetadataArrayBuilder {
        MetadataArrayBuilder { entries: vec![] }
    }

    pub fn add(mut self, key: Vec<u8>, value: Vec<u8>) -> MetadataArrayBuilder {
        // todo: perhaps assert that key is lowercase
        let value_size = value.len();
        let pair = (
            CString::new(key).unwrap().into_raw(),
            CString::new(value).unwrap().into_raw(),
            value_size
        );
        self.entries.push(pair);
        self
    }

    pub fn build(self) -> MetadataArray {
        let array_size = self.entries.len();
        let array = unsafe { grpc_sys::grpcwrap_metadata_array_create(array_size) };

        for (key, value, value_size) in self.entries {
            unsafe { grpc_sys::grpcwrap_metadata_array_add(array, key, value, value_size) };
        }

        MetadataArray { array }
    }
}

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
