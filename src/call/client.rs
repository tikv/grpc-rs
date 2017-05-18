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


use std::ptr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use grpc_sys;

use async::{self, BatchMessage, BatchType, CqFuture};
use async::lock::SpinLock;
use call::{Call, Method, check_run};
use channel::Channel;
use codec::{DeserializeFn, SerializeFn};
use error::Error;
use super::{SinkBase, StreamingBase};

/// Update the flag bit in res.
#[inline]
fn change_flag(res: &mut u32, flag: u32, set: bool) {
    if set {
        *res |= flag;
    } else {
        *res &= !flag;
    }
}

#[derive(Default)]
pub struct CallOption {
    timeout: Option<Duration>,
    write_flags: u32,
    call_flags: u32,
}

impl CallOption {
    /// Signal that the call is idempotent
    pub fn idempotent(mut self, is_idempotent: bool) -> CallOption {
        change_flag(&mut self.call_flags,
                    grpc_sys::GRPC_INITIAL_METADATA_IDEMPOTENT_REQUEST,
                    is_idempotent);
        self
    }

    /// Signal that the call should not return UNAVAILABLE before it has started
    pub fn wait_for_ready(mut self, wait_for_ready: bool) -> CallOption {
        change_flag(&mut self.call_flags,
                    grpc_sys::GRPC_INITIAL_METADATA_WAIT_FOR_READY,
                    wait_for_ready);
        self
    }

    /// Signal that the call is cacheable. GRPC is free to use GET verb
    pub fn cacheable(mut self, cacheable: bool) -> CallOption {
        change_flag(&mut self.call_flags,
                    grpc_sys::GRPC_INITIAL_METADATA_CACHEABLE_REQUEST,
                    cacheable);
        self
    }

    /// Hint that the write may be buffered and need not go out on the wire immediately.
    pub fn buffer_hint(mut self, need_buffered: bool) -> CallOption {
        change_flag(&mut self.write_flags,
                    grpc_sys::GRPC_WRITE_BUFFER_HINT,
                    need_buffered);
        self
    }

    /// Force compression to be disabled.
    pub fn force_no_compress(mut self, no_compress: bool) -> CallOption {
        change_flag(&mut self.write_flags,
                    grpc_sys::GRPC_WRITE_NO_COMPRESS,
                    no_compress);
        self
    }

    /// Set a timeout.
    pub fn timeout(mut self, timeout: Duration) -> CallOption {
        self.timeout = Some(timeout);
        self
    }

    /// Get the timeout.
    pub fn get_timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

impl Call {
    pub fn unary_async<P, Q>(channel: &Channel,
                             method: &Method<P, Q>,
                             req: P,
                             opt: CallOption)
                             -> UnaryCallHandler<Q> {
        let call = channel.create_call(method, &opt);
        let mut payload = vec![];
        (method.req_ser())(&req, &mut payload);
        let cq_f = check_run(BatchType::CheckRead, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_unary(call.call,
                                                ctx,
                                                payload.as_ptr() as *const _,
                                                payload.len(),
                                                opt.write_flags,
                                                ptr::null_mut(),
                                                opt.call_flags,
                                                tag)
        });
        UnaryCallHandler::new(call, cq_f, method.resp_de())
    }

    pub fn client_streaming<P, Q>(channel: &Channel,
                                  method: &Method<P, Q>,
                                  opt: CallOption)
                                  -> (ClientStreamingSink<P>, ClientStreamingReceiver<Q>) {
        let call = channel.create_call(method, &opt);
        let cq_f = check_run(BatchType::CheckRead, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_client_streaming(call.call,
                                                           ctx,
                                                           ptr::null_mut(),
                                                           opt.call_flags,
                                                           tag)
        });

