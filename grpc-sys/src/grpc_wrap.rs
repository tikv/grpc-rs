use super::*;

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

impl grpc_slice {
    pub fn with_capacity(len: usize) -> Self {
        unsafe { grpc_slice_malloc(len) }
    }

    pub fn len(&self) -> usize {
        unsafe { grpcwrap_slice_length(self) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn range_from(&self, offset: usize) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = grpcwrap_slice_raw_offset(self, offset, &mut len);
            slice::from_raw_parts(ptr as _, len)
        }
    }

    pub fn range_to(&self, size: usize) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = grpcwrap_slice_raw_offset(self, 0, &mut len);
            slice::from_raw_parts(ptr as _, size)
        }
    }

    pub unsafe fn range_from_unsafe(&mut self, offset: usize) -> &mut [u8] {
        let mut len = 0;
        let ptr = grpcwrap_slice_raw_offset(self, offset, &mut len);
        slice::from_raw_parts_mut(ptr as _, len)
    }
}

impl Clone for grpc_slice {
    fn clone(&self) -> Self {
        unsafe { grpcwrap_slice_ref(self) }
    }
}

impl Default for grpc_slice {
    fn default() -> Self {
        unsafe { grpc_empty_slice() }
    }
}

impl Drop for grpc_slice {
    fn drop(&mut self) {
        unsafe {
            grpcwrap_slice_unref(self);
        }
    }
}

impl<'a> From<&'a [u8]> for grpc_slice {
    fn from(data: &'a [u8]) -> Self {
        unsafe { grpc_slice_from_copied_buffer(data.as_ptr() as _, data.len()) }
    }
}

impl grpc_byte_buffer_reader {
    pub fn len(&self) -> usize {
        unsafe { grpc_byte_buffer_length(self.buffer_out) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn next_slice(&mut self) -> grpc_slice {
        unsafe {
            let mut slice = Default::default();
            let code = grpc_byte_buffer_reader_next(self, &mut slice);
            debug_assert_ne!(code, 0);
            slice
        }
    }
}
