// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ffi::CStr;

use grpc_sys::{self, GprLogFuncArgs, GprLogSeverity};
use log::{self, Level, LevelFilter, Record};

#[inline]
fn severity_to_log_level(severity: GprLogSeverity) -> Level {
    match severity {
        GprLogSeverity::Debug => Level::Debug,
        GprLogSeverity::Info => Level::Info,
        GprLogSeverity::Error => Level::Error,
    }
}

extern "C" fn delegate(c_args: *mut GprLogFuncArgs) {
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
            .args(format_args!("{}", msg))
            .level(level)
            .file(file_str.into())
            .line(line.into())
            .module_path(module_path!().into())
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
        LevelFilter::Error | LevelFilter::Warn => GprLogSeverity::Error,
        LevelFilter::Info => GprLogSeverity::Info,
        LevelFilter::Debug | LevelFilter::Trace => GprLogSeverity::Debug,
    };

    unsafe {
        grpc_sys::gpr_set_log_verbosity(level);
        grpc_sys::gpr_set_log_function(Some(delegate));
    }
}
