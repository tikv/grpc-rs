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

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use futures::{Async, Future, Poll};
use grpc_sys::{self, GrpcCallStatus, GrpcServer};

use async::{CallTag, CqFuture};
use call::server::*;
use call::{Method, MethodType};
use channel::ChannelArgs;
use cq::CompletionQueue;
use env::Environment;
use error::{Error, Result};
use RpcContext;

const DEFAULT_REQUEST_SLOTS_PER_CQ: usize = 1024;

/// An RPC call holder.
#[derive(Clone)]
pub struct Handler<F> {
    method_type: MethodType,
    cb: F,
}

impl<F> Handler<F> {
    pub fn new(method_type: MethodType, cb: F) -> Handler<F> {
        Handler { method_type, cb }
    }
}

pub trait CloneableHandler: Send {
    fn handle(&mut self, ctx: RpcContext, reqs: &[u8]);
    fn box_clone(&self) -> Box<CloneableHandler>;
    fn method_type(&self) -> MethodType;
}

impl<F: 'static> CloneableHandler for Handler<F>
where
    F: FnMut(RpcContext, &[u8]) + Send + Clone + 'static,
{
    #[inline]
    fn handle(&mut self, ctx: RpcContext, reqs: &[u8]) {
        (self.cb)(ctx, reqs)
    }

    #[inline]
    fn box_clone(&self) -> Box<CloneableHandler> {
        Box::new(self.clone())
    }

    #[inline]
    fn method_type(&self) -> MethodType {
        self.method_type
    }
}

#[cfg(feature = "secure")]
mod imp {
    use grpc_sys::{self, GrpcServer};

    use credentials::ServerCredentials;

    pub struct Binder {
        pub host: String,
        pub port: u16,
        cred: Option<ServerCredentials>,
    }

    impl Binder {
        pub fn new(host: String, port: u16) -> Binder {
            let cred = None;
            Binder { host, port, cred }
        }

        pub fn with_cred(host: String, port: u16, cred: ServerCredentials) -> Binder {
            let cred = Some(cred);
            Binder { host, port, cred }
        }

        pub unsafe fn bind(&mut self, server: *mut GrpcServer) -> u16 {
            let addr = format!("{}:{}\0", self.host, self.port);
            let port = match self.cred.take() {
                None => grpc_sys::grpc_server_add_insecure_http2_port(server, addr.as_ptr() as _),
                Some(mut cert) => grpc_sys::grpc_server_add_secure_http2_port(
                    server,
                    addr.as_ptr() as _,
                    cert.as_mut_ptr(),
                ),
            };
            port as u16
        }
    }
}

#[cfg(not(feature = "secure"))]
mod imp {
    use grpc_sys::{self, GrpcServer};

    pub struct Binder {
        pub host: String,
        pub port: u16,
    }

    impl Binder {
        pub fn new(host: String, port: u16) -> Binder {
            Binder { host, port }
        }

        pub unsafe fn bind(&mut self, server: *mut GrpcServer) -> u16 {
            let addr = format!("{}:{}\0", self.host, self.port);
            grpc_sys::grpc_server_add_insecure_http2_port(server, addr.as_ptr() as _) as u16
        }
    }
}

use self::imp::Binder;

/// [`Service`] factory in order to configure the properties.
///
/// Use it to build a service which can be registered to a server.
pub struct ServiceBuilder {
    handlers: HashMap<&'static [u8], BoxHandler>,
}

impl ServiceBuilder {
    /// Initialize a new [`ServiceBuilder`].
    pub fn new() -> ServiceBuilder {
        ServiceBuilder {
            handlers: HashMap::new(),
        }
    }

