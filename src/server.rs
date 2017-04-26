use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ptr;

use protobuf::{Message, MessageStatic};
use futures::{Future, Poll, Async};
use grpc_sys::{self, GrpcServer, GrpcCallStatus};

use env::Environment;
use error::Error;
use cq::CompletionQueue;
use call::Method;
use call::server::*;
use channel::ChannelArgs;
use promise::{self, CqFuture};
use RpcContext;

const DEFAULT_REQUEST_SLOTS_PER_CQ: usize = 1024;

pub struct ServerBuilder {
    env: Arc<Environment>,
    addrs: Vec<(String, u32)>,
    args: Option<ChannelArgs>,
    slots_per_cq: usize,
    handlers: HashMap<&'static [u8], Box<Fn(RpcContext)>>,
}

impl ServerBuilder {
    pub fn new(env: Arc<Environment>) -> ServerBuilder {
        ServerBuilder {
            env: env,
            addrs: Vec::new(),
            args: None,
            slots_per_cq: DEFAULT_REQUEST_SLOTS_PER_CQ,
            handlers: HashMap::new(),
        }
    }

    pub fn bind<S: Into<String>>(mut self, host: S, port: u32) -> ServerBuilder {
        self.addrs.push((host.into(), port));
        self
    }

    pub fn channel_args(mut self, args: ChannelArgs) -> ServerBuilder {
        self.args = Some(args);
        self
    }

    pub fn add_unary_handler<P, Q, F>(mut self, method: &Method, handler: F) -> ServerBuilder
        where P: MessageStatic,
            Q: Message,
            F: Fn(RpcContext, UnaryRequest<P>, UnaryResponseSink<Q>) + 'static {
        self.handlers.insert(method.name.as_bytes(), Box::new(move |ctx| {
            execute_unary(ctx, &handler)
        }));
        self
    }

    pub fn add_client_streaming_handler<P, Q, F>(mut self, method: &Method, handler: F) -> ServerBuilder
        where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, RequestStream<P>, ClientStreamingResponseSink<Q>) + 'static {
        self.handlers.insert(method.name.as_bytes(), Box::new(move |ctx| {
            execute_client_streaming(ctx, &handler)
        }));
        self
    }

    pub fn add_server_streaming_handler<P, Q, F>(mut self, method: &Method, handler: F) -> ServerBuilder 
        where P: MessageStatic,
            Q: Message,
            F: Fn(RpcContext, UnaryRequest<P>, ResponseSink<Q>) + 'static {
        self.handlers.insert(method.name.as_bytes(), Box::new(move |ctx| {
            execute_server_streaming(ctx, &handler)
        }));
        self
    }

    pub fn add_duplex_streaming_handler<P, Q, F>(mut self, method: &Method, handler: F) -> ServerBuilder 
        where P: MessageStatic,
            Q: Message,
            F: Fn(RpcContext, RequestStream<P>, ResponseSink<Q>) + 'static {
        self.handlers.insert(method.name.as_bytes(), Box::new(move |ctx| {
            execute_duplex_streaming(ctx, &handler)
        }));
        self
    }

    pub fn requests_slot_per_cq(mut self, slots: usize) -> ServerBuilder {
        self.slots_per_cq = slots;
        self
    }

    pub fn build(mut self) -> Server {
        let args = self.args.map_or_else(ptr::null, |args| args.as_ptr());
        unsafe {
            let server = grpc_sys::grpc_server_create(args, ptr::null_mut());
            let bind_addrs: Vec<_> = self.addrs.drain(..).map(|(host, port)| {
                let addr = format!("{}:{}\0", host, port);
                let bind_port = grpc_sys::grpc_server_add_insecure_http2_port(server, addr.as_ptr() as _);
                (host, bind_port as u32)
            }).collect();

            for cq in self.env.completion_queues() {
                grpc_sys::grpc_server_register_completion_queue(server, cq.as_ptr(), ptr::null_mut());
            }

            Server {
                inner: Arc::new(Inner {
                    env: self.env,
                    server: server,
                    shutdown: AtomicBool::new(false),
                    bind_addrs: bind_addrs,
                    slots_per_cq: self.slots_per_cq,
                    handlers: self.handlers,
                })
            }
        }
    }
}

pub struct Inner {
    env: Arc<Environment>,
    server: *mut GrpcServer,
    bind_addrs: Vec<(String, u32)>,
    slots_per_cq: usize,
    shutdown: AtomicBool,
    handlers: HashMap<&'static [u8], Box<Fn(RpcContext)>>,
}

impl Inner {
    pub fn handle(&self, ctx: RequestContext) {
        let handler = {
            match self.handlers.get(ctx.method()) {
                Some(handler) => handler,
                None => {
                    // TODO handle unimplement method.
                    return;
                }
            }
        };
        execute(ctx, handler)
    }
}

pub fn request_call(inner: Arc<Inner>, cq: &CompletionQueue) {
    if inner.shutdown.load(Ordering::Relaxed) {
        return;
    }
    let server_ptr = inner.server;
    let (_, prom) = promise::request_pair(inner);
    let request_ptr = prom.request_ctx().unwrap().as_ptr();
    let prom_box = Box::new(prom);
    let tag = Box::into_raw(prom_box);
    let code = unsafe {
        grpc_sys::grpcwrap_server_request_call(server_ptr, cq.as_ptr(), request_ptr, tag as *mut _)
    };
    if code != GrpcCallStatus::Ok {
        Box::from(tag);
        panic!("failed to request call: {:?}", code);
    }
}

pub struct ShutdownFuture {
    cq_f: CqFuture,
}

impl Future for ShutdownFuture {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        try_ready!(self.cq_f.poll_raw_resp());
        Ok(Async::Ready(()))
    }
}

pub struct Server {
    inner: Arc<Inner>,
}

impl Server {
    pub fn shutdown(&mut self) -> ShutdownFuture {
        let (cq_f, prom) = promise::shutdown_pair();
        let prom_box = Box::new(prom);
        let tag = Box::into_raw(prom_box);
        unsafe {
            let cq_ptr = self.inner.env.completion_queues()[0].as_ptr();
            // TODO: async
            grpc_sys::grpc_server_shutdown_and_notify(self.inner.server, cq_ptr, tag as *mut _)
        }
        self.inner.shutdown.store(true, Ordering::SeqCst);
        ShutdownFuture { cq_f: cq_f }
    }

    pub fn cancel_all_calls(&mut self) {
        unsafe {
            grpc_sys::grpc_server_cancel_all_calls(self.inner.server)
        }
    }

    pub fn start(&mut self) {
        unsafe {
            grpc_sys::grpc_server_start(self.inner.server);
            for cq in self.inner.env.completion_queues() {
                for _ in 0..self.inner.slots_per_cq {
                    request_call(self.inner.clone(), &cq);
                }
            }
        }
    }

    pub fn bind_addrs(&self) -> &[(String, u32)] {
        &self.inner.bind_addrs
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.shutdown();
        unsafe {
            grpc_sys::grpc_server_destroy(self.inner.server)
        }
    }
}
