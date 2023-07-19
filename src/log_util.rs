// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::ffi::CStr;

use crate::grpc_sys::{self, gpr_log_func_args, gpr_log_severity};
use log::{self, Level, LevelFilter, Record};

#[inline]
fn severity_to_log_level(severity: gpr_log_severity) -> Level {
    match severity {
        gpr_log_severity::GPR_LOG_SEVERITY_DEBUG => Level::Debug,
        gpr_log_severity::GPR_LOG_SEVERITY_INFO => Level::Info,
        gpr_log_severity::GPR_LOG_SEVERITY_ERROR => Level::Error,
    }
}

extern "C" fn delegate(c_args: *mut gpr_log_func_args) {
    let args = unsafe { &*c_args };
    let level = severity_to_log_level(args.severity);
    if !log_enabled!(level) {
        return;
    }

    // can't panic.
    let file_str = unsafe { CStr::from_ptr(args.file).to_str().unwrap() };
    let line = args.line as u32;

    let msg = unsafe { CStr::from_ptr(args.message).to_string_lossy() };
    log::logger().log(
        &Record::builder()
            .args(format_args!("{msg}"))
            .level(level)
            .file_static(file_str.into())
            .line(line.into())
            .module_path_static(module_path!().into())
            .build(),
    );
}

/// Redirect grpc log to rust's log implementation.
pub fn redirect_log() {
    let level = match log::max_level() {
        LevelFilter::Off => unsafe {
            // disable log.
            grpc_sys::gpr_set_log_function(None);
            return;
        },
        LevelFilter::Error | LevelFilter::Warn => gpr_log_severity::GPR_LOG_SEVERITY_ERROR,
        LevelFilter::Info => gpr_log_severity::GPR_LOG_SEVERITY_INFO,
        LevelFilter::Debug | LevelFilter::Trace => gpr_log_severity::GPR_LOG_SEVERITY_DEBUG,
    };

    unsafe {
        grpc_sys::gpr_set_log_verbosity(level);
        grpc_sys::gpr_set_log_function(Some(delegate));
    }
}
