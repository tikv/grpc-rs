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


// TODO: remove following line once all changes are merged into master
#![allow(dead_code)]

pub mod server;

use std::{ptr, result, slice, usize};
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll};
use grpc_sys::{self, GrpcBatchContext, GrpcCall, GrpcCallStatus, GrpcStatusCode};
use libc::c_void;

use async::{BatchFuture, BatchType, Promise};
use error::{Error, Result};

#[derive(Clone, Copy)]
pub enum MethodType {
    Unary,
    ClientStreaming,
    ServerStreaming,
    Duplex,
}

// TODO: add serializer and deserializer.
pub struct Method {
    pub ty: MethodType,
    pub name: &'static str,
}

impl Method {
    pub fn new(ty: MethodType, name: &'static str) -> Method {
        Method { ty: ty, name: name }
    }
}

/// Status return from server.
#[derive(Debug)]
pub struct RpcStatus {
    pub status: GrpcStatusCode,
    pub details: Option<String>,
}

impl RpcStatus {
    pub fn new(status: GrpcStatusCode, details: Option<String>) -> RpcStatus {
        RpcStatus {
            status: status,
            details: details,
        }
    }

    /// Generate an Ok status.
    pub fn ok() -> RpcStatus {
        RpcStatus::new(GrpcStatusCode::Ok, None)
    }
}

/// Context for batch request.
pub struct BatchContext {
    ctx: *mut GrpcBatchContext,
}

impl BatchContext {
    pub fn new() -> BatchContext {
        BatchContext { ctx: unsafe { grpc_sys::grpcwrap_batch_context_create() } }
    }

    pub fn as_ptr(&self) -> *mut GrpcBatchContext {
        self.ctx
    }

    /// Get the status of the rpc call.
    pub fn rpc_status(&self) -> RpcStatus {
        let status =
            unsafe { grpc_sys::grpcwrap_batch_context_recv_status_on_client_status(self.ctx) };
        let details = if status == GrpcStatusCode::Ok {
            None
        } else {
            unsafe {
                let mut details_len = 0;
                let details_ptr = grpc_sys::grpcwrap_batch_context_recv_status_on_client_details(
                    self.ctx, &mut details_len);
                let details_slice = slice::from_raw_parts(details_ptr as *const _, details_len);
                Some(String::from_utf8_lossy(details_slice).into_owned())
            }
        };

        RpcStatus {
            status: status,
            details: details,
        }
    }

    /// Fetch the response bytes of the rpc call.
    // TODO: return Read instead.
    pub fn recv_message(&self) -> Option<Vec<u8>> {
        // TODO: avoid copy
        let len = unsafe { grpc_sys::grpcwrap_batch_context_recv_message_length(self.ctx) };
        if len == usize::MAX {
            return None;
        }
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            grpc_sys::grpcwrap_batch_context_recv_message_to_buffer(self.ctx,
                                                                    buffer.as_mut_ptr() as *mut _,
                                                                    len);
            buffer.set_len(len);
        }
        Some(buffer)
    }
}

impl Drop for BatchContext {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpcwrap_batch_context_destroy(self.ctx) }
    }
}

/// A helper function that runs the batch call and checks the result.
fn check_run<F>(bt: BatchType, f: F) -> BatchFuture
    where F: FnOnce(*mut GrpcBatchContext, *mut c_void) -> GrpcCallStatus
{
    let (cq_f, prom) = Promise::batch_pair(bt);
    let prom_box = Box::new(prom);
    let batch_ptr = prom_box.batch_ctx().unwrap().as_ptr();
    let prom_ptr = Box::into_raw(prom_box);
    let code = f(batch_ptr, prom_ptr as *mut c_void);
    if code != GrpcCallStatus::Ok {
        unsafe {
            Box::from_raw(prom_ptr);
        }
        panic!("create call fail: {:?}", code);
    }
    cq_f
}

/// A Call represents an RPC.
///
/// When created, it is in a configuration state allowing properties to be
/// set until it is invoked. After invoke, the Call can have messages
/// written to it and read from it.
pub struct Call {
    call: *mut GrpcCall,
}

