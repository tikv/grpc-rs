// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet};
use std::ffi::{c_void, CString};
use std::fmt::{self, Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use crate::grpc_sys::{
    self, grpc_call_error, grpc_server, grpc_server_register_method_payload_handling,
};
use futures_util::ready;

use crate::call::server::*;
use crate::call::{MessageReader, Method, MethodType};
use crate::channel::ChannelArgs;
use crate::cq::CompletionQueue;
use crate::env::Environment;
use crate::error::{Error, Result};
use crate::task::{CallTag, CqFuture};
use crate::RpcStatus;
use crate::{RpcContext, ServerCredentials};

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

/// Used to indicate the result of the check. If it returns `Abort`,
/// skip the subsequent checkers and abort the grpc call.
pub enum CheckResult {
    Continue,
    Abort(RpcStatus),
}

pub trait ServerChecker: Send {
    fn check(&mut self, ctx: &RpcContext) -> CheckResult;
    fn box_clone(&self) -> Box<dyn ServerChecker>;
}

impl Clone for Box<dyn ServerChecker> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

/// A gRPC service.
///
/// Use [`ServiceBuilder`] to build a [`Service`].
pub struct Service {
    handlers: HashMap<&'static [u8], BoxHandler>,
}

/// Methods whose calls are served on the completion queues of `env`.
struct MethodRoute {
    env: Arc<Environment>,
    methods: Vec<&'static str>,
}

/// [`Server`] factory in order to configure the properties.
pub struct ServerBuilder {
    env: Arc<Environment>,
    args: Option<ChannelArgs>,
    slots_per_cq: usize,
    handlers: HashMap<&'static [u8], BoxHandler>,
    checkers: Vec<Box<dyn ServerChecker>>,
    routes: Vec<MethodRoute>,
}

impl ServerBuilder {
    /// Initialize a new [`ServerBuilder`].
    pub fn new(env: Arc<Environment>) -> ServerBuilder {
        ServerBuilder {
            env,
            args: None,
            slots_per_cq: DEFAULT_REQUEST_SLOTS_PER_CQ,
            handlers: HashMap::new(),
            checkers: Vec::new(),
            routes: Vec::new(),
        }
    }

    /// Add additional configuration for each incoming channel.
    pub fn channel_args(mut self, args: ChannelArgs) -> ServerBuilder {
        self.args = Some(args);
        self
    }

    /// Set the number of pending requests posted to each completion queue.
    ///
    /// The builder's environment posts this many generic requests per queue.
    /// Routed environments post this many requests per method per queue.
    pub fn requests_slot_per_cq(mut self, slots: usize) -> ServerBuilder {
        self.slots_per_cq = slots;
        self
    }

    /// Register a service.
    pub fn register_service(mut self, service: Service) -> ServerBuilder {
        self.handlers.extend(service.handlers);
        self
    }

    /// Add a custom checker to handle some tasks before the grpc call handler starts.
    /// This allows users to operate grpc call based on the context. Users can add
    /// multiple checkers and they will be executed in the order added.
    ///
    /// TODO: Extend this interface to intercepte each payload like grpc-c++.
    pub fn add_checker<C: ServerChecker + 'static>(mut self, checker: C) -> ServerBuilder {
        self.checkers.push(Box::new(checker));
        self
    }

    /// Serve calls of `methods` on the completion queues of `env`.
    ///
    /// A method is identified by its full path, such as `/pkg.Service/Method`,
    /// and must have a handler registered by a service. The server matches
    /// these calls by path before binding them to a completion queue. Their
    /// handlers and checkers run on `env`; calls of other methods use the
    /// builder's environment. Routed calls do not expose the client's
    /// authority: [`RpcContext::host`] returns an empty slice because gRPC
    /// Core's registered-call API does not return that field. Do not route
    /// methods whose handlers or checkers require the authority.
    ///
    /// Build `env` with [`EnvBuilder::set_non_listening`](crate::EnvBuilder::set_non_listening)
    /// to exclude its queues from the server's listener pollsets. Routing
    /// separates handler execution and completion-event consumption, but does
    /// not guarantee independent progress for connection acceptance or RPC
    /// setup before CQ binding. Those stages can still depend on the builder's
    /// environment and the polling backend; lower-level transport I/O can also
    /// remain shared after binding.
    ///
    /// Calls of these methods are not balanced across the completion queues of
    /// `env`: gRPC Core binds each of them to the first completion queue of
    /// `env` that has a pending request.
    ///
    /// # Panics
    ///
    /// [`build`](ServerBuilder::build) panics if a method has no handler or is
    /// routed more than once.
    pub fn route_methods(
        mut self,
        env: Arc<Environment>,
        methods: &[&'static str],
    ) -> ServerBuilder {
        self.routes.push(MethodRoute {
            env,
            methods: methods.to_vec(),
        });
        self
    }

    /// Finalize the [`ServerBuilder`] and build the [`Server`].
    pub fn build(self) -> Result<Server> {
        self.check_routes();
        let args = self
            .args
            .as_ref()
            .map_or_else(ptr::null, ChannelArgs::as_ptr);
        unsafe {
            let server = grpc_sys::grpc_server_create(args, ptr::null_mut());
            let route_envs = self.routes.iter().map(|route| &route.env);
            let mut registered_cqs = HashSet::new();
            for env in std::iter::once(&self.env).chain(route_envs) {
                for cq in env.completion_queues() {
                    let cq_ref = cq.borrow()?;
                    if !registered_cqs.insert(cq_ref.as_ptr()) {
                        continue;
                    }
                    grpc_sys::grpc_server_register_completion_queue(
                        server,
                        cq_ref.as_ptr(),
                        ptr::null_mut(),
                    );
                }
            }
            let routes = self
                .routes
                .into_iter()
                .map(|route| register_route(server, route))
                .collect();

            Ok(Server {
                env: self.env,
                core: Arc::new(ServerCore {
                    server,
                    creds: Mutex::new(Vec::new()),
                    shutdown: AtomicBool::new(false),
                    slots_per_cq: self.slots_per_cq,
                }),
                handlers: self.handlers,
                checkers: self.checkers,
                routes,
            })
        }
    }

    fn check_routes(&self) {
        let mut routed = HashSet::new();
        for &method in self.routes.iter().flat_map(|route| &route.methods) {
            assert!(
                self.handlers.contains_key(method.as_bytes()),
                "routed method {} has no handler",
                method
            );
            assert!(
                routed.insert(method),
                "method {} is routed more than once",
                method
            );
        }
    }
}

/// Registers the methods of `route` to `server`.
///
/// It must be called before the server starts.
unsafe fn register_route(server: *mut grpc_server, route: MethodRoute) -> RegisteredRoute {
    let methods = route
        .methods
        .into_iter()
        .map(|path| {
            let c_path = CString::new(path).unwrap();
            let handle = grpc_sys::grpc_server_register_method(
                server,
                c_path.as_ptr(),
                ptr::null(),
                grpc_server_register_method_payload_handling::GRPC_SRM_PAYLOAD_NONE,
                0,
            );
            assert!(!handle.is_null(), "failed to register method {}", path);
            RegisteredMethod {
                path: path.as_bytes(),
                handle,
            }
        })
        .collect();
    RegisteredRoute {
        env: route.env,
        methods,
    }
}

/// Methods registered to a server, and the environment serving their calls.
struct RegisteredRoute {
    env: Arc<Environment>,
    methods: Vec<RegisteredMethod>,
}

/// A method registered to a server by `grpc_server_register_method`.
#[derive(Clone, Copy)]
struct RegisteredMethod {
    path: &'static [u8],
    /// The registration handle, owned by the server.
    handle: *mut c_void,
}

// The handle is only passed back to gRPC Core, which is thread safe.
unsafe impl Send for RegisteredMethod {}
unsafe impl Sync for RegisteredMethod {}

struct ServerCore {
    server: *mut grpc_server,
    creds: Mutex<Vec<ServerCredentials>>,
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
    checkers: Vec<Box<dyn ServerChecker>>,
    /// The registered method whose calls are requested, or `None` to request
    /// calls of methods that are not registered.
    method: Option<RegisteredMethod>,
}

impl RequestCallContext {
    /// Users should guarantee the method is always called from the same thread.
    /// TODO: Is there a better way?
    #[inline]
    pub unsafe fn get_handler(&mut self, path: &[u8]) -> Option<&mut BoxHandler> {
        let registry = &mut *self.registry.get();
        registry.get_mut(path)
    }

    pub(crate) fn get_checker(&self) -> Vec<Box<dyn ServerChecker>> {
        self.checkers.clone()
    }

    /// Returns the path of the registered method whose calls are requested.
    ///
    /// gRPC Core does not fill in call details for calls of registered methods.
    pub(crate) fn registered_method_path(&self) -> Option<&'static [u8]> {
        self.method.map(|method| method.path)
    }
}

