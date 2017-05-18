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

use async::{BatchMessage, BatchType, CqFuture};
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
                                  -> ClientStreamingCallHandler<P, Q> {
        let call = channel.create_call(method, &opt);
        let cq_f = check_run(BatchType::CheckRead, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_client_streaming(call.call,
                                                           ctx,
                                                           ptr::null_mut(),
                                                           opt.call_flags,
                                                           tag)
        });
        ClientStreamingCallHandler::new(call,
                                        cq_f,
                                        opt.write_flags,
                                        method.req_ser(),
                                        method.resp_de())
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

/// A unary response receiver. It's used for client streaming request.
pub struct UnaryResponseReceiver<T> {
    _call: Call,
    resp_f: CqFuture<BatchMessage>,
    resp: Option<BatchMessage>,
    resp_de: DeserializeFn<T>,
}

impl<T> Future for UnaryResponseReceiver<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let data = match self.resp.take() {
            Some(msg) => msg,
            None => try_ready!(self.resp_f.poll()),
        };
        let t = try!((self.resp_de)(&data.unwrap()));
        Ok(Async::Ready(t))
    }
}

/// A handler for client streaming call.
///
/// Once all requests are flushed, it can be converted to `UnaryResponseReceiver`
/// to receive response asynchronously.
pub struct ClientStreamingCallHandler<P, Q> {
    call: Call,
    sink_base: SinkBase,
    req_ser: SerializeFn<P>,
    resp_de: DeserializeFn<Q>,
}

impl<P, Q> ClientStreamingCallHandler<P, Q> {
    fn new(call: Call,
           resp_f: CqFuture<BatchMessage>,
           flags: u32,
           ser: SerializeFn<P>,
           de: DeserializeFn<Q>)
           -> ClientStreamingCallHandler<P, Q> {
        ClientStreamingCallHandler {
            call: call,
            sink_base: SinkBase::new(flags, Some(resp_f), false),
            req_ser: ser,
            resp_de: de,
        }
    }
}

impl<P, Q> Sink for ClientStreamingCallHandler<P, Q> {
    type SinkItem = P;
    type SinkError = Error;

    fn start_send(&mut self, item: P) -> StartSend<P, Error> {
        self.sink_base
            .start_send(&mut self.call, &item, self.req_ser)
            .map(|s| if s {
                     AsyncSink::Ready
                 } else {
                     AsyncSink::NotReady(item)
                 })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        self.sink_base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        self.sink_base.close(&mut self.call)
    }
}

impl<P, Q> ClientStreamingCallHandler<P, Q> {
    pub fn cancel(self) -> UnaryResponseReceiver<Q> {
        self.call.cancel();
        self.into_receiver()
    }

    pub fn into_receiver(self) -> UnaryResponseReceiver<Q> {
        UnaryResponseReceiver {
            _call: self.call,
            resp_f: self.sink_base.abort_f.unwrap(),
            resp: self.sink_base.res,
            resp_de: self.resp_de,
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

/// A handler for duplex streaming call.
///
/// A receiver can be taken at any time. Request and response can be handled
/// asynchronously.
///
/// Please note that, if the call finished early and the handler won't receive the
/// notification.
pub struct DuplexCallHandler<P, Q> {
    // start_batch needs to be synchronized;
    call: Arc<Mutex<Call>>,
    resp_f: Option<CqFuture<BatchMessage>>,
    sink_base: SinkBase,
    req_ser: SerializeFn<P>,
    resp_de: DeserializeFn<Q>,
}

impl<P, Q> DuplexCallHandler<P, Q> {
    fn new(call: Call,
           resp_f: CqFuture<BatchMessage>,
           write_flags: u32,
           ser: SerializeFn<P>,
           de: DeserializeFn<Q>)
           -> DuplexCallHandler<P, Q> {
        DuplexCallHandler {
            call: Arc::new(Mutex::new(call)),
            resp_f: Some(resp_f),
            sink_base: SinkBase::new(write_flags, None, false),
            req_ser: ser,
            resp_de: de,
        }
    }
}

impl<P, Q> Sink for DuplexCallHandler<P, Q> {
    type SinkItem = P;
    type SinkError = Error;

    fn start_send(&mut self, item: P) -> StartSend<P, Error> {
        self.sink_base
            .start_send(&mut self.call, &item, self.req_ser)
            .map(|s| if s {
                     AsyncSink::Ready
                 } else {
                     AsyncSink::NotReady(item)
                 })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        self.sink_base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        self.sink_base.close(&mut self.call)
    }
}

/// A response receiver for duplex call.
pub struct StreamingResponseReceiver<Q> {
    call: Arc<Mutex<Call>>,
    base: StreamingBase,
    resp_de: DeserializeFn<Q>,
}

impl<P, Q> DuplexCallHandler<P, Q> {
    pub fn take_receiver(&mut self) -> Option<StreamingResponseReceiver<Q>> {
        let resp_f = match self.resp_f.take() {
            Some(resp_f) => resp_f,
            None => return None,
        };

        Some(StreamingResponseReceiver {
                 call: self.call.clone(),
                 base: StreamingBase::new(Some(resp_f)),
                 resp_de: self.resp_de,
             })
    }

    pub fn cancel(mut self) -> Option<StreamingResponseReceiver<Q>> {
        {
            let call = self.call.lock().unwrap();
            call.cancel();
        }
        self.take_receiver()
    }
}

impl<Q> Stream for StreamingResponseReceiver<Q> {
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
