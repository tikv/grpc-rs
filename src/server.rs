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


use std::collections::HashMap;
use std::ptr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fmt;
use std::fmt::{Debug, Formatter};

use futures::{Async, Future, Poll};
use grpc_sys::{self, GrpcCallStatus, GrpcServer};

use RpcContext;
use async::{CallTag, CqFuture};
use call::{Method, MethodType};
use call::server::*;
use channel::ChannelArgs;
use cq::CompletionQueue;
use env::Environment;
use error::{Error, Result};

const DEFAULT_REQUEST_SLOTS_PER_CQ: usize = 1024;

pub type CallBack = Box<Fn(RpcContext, &[u8])>;

/// Handler is an rpc call holder.
pub struct Handler {
    method_type: MethodType,
    cb: CallBack,
}

impl Handler {
    pub fn new(method_type: MethodType, cb: CallBack) -> Handler {
        Handler {
            method_type: method_type,
            cb: cb,
        }
    }

    pub fn cb(&self) -> &CallBack {
        &self.cb
    }

    pub fn method_type(&self) -> MethodType {
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

/// Service configuration struct.
///
/// Use it to build a service which can be registered to a server.
pub struct ServiceBuilder {
    handlers: HashMap<&'static [u8], Handler>,
}

impl ServiceBuilder {
    pub fn new() -> ServiceBuilder {
        ServiceBuilder {
            handlers: HashMap::new(),
        }
    }

    /// Add a unary rpc call handler.
    pub fn add_unary_handler<P, Q, F>(mut self, method: &Method<P, Q>, handler: F) -> ServiceBuilder
    where
        P: 'static,
        Q: 'static,
        F: Fn(RpcContext, P, UnarySink<Q>) + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = Box::new(move |ctx: RpcContext, payload: &[u8]| {
            execute_unary(ctx, ser, de, payload, &handler)
        });
        self.handlers
            .insert(method.name.as_bytes(), Handler::new(MethodType::Unary, h));
        self
    }

    /// Add a client streaming rpc call handler.
    pub fn add_client_streaming_handler<P, Q, F>(
        mut self,
        method: &Method<P, Q>,
        handler: F,
    ) -> ServiceBuilder
    where
        P: 'static,
        Q: 'static,
        F: Fn(RpcContext, RequestStream<P>, ClientStreamingSink<Q>) + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = Box::new(move |ctx: RpcContext, _: &[u8]| {
            execute_client_streaming(ctx, ser, de, &handler)
        });
        self.handlers.insert(
            method.name.as_bytes(),
            Handler::new(MethodType::ClientStreaming, h),
        );
        self
    }

    /// Add a server streaming rpc call handler.
    pub fn add_server_streaming_handler<P, Q, F>(
        mut self,
        method: &Method<P, Q>,
        handler: F,
    ) -> ServiceBuilder
    where
        P: 'static,
        Q: 'static,
        F: Fn(RpcContext, P, ServerStreamingSink<Q>) + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = Box::new(move |ctx: RpcContext, payload: &[u8]| {
            execute_server_streaming(ctx, ser, de, payload, &handler)
        });
        self.handlers.insert(
            method.name.as_bytes(),
            Handler::new(MethodType::ServerStreaming, h),
        );
        self
    }

    /// Add a duplex streaming rpc call handler.
    pub fn add_duplex_streaming_handler<P, Q, F>(
        mut self,
        method: &Method<P, Q>,
        handler: F,
    ) -> ServiceBuilder
    where
        P: 'static,
        Q: 'static,
        F: Fn(RpcContext, RequestStream<P>, DuplexSink<Q>) + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = Box::new(move |ctx: RpcContext, _: &[u8]| {
            execute_duplex_streaming(ctx, ser, de, &handler)
        });
        self.handlers
            .insert(method.name.as_bytes(), Handler::new(MethodType::Duplex, h));
        self
    }

    pub fn build(self) -> Service {
        Service {
            handlers: self.handlers,
        }
    }
}

pub struct Service {
    handlers: HashMap<&'static [u8], Handler>,
}

