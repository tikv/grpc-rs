// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::net::{IpAddr, SocketAddr};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::grpc_sys::{self, grpc_call_error, grpc_server};
use futures::{Async, Future, Poll};

use crate::call::server::*;
use crate::call::{MessageReader, Method, MethodType};
use crate::channel::ChannelArgs;
use crate::cq::CompletionQueue;
use crate::env::Environment;
use crate::error::{Error, Result};
use crate::task::{CallTag, CqFuture};
use crate::RpcContext;

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
    fn handle(&mut self, ctx: RpcContext<'_>, reqs: Option<MessageReader>);
    fn box_clone(&self) -> Box<dyn CloneableHandler>;
    fn method_type(&self) -> MethodType;
}

impl<F: 'static> CloneableHandler for Handler<F>
where
    F: FnMut(RpcContext<'_>, Option<MessageReader>) + Send + Clone,
{
    #[inline]
    fn handle(&mut self, ctx: RpcContext<'_>, reqs: Option<MessageReader>) {
        (self.cb)(ctx, reqs)
    }

    #[inline]
    fn box_clone(&self) -> Box<dyn CloneableHandler> {
        Box::new(self.clone())
    }

    #[inline]
    fn method_type(&self) -> MethodType {
        self.method_type
    }
}

/// Given a host and port, creates a string of the form "host:port" or
/// "[host]:port", depending on whether the host is an IPv6 literal.
fn join_host_port(host: &str, port: u16) -> String {
    if let Ok(ip) = host.parse::<IpAddr>() {
        format!("{}\0", SocketAddr::new(ip, port))
    } else {
        format!("{}:{}\0", host, port)
    }
}

#[cfg(feature = "secure")]
mod imp {
    use super::join_host_port;
    use crate::grpc_sys::{self, grpc_server};
    use crate::security::ServerCredentialsFetcher;
    use crate::ServerCredentials;

    pub struct Binder {
        pub host: String,
        pub port: u16,
        cred: Option<ServerCredentials>,
        _fetcher: Option<Box<Box<dyn ServerCredentialsFetcher + Send + Sync>>>,
    }

    impl Binder {
        pub fn new(host: String, port: u16) -> Binder {
            let cred = None;
            Binder {
                host,
                port,
                cred,
                _fetcher: None,
            }
        }

        pub fn with_cred(
            host: String,
            port: u16,
            cred: ServerCredentials,
            _fetcher: Option<Box<Box<dyn ServerCredentialsFetcher + Send + Sync>>>,
        ) -> Binder {
            let cred = Some(cred);
            Binder {
                host,
                port,
                cred,
                _fetcher,
            }
        }

        pub unsafe fn bind(&mut self, server: *mut grpc_server) -> u16 {
            let addr = join_host_port(&self.host, self.port);
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
    use super::join_host_port;
    use crate::grpc_sys::{self, grpc_server};

    pub struct Binder {
        pub host: String,
        pub port: u16,
    }

    impl Binder {
        pub fn new(host: String, port: u16) -> Binder {
            Binder { host, port }
        }

        pub unsafe fn bind(&mut self, server: *mut grpc_server) -> u16 {
            let addr = join_host_port(&self.host, self.port);
            grpc_sys::grpc_server_add_insecure_http2_port(server, addr.as_ptr() as _) as u16
        }
    }
}

use self::imp::Binder;

impl Debug for Binder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Binder {{ host: {}, port: {} }}", self.host, self.port)
    }
}

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
        F: FnMut(RpcContext<'_>, Req, UnarySink<Resp>) + Send + Clone + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = move |ctx: RpcContext<'_>, payload: Option<MessageReader>| {
            execute_unary(ctx, ser, de, payload.unwrap(), &mut handler)
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
        F: FnMut(RpcContext<'_>, RequestStream<Req>, ClientStreamingSink<Resp>)
            + Send
            + Clone
            + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = move |ctx: RpcContext<'_>, _: Option<MessageReader>| {
            execute_client_streaming(ctx, ser, de, &mut handler)
        };
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
        F: FnMut(RpcContext<'_>, Req, ServerStreamingSink<Resp>) + Send + Clone + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = move |ctx: RpcContext<'_>, payload: Option<MessageReader>| {
            execute_server_streaming(ctx, ser, de, payload.unwrap(), &mut handler)
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
        F: FnMut(RpcContext<'_>, RequestStream<Req>, DuplexSink<Resp>) + Send + Clone + 'static,
    {
        let (ser, de) = (method.resp_ser(), method.req_de());
        let h = move |ctx: RpcContext<'_>, _: Option<MessageReader>| {
            execute_duplex_streaming(ctx, ser, de, &mut handler)
        };
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
            .map_or_else(ptr::null, ChannelArgs::as_ptr);
        unsafe {
            let server = grpc_sys::grpc_server_create(args, ptr::null_mut());
            for binder in self.binders.iter_mut() {
                let bind_port = binder.bind(server);
                if bind_port == 0 {
                    grpc_sys::grpc_server_destroy(server);
                    return Err(Error::BindFail(binder.host.clone(), binder.port));
                }
                binder.port = bind_port;
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
                    binders: self.binders,
                    slots_per_cq: self.slots_per_cq,
                }),
                handlers: self.handlers,
            })
        }
    }
}

#[cfg(feature = "secure")]
mod secure_server {
    use super::{Binder, ServerBuilder};
    use crate::grpc_sys;
    use crate::security::{
        server_cert_fetcher_wrapper, CertificateRequestType, ServerCredentials,
        ServerCredentialsFetcher,
    };

    impl ServerBuilder {
        /// Bind to an address with credentials for secure connection.
        ///
        /// This function can be called multiple times to bind to multiple ports.
        pub fn bind_with_cred<S: Into<String>>(
            mut self,
            host: S,
            port: u16,
            c: ServerCredentials,
        ) -> ServerBuilder {
            self.binders
                .push(Binder::with_cred(host.into(), port, c, None));
            self
        }

        /// Bind to an address for secure connection.
        ///
        /// The required credentials will be fetched using provided `fetcher`. This
        /// function can be called multiple times to bind to multiple ports.
        pub fn bind_with_fetcher<S: Into<String>>(
            mut self,
            host: S,
            port: u16,
            fetcher: Box<dyn ServerCredentialsFetcher + Send + Sync>,
            cer_request_type: CertificateRequestType,
        ) -> ServerBuilder {
            let fetcher_wrap = Box::new(fetcher);
            let fetcher_wrap_ptr = Box::into_raw(fetcher_wrap);
            let (sc, fb) = unsafe {
                let opt = grpc_sys::grpc_ssl_server_credentials_create_options_using_config_fetcher(
                    cer_request_type.to_native(),
                    Some(server_cert_fetcher_wrapper),
                    fetcher_wrap_ptr as _,
                );
                (
                    ServerCredentials::frow_raw(
                        grpcio_sys::grpc_ssl_server_credentials_create_with_options(opt),
                    ),
                    Box::from_raw(fetcher_wrap_ptr),
                )
            };
            self.binders
                .push(Binder::with_cred(host.into(), port, sc, Some(fb)));
            self
        }
    }
}

struct ServerCore {
    server: *mut grpc_server,
    binders: Vec<Binder>,
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

pub type BoxHandler = Box<dyn CloneableHandler>;

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

// Apparently, its life time is guaranteed by the ref count, hence is safe to be sent
// to other thread. However it's not `Sync`, as `BoxHandler` is unnecessarily `Sync`.
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
    if code != grpc_call_error::GRPC_CALL_OK {
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

    /// Get binded addresses pairs.
    pub fn bind_addrs(&self) -> impl ExactSizeIterator<Item = (&String, u16)> {
        self.core.binders.iter().map(|b| (&b.host, b.port))
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Server {:?}", self.core.binders)
    }
}

#[cfg(test)]
mod tests {
    use super::join_host_port;

    #[test]
    fn test_join_host_port() {
        let tbl = vec![
            ("localhost", 0u16, "localhost:0\0"),
            ("127.0.0.1", 100u16, "127.0.0.1:100\0"),
            ("::1", 0u16, "[::1]:0\0"),
            (
                "fe80::7376:45d5:fb08:61e3",
                10028u16,
                "[fe80::7376:45d5:fb08:61e3]:10028\0",
            ),
        ];

        for (h, p, e) in &tbl {
            assert_eq!(join_host_port(h, *p), e.to_owned());
        }
    }
}
