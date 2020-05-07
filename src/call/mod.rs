// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

pub mod client;
pub mod server;

use std::fmt::{self, Debug, Display};
use std::sync::Arc;
use std::{ptr, slice};

use crate::cq::CompletionQueue;
use crate::grpc_sys::{self, grpc_call, grpc_call_error, grpcwrap_batch_context};
use futures::{Async, Future, Poll};
use libc::c_void;

use crate::buf::{GrpcByteBuffer, GrpcByteBufferReader};
use crate::codec::{DeserializeFn, Marshaller, SerializeFn};
use crate::error::{Error, Result};
use crate::grpc_sys::grpc_status_code::*;
use crate::task::{self, BatchFuture, BatchType, CallTag, SpinLock};

// By default buffers in `SinkBase` will be shrink to 4K size.
const BUF_SHRINK_SIZE: usize = 4 * 1024;

/// An gRPC status code structure.
/// This type contains constants for all gRPC status codes.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct RpcStatusCode(i32);

impl From<i32> for RpcStatusCode {
    fn from(code: i32) -> RpcStatusCode {
        RpcStatusCode(code)
    }
}

impl Into<i32> for RpcStatusCode {
    fn into(self) -> i32 {
        self.0
    }
}

impl Display for RpcStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

macro_rules! status_codes {
    (
        $(
            ($num:path, $konst:ident);
        )+
    ) => {
        impl RpcStatusCode {
        $(
            pub const $konst: RpcStatusCode = RpcStatusCode($num);
        )+
        }

        impl Debug for RpcStatusCode {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}-{}",
                    self.0,
                    match self {
                        $(RpcStatusCode($num) => stringify!($konst),)+
                        RpcStatusCode(_) => "INVALID_STATUS_CODE",
                    }
                )
            }
        }
    }
}

status_codes! {
    (GRPC_STATUS_OK, OK);
    (GRPC_STATUS_CANCELLED, CANCELLED);
    (GRPC_STATUS_UNKNOWN, UNKNOWN);
    (GRPC_STATUS_INVALID_ARGUMENT, INVALID_ARGUMENT);
    (GRPC_STATUS_DEADLINE_EXCEEDED, DEADLINE_EXCEEDED);
    (GRPC_STATUS_NOT_FOUND, NOT_FOUND);
    (GRPC_STATUS_ALREADY_EXISTS, ALREADY_EXISTS);
    (GRPC_STATUS_PERMISSION_DENIED, PERMISSION_DENIED);
    (GRPC_STATUS_RESOURCE_EXHAUSTED, RESOURCE_EXHAUSTED);
    (GRPC_STATUS_FAILED_PRECONDITION, FAILED_PRECONDITION);
    (GRPC_STATUS_ABORTED, ABORTED);
    (GRPC_STATUS_OUT_OF_RANGE, OUT_OF_RANGE);
    (GRPC_STATUS_UNIMPLEMENTED, UNIMPLEMENTED);
    (GRPC_STATUS_INTERNAL, INTERNAL);
    (GRPC_STATUS_UNAVAILABLE, UNAVAILABLE);
    (GRPC_STATUS_DATA_LOSS, DATA_LOSS);
    (GRPC_STATUS_UNAUTHENTICATED, UNAUTHENTICATED);
    (GRPC_STATUS__DO_NOT_USE, DO_NOT_USE);
}

/// Method types supported by gRPC.
#[derive(Clone, Copy)]
pub enum MethodType {
    /// Single request sent from client, single response received from server.
    Unary,

    /// Stream of requests sent from client, single response received from server.
    ClientStreaming,

    /// Single request sent from client, stream of responses received from server.
    ServerStreaming,

    /// Both server and client can stream arbitrary number of requests and responses simultaneously.
    Duplex,
}

/// A description of a remote method.
// TODO: add serializer and deserializer.
pub struct Method<Req, Resp> {
    /// Type of method.
    pub ty: MethodType,

    /// Full qualified name of the method.
    pub name: &'static str,

    /// The marshaller used for request messages.
    pub req_mar: Marshaller<Req>,

    /// The marshaller used for response messages.
    pub resp_mar: Marshaller<Resp>,
}

impl<Req, Resp> Method<Req, Resp> {
    /// Get the request serializer.
    #[inline]
    pub fn req_ser(&self) -> SerializeFn<Req> {
        self.req_mar.ser
    }

    /// Get the request deserializer.
    #[inline]
    pub fn req_de(&self) -> DeserializeFn<Req> {
        self.req_mar.de
    }

    /// Get the response serializer.
    #[inline]
    pub fn resp_ser(&self) -> SerializeFn<Resp> {
        self.resp_mar.ser
    }