// Apparently, its life time is guaranteed by the ref count, hence is safe to be sent
// to other thread. However it's not `Sync`, as `BoxHandler` is unnecessarily `Sync`.
#[allow(clippy::non_send_fields_in_send_ty)]
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
    let method = ctx.method;
    let prom = CallTag::request(ctx);
    let request_ptr = prom.request_ctx().unwrap().as_ptr();
    let prom_box = Box::new(prom);
    let tag = Box::into_raw(prom_box);
    let code = unsafe {
        match method {
            None => grpc_sys::grpcwrap_server_request_call(
                server_ptr,
                cq_ref.as_ptr(),
                request_ptr,
                tag as *mut _,
            ),
            Some(method) => grpc_sys::grpc_server_request_registered_call(
                server_ptr,
                method.handle,
                &mut (*request_ptr).call,
                &mut (*request_ptr).call_details.deadline,
                &mut (*request_ptr).request_metadata,
                ptr::null_mut(),
                cq_ref.as_ptr(),
                cq_ref.as_ptr(),
                tag as *mut _,
            ),
        }
    };
    if code != grpc_call_error::GRPC_CALL_OK {
        drop(Box::from(tag));
        panic!("failed to request call: {:?}", code);
    }
}

/// A `Future` that will resolve when shutdown completes.
pub struct ShutdownFuture {
    /// `true` means the future finishes successfully.
    cq_f: CqFuture<bool>,
}

