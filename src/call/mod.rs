pub mod client;
pub mod server;

use std::{mem, slice, ptr, result, usize};

use grpc_sys::{self, GrpcStatusCode, GrpcCallStatus, GrpcBatchContext, GrpcCall};
use protobuf::Message;
use futures::{Poll, Async, Sink, AsyncSink, Stream};

use promise::{self, CqFuture, Promise, PromiseType};
use channel::Channel;
use self::client::{CallOption, UnaryCallHandler, ClientStreamingCallHandler, ServerStreamingCallHandler, DuplexStreamingCallHandler};
use error::{Result, Error};

#[derive(Clone, Copy)]
pub enum MethodType {
    Unary,
    ClientStreaming,
    ServerStreaming,
    Dulex
}

pub struct Method {
    pub ty: MethodType,
    pub name: &'static str,
}

impl Method {
    pub fn new(ty: MethodType, name: &'static str) -> Method {
        Method {
            ty: ty,
            name: name,
        }
    }
}

#[derive(Debug)]
pub struct RpcStatus {
    pub status: GrpcStatusCode,
    pub details: String,
}

impl RpcStatus {
    pub fn new(status: GrpcStatusCode) -> RpcStatus {
        RpcStatus {
            status: status,
            details: "".to_owned(),
        }
    }

    pub fn ok() -> RpcStatus {
        RpcStatus::new(GrpcStatusCode::Ok)
    }
}

pub struct BatchContext {
    ctx: *mut GrpcBatchContext,
}

impl BatchContext {
    pub fn new() -> BatchContext {
        BatchContext {
            ctx: unsafe {
                grpc_sys::grpcwrap_batch_context_create()
            }
        }
    }

    pub unsafe fn from_raw(ctx: *mut GrpcBatchContext) -> BatchContext {
        BatchContext {
            ctx: ctx,
        }
    }

    pub fn into_raw(self) -> *mut GrpcBatchContext {
        let ctx = self.ctx;
        // So the drop method won't be called.
        mem::forget(self);
        ctx
    }

    pub fn rpc_status(&self) -> RpcStatus {
        let status = unsafe {
            grpc_sys::grpcwrap_batch_context_recv_status_on_client_status(self.ctx)
        };
        let details = if status == GrpcStatusCode::Ok {
            String::new()
        } else {
            unsafe {
                let mut details_len = 0;
                let details_ptr = grpc_sys::grpcwrap_batch_context_recv_status_on_client_details(self.ctx, &mut details_len);
                let details_slice = slice::from_raw_parts(details_ptr as *const _, details_len);
                String::from_utf8_lossy(details_slice).into_owned()
            }
        };
        
        RpcStatus {
            status: status,
            details: details,
        }
    }

    pub fn recv_message(&self) -> Vec<u8> {
        // TODO: avoid copy
        let len = unsafe {
            grpc_sys::grpcwrap_batch_context_recv_message_length(self.ctx)
        };
        if len == usize::MAX {
            return Vec::new();
        }
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            grpc_sys::grpcwrap_batch_context_recv_message_to_buffer(self.ctx, buffer.as_mut_ptr() as *mut _, len);
            buffer.set_len(len);
        }
        buffer
    }

    pub fn is_closed_on_server_canceled(&self) -> bool {
        unsafe {
            grpc_sys::grpcwrap_batch_context_recv_close_on_server_cancelled(self.ctx) != 0
        }
    }

    fn set_promise(&mut self, cb: Box<Promise>) {
        unsafe {
            grpc_sys::grpcwrap_batch_context_set_tag(self.ctx, Box::into_raw(cb) as *mut _)
        }
    }

    pub fn take_promise(&mut self) -> Option<Box<Promise>> {
        unsafe {
            let tag = grpc_sys::grpcwrap_batch_context_take_tag(self.ctx);
            if tag.is_null() {
                return None;
            }
            Some(Box::from_raw(tag as *mut _))
        }
    }
}

impl Drop for BatchContext {
    fn drop(&mut self) {
        self.take_promise();

        unsafe {
            grpc_sys::grpcwrap_batch_context_destroy(self.ctx)
        }
    }
}

fn check_run<F>(promise_type: PromiseType, f: F) -> Result<CqFuture> where F: FnOnce(*mut GrpcBatchContext) -> GrpcCallStatus {
    let (cq_f, prom) = promise::pair(promise_type);
    try!(_check_run(Some(prom), f));
    Ok(cq_f)
}

fn _check_run<F>(promise: Option<Promise>, f: F) -> Result<()> where F: FnOnce(*mut GrpcBatchContext) -> GrpcCallStatus {
    let mut ctx = BatchContext::new();
    if let Some(promise) = promise {
        ctx.set_promise(Box::new(promise));
    }
    let tag = ctx.into_raw();
    let code = f(tag);
    if code != GrpcCallStatus::Ok {
        unsafe {
            BatchContext::from_raw(tag);
        }
        return Err(Error::CallFailure(code));
    }
    Ok(())
}

fn check_run_without_promise<F>(f: F) -> Result<()> where F: FnOnce(*mut GrpcBatchContext) -> GrpcCallStatus {
    _check_run(None, f)
}

pub struct Call {
    call: *mut GrpcCall,
}

unsafe impl Send for Call {}

impl Call {
    pub unsafe fn from_raw(call: *mut grpc_sys::GrpcCall) -> Call {
        assert!(!call.is_null());
        Call { call: call }
    }

