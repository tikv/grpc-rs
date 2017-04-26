use std::collections::HashMap;
use std::sync::Arc;
use std::ptr;
use std::ffi::CString;

use protobuf::{Message, MessageStatic};
use grpc_sys::{self, GrpcServer, GrpcCallStatus};

use env::Environment;
use call::Method;
use call::server::*;
use channel::ChannelArgs;
use RpcContext;

const DEFAULT_REQUEST_SLOTS_PER_CQ: usize = 1024;

pub struct ServerBuilder {
    env: Arc<Environment>,
    addrs: Vec<(String, u32)>,
    args: Option<ChannelArgs>,
    slots_per_cq: usize,
    handlers: HashMap<&'static str, Box<Fn(RpcContext)>>,
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

    pub fn bind(mut self, host: String, port: u32) -> ServerBuilder {
        self.addrs.push((host, port));
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
        self.handlers.insert(method.name, Box::new(move |ctx| {
            execute_unary(ctx, &handler)
        }));
        self
    }

    pub fn add_client_streaming_handler<P, Q, F>(mut self, method: &Method, handler: F) -> ServerBuilder
        where P: MessageStatic,
          Q: Message,
          F: Fn(RpcContext, RequestStream<P>, ClientStreamingResponseSink<Q>) + 'static {
        self.handlers.insert(method.name, Box::new(move |ctx| {
            execute_client_streaming(ctx, &handler)
        }));
        self
    }

    pub fn add_server_streaming_handler<P, Q, F>(mut self, method: &Method, handler: F) -> ServerBuilder 
        where P: MessageStatic,
            Q: Message,
            F: Fn(RpcContext, UnaryRequest<P>, ResponseSink<Q>) + 'static {
        self.handlers.insert(method.name, Box::new(move |ctx| {
            execute_server_streaming(ctx, &handler)
        }));
        self
    }

    pub fn add_duplex_streaming_handler<P, Q, F>(mut self, method: &Method, handler: F) -> ServerBuilder 
        where P: MessageStatic,
            Q: Message,
            F: Fn(RpcContext, RequestStream<P>, ResponseSink<Q>) + 'static {
        self.handlers.insert(method.name, Box::new(move |ctx| {
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
                (host, port)
            }).collect();

            for cq in self.env.completion_queues() {
                grpc_sys::grpc_server_register_completion_queue(server, cq.as_ptr(), ptr::null_mut());
            }

            Server {
                env: self.env,
                server: server,
                bind_addrs: bind_addrs,
                slots_per_cq: self.slots_per_cq,
                handlers: self.handlers,
            }
        }
    }
}

pub struct Server {
    env: Arc<Environment>,
    server: *mut GrpcServer,
    bind_addrs: Vec<(String, u32)>,
    slots_per_cq: usize,
    handlers: HashMap<&'static str, Box<Fn(RpcContext)>>,
}

impl Server {
    fn shutdown(&mut self) {
        unsafe {
            let cq_ptr = self.env.completion_queues()[0].as_ptr();
            // TODO: async
            grpc_sys::grpc_server_shutdown_and_notify(self.server, cq_ptr, ptr::null_mut())
        }
    }

    fn cancel_all_calls(&mut self) {
        unsafe {
            grpc_sys::grpc_server_cancel_all_calls(self.server)
        }
    }

    fn start(&mut self) {
        unsafe {
            grpc_sys::grpc_server_start(self.server);
            for cq in self.env.completion_queues() {
                for _ in 0..self.slots_per_cq {
                    let request_ctx = RequestContext::new();
                    let ptr = request_ctx.into_raw();
                    let code = grpc_sys::grpcwrap_server_request_call(self.server, cq.as_ptr(), ptr);
                    if code != GrpcCallStatus::Ok {
                        RequestContext::from_raw(ptr);
                        panic!("failed to start server: {:?}", code);
                    }
                }
            }
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe {
            grpc_sys::grpc_server_destroy(self.server)
        }
    }
}