unsafe impl Send for Call {}

impl Call {
    pub unsafe fn from_raw(call: *mut grpc_sys::GrpcCall) -> Call {
        assert!(!call.is_null());
        Call { call: call }
    }

    /// Send a message asynchronously.
    pub fn start_send_message(&mut self,
                              msg: &[u8],
                              write_flags: u32,
                              initial_meta: bool)
                              -> BatchFuture {
        let i = if initial_meta { 1 } else { 0 };
        check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_send_message(self.call,
                                                 ctx,
                                                 msg.as_ptr() as _,
                                                 msg.len(),
                                                 write_flags,
                                                 i,
                                                 tag)
        })
    }

    /// Finish the rpc call from client.
    pub fn start_send_close_client(&mut self) -> BatchFuture {
        check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_send_close_from_client(self.call, ctx, tag)
        })
    }

    /// Receive a message asynchronously.
    pub fn start_recv_message(&mut self) -> BatchFuture {
        check_run(BatchType::Read,
                  |ctx, tag| unsafe { grpc_sys::grpcwrap_call_recv_message(self.call, ctx, tag) })
    }

    /// Start handling from server side.
    ///
    /// Future will finish once close is received by the server.
    pub fn start_server_side(&mut self) -> BatchFuture {
        check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_serverside(self.call, ctx, tag)
        })
    }

    /// Send a status from server.
    pub fn start_send_status_from_server(&mut self,
                                         status: &RpcStatus,
                                         send_empty_metadata: bool,
                                         payload: Option<Vec<u8>>,
                                         write_flags: u32)
                                         -> BatchFuture {
        let send_empty_metadata = if send_empty_metadata { 1 } else { 0 };
        let (payload_ptr, payload_len) = payload
            .as_ref()
            .map_or((ptr::null(), 0), |b| (b.as_ptr(), b.len()));
        check_run(BatchType::Finish, |ctx, tag| unsafe {
            let details_ptr = status
                .details
                .as_ref()
                .map_or_else(ptr::null, |s| s.as_ptr() as _);
            let details_len = status.details.as_ref().map_or(0, String::len);
            grpc_sys::grpcwrap_call_send_status_from_server(self.call,
                                                            ctx,
                                                            status.status,
                                                            details_ptr,
                                                            details_len,
                                                            ptr::null_mut(),
                                                            send_empty_metadata,
                                                            payload_ptr as _,
                                                            payload_len,
                                                            write_flags,
                                                            tag)
        })
    }

    /// Abort a rpc call before handler is called.
    pub fn abort(self, status: RpcStatus) {
        let call_ptr = self.call;
        let prom = Promise::abort(self);
        let prom_box = Box::new(prom);
        let batch_ptr = prom_box.batch_ctx().unwrap().as_ptr();
        let prom_ptr = Box::into_raw(prom_box);

        let code = unsafe {
            let details_ptr = status
                .details
                .as_ref()
                .map_or_else(ptr::null, |s| s.as_ptr() as _);
            let details_len = status.details.as_ref().map_or(0, String::len);
            grpc_sys::grpcwrap_call_send_status_from_server(call_ptr,
                                                            batch_ptr,
                                                            GrpcStatusCode::Unimplemented,
                                                            details_ptr,
                                                            details_len,
                                                            ptr::null_mut(),
                                                            1,
                                                            ptr::null(),
                                                            0,
                                                            0,
                                                            prom_ptr as *mut c_void)
        };
        if code != GrpcCallStatus::Ok {
            unsafe {
                Box::from_raw(prom_ptr);
            }
            panic!("create call fail: {:?}", code);
        }
    }

    /// Cancel the rpc call by client.
    fn cancel(&self) {
        unsafe { grpc_sys::grpc_call_cancel(self.call, ptr::null_mut()) }
    }
}

impl Drop for Call {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_call_destroy(self.call) }
    }
}

/// A helper trait that allow executing function on the inernal call struct.
trait CallHolder {
    fn call<R, F: FnOnce(&mut Call) -> R>(&mut self, f: F) -> R;
}

