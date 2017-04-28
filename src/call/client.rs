
use async::{BatchType, CqFuture};
use call::{Call, Method, check_run};

use channel::Channel;
use error::{Error, Result};

use futures::{Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use grpc_sys;
use protobuf::{self, Message, MessageStatic};
use std::marker::PhantomData;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use super::{SinkBase, StreamingBase};

fn change_flag(res: &mut u32, flag: u32, set: bool) {
    if set {
        *res = flag;
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
    pub fn with_idempotent(mut self, is_idempotent: bool) -> CallOption {
        change_flag(&mut self.call_flags,
                    grpc_sys::GRPC_INITIAL_METADATA_IDEMPOTENT_REQUEST,
                    is_idempotent);
        self
    }

    pub fn with_wait_for_ready(mut self, wait_for_ready: bool) -> CallOption {
        change_flag(&mut self.call_flags,
                    grpc_sys::GRPC_INITIAL_METADATA_WAIT_FOR_READY,
                    wait_for_ready);
        self
    }

    pub fn with_cacheable(mut self, cacheable: bool) -> CallOption {
        change_flag(&mut self.call_flags,
                    grpc_sys::GRPC_INITIAL_METADATA_CACHEABLE_REQUEST,
                    cacheable);
        self
    }

    pub fn with_buffer_hint(mut self, need_buffered: bool) -> CallOption {
        change_flag(&mut self.write_flags,
                    grpc_sys::GRPC_WRITE_BUFFER_HINT,
                    need_buffered);
        self
    }

    pub fn with_force_no_compress(mut self, no_compress: bool) -> CallOption {
        change_flag(&mut self.write_flags,
                    grpc_sys::GRPC_WRITE_NO_COMPRESS,
                    no_compress);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> CallOption {
        self.timeout = Some(timeout);
        self
    }

    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

impl Call {
    pub fn unary_async<P: Message, Q>(channel: &Channel,
                                      method: &Method,
                                      req: P,
                                      opt: CallOption)
                                      -> Result<UnaryCallHandler<Q>> {
        let call = channel.create_call(method, &opt);
        let payload = try!(req.write_to_bytes());
        let cq_f = try!(check_run(BatchType::FinishUnary, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_unary(call.call,
                                                ctx,
                                                payload.as_ptr() as *const _,
                                                payload.len(),
                                                opt.write_flags,
                                                ptr::null_mut(),
                                                opt.call_flags,
                                                tag)
        }));
        Ok(UnaryCallHandler::new(call, cq_f))
    }

    pub fn client_streaming<P, Q>(channel: &Channel,
                                  method: &Method,
                                  opt: CallOption)
                                  -> Result<ClientStreamingCallHandler<P, Q>> {
        let call = channel.create_call(method, &opt);
        let cq_f = try!(check_run(BatchType::FinishUnary, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_client_streaming(call.call,
                                                           ctx,
                                                           ptr::null_mut(),
                                                           opt.call_flags,
                                                           tag)
        }));
        Ok(ClientStreamingCallHandler::new(call, cq_f, opt.write_flags))
    }

    pub fn server_streaming<P: Message, Q>(channel: &Channel,
                                           method: &Method,
                                           req: P,
                                           opt: CallOption)
                                           -> Result<ServerStreamingCallHandler<Q>> {
        let call = channel.create_call(method, &opt);
        let payload = try!(req.write_to_bytes());
        let cq_f = try!(check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_server_streaming(call.call,
                                                           ctx,
                                                           payload.as_ptr() as _,
                                                           payload.len(),
                                                           opt.write_flags,
                                                           ptr::null_mut(),
                                                           opt.call_flags,
                                                           tag)
        }));

        // ignore header for now
        check_run(BatchType::Finish, |ctx, tag| unsafe {
                grpc_sys::grpcwrap_call_recv_initial_metadata(call.call, ctx, tag)
            })
            .unwrap_or_else(|e| {
                panic!("failed to start receiving headers: {:?}", e);
            });

        Ok(ServerStreamingCallHandler::new(call, cq_f))
    }

    pub fn duplex_streaming<P, Q>(channel: &Channel,
                                  method: &Method,
                                  opt: CallOption)
                                  -> Result<DuplexStreamingCallHandler<P, Q>> {
        let call = channel.create_call(method, &opt);
        let cq_f = try!(check_run(BatchType::Finish, |ctx, tag| unsafe {
            grpc_sys::grpcwrap_call_start_duplex_streaming(call.call,
                                                           ctx,
                                                           ptr::null_mut(),
                                                           opt.call_flags,
                                                           tag)
        }));

        // ignore header for now.
        check_run(BatchType::Finish, |ctx, tag| unsafe {
                grpc_sys::grpcwrap_call_recv_initial_metadata(call.call, ctx, tag)
            })
            .unwrap_or_else(|e| {
                panic!("failed to start receiving headers: {:?}", e);
            });

        Ok(DuplexStreamingCallHandler::new(call, cq_f, opt.write_flags))
    }
}

pub struct UnaryCallHandler<T> {
    call: Call,
    resp_f: CqFuture<Vec<u8>>,
    _resp: PhantomData<T>,
}

impl<T> UnaryCallHandler<T> {
    fn new(call: Call, resp_f: CqFuture<Vec<u8>>) -> UnaryCallHandler<T> {
        UnaryCallHandler {
            call: call,
            resp_f: resp_f,
            _resp: PhantomData,
        }
    }

    pub fn cancel(&self) {
        self.call.cancel()
    }
}

impl<T: MessageStatic> Future for UnaryCallHandler<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let data = try_ready!(self.resp_f.poll());
        let t = try!(protobuf::parse_from_bytes(&data));
        Ok(Async::Ready(t))
    }
}

