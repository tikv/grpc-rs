use super::*;

use std::mem;
use std::ptr;
use std::slice;
use std::time::Duration;

impl gpr_timespec {
    pub fn inf_future() -> gpr_timespec {
        unsafe { gpr_inf_future(gpr_clock_type::GPR_CLOCK_REALTIME) }
    }
}

impl Copy for gpr_timespec {}

impl Clone for gpr_timespec {
    fn clone(&self) -> Self {
        gpr_timespec {
            tv_sec: self.tv_sec,
            tv_nsec: self.tv_nsec,
            clock_type: self.clock_type,
        }
    }
}

impl From<Duration> for gpr_timespec {
    fn from(dur: Duration) -> gpr_timespec {
        gpr_timespec {
            tv_sec: dur.as_secs() as i64,
            tv_nsec: dur.subsec_nanos() as i32,
            clock_type: gpr_clock_type::GPR_TIMESPAN,
        }
    }
}

pub struct GrpcSlice(grpc_slice);

impl GrpcSlice {
    pub fn with_capacity(len: usize) -> Self {
        GrpcSlice(unsafe { grpc_slice_malloc(len) })
    }

    pub fn len(&self) -> usize {
        unsafe { grpcwrap_slice_length(&self.0) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn range_from(&self, offset: usize) -> &mut [u8] {
        unsafe {
            let mut len = 0;
            let ptr = grpcwrap_slice_raw_offset(&self.0, offset, &mut len);
            slice::from_raw_parts_mut(ptr as _, len)
        }
    }

    pub fn range_to(&self, size: usize) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = grpcwrap_slice_raw_offset(&self.0, 0, &mut len);
            slice::from_raw_parts(ptr as _, size)
        }
    }
}

impl Clone for GrpcSlice {
    fn clone(&self) -> Self {
        GrpcSlice(unsafe { grpcwrap_slice_ref(&self.0) })
    }
}

impl Default for GrpcSlice {
    fn default() -> Self {
        GrpcSlice(unsafe { grpc_empty_slice() })
    }
}

impl Drop for GrpcSlice {
    fn drop(&mut self) {
        unsafe {
            grpcwrap_slice_unref(&mut self.0);
        }
    }
}

impl<'a> From<&'a [u8]> for GrpcSlice {
    fn from(data: &'a [u8]) -> Self {
        GrpcSlice(unsafe { grpc_slice_from_copied_buffer(data.as_ptr() as _, data.len()) })
    }
}

pub struct GrpcByteBufferReader(grpc_byte_buffer_reader);

impl GrpcByteBufferReader {
    pub fn new(buf: *mut grpc_byte_buffer) -> GrpcByteBufferReader {
        unsafe {
            let mut reader = mem::zeroed();
            let init_result = grpc_byte_buffer_reader_init(&mut reader, buf);
            assert_eq!(init_result, 1);
            GrpcByteBufferReader(reader)
        }
    }

    pub fn len(&self) -> usize {
        unsafe { grpc_byte_buffer_length(self.0.buffer_out) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn next_slice(&mut self) -> GrpcSlice {
        unsafe {
            let mut slice = GrpcSlice::default();
            let code = grpc_byte_buffer_reader_next(&mut self.0, &mut slice.0);
            debug_assert_ne!(code, 0);
            slice
        }
    }
}

impl Drop for GrpcByteBufferReader {
    fn drop(&mut self) {
        unsafe {
            grpc_byte_buffer_reader_destroy(&mut self.0);
        }
    }
}

// TODO should probably live elsewhere
pub fn vec_slice(v: Vec<u8>) -> Box<grpc_slice> {
    let mut data = grpc_slice_grpc_slice_data::default();
    unsafe {
        *data.refcounted.as_mut() = grpc_slice_grpc_slice_data_grpc_slice_refcounted {
            bytes: v.as_ptr() as *const _ as *mut _,
            length: v.len(),
        };
    }
    let mut refcount = VecSliceRefCount::new(v);
    let mut result = Box::new(grpc_slice {
        refcount: ptr::null_mut(),
        data,
    });
    refcount.slice = result.as_mut();
    result.refcount = Box::into_raw(refcount) as *mut _;
    result
}

// comment: grpc_slice_refcount
#[repr(C)]
struct VecSliceRefCount {
    vtable: *const grpc_slice_refcount_vtable,
    sub_refcount: *mut grpc_slice_refcount,
    vec: Vec<u8>,
    count: usize,
    slice: *mut grpc_slice,
}

impl VecSliceRefCount {
    fn new(vec: Vec<u8>) -> Box<VecSliceRefCount> {
        let mut result = Box::new(VecSliceRefCount {
            vtable: &VEC_SLICE_VTABLE,
            sub_refcount: ptr::null_mut(),
            vec,
            count: 1,
            slice: ptr::null_mut(),
        });
        result.sub_refcount = &*result as *const _ as *mut _;
        result
    }
}

static VEC_SLICE_VTABLE: grpc_slice_refcount_vtable = grpc_slice_refcount_vtable {
    ref_: Some(vec_slice_ref),
    unref: Some(vec_slice_unref),
    eq: Some(grpc_slice_default_eq_impl),
    hash: Some(grpc_slice_default_hash_impl),
};

unsafe extern "C" fn vec_slice_ref(arg1: *mut ::std::os::raw::c_void) {
    let refcount = arg1 as *mut VecSliceRefCount;
    (*refcount).count += 1;
}
unsafe extern "C" fn vec_slice_unref(arg1: *mut ::std::os::raw::c_void) {
    let refcount = arg1 as *mut VecSliceRefCount;
    (*refcount).count -= 1;
    if (*refcount).count == 0 {
        let refcount = Box::from_raw(refcount);
        let slice = Box::from_raw(refcount.slice);
        mem::drop(refcount);
        mem::drop(slice);
    }
}