/// Server configuration struct.
pub struct ServerBuilder {
    env: Arc<Environment>,
    binders: Vec<Binder>,
    args: Option<ChannelArgs>,
    slots_per_cq: usize,
    handlers: HashMap<&'static [u8], Handler>,
}

impl ServerBuilder {
    pub fn new(env: Arc<Environment>) -> ServerBuilder {
        ServerBuilder {
            env: env,
            binders: Vec::new(),
            args: None,
            slots_per_cq: DEFAULT_REQUEST_SLOTS_PER_CQ,
            handlers: HashMap::new(),
        }
    }

    /// Bind to an address.
    ///
    /// This function can be called multiple times.
    pub fn bind<S: Into<String>>(mut self, host: S, port: u16) -> ServerBuilder {
        self.binders.push(Binder::new(host.into(), port));
        self
    }

    /// Add additional configuration for each incoming channel.
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

    pub fn build(mut self) -> Result<Server> {
        let args = self.args
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
                inner: Arc::new(Inner {
                    env: self.env,
                    server: server,
                    shutdown: AtomicBool::new(false),
                    bind_addrs: bind_addrs,
                    slots_per_cq: self.slots_per_cq,
                    handlers: self.handlers,
                }),
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
        /// This function can be called multiple times.
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

pub struct Inner {
    env: Arc<Environment>,
    server: *mut GrpcServer,
    bind_addrs: Vec<(String, u16)>,
    slots_per_cq: usize,
    shutdown: AtomicBool,
    handlers: HashMap<&'static [u8], Handler>,
}

impl Inner {
    /// Get the handler for the requested method path.
    pub fn get_handler(&self, method: &[u8]) -> Option<&Handler> {
        self.handlers.get(method)
    }
}

impl Debug for Inner {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Server {:?}", self.bind_addrs)
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe { grpc_sys::grpc_server_destroy(self.server) }
    }
}

/// Request notification of a new call.
pub fn request_call(inner: Arc<Inner>, cq: &CompletionQueue) {
    if inner.shutdown.load(Ordering::Relaxed) {
        return;
    }
    let cq_ref = match cq.borrow() {
        // Shutting down, skip.
        Err(_) => return,
        Ok(c) => c,
    };
    let server_ptr = inner.server;
    let prom = CallTag::request(inner);
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

/// An asynchronize shutdown future.
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

// It's safe to request call simultaneously.
unsafe impl Sync for Inner {}
unsafe impl Send for Inner {}

pub struct Server {
    inner: Arc<Inner>,
}

impl Server {
    /// Shutdown the server asynchronously.
    pub fn shutdown(&mut self) -> ShutdownFuture {
        let (cq_f, prom) = CallTag::shutdown_pair();
        let prom_box = Box::new(prom);
        let tag = Box::into_raw(prom_box);
        unsafe {
            // Since env still exists, no way can cq been shutdown.
            let cq_ref = self.inner.env.completion_queues()[0].borrow().unwrap();
            grpc_sys::grpc_server_shutdown_and_notify(
                self.inner.server,
                cq_ref.as_ptr(),
                tag as *mut _,
            )
        }
        self.inner.shutdown.store(true, Ordering::SeqCst);
        ShutdownFuture { cq_f: cq_f }
    }

    /// Cancel all in-progress calls.
    ///
    /// Only usable after shutdown.
    pub fn cancel_all_calls(&mut self) {
        unsafe { grpc_sys::grpc_server_cancel_all_calls(self.inner.server) }
    }

    /// Start a server.
    ///
    /// Tells all listeners to start listening.
    pub fn start(&mut self) {
        unsafe {
            grpc_sys::grpc_server_start(self.inner.server);
            for cq in self.inner.env.completion_queues() {
                for _ in 0..self.inner.slots_per_cq {
                    request_call(self.inner.clone(), cq);
                }
            }
        }
    }

    /// Get the binded addresses.
    pub fn bind_addrs(&self) -> &[(String, u16)] {
        &self.inner.bind_addrs
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
        write!(f, "{:?}", self.inner)
    }
}
