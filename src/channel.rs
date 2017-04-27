
use CallOption;
use call::{Call, Method};

use cq::CompletionQueue;
use env::Environment;
use grpc_sys::{self, GprTimespec, GrpcChannel, GrpcChannelArgs};

use libc::{c_char, c_int};
use std::{mem, ptr};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ffi::CString;
use std::sync::Arc;

// hack: add a '\0' to be compatible with c string without extra allocation.
const OPT_MAX_CONCURRENT_STREAMS: &'static str = "grpc.max_concurrent_streams\0";
const OPT_MAX_RECEIVE_MESSAGE_LENGTH: &'static str = "grpc.max_receive_message_length\0";
const OPT_MAX_SEND_MESSAGE_LENGTH: &'static str = "grpc.max_send_message_length\0";
const OPT_HTTP2_INITIAL_SEQUENCE_NUMBER: &'static str = "grpc.http2.initial_sequence_number\0";
const PRIMARY_USER_AGENT_STRING: &'static str = "grpc.primary_user_agent\0";

fn format_user_agent_string(agent: &str) -> String {
    let version = env!("CARGO_PKG_VERSION");
    let trimed_agent = agent.trim();
    if trimed_agent.trim().is_empty() {
        format!("grpc-rust/{}\0", version)
    } else {
        format!("{} grpc-rust/{}\0", trimed_agent, version)
    }
}

enum Options {
    Integer(usize),
    String(String),
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

    pub fn max_concurrent_stream(mut self, num: usize) -> ChannelBuilder {
        self.options.insert(OPT_MAX_CONCURRENT_STREAMS, Options::Integer(num));
        self
    }

    pub fn max_receive_message_len(mut self, len: usize) -> ChannelBuilder {
        self.options.insert(OPT_MAX_RECEIVE_MESSAGE_LENGTH, Options::Integer(len));
        self
    }

    pub fn max_send_message_len(mut self, len: usize) -> ChannelBuilder {
        self.options.insert(OPT_MAX_SEND_MESSAGE_LENGTH, Options::Integer(len));
        self
    }

    pub fn https_initial_seq_number(mut self, number: usize) -> ChannelBuilder {
        self.options.insert(OPT_HTTP2_INITIAL_SEQUENCE_NUMBER, Options::Integer(number));
        self
    }

    pub fn primary_user_agent(mut self, agent: &str) -> ChannelBuilder {
        let agent_string = format_user_agent_string(agent);
        self.options.insert(PRIMARY_USER_AGENT_STRING, Options::String(agent_string));
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
                Options::String(ref val) => unsafe {
                    grpc_sys::grpcwrap_channel_args_set_string(args,
                                                               i,
                                                               key,
                                                               val.as_ptr() as *const c_char)
                },
            }
        }
        unsafe { ChannelArgs::from_raw(args) }
    }

    // TODO: support ssl
    pub fn connect(mut self, addr: &str) -> Channel {
        let addr = CString::new(addr).unwrap();
        if let Entry::Vacant(e) = self.options.entry(PRIMARY_USER_AGENT_STRING) {
            e.insert(Options::String(format_user_agent_string("")));
        }
        let args = self.build_args();
        let channel = unsafe {
            grpc_sys::grpc_insecure_channel_create(addr.as_ptr(), args.args, ptr::null_mut())
        };

        Channel {
            cq: self.environ.pick_a_cq(),
            inner: Arc::new(ChannelInner {
                _environ: self.environ,
                channel: channel,
            }),
        }
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

impl Channel {
    pub fn create_call(&self, method: &Method, opt: &CallOption) -> Call {
        let raw_call = unsafe {
            let ch = self.inner.channel;
            let cq = self.cq.as_ptr();
            let method_ptr = method.name.as_ptr();
            let method_len = method.name.len();
            let timeout = opt.timeout().map_or_else(GprTimespec::inf_future, GprTimespec::from);
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