        let share_call = Arc::new(SpinLock::new(ShareCall::new(call, cq_f)));
        let sink = ClientStreamingSink::new(share_call.clone(), opt.write_flags, method.req_ser());
        let recv = ClientStreamingReceiver {
            call: share_call,
            resp_de: method.resp_de(),
        };
        (sink, recv)
    }

    pub fn server_streaming<P, Q>(channel: &Channel,
                                  method: &Method<P, Q>,
                                  req: P,
                                  opt: CallOption)
                                  -> ServerStreamingCallHandler<Q> {
        let call = channel.create_call(method, &opt);
        let mut payload = vec![];
        (method.req_ser())(&req, &mut payload);
        let cq_f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_server_streaming(call.call,
                                                           ctx,
                                                           payload.as_ptr() as _,
                                                           payload.len(),
                                                           opt.write_flags,
                                                           ptr::null_mut(),
                                                           opt.call_flags,
                                                           tag)
        });

        // TODO: handle header
        check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_recv_initial_metadata(call.call, ctx, tag)
        });

        ServerStreamingCallHandler::new(call, cq_f, method.resp_de())
    }

    pub fn duplex_streaming<P, Q>(channel: &Channel,
                                  method: &Method<P, Q>,
                                  opt: CallOption)
                                  -> DuplexCallHandler<P, Q> {
        let call = channel.create_call(method, &opt);
        let cq_f = check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_duplex_streaming(call.call,
                                                           ctx,
                                                           ptr::null_mut(),
                                                           opt.call_flags,
                                                           tag)
        });

        // TODO: handle header.
        check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_recv_initial_metadata(call.call, ctx, tag)
        });

        DuplexCallHandler::new(call,
                               cq_f,
                               opt.write_flags,
                               method.req_ser(),
                               method.resp_de())
    }
}

/// A handler to handle a uanry async call.
///
/// The future is resolved once response is received.
pub struct UnaryCallHandler<T> {
    call: Call,
    resp_f: CqFuture<BatchMessage>,
    resp_de: DeserializeFn<T>,
}

impl<T> UnaryCallHandler<T> {
    fn new(call: Call,
           resp_f: CqFuture<BatchMessage>,
           de: DeserializeFn<T>)
           -> UnaryCallHandler<T> {
        UnaryCallHandler {
            call: call,
            resp_f: resp_f,
            resp_de: de,
        }
    }

    /// Cancel the call.
    pub fn cancel(&self) {
        self.call.cancel()
    }
}

impl<T> Future for UnaryCallHandler<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let data = try_ready!(self.resp_f.poll());
        let t = try!((self.resp_de)(&data.unwrap()));
        Ok(Async::Ready(t))
    }
}

/// A share object for client streaming and duplex streaming call.
///
/// In both cases, receiver and sink can be polled in the same time,
/// hence we need to share the call in the both sides and abort the sink
/// once the call is canceled or finished early.
struct ShareCall {
    call: Call,
    close_f: CqFuture<BatchMessage>,
    finished: bool,
    status: Option<RpcStatus>,
}

impl ShareCall {
    fn new(call: Call, close_f: CqFuture<BatchMessage>) -> ShareCall {
        ShareCall {
            call: call,
            close_f: close_f,
            finished: false,
            status: None,
        }
    }

    /// Poll if the call is still alive.
    ///
    /// If the call is still running, will register a notification for its completion.
    fn poll_finish(&mut self) -> Poll<BatchMessage, Error> {
        let res = match self.close_f.poll() {
            Err(Error::RpcFailure(status)) => {
                call.status = Some(status.clone());
                Err(Error::RpcFailure(status))
            }
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(msg)) => {
                call.status = Some(RpcStatus::ok());
                Ok(Async::Ready(msg))
            }
            res => res,
        };

        call.finished = true;
        res
    }

    /// Check if the call is finished.
    fn check_alive(&mut self) -> Result<()> {
        if self.finished {
            // maybe can just take here.
            return Err(Error::RpcFinished(self.status.clone()));
        }

        async::check_alive(&self.close_f)
    }
}

