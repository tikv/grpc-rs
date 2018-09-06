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

use futures::Future;

use async::Executor;
use async::Kicker;
use call::client::{
    CallOption, ClientCStreamReceiver, ClientCStreamSender, ClientDuplexReceiver,
    ClientDuplexSender, ClientSStreamReceiver, ClientUnaryReceiver,
};
use call::{Call, Method};
use channel::Channel;

use error::Result;

/// A generic client for making RPC calls.
pub struct Client {
    channel: Channel,
    // Used to kick its completion queue.
    kicker: Kicker,
}

impl Client {
    /// Initialize a new [`Client`].
    pub fn new(channel: Channel) -> Client {
        let kicker = channel.create_kicker().unwrap();
        Client { channel, kicker }
    }

    /// Create a synchronized unary RPC call.
    pub fn unary_call<Req, Resp>(
        &self,
        method: &Method<Req, Resp>,
        req: &Req,
        opt: CallOption,
    ) -> Result<Resp> {
        let f = self.unary_call_async(method, req, opt)?;
        f.wait()
    }

    /// Create an asynchronized unary RPC call.
    pub fn unary_call_async<Req, Resp>(
        &self,
        method: &Method<Req, Resp>,
        req: &Req,
        opt: CallOption,
    ) -> Result<ClientUnaryReceiver<Resp>> {
        Call::unary_async(&self.channel, method, req, opt)
    }

    /// Create an asynchronized client streaming call.
    ///
    /// Client can send a stream of requests and server responds with a single response.
    pub fn client_streaming<Req, Resp>(
        &self,
        method: &Method<Req, Resp>,
        opt: CallOption,
    ) -> Result<(ClientCStreamSender<Req>, ClientCStreamReceiver<Resp>)> {
        Call::client_streaming(&self.channel, method, opt)
    }

    /// Create an asynchronized server streaming call.
    ///
    /// Client sends on request and server responds with a stream of responses.
    pub fn server_streaming<Req, Resp>(
        &self,
        method: &Method<Req, Resp>,
        req: &Req,
        opt: CallOption,
    ) -> Result<ClientSStreamReceiver<Resp>> {
        Call::server_streaming(&self.channel, method, req, opt)
    }

    /// Create an asynchronized duplex streaming call.
    ///
    /// Client sends a stream of requests and server responds with a stream of responses.
    /// The response stream is completely independent and both side can be sending messages
    /// at the same time.
    pub fn duplex_streaming<Req, Resp>(
        &self,
        method: &Method<Req, Resp>,
        opt: CallOption,
    ) -> Result<(ClientDuplexSender<Req>, ClientDuplexReceiver<Resp>)> {
        Call::duplex_streaming(&self.channel, method, opt)
    }

    /// Spawn the future into current gRPC poll thread.
    ///
    /// This can reduce a lot of context switching, but please make
    /// sure there is no heavy work in the future.
    pub fn spawn<F>(&self, f: F)
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        let kicker = self.kicker.clone();
        Executor::new(self.channel.cq()).spawn(f, kicker)
    }
}
