use super::*;

use std::os::raw;
use std::ptr;

use libc::{c_char, c_int, c_uint, c_void, int32_t, int64_t, size_t, uint32_t, uint8_t};
use std::time::Duration;
use std::{mem, slice};

pub const GRPC_INITIAL_METADATA_IDEMPOTENT_REQUEST: u32 = 0x0000_0010;
pub const GRPC_INITIAL_METADATA_WAIT_FOR_READY: u32 = 0x0000_0020;
pub const GRPC_INITIAL_METADATA_CACHEABLE_REQUEST: u32 = 0x0000_0040;

pub const GRPC_WRITE_BUFFER_HINT: u32 = 0x0000_0001;
pub const GRPC_WRITE_NO_COMPRESS: u32 = 0x0000_0002;

impl gpr_timespec {
    pub fn inf_future() -> gpr_timespec {
        unsafe { gpr_inf_future(gpr_clock_type::GPR_CLOCK_REALTIME) }
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

impl From<i32> for grpc_status_code {
    fn from(code: i32) -> grpc_status_code {
        match code {
            -1 => grpc_status_code::GRPC_STATUS__DO_NOT_USE,
            0 => grpc_status_code::GRPC_STATUS_OK,
            1 => grpc_status_code::GRPC_STATUS_CANCELLED,
            2 => grpc_status_code::GRPC_STATUS_UNKNOWN,
            3 => grpc_status_code::GRPC_STATUS_INVALID_ARGUMENT,
            4 => grpc_status_code::GRPC_STATUS_DEADLINE_EXCEEDED,
            5 => grpc_status_code::GRPC_STATUS_NOT_FOUND,
            6 => grpc_status_code::GRPC_STATUS_ALREADY_EXISTS,
            7 => grpc_status_code::GRPC_STATUS_PERMISSION_DENIED,
            8 => grpc_status_code::GRPC_STATUS_RESOURCE_EXHAUSTED,
            9 => grpc_status_code::GRPC_STATUS_FAILED_PRECONDITION,
            10 => grpc_status_code::GRPC_STATUS_ABORTED,
            11 => grpc_status_code::GRPC_STATUS_OUT_OF_RANGE,
            12 => grpc_status_code::GRPC_STATUS_UNIMPLEMENTED,
            13 => grpc_status_code::GRPC_STATUS_INTERNAL,
            14 => grpc_status_code::GRPC_STATUS_UNAVAILABLE,
            15 => grpc_status_code::GRPC_STATUS_DATA_LOSS,
            16 => grpc_status_code::GRPC_STATUS_UNAUTHENTICATED,
            _ => panic!("get wrong grpc_status_code"),
        }
    }
}