impl Future for ShutdownFuture {
    type Output = Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match ready!(Pin::new(&mut self.cq_f).poll(cx)) {
            Ok(true) => Poll::Ready(Ok(())),
            Ok(false) => Poll::Ready(Err(Error::ShutdownFailed)),
            Err(e) => unreachable!("action future should never resolve to error: {}", e),
        }
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
    checkers: Vec<Box<dyn ServerChecker>>,
    routes: Vec<RegisteredRoute>,
}

impl Server {
    /// Shutdown the server asynchronously.
    pub fn shutdown(&mut self) -> ShutdownFuture {
        let (cq_f, prom) = CallTag::action_pair();
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
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn start(&mut self) {
        unsafe {
            grpc_sys::grpc_server_start(self.core.server);
            for cq in self.env.completion_queues() {
                let rc = self.request_call_context(self.handler_registry(), None);
                for _ in 0..self.core.slots_per_cq {
                    request_call(rc.clone(), cq);
                }
            }
            for route in &self.routes {
                for cq in route.env.completion_queues() {
                    let registry = self.handler_registry();
                    for &method in &route.methods {
                        let rc = self.request_call_context(registry.clone(), Some(method));
                        for _ in 0..self.core.slots_per_cq {
                            request_call(rc.clone(), cq);
                        }
                    }
                }
            }
        }
    }

    /// Creates a replica of the handlers for one completion queue.
    ///
    /// Handlers are Send and Clone, but not Sync. So we need to provide a
    /// replica for each completion queue.
    #[allow(clippy::arc_with_non_send_sync)]
    fn handler_registry(&self) -> Arc<UnsafeCell<HashMap<&'static [u8], BoxHandler>>> {
        let registry = self
            .handlers
            .iter()
            .map(|(k, v)| (k.to_owned(), v.box_clone()))
            .collect();
        Arc::new(UnsafeCell::new(registry))
    }

    fn request_call_context(
        &self,
        registry: Arc<UnsafeCell<HashMap<&'static [u8], BoxHandler>>>,
        method: Option<RegisteredMethod>,
    ) -> RequestCallContext {
        RequestCallContext {
            server: self.core.clone(),
            registry,
            checkers: self.checkers.clone(),
            method,
        }
    }

    /// Try binding the server to the given `addr` endpoint (eg, `localhost:1234`,
    /// `192.168.1.1:31416`, `[::1]:27182`, etc.).
    ///
    /// It can be invoked multiple times. Should be used before starting the server.
    ///
    /// # Return
    ///
    /// The bound port is returned on success.
    pub fn add_listening_port(
        &mut self,
        addr: impl Into<String>,
        mut creds: ServerCredentials,
    ) -> Result<u16> {
        // There is no Null in UTF-8 string.
        let addr = CString::new(addr.into()).unwrap();
        let port = unsafe {
            grpcio_sys::grpc_server_add_http2_port(
                self.core.server,
                addr.as_ptr() as _,
                creds.as_mut_ptr(),
            ) as u16
        };
        if port != 0 {
            self.core.creds.lock().unwrap().push(creds);
            Ok(port)
        } else {
            Err(Error::BindFail(addr))
        }
    }

    /// Add an rpc channel for an established connection represented as a file
    /// descriptor. Takes ownership of the file descriptor, closing it when
    /// channel is closed.
    ///
    /// # Safety
    ///
    /// The file descriptor must correspond to a connected stream socket. After
    /// this call, the socket must not be accessed (read / written / closed)
    /// by other code.
    #[cfg(unix)]
    pub unsafe fn add_channel_from_fd(&mut self, fd: ::std::os::raw::c_int) {
        let mut creds = ServerCredentials::insecure();
        grpcio_sys::grpc_server_add_channel_from_fd(self.core.server, fd, creds.as_mut_ptr())
    }

    /// Add an RPC channel for an established connection represented as a file
    /// descriptor. The file descriptor must correspond to a connected stream
    /// socket. Takes ownership of the file descriptor, closing it when channel
    /// is closed.
    #[cfg(unix)]
    pub fn add_channel_from_ownedfd(&mut self, fd: ::std::os::fd::OwnedFd) {
        use std::os::fd::IntoRawFd;

        // SAFETY: `OwnedFd` confers ownership of the file descriptor, so it
        // won't be accessed by other code.
        unsafe { self.add_channel_from_fd(fd.into_raw_fd()) }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        // if the server is not shutdown completely, destroy a server will core.
        // TODO: don't wait here
        let f = if !self.core.shutdown.load(Ordering::SeqCst) {
            Some(self.shutdown())
        } else {
            None
        };
        self.cancel_all_calls();
        let _ = f.map(futures_executor::block_on);
    }
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Server {{ handlers: {}, checkers: {} }}",
            self.handlers.len(),
            self.checkers.len()
        )
    }
}
