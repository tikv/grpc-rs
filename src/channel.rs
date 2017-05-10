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


use CallOption;
use call::{Call, Method};

use cq::CompletionQueue;
use credentials::ChannelCredentials;
use env::Environment;
use grpc_sys::{self, GprTimespec, GrpcChannel, GrpcChannelArgs};

use libc::{c_char, c_int};
use std::{mem, ptr};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ffi::CString;
use std::sync::Arc;

// hack: add a '\0' to be compatible with c string without extra allocation.
const OPT_DEFAULT_AUTHORITY: &'static str = "grpc.default_authority\0";
const OPT_MAX_CONCURRENT_STREAMS: &'static str = "grpc.max_concurrent_streams\0";
const OPT_MAX_RECEIVE_MESSAGE_LENGTH: &'static str = "grpc.max_receive_message_length\0";
const OPT_MAX_SEND_MESSAGE_LENGTH: &'static str = "grpc.max_send_message_length\0";
const OPT_HTTP2_INITIAL_SEQUENCE_NUMBER: &'static str = "grpc.http2.initial_sequence_number\0";
const OPT_SO_REUSE_PORT: &'static str = "grpc.so_reuseport\0";
const OPT_SSL_TARGET_NAME_OVERRIDE: &'static str = "grpc.ssl_target_name_override\0";
const PRIMARY_USER_AGENT_STRING: &'static str = "grpc.primary_user_agent\0";

fn format_user_agent_string(agent: &str) -> CString {
    let version = env!("CARGO_PKG_VERSION");
    let trimed_agent = agent.trim();
    let val = if trimed_agent.trim().is_empty() {
        format!("grpc-rust/{}", version)
    } else {
        format!("{} grpc-rust/{}", trimed_agent, version)
    };
    CString::new(val).unwrap()
}

enum Options {
    Integer(usize),
    String(CString),
}

pub struct ChannelBuilder {
    environ: Arc<Environment>,
    options: HashMap<&'static str, Options>,
}

impl ChannelBuilder {
    pub fn new(environ: Arc<Environment>) -> ChannelBuilder {
        ChannelBuilder {
            environ: environ,
            options: HashMap::new(),
        }
    }

    pub fn default_authority(mut self, authority: String) -> ChannelBuilder {
        let authority = CString::new(authority).unwrap();
        self.options
            .insert(OPT_DEFAULT_AUTHORITY, Options::String(authority));
        self
    }

    pub fn max_concurrent_stream(mut self, num: usize) -> ChannelBuilder {
        self.options
            .insert(OPT_MAX_CONCURRENT_STREAMS, Options::Integer(num));
        self
    }

    pub fn max_receive_message_len(mut self, len: usize) -> ChannelBuilder {
        self.options
            .insert(OPT_MAX_RECEIVE_MESSAGE_LENGTH, Options::Integer(len));
        self
    }

    pub fn max_send_message_len(mut self, len: usize) -> ChannelBuilder {
        self.options
            .insert(OPT_MAX_SEND_MESSAGE_LENGTH, Options::Integer(len));
        self
    }

    pub fn https_initial_seq_number(mut self, number: usize) -> ChannelBuilder {
        self.options
            .insert(OPT_HTTP2_INITIAL_SEQUENCE_NUMBER, Options::Integer(number));
        self
    }

    pub fn primary_user_agent(mut self, agent: &str) -> ChannelBuilder {
        let agent_string = format_user_agent_string(agent);
        self.options
            .insert(PRIMARY_USER_AGENT_STRING, Options::String(agent_string));
        self
    }

    pub fn reuse_port(mut self, reuse: bool) -> ChannelBuilder {
        let opt = if reuse { 1 } else { 0 };
        self.options
            .insert(OPT_SO_REUSE_PORT, Options::Integer(opt));
        self
    }

    pub fn override_ssl_target(mut self, target: String) -> ChannelBuilder {
        let target = CString::new(target).unwrap();
        self.options
            .insert(OPT_SSL_TARGET_NAME_OVERRIDE, Options::String(target));
        self
    }

    pub fn build_args(&self) -> ChannelArgs {
        let args = unsafe { grpc_sys::grpcwrap_channel_args_create(self.options.len()) };
        for (i, (k, v)) in self.options.iter().enumerate() {
            let key = k.as_ptr() as *const c_char;
            match *v {
                Options::Integer(val) => unsafe {
                    grpc_sys::grpcwrap_channel_args_set_integer(args, i, key, val as c_int)
                },
                Options::String(ref val) => {
                    unsafe {
                        grpc_sys::grpcwrap_channel_args_set_string(args, i, key, val.as_ptr())
                    }
                }
            }
        }
        unsafe { ChannelArgs::from_raw(args) }
    }

    pub fn connect(self, addr: &str) -> Channel {
        self.connect_with_creds(addr, None)
    }

    fn connect_with_creds(mut self, addr: &str, creds: Option<ChannelCredentials>) -> Channel {
        let addr = CString::new(addr).unwrap();
        if let Entry::Vacant(e) = self.options.entry(PRIMARY_USER_AGENT_STRING) {
            e.insert(Options::String(format_user_agent_string("")));
        }
        let args = self.build_args();
        let addr_ptr = addr.as_ptr();
        let channel = unsafe {
            match creds {
                None => {
                    grpc_sys::grpc_insecure_channel_create(addr_ptr, args.args, ptr::null_mut())
                }
                Some(mut creds) => {
                    grpc_sys::grpc_secure_channel_create(creds.as_mut_ptr(),
                                                         addr_ptr,
                                                         args.args,
                                                         ptr::null_mut())
                }
            }
        };

        Channel {
            cq: self.environ.pick_cq(),
            inner: Arc::new(ChannelInner {
                                _environ: self.environ,
                                channel: channel,
                            }),
        }
    }

    pub fn secure_connect(self, addr: &str, creds: ChannelCredentials) -> Channel {
        self.connect_with_creds(addr, Some(creds))
    }
}

pub struct ChannelArgs {
    args: *mut GrpcChannelArgs,
}

impl ChannelArgs {
    pub unsafe fn from_raw(args: *mut GrpcChannelArgs) -> ChannelArgs {
        ChannelArgs { args: args }
    }

    pub fn as_ptr(&self) -> *const GrpcChannelArgs {
        self.args
    }

    pub fn into_raw(self) -> *mut GrpcChannelArgs {
        let args = self.args;
        mem::forget(self);
        args
    }
}

impl Drop for ChannelArgs {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpcwrap_channel_args_destroy(self.args) }
    }
}

struct ChannelInner {
    _environ: Arc<Environment>,
    channel: *mut GrpcChannel,
}

impl Drop for ChannelInner {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_channel_destroy(self.channel);
        }
    }
}

#[derive(Clone)]
pub struct Channel {
    inner: Arc<ChannelInner>,
    cq: Arc<CompletionQueue>,
}

unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}

impl Channel {
    pub fn create_call(&self, method: &Method, opt: &CallOption) -> Call {
        let raw_call = unsafe {
            let ch = self.inner.channel;
            let cq = self.cq.as_ptr();
            let method_ptr = method.name.as_ptr();
            let method_len = method.name.len();
            let timeout = opt.get_timeout()
                .map_or_else(GprTimespec::inf_future, GprTimespec::from);
            grpc_sys::grpcwrap_channel_create_call(ch,
                                                   ptr::null_mut(),
                                                   0,
                                                   cq,
                                                   method_ptr as *const _,
                                                   method_len,
                                                   ptr::null(),
                                                   0,
                                                   timeout,
                                                   ptr::null_mut())
        };

        unsafe { Call::from_raw(raw_call) }
    }
}