pub struct UnaryResponseReceiver<T> {
    _call: Call,
    resp_f: CqFuture<Vec<u8>>,
    _resp: PhantomData<T>,
}

impl<T: MessageStatic> Future for UnaryResponseReceiver<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<T, Error> {
        let data = try_ready!(self.resp_f.poll());
        let t = try!(protobuf::parse_from_bytes(&data));
        Ok(Async::Ready(t))
    }
}

pub struct ClientStreamingCallHandler<P, Q> {
    call: Call,
    resp_f: CqFuture<Vec<u8>>,
    sink_base: SinkBase,
    _req: PhantomData<P>,
    _resp: PhantomData<Q>,
}

impl<P, Q> ClientStreamingCallHandler<P, Q> {
    fn new(call: Call, resp_f: CqFuture<Vec<u8>>, flags: u32) -> ClientStreamingCallHandler<P, Q> {
        ClientStreamingCallHandler {
            call: call,
            resp_f: resp_f,
            sink_base: SinkBase::new(flags, false),
            _req: PhantomData,
            _resp: PhantomData,
        }
    }
}

impl<P: Message, Q> Sink for ClientStreamingCallHandler<P, Q> {
    type SinkItem = P;
    type SinkError = Error;

    fn start_send(&mut self, item: P) -> StartSend<P, Error> {
        self.sink_base.start_send(&mut self.call, |buf| item.write_to_vec(buf)).map(|s| {
            if s {
                AsyncSink::Ready
            } else {
                AsyncSink::NotReady(item)
            }
        })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        self.sink_base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        self.sink_base.close(&mut self.call)
    }
}

impl<P, Q: MessageStatic> ClientStreamingCallHandler<P, Q> {
    pub fn into_receiver(self) -> UnaryResponseReceiver<Q> {
        UnaryResponseReceiver {
            _call: self.call,
            resp_f: self.resp_f,
            _resp: PhantomData,
        }
    }
}

pub struct ServerStreamingCallHandler<Q> {
    call: Call,
    base: StreamingBase,
    _resp: PhantomData<Q>,
}

impl<Q> ServerStreamingCallHandler<Q> {
    fn new(call: Call, finish_f: CqFuture<Vec<u8>>) -> ServerStreamingCallHandler<Q> {
        ServerStreamingCallHandler {
            call: call,
            base: StreamingBase::new(Some(finish_f)),
            _resp: PhantomData,
        }
    }
}

impl<Q: MessageStatic> Stream for ServerStreamingCallHandler<Q> {
    type Item = Q;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Q>, Error> {
        match try_ready!(self.base.poll(&mut self.call, false)) {
            None => Ok(Async::Ready(None)),
            Some(data) => {
                let msg = try!(protobuf::parse_from_bytes(&data));
                Ok(Async::Ready(Some(msg)))
            }
        }
    }
}

pub struct DuplexStreamingCallHandler<P, Q> {
    // start_batch needs to be synchronized;
    call: Arc<Mutex<Call>>,
    resp_f: Option<CqFuture<Vec<u8>>>,
    sink_base: SinkBase,
    _req: PhantomData<P>,
    _resp: PhantomData<Q>,
}

impl<P, Q> DuplexStreamingCallHandler<P, Q> {
    fn new(call: Call,
           resp_f: CqFuture<Vec<u8>>,
           write_flags: u32)
           -> DuplexStreamingCallHandler<P, Q> {
        DuplexStreamingCallHandler {
            call: Arc::new(Mutex::new(call)),
            resp_f: Some(resp_f),
            sink_base: SinkBase::new(write_flags, false),
            _req: PhantomData,
            _resp: PhantomData,
        }
    }
}

impl<P: Message, Q> Sink for DuplexStreamingCallHandler<P, Q> {
    type SinkItem = P;
    type SinkError = Error;

    fn start_send(&mut self, item: P) -> StartSend<P, Error> {
        let mut call = self.call.lock().unwrap();
        self.sink_base.start_send(&mut call, |buf| item.write_to_vec(buf)).map(|s| {
            if s {
                AsyncSink::Ready
            } else {
                AsyncSink::NotReady(item)
            }
        })
    }

    fn poll_complete(&mut self) -> Poll<(), Error> {
        self.sink_base.poll_complete()
    }

    fn close(&mut self) -> Poll<(), Error> {
        let mut call = self.call.lock().unwrap();
        self.sink_base.close(&mut call)
    }
}

pub struct StreamingResponseReceiver<Q> {
    call: Arc<Mutex<Call>>,
    base: StreamingBase,
    _resp: PhantomData<Q>,
}

impl<P, Q: MessageStatic> DuplexStreamingCallHandler<P, Q> {
    pub fn take_receiver(&mut self) -> Option<StreamingResponseReceiver<Q>> {
        let resp_f = match self.resp_f.take() {
            Some(resp_f) => resp_f,
            None => return None,
        };

        Some(StreamingResponseReceiver {
            call: self.call.clone(),
            base: StreamingBase::new(Some(resp_f)),
            _resp: PhantomData,
        })
    }
}

impl<Q: MessageStatic> Stream for StreamingResponseReceiver<Q> {
    type Item = Q;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Q>, Error> {
        let mut call = self.call.lock().unwrap();
        match try_ready!(self.base.poll(&mut call, false)) {
            None => Ok(Async::Ready(None)),
            Some(data) => {
                let msg = try!(protobuf::parse_from_bytes(&data));
                Ok(Async::Ready(Some(msg)))
            }
        }
    }
}