    /// Add a unary RPC call handler.
    pub fn add_unary_handler<Req, Resp, F>(
        mut self,
        method: &Method<Req, Resp>,
        mut handler: F,
    ) -> ServiceBuilder
    where
        Req: 'static,
        Resp: 'static,
        F: FnMut(RpcContext, Req, UnarySink<Resp>) + Send + Clone + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = move |ctx: RpcContext, payload: &[u8]| {
            execute_unary(ctx, ser, de, payload, &mut handler)
        };
        let ch = Box::new(Handler::new(MethodType::Unary, h));
        self.handlers.insert(method.name.as_bytes(), ch);
        self
    }

    /// Add a client streaming RPC call handler.
    pub fn add_client_streaming_handler<Req, Resp, F>(
        mut self,
        method: &Method<Req, Resp>,
        mut handler: F,
    ) -> ServiceBuilder
    where
        Req: 'static,
        Resp: 'static,
        F: FnMut(RpcContext, RequestStream<Req>, ClientStreamingSink<Resp>)
            + Send
            + Clone
            + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h =
            move |ctx: RpcContext, _: &[u8]| execute_client_streaming(ctx, ser, de, &mut handler);
        let ch = Box::new(Handler::new(MethodType::ClientStreaming, h));
        self.handlers.insert(method.name.as_bytes(), ch);
        self
    }

    /// Add a server streaming RPC call handler.
    pub fn add_server_streaming_handler<Req, Resp, F>(
        mut self,
        method: &Method<Req, Resp>,
        mut handler: F,
    ) -> ServiceBuilder
    where
        Req: 'static,
        Resp: 'static,
        F: FnMut(RpcContext, Req, ServerStreamingSink<Resp>) + Send + Clone + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = move |ctx: RpcContext, payload: &[u8]| {
            execute_server_streaming(ctx, ser, de, payload, &mut handler)
        };
        let ch = Box::new(Handler::new(MethodType::ServerStreaming, h));
        self.handlers.insert(method.name.as_bytes(), ch);
        self
    }

    /// Add a duplex streaming RPC call handler.
    pub fn add_duplex_streaming_handler<Req, Resp, F>(
        mut self,
        method: &Method<Req, Resp>,
        mut handler: F,
    ) -> ServiceBuilder
    where
        Req: 'static,
        Resp: 'static,
        F: FnMut(RpcContext, RequestStream<Req>, DuplexSink<Resp>) + Send + Clone + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h =
            move |ctx: RpcContext, _: &[u8]| execute_duplex_streaming(ctx, ser, de, &mut handler);
        let ch = Box::new(Handler::new(MethodType::Duplex, h));
        self.handlers.insert(method.name.as_bytes(), ch);
        self
    }

    /// Finalize the [`ServiceBuilder`] and build the [`Service`].
    pub fn build(self) -> Service {
        Service {
            handlers: self.handlers,
        }
    }
}

/// A gRPC service.
///
/// Use [`ServiceBuilder`] to build a [`Service`].
pub struct Service {
    handlers: HashMap<&'static [u8], BoxHandler>,
}

/// [`Server`] factory in order to configure the properties.
pub struct ServerBuilder {
    env: Arc<Environment>,
    binders: Vec<Binder>,
    args: Option<ChannelArgs>,
    slots_per_cq: usize,
    handlers: HashMap<&'static [u8], BoxHandler>,
}

impl ServerBuilder {
    /// Initialize a new [`ServerBuilder`].
    pub fn new(env: Arc<Environment>) -> ServerBuilder {
        ServerBuilder {
            env,
            binders: Vec::new(),
            args: None,
            slots_per_cq: DEFAULT_REQUEST_SLOTS_PER_CQ,
            handlers: HashMap::new(),
        }
    }

    /// Bind to an address.
    ///
    /// This function can be called multiple times to bind to multiple ports.
    pub fn bind<S: Into<String>>(mut self, host: S, port: u16) -> ServerBuilder {
        self.binders.push(Binder::new(host.into(), port));
        self
    }

    /// Add additional configuration for each incoming channel.
    #[doc(hidden)]
    pub fn channel_args(mut self, args: ChannelArgs) -> ServerBuilder {
        self.args = Some(args);
        self
    }

    /// Set how many requests a completion queue can handle.
    pub fn requests_slot_per_cq(mut self, slots: usize) -> ServerBuilder {
        self.slots_per_cq = slots;
        self
    }

    /// Register a service.
    pub fn register_service(mut self, service: Service) -> ServerBuilder {
        self.handlers.extend(service.handlers);
        self
    }

    /// Finalize the [`ServerBuilder`] and build the [`Server`].
    pub fn build(mut self) -> Result<Server> {
        let args = self
            .args
            .as_ref()
            .map_or_else(ptr::null, |args| args.as_ptr());
        unsafe {
            let server = grpc_sys::grpc_server_create(args, ptr::null_mut());
            let mut bind_addrs = Vec::with_capacity(self.binders.len());
            for mut binder in self.binders.drain(..) {
                let bind_port = binder.bind(server);
                if bind_port == 0 {
                    grpc_sys::grpc_server_destroy(server);
                    return Err(Error::BindFail(binder.host, binder.port));
                }

                bind_addrs.push((binder.host, bind_port as u16));
            }

            for cq in self.env.completion_queues() {
                let cq_ref = cq.borrow()?;
                grpc_sys::grpc_server_register_completion_queue(
                    server,
                    cq_ref.as_ptr(),
                    ptr::null_mut(),
                );
            }

            Ok(Server {
                env: self.env,
                core: Arc::new(ServerCore {
                    server,
                    shutdown: AtomicBool::new(false),
                    bind_addrs,
                    slots_per_cq: self.slots_per_cq,
                }),
                handlers: self.handlers,
            })
        }
    }
}

#[cfg(feature = "secure")]
mod secure_server {
    use credentials::ServerCredentials;

    use super::{Binder, ServerBuilder};