    pub fn start_send_message(&mut self, msg: &[u8], write_flags: u32, initial_meta: bool) -> Result<CqFuture> {
        let i = if initial_meta { 1 } else { 0 };
        check_run(PromiseType::Finish, |tag| unsafe {
            grpc_sys::grpcwrap_call_send_message(self.call, tag, msg.as_ptr() as _, msg.len(), write_flags, i)
        })
    }

    pub fn start_send_close_client(&mut self) -> Result<CqFuture> {
        check_run(PromiseType::Finish, |tag| unsafe {
            grpc_sys::grpcwrap_call_send_close_from_client(self.call, tag)
        })
    }

    pub fn start_recv_message(&mut self) -> Result<CqFuture> {
        check_run(PromiseType::ReadOne, |tag| unsafe {
            grpc_sys::grpcwrap_call_recv_message(self.call, tag)
        })
    }

    pub fn start_server_side(&mut self) -> Result<CqFuture> {
        check_run(PromiseType::Finish, |tag| unsafe {
            grpc_sys::grpcwrap_call_start_serverside(self.call, tag)
        })
    }

    pub fn start_send_status_from_server(&mut self, status: &RpcStatus, send_empty_metadata: bool, write_flags: u32) -> Result<CqFuture> {
        let send_empty_metadata = if send_empty_metadata { 1 } else { 0 };
        check_run(PromiseType::Finish, |tag| unsafe {
            grpc_sys::grpcwrap_call_send_status_from_server(self.call, tag, status.status, status.details.as_ptr() as _, status.details.len(), ptr::null_mut(), send_empty_metadata, ptr::null(), 0, write_flags)
        })
    }

    fn cancel(&self) {
        unsafe {
            grpc_sys::grpc_call_cancel(self.call, ptr::null_mut())
        }
    }
}

impl Drop for Call {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_call_destroy(self.call)
        }
    }
}

struct StreamingBase {
    resp_f: CqFuture,
    msg_f: Option<CqFuture>,
    stale: bool,
}

impl StreamingBase {
    fn new(resp_f: CqFuture) -> StreamingBase {
        StreamingBase {
            resp_f: resp_f,
            msg_f: None,
            stale: false,
        }
    }

    fn poll(&mut self, call: &mut Call, skip_finish_check: bool) -> Poll<Option<Result<Vec<u8>>>, Error> {
        if self.stale {
            return Err(Error::FutureStale);
        }
        let mut repoll_resp = self.msg_f.is_none();
        if let Some(ref msg_f) = self.msg_f {
            match msg_f.poll_raw_resp() {
                // maybe we can schedule next poll immediately?
                Ok(Async::Ready(Ok(bytes))) => {
                    if bytes.is_empty() {
                        self.stale = true;
                        return Ok(Async::Ready(None));
                    }
                    
                    return Ok(Async::Ready(Some(Ok(bytes))))
                }
                Ok(Async::Ready(Err(e))) => return Ok(Async::Ready(Some(Err(e)))),
                Err(Error::FutureStale) => repoll_resp = true,
                Ok(Async::NotReady) => {
                    if skip_finish_check {
                        return Ok(Async::NotReady);
                    }
                },
                e => panic!("unexpected result: {:?}", e),
            }
        }

        match self.resp_f.poll_raw_resp() {
            Ok(Async::Ready(Ok(_))) => {
                self.stale = true;
                return Ok(Async::Ready(None))
            },
            Ok(Async::Ready(Err(e))) | Err(e) => {
                self.stale = true;
                return Err(e);
            },
            Ok(Async::NotReady) => {},
        }

        if !repoll_resp {
            return Ok(Async::NotReady);
        }

        // so msg_f must be either stale or not initialised yet.
        self.msg_f.take();
        match call.start_recv_message() {
            Err(e) => Err(e),
            Ok(msg_f) => {
                self.msg_f = Some(msg_f);
                self.poll(call, true)
            },
        }
    }
}

struct SinkBase {
    write_f: Option<CqFuture>,
    close_f: Option<CqFuture>,
    buf: Vec<u8>,
    flags: u32,
    send_metadata: bool,
    closed: bool,
}

impl SinkBase {
    fn new(flags: u32, send_metadata: bool) -> SinkBase {
        SinkBase {
            write_f: None,
            close_f: None,
            buf: Vec::new(),
            send_metadata: send_metadata,
            flags: flags,
            closed: false,
        }
    }

    fn start_send<F, E>(&mut self, call: &mut Call, fill_buf: F) -> Result<bool>
         where F: FnOnce(&mut Vec<u8>) -> result::Result<(), E>,
               E: Into<Error> {
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
        let write_f = try!(call.start_send_message(&self.buf, self.flags, self.send_metadata));
        self.write_f = Some(write_f);
        self.send_metadata = false;
        Ok(true)
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        if let Some(ref write_f) = self.write_f {
            match write_f.poll_raw_resp() {
                Ok(Async::Ready(Ok(_))) => {}
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) | Ok(Async::Ready(Err(e))) => return Err(e),
            }
        }
        
        self.write_f.take();
        Ok(Async::Ready(()))
    }

    fn close(&mut self, call: &mut Call) -> Poll<(), Error> {
        if self.closed {
            return Err(Error::FutureStale);
        }

        if self.close_f.is_none() {
            if let Async::NotReady = try!(self.poll_complete()) {
                return Ok(Async::NotReady);
            }

            let close_f = try!(call.start_send_close_client());
            self.close_f = Some(close_f);
        }

        self.close_f.as_ref().unwrap().poll_raw_resp().map(|res| {
            res.map(|_| {
                self.closed = true;
            })
        })
    }
}
