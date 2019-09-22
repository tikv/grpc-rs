use crate::grpc_sys::{self, grpc_resource_quota};
use std::ffi::CString;
use std::ptr;

/// ResourceQuota represents a bound on memory and thread usage by the gRPC
pub struct ResourceQuota {
    raw: *mut grpc_resource_quota,
}

impl ResourceQuota {
    /// Create a control block for resource quota. If a name is
    /// not declared for this control block, a name is automatically
    /// generated in grpc core.
    pub fn new(name: Option<&str>) -> ResourceQuota {
        match name {
            Some(name_str) => {
                let name_cstr = CString::new(name_str).unwrap();
                ResourceQuota {
                    raw: unsafe { grpc_sys::grpc_resource_quota_create(name_cstr.as_ptr() as _) },
                }
            }
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

    pub(crate) fn get_ptr(&self) -> *mut grpc_resource_quota {
        self.raw
    }
}

impl Drop for ResourceQuota {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_resource_quota_unref(self.raw);
        }
    }
}