/// Client streaming call response resolver.
pub struct ClientStreamingReceiver<T> {
    call: Arc<SpinLock<ShareCall>>,
    resp_de: DeserializeFn<T>,
}

impl<T> Future for ClientStreamingReceiver<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let data = {
            let call = self.call.lock();
            try_ready!(call.poll_finish())
        };
        let t = try!((self.resp_de)(&data.unary()));
        Ok(Async::Ready(t))
    }
}

/// A sink for client streaming call and duplex streaming call.
pub struct StreamingCallSink<P> {
    call: Arc<SpinLock<ShareCall>>,
    sink_base: SinkBase,
    req_ser: SerializeFn<P>,
}

impl<P> StreamingCallSink<P> {
    fn new(call: Arc<SpinLock<ShareCall>>,
           flags: u32,
           ser: SerializeFn<P>)
           -> StreamingCallSink<P> {
        StreamingCallSink {
            call: call,
            sink_base: SinkBase::new(flags, false),
            req_ser: ser,
        }
    }

    pub fn cancel(self) {
        let call = self.call.lock();
        call.call.cancel();
    }
}

impl<P> Sink for StreamingCallSink<P> {
    type SinkItem = P;
    type SinkError = Error;

    fn start_send(&mut self, item: P) -> StartSend<P, Error> {
        let call = self.call.lock();
        try!(call.check_alive());
        self.sink_base
            .start_send(&mut call.call, &item, self.req_ser)
            .map(|s| if s {
                     AsyncSink::Ready
                 } else {
                     AsyncSink::NotReady(item)
                 })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        {
            let call = self.call.lock();
            try!(call.check_alive());
        }
        self.sink_base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        match self.sink_base.close(&mut self.call) {
            Ok(Async::NotReady) => {
                let call = self.call.lock();
                try!(call.check_alive());
                Ok(Async::NotReady)
            },
            res => res
        }
    }
}

/// A handler for server streaming call.
pub struct ServerStreamingCallHandler<Q> {
    call: Call,
    base: StreamingBase,
    resp_de: DeserializeFn<Q>,
}

impl<Q> ServerStreamingCallHandler<Q> {
    fn new(call: Call,
           finish_f: CqFuture<BatchMessage>,
           de: DeserializeFn<Q>)
           -> ServerStreamingCallHandler<Q> {
        ServerStreamingCallHandler {
            call: call,
            base: StreamingBase::new(Some(finish_f)),
            resp_de: de,
        }
    }

    pub fn cancel(&self) {
        self.call.cancel()
    }
}

impl<Q> Stream for ServerStreamingCallHandler<Q> {
    type Item = Q;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Q>, Error> {
        match try_ready!(self.base.poll(&mut self.call, false)) {
            None => Ok(Async::Ready(None)),
            Some(data) => {
                let msg = try!((self.resp_de)(&data));
                Ok(Async::Ready(Some(msg)))
            }
        }
    }
}

/// A response receiver for duplex call.
pub struct StreamingResponseReceiver<Q> {
    call: Arc<SpinLock<ShareCall>>,
    base: StreamingBase,
    resp_de: DeserializeFn<Q>,
}

impl<Q> Stream for StreamingResponseReceiver<Q> {
    type Item = Q;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Q>, Error> {
        {
            let call = self.call.lock();
            match call.poll_finish() {
                Ok(Async::NotReady) => {}
                Ok(Async::Ready(_)) => 
            }
        }
        match try_ready!(self.base.poll(&mut self.call, false)) {
            None => Ok(Async::Ready(None)),
            Some(data) => {
                let msg = try!((self.resp_de)(&data));
                Ok(Async::Ready(Some(msg)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_change_flag() {
        let mut flag = 2 | 4;
        super::change_flag(&mut flag, 8, true);
        assert_eq!(flag, 2 | 4 | 8);
        super::change_flag(&mut flag, 4, false);
        assert_eq!(flag, 2 | 8);
    }
}
