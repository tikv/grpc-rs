
use call::{Call, Method};
use call::client::{CallOption, ClientStreamingCallHandler, DuplexCallHandler,
                   ServerStreamingCallHandler, UnaryCallHandler};
use channel::Channel;

use error::Result;
use futures::Future;

pub struct Client {
    channel: Channel,
}

impl Client {
    pub fn new(channel: Channel) -> Client {
        Client { channel: channel }
    }

    pub fn unary_call<P, Q>(&self, method: &Method<P, Q>, req: P, opt: CallOption) -> Result<Q> {
        let f = self.unary_call_async(method, req, opt);
        f.wait()
    }

    pub fn unary_call_async<P, Q>(&self,
                                  method: &Method<P, Q>,
                                  req: P,
                                  opt: CallOption)
                                  -> UnaryCallHandler<Q> {
        Call::unary_async(&self.channel, method, req, opt)
    }

    pub fn client_streaming<P, Q>(&self,
                                  method: &Method<P, Q>,
                                  opt: CallOption)
                                  -> ClientStreamingCallHandler<P, Q> {
        Call::client_streaming(&self.channel, method, opt)
    }

    pub fn server_streaming<P, Q>(&self,
                                  method: &Method<P, Q>,
                                  req: P,
                                  opt: CallOption)
                                  -> ServerStreamingCallHandler<Q> {
        Call::server_streaming(&self.channel, method, req, opt)
    }

    pub fn duplex_streaming<P, Q>(&self,
                                  method: &Method<P, Q>,
                                  opt: CallOption)
                                  -> DuplexCallHandler<P, Q> {
        Call::duplex_streaming(&self.channel, method, opt)
    }
}