    /// Get the response deserializer.
    #[inline]
    pub fn resp_de(&self) -> DeserializeFn<Resp> {
        self.resp_mar.de
    }
}

/// RPC result returned from the server.
#[derive(Debug, Clone)]
pub struct RpcStatus {
    /// gRPC status code. `Ok` indicates success, all other values indicate an error.
    pub status: RpcStatusCode,

    /// Optional detail string.
    pub details: Option<String>,
}

impl Display for RpcStatus {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, fmt)
    }
}

impl RpcStatus {
    /// Create a new [`RpcStatus`].
    pub fn new<T: Into<RpcStatusCode>>(code: T, details: Option<String>) -> RpcStatus {
        RpcStatus {
            status: code.into(),
            details,
        }
    }

    /// Create a new [`RpcStatus`] that status code is Ok.
    pub fn ok() -> RpcStatus {
        RpcStatus::new(RpcStatusCode::OK, None)
    }
}

pub type MessageReader = GrpcByteBufferReader;

/// Context for batch request.
pub struct BatchContext {
    ctx: *mut grpcwrap_batch_context,
}

impl BatchContext {
    pub fn new() -> BatchContext {
        BatchContext {
            ctx: unsafe { grpc_sys::grpcwrap_batch_context_create() },
        }
    }

    pub fn as_ptr(&self) -> *mut grpcwrap_batch_context {
        self.ctx
    }

    pub fn take_recv_message(&self) -> Option<GrpcByteBuffer> {
        let ptr = unsafe { grpc_sys::grpcwrap_batch_context_take_recv_message(self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { GrpcByteBuffer::from_raw(ptr) })
        }
    }

    /// Get the status of the rpc call.
    pub fn rpc_status(&self) -> RpcStatus {
        let status = RpcStatusCode(unsafe {
            grpc_sys::grpcwrap_batch_context_recv_status_on_client_status(self.ctx)
        });

        let details = if status == RpcStatusCode::OK {
            None
        } else {
            unsafe {
                let mut details_len = 0;
                let details_ptr = grpc_sys::grpcwrap_batch_context_recv_status_on_client_details(
                    self.ctx,
                    &mut details_len,
                );
                let details_slice = slice::from_raw_parts(details_ptr as *const _, details_len);
                Some(String::from_utf8_lossy(details_slice).into_owned())
            }
        };

        RpcStatus::new(status, details)
    }

    /// Fetch the response bytes of the rpc call.
    pub fn recv_message(&mut self) -> Option<MessageReader> {
        let buf = self.take_recv_message()?;
        Some(GrpcByteBufferReader::new(buf))
    }
}

impl Drop for BatchContext {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpcwrap_batch_context_destroy(self.ctx) }
    }
}

#[inline]
fn box_batch_tag(tag: CallTag) -> (*mut grpcwrap_batch_context, *mut c_void) {
    let tag_box = Box::new(tag);
    (
        tag_box.batch_ctx().unwrap().as_ptr(),
        Box::into_raw(tag_box) as _,
    )
}

