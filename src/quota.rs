use crate::grpc_sys::{self, grpc_resource_quota};
use std::ptr;

/// ResourceQuota represents a bound on memory and thread usage by the gRPC,
/// A ResourceQuota need to be ensure lifetime when the attached Channel is
/// working or the program is aborted.
pub struct ResourceQuota {
    raw: *mut grpc_resource_quota,
}

impl ResourceQuota {
    /// Create a control block for resource quota
    pub fn new(name: Option<&str>) -> ResourceQuota {
        match name {
            Some(name_str) => ResourceQuota {
                raw: unsafe { grpc_sys::grpc_resource_quota_create(name_str.as_ptr() as _) },
            },
            None => ResourceQuota {
                raw: unsafe { grpc_sys::grpc_resource_quota_create(ptr::null()) },
            },
        }
    }

    /// Resize this ResourceQuota to a new memory size.
    pub fn resize(self, new_size: usize) -> ResourceQuota {
        unsafe { grpc_sys::grpc_resource_quota_resize(self.raw, new_size) };
        self
    }

    pub fn get_raw(&self) -> *mut grpc_resource_quota {
        self.raw
    }
}

impl Drop for ResourceQuota {
    fn drop(&mut self) {
        println!("drop ResourceQuota");
        unsafe {
            grpc_sys::grpc_resource_quota_unref(self.raw);
        }
    }
}
