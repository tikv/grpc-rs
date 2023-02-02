/// The request message containing the user's name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response message containing the greetings
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
const METHOD_GREETER_SAY_HELLO: ::grpcio::Method<HelloRequest, HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/helloworld.Greeter/SayHello",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
#[derive(Clone)]
pub struct GreeterClient {
    pub client: ::grpcio::Client,
}
impl GreeterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GreeterClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn say_hello_opt(
        &self,
        req: &HelloRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<HelloReply> {
        self.client.unary_call(&METHOD_GREETER_SAY_HELLO, req, opt)
    }
    pub fn say_hello(&self, req: &HelloRequest) -> ::grpcio::Result<HelloReply> {
        self.say_hello_opt(req, ::grpcio::CallOption::default())
    }
    pub fn say_hello_async_opt(
        &self,
        req: &HelloRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<HelloReply>> {
        self.client
            .unary_call_async(&METHOD_GREETER_SAY_HELLO, req, opt)
    }
    pub fn say_hello_async(
        &self,
        req: &HelloRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<HelloReply>> {
        self.say_hello_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::std::future::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait Greeter {
    fn say_hello(
        &mut self,
        ctx: ::grpcio::RpcContext,
        _req: HelloRequest,
        sink: ::grpcio::UnarySink<HelloReply>,
    ) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}
pub fn create_greeter<S: Greeter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_GREETER_SAY_HELLO, move |ctx, req, resp| {
        instance.say_hello(ctx, req, resp)
    });
    builder.build()
}