/// A helper function that runs the batch call and checks the result.
fn check_run<F>(bt: BatchType, f: F) -> BatchFuture
where
    F: FnOnce(*mut grpcwrap_batch_context, *mut c_void) -> grpc_call_error,
{
    let (cq_f, tag) = CallTag::batch_pair(bt);
    let (batch_ptr, tag_ptr) = box_batch_tag(tag);
    let code = f(batch_ptr, tag_ptr);
    if code != grpc_call_error::GRPC_CALL_OK {
        unsafe {
            Box::from_raw(tag_ptr);
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
    pub call: *mut grpc_call,
    pub cq: CompletionQueue,
}

unsafe impl Send for Call {}

impl Call {
    pub unsafe fn from_raw(call: *mut grpc_sys::grpc_call, cq: CompletionQueue) -> Call {
        assert!(!call.is_null());
        Call { call, cq }
    }

    /// Send a message asynchronously.
    pub fn start_send_message(
        &mut self,
        msg: &[u8],
        write_flags: u32,
        initial_meta: bool,
    ) -> Result<BatchFuture> {
        let _cq_ref = self.cq.borrow()?;
        let i = if initial_meta { 1 } else { 0 };
        let f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_send_message(
                self.call,
                ctx,
                msg.as_ptr() as _,
                msg.len(),
                write_flags,
                i,
                tag,
            )
        });
        Ok(f)
    }

    /// Finish the rpc call from client.
    pub fn start_send_close_client(&mut self) -> Result<BatchFuture> {
        let _cq_ref = self.cq.borrow()?;
        let f = check_run(BatchType::Finish, |_, tag| unsafe {
            grpc_sys::grpcwrap_call_send_close_from_client(self.call, tag)
        });
        Ok(f)
    }

    /// Receive a message asynchronously.
    pub fn start_recv_message(&mut self) -> Result<BatchFuture> {
        let _cq_ref = self.cq.borrow()?;
        let f = check_run(BatchType::Read, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_recv_message(self.call, ctx, tag)
        });
        Ok(f)
    }

    /// Start handling from server side.
    ///
    /// Future will finish once close is received by the server.
    pub fn start_server_side(&mut self) -> Result<BatchFuture> {
        let _cq_ref = self.cq.borrow()?;
        let f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_serverside(self.call, ctx, tag)
        });
        Ok(f)
    }

    /// Send a status from server.
    pub fn start_send_status_from_server(
        &mut self,
        status: &RpcStatus,
        send_empty_metadata: bool,
        payload: &Option<Vec<u8>>,
        write_flags: u32,
    ) -> Result<BatchFuture> {
        let _cq_ref = self.cq.borrow()?;
        let send_empty_metadata = if send_empty_metadata { 1 } else { 0 };
        let (payload_ptr, payload_len) = payload
            .as_ref()
            .map_or((ptr::null(), 0), |b| (b.as_ptr(), b.len()));
        let f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            let details_ptr = status
                .details
                .as_ref()
                .map_or_else(ptr::null, |s| s.as_ptr() as _);
            let details_len = status.details.as_ref().map_or(0, String::len);
            grpc_sys::grpcwrap_call_send_status_from_server(
                self.call,
                ctx,
                status.status.into(),
                details_ptr,
                details_len,
                ptr::null_mut(),
                send_empty_metadata,
                payload_ptr as _,
                payload_len,
                write_flags,
                tag,
            )
        });
        Ok(f)
    }

    /// Abort an rpc call before handler is called.
    pub fn abort(self, status: &RpcStatus) {
        match self.cq.borrow() {
            // Queue is shutdown, ignore.
            Err(Error::QueueShutdown) => return,
            Err(e) => panic!("unexpected error when aborting call: {:?}", e),
            _ => {}
        }
        let call_ptr = self.call;
        let tag = CallTag::abort(self);
        let (batch_ptr, tag_ptr) = box_batch_tag(tag);

        let code = unsafe {
            let details_ptr = status
                .details
                .as_ref()
                .map_or_else(ptr::null, |s| s.as_ptr() as _);
            let details_len = status.details.as_ref().map_or(0, String::len);
            grpc_sys::grpcwrap_call_send_status_from_server(
                call_ptr,
                batch_ptr,
                status.status.into(),
                details_ptr,
                details_len,
                ptr::null_mut(),
                1,
                ptr::null(),
                0,
                0,
                tag_ptr as *mut c_void,
            )
        };
        if code != grpc_call_error::GRPC_CALL_OK {
            unsafe {
                Box::from_raw(tag_ptr);
            }
            panic!("create call fail: {:?}", code);
        }
    }

    /// Cancel the rpc call by client.
    fn cancel(&self) {
        match self.cq.borrow() {
            // Queue is shutdown, ignore.
            Err(Error::QueueShutdown) => return,
            Err(e) => panic!("unexpected error when canceling call: {:?}", e),
            _ => {}
        }
        unsafe {
            grpc_sys::grpc_call_cancel(self.call, ptr::null_mut());
        }
    }
}

impl Drop for Call {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_call_unref(self.call) }
    }
}

/// A share object for client streaming and duplex streaming call.
///
/// In both cases, receiver and sender can be polled in the same time,
/// hence we need to share the call in the both sides and abort the sink
/// once the call is canceled or finished early.
struct ShareCall {
    call: Call,
    close_f: BatchFuture,
    finished: bool,
    status: Option<RpcStatus>,
}

impl ShareCall {
    fn new(call: Call, close_f: BatchFuture) -> ShareCall {
        ShareCall {
            call,
            close_f,
            finished: false,
            status: None,
        }
    }

    /// Poll if the call is still alive.
    ///
    /// If the call is still running, will register a notification for its completion.
    fn poll_finish(&mut self) -> Poll<Option<MessageReader>, Error> {
        let res = match self.close_f.poll() {
            Err(Error::RpcFailure(status)) => {
                self.status = Some(status.clone());
                Err(Error::RpcFailure(status))
            }
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(msg)) => {
                self.status = Some(RpcStatus::ok());
                Ok(Async::Ready(msg))
            }
            res => res,
        };

        self.finished = true;
        res
    }

    /// Check if the call is finished.
    fn check_alive(&mut self) -> Result<()> {
        if self.finished {
            // maybe can just take here.
            return Err(Error::RpcFinished(self.status.clone()));
        }

        task::check_alive(&self.close_f)
    }
}