    impl ServerBuilder {
        /// Bind to an address for secure connection.
        ///
        /// This function can be called multiple times to bind to multiple ports.
        pub fn bind_secure<S: Into<String>>(
            mut self,
            host: S,
            port: u16,
            c: ServerCredentials,
        ) -> ServerBuilder {
            self.binders.push(Binder::with_cred(host.into(), port, c));
            self
        }
    }
}

struct ServerCore {
    server: *mut GrpcServer,
    bind_addrs: Vec<(String, u16)>,
    slots_per_cq: usize,
    shutdown: AtomicBool,
}

impl Drop for ServerCore {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_server_destroy(self.server) }
    }
}

unsafe impl Send for ServerCore {}
unsafe impl Sync for ServerCore {}

pub type BoxHandler = Box<CloneableHandler>;

#[derive(Clone)]
pub struct RequestCallContext {
    server: Arc<ServerCore>,
    registry: Arc<UnsafeCell<HashMap<&'static [u8], BoxHandler>>>,
}

impl RequestCallContext {
    /// Users should guarantee the method is always called from the same thread.
    /// TODO: Is there a better way?
    #[inline]
    pub unsafe fn get_handler(&mut self, path: &[u8]) -> Option<&mut BoxHandler> {
        let registry = &mut *self.registry.get();
        registry.get_mut(path)
    }
}

// Apprently, its life time is guaranteed by the ref count, hence is safe to be sent
// to other thread. However it's not `Sync`, as `BoxHandler` is not neccessary `Sync`.
unsafe impl Send for RequestCallContext {}

/// Request notification of a new call.
pub fn request_call(ctx: RequestCallContext, cq: &CompletionQueue) {
    if ctx.server.shutdown.load(Ordering::Relaxed) {
        return;
    }
    let cq_ref = match cq.borrow() {
        // Shutting down, skip.
        Err(_) => return,
        Ok(c) => c,
    };
    let server_ptr = ctx.server.server;
    let prom = CallTag::request(ctx);
    let request_ptr = prom.request_ctx().unwrap().as_ptr();
    let prom_box = Box::new(prom);
    let tag = Box::into_raw(prom_box);
    let code = unsafe {
        grpc_sys::grpcwrap_server_request_call(
            server_ptr,
            cq_ref.as_ptr(),
            request_ptr,
            tag as *mut _,
        )
    };
    if code != GrpcCallStatus::Ok {
        Box::from(tag);
        panic!("failed to request call: {:?}", code);
    }
}

/// A `Future` that will resolve when shutdown completes.
pub struct ShutdownFuture {
    cq_f: CqFuture<()>,
}

impl Future for ShutdownFuture {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        try_ready!(self.cq_f.poll());
        Ok(Async::Ready(()))
    }
}

/// A gRPC server.
///
/// A single server can serve arbitrary number of services and can listen on more than one port.
///
/// Use [`ServerBuilder`] to build a [`Server`].
pub struct Server {
    env: Arc<Environment>,
    core: Arc<ServerCore>,
    handlers: HashMap<&'static [u8], BoxHandler>,
}

impl Server {
    /// Shutdown the server asynchronously.
    pub fn shutdown(&mut self) -> ShutdownFuture {
        let (cq_f, prom) = CallTag::shutdown_pair();
        let prom_box = Box::new(prom);
        let tag = Box::into_raw(prom_box);
        unsafe {
            // Since env still exists, no way can cq been shutdown.
            let cq_ref = self.env.completion_queues()[0].borrow().unwrap();
            grpc_sys::grpc_server_shutdown_and_notify(
                self.core.server,
                cq_ref.as_ptr(),
                tag as *mut _,
            )
        }
        self.core.shutdown.store(true, Ordering::SeqCst);
        ShutdownFuture { cq_f }
    }

    /// Cancel all in-progress calls.
    ///
    /// Only usable after shutdown.
    pub fn cancel_all_calls(&mut self) {
        unsafe { grpc_sys::grpc_server_cancel_all_calls(self.core.server) }
    }

    /// Start the server.
    pub fn start(&mut self) {
        unsafe {
            grpc_sys::grpc_server_start(self.core.server);
            for cq in self.env.completion_queues() {
                // Handlers are Send and Clone, but not Sync. So we need to
                // provide a replica for each completion queue.
                let registry = self
                    .handlers
                    .iter()
                    .map(|(k, v)| (k.to_owned(), v.box_clone()))
                    .collect();
                let rc = RequestCallContext {
                    server: self.core.clone(),
                    registry: Arc::new(UnsafeCell::new(registry)),
                };
                for _ in 0..self.core.slots_per_cq {
                    request_call(rc.clone(), cq);
                }
            }
        }
    }

    /// Get binded addresses.
    pub fn bind_addrs(&self) -> &[(String, u16)] {
        &self.core.bind_addrs
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        // if the server is not shutdown completely, destroy a server will core.
        // TODO: don't wait here
        let f = self.shutdown();
        self.cancel_all_calls();
        let _ = f.wait();
    }
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Server {:?}", self.core.bind_addrs)
    }
}
