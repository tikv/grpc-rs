// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GREETER_SAY_HELLO: ::grpcio::Method<
    super::helloworld::HelloRequest,
    super::helloworld::HelloReply,
> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/helloworld.Greeter/SayHello",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pb_ser,
        de: ::grpcio::pb_de,
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
        req: &super::helloworld::HelloRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<super::helloworld::HelloReply> {
        self.client.unary_call(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello(
        &self,
        req: &super::helloworld::HelloRequest,
    ) -> ::grpcio::Result<super::helloworld::HelloReply> {
        self.say_hello_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_hello_async_opt(
        &self,
        req: &super::helloworld::HelloRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::helloworld::HelloReply>> {
        self.client
            .unary_call_async(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello_async(
        &self,
        req: &super::helloworld::HelloRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::helloworld::HelloReply>> {
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
        _req: super::helloworld::HelloRequest,
        sink: ::grpcio::UnarySink<super::helloworld::HelloReply>,
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