/// A helper trait that allows executing function on the inernal `ShareCall` struct.
trait ShareCallHolder {
    fn call<R, F: FnOnce(&mut ShareCall) -> R>(&mut self, f: F) -> R;
}

impl ShareCallHolder for ShareCall {
    fn call<R, F: FnOnce(&mut ShareCall) -> R>(&mut self, f: F) -> R {
        f(self)
    }
}

impl ShareCallHolder for Arc<SpinLock<ShareCall>> {
    fn call<R, F: FnOnce(&mut ShareCall) -> R>(&mut self, f: F) -> R {
        let mut call = self.lock();
        f(&mut call)
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
            close_f,
            msg_f: None,
            read_done: false,
        }
    }

    fn poll<C: ShareCallHolder>(
        &mut self,
        call: &mut C,
        skip_finish_check: bool,
    ) -> Poll<Option<MessageReader>, Error> {
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
                return Ok(Async::Ready(bytes));
            }
            return Ok(Async::NotReady);
        }

        // so msg_f must be either stale or not initialised yet.
        self.msg_f.take();
        let msg_f = call.call(|c| c.call.start_recv_message())?;
        self.msg_f = Some(msg_f);
        if bytes.is_none() {
            self.poll(call, true)
        } else {
            Ok(Async::Ready(bytes))
        }
    }

    // Cancel the call if we still have some messages or did not
    // receive status code.
    fn on_drop<C: ShareCallHolder>(&self, call: &mut C) {
        if !self.read_done || self.close_f.is_some() {
            call.call(|c| c.call.cancel());
        }
    }
}

/// Flags for write operations.
#[derive(Default, Clone, Copy)]
pub struct WriteFlags {
    flags: u32,
}

impl WriteFlags {
    /// Hint that the write may be buffered and need not go out on the wire immediately.
    ///
    /// gRPC is free to buffer the message until the next non-buffered write, or until write stream
    /// completion, but it need not buffer completely or at all.
    pub fn buffer_hint(mut self, need_buffered: bool) -> WriteFlags {
        client::change_flag(
            &mut self.flags,
            grpc_sys::GRPC_WRITE_BUFFER_HINT,
            need_buffered,
        );
        self
    }

    /// Force compression to be disabled.
    pub fn force_no_compress(mut self, no_compress: bool) -> WriteFlags {
        client::change_flag(
            &mut self.flags,
            grpc_sys::GRPC_WRITE_NO_COMPRESS,
            no_compress,
        );
        self
    }

    /// Get whether buffer hint is enabled.
    pub fn get_buffer_hint(self) -> bool {
        (self.flags & grpc_sys::GRPC_WRITE_BUFFER_HINT) != 0
    }

    /// Get whether compression is disabled.
    pub fn get_force_no_compress(self) -> bool {
        (self.flags & grpc_sys::GRPC_WRITE_NO_COMPRESS) != 0
    }
}

/// A helper struct for constructing Sink object for batch requests.
struct SinkBase {
    batch_f: Option<BatchFuture>,
    buf: Vec<u8>,
    send_metadata: bool,
}

impl SinkBase {
    fn new(send_metadata: bool) -> SinkBase {
        SinkBase {
            batch_f: None,
            buf: Vec::new(),
            send_metadata,
        }
    }

    fn start_send<T, C: ShareCallHolder>(
        &mut self,
        call: &mut C,
        t: &T,
        mut flags: WriteFlags,
        ser: SerializeFn<T>,
    ) -> Result<bool> {
        if self.batch_f.is_some() {
            // try its best not to return false.
            self.poll_complete()?;
            if self.batch_f.is_some() {
                return Ok(false);
            }
        }

        self.buf.clear();
        ser(t, &mut self.buf);
        if flags.get_buffer_hint() && self.send_metadata {
            // temporary fix: buffer hint with send meta will not send out any metadata.
            flags = flags.buffer_hint(false);
        }
        let write_f = call.call(|c| {
            c.call
                .start_send_message(&self.buf, flags.flags, self.send_metadata)
        })?;
        // NOTE: Content of `self.buf` is copied into grpc internal.
        if self.buf.capacity() > BUF_SHRINK_SIZE {
            self.buf.truncate(BUF_SHRINK_SIZE);
            self.buf.shrink_to_fit();
        }
        self.batch_f = Some(write_f);
        self.send_metadata = false;
        Ok(true)
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        if let Some(ref mut batch_f) = self.batch_f {
            try_ready!(batch_f.poll());
        }

        self.batch_f.take();
        Ok(Async::Ready(()))
    }
}