impl CallHolder for Call {
    #[inline]
    fn call<R, F: FnOnce(&mut Call) -> R>(&mut self, f: F) -> R {
        f(self)
    }
}

impl CallHolder for Arc<Mutex<Call>> {
    #[inline]
    fn call<R, F: FnOnce(&mut Call) -> R>(&mut self, f: F) -> R {
        let mut lock = self.lock().unwrap();
        f(&mut lock)
    }
}

/// A helper struct for constructing Stream object for batch requests.
struct StreamingBase {
    close_f: Option<BatchFuture>,
    msg_f: Option<BatchFuture>,
    read_done: bool,
}

impl StreamingBase {
    fn new(close_f: Option<BatchFuture>) -> StreamingBase {
        StreamingBase {
            close_f: close_f,
            msg_f: None,
            read_done: false,
        }
    }

    fn poll<C: CallHolder>(&mut self,
                           call: &mut C,
                           skip_finish_check: bool)
                           -> Poll<Option<Vec<u8>>, Error> {
        if !skip_finish_check {
            let mut finished = false;
            if let Some(ref mut close_f) = self.close_f {
                match close_f.poll() {
                    Ok(Async::Ready(_)) => {
                        // don't return immediately, there maybe pending data.
                        finished = true;
                    }
                    Err(e) => return Err(e),
                    Ok(Async::NotReady) => {}
                }
            }
            if finished {
                self.close_f.take();
            }
        }

        let mut bytes = None;
        if !self.read_done {
            if let Some(ref mut msg_f) = self.msg_f {
                bytes = try_ready!(msg_f.poll());
                if bytes.is_none() {
                    self.read_done = true;
                }
            }
        }

        if self.read_done {
            if self.close_f.is_none() {
                return Ok(Async::Ready(None));
            }
            return Ok(Async::NotReady);
        }

        // so msg_f must be either stale or not initialised yet.
        self.msg_f.take();
        let msg_f = call.call(|c| c.start_recv_message());
        self.msg_f = Some(msg_f);
        if bytes.is_none() {
            self.poll(call, true)
        } else {
            Ok(Async::Ready(bytes))
        }
    }
}

/// A helper struct for constructing Sink object for batch requests.
struct SinkBase {
    write_f: Option<BatchFuture>,
    close_f: Option<BatchFuture>,
    buf: Vec<u8>,
    flags: u32,
    send_metadata: bool,
}

impl SinkBase {
    fn new(flags: u32, send_metadata: bool) -> SinkBase {
        SinkBase {
            write_f: None,
            close_f: None,
            buf: Vec::new(),
            send_metadata: send_metadata,
            flags: flags,
        }
    }

    fn start_send<F, E, C: CallHolder>(&mut self, call: &mut C, fill_buf: F) -> Result<bool>
        where F: FnOnce(&mut Vec<u8>) -> result::Result<(), E>,
              E: Into<Error>
    {
        if self.write_f.is_some() {
            // try its best not to return false.
            try!(self.poll_complete());
            if self.write_f.is_some() {
                return Ok(false);
            }
        }

        self.buf.clear();
        if let Err(e) = fill_buf(&mut self.buf) {
            return Err(e.into());
        }
        let write_f =
            call.call(|c| c.start_send_message(&self.buf, self.flags, self.send_metadata));
        self.write_f = Some(write_f);
        self.send_metadata = false;
        Ok(true)
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        if let Some(ref mut write_f) = self.write_f {
            try_ready!(write_f.poll());
        }

        self.write_f.take();
        Ok(Async::Ready(()))
    }

    fn close<C: CallHolder>(&mut self, call: &mut C) -> Poll<(), Error> {
        if self.close_f.is_none() {
            try_ready!(self.poll_complete());

            let close_f = call.call(|c| c.start_send_close_client());
            self.close_f = Some(close_f);
        }

        try_ready!(self.close_f.as_mut().unwrap().poll());
        Ok(Async::Ready(()))
    }
}
