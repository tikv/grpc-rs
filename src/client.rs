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
use call::{Call, Method};
use call::client::{CallOption, ClientCStreamReceiver, ClientCStreamSender, ClientDuplexReceiver,
                   ClientDuplexSender, ClientSStreamReceiver, ClientUnaryReceiver};
use channel::Channel;

use error::Result;

/// A generic client for making rpc calls.
pub struct Client {
    channel: Channel,
}

impl Client {
    pub fn new(channel: Channel) -> Client {
        Client { channel: channel }
    }

    /// Create a synchronized unary rpc call.
    pub fn unary_call<P, Q>(&self, method: &Method<P, Q>, req: &P, opt: CallOption) -> Result<Q> {
        let f = self.unary_call_async(method, req, opt)?;
        f.wait()
    }

    /// Create a asynchronized unary rpc call.
    pub fn unary_call_async<P, Q>(
        &self,
        method: &Method<P, Q>,
        req: &P,
        opt: CallOption,
    ) -> Result<ClientUnaryReceiver<Q>> {
        Call::unary_async(&self.channel, method, req, opt)
    }

    /// Create a asynchronized client streaming call.
    ///
    /// Client can send a stream of requests and server responds with a single response.
    pub fn client_streaming<P, Q>(
        &self,
        method: &Method<P, Q>,
        opt: CallOption,
    ) -> Result<(ClientCStreamSender<P>, ClientCStreamReceiver<Q>)> {
        Call::client_streaming(&self.channel, method, opt)
    }

    /// Create a asynchronized server streaming call.
    ///
    /// Client sends on request and server responds with a stream of responses.
    pub fn server_streaming<P, Q>(
        &self,
        method: &Method<P, Q>,
        req: &P,
        opt: CallOption,
    ) -> Result<ClientSStreamReceiver<Q>> {
        Call::server_streaming(&self.channel, method, req, opt)
    }

    /// Create an asynchronized duplex streaming call.
    ///
    /// Client sends a stream of requests and server responds with a stream of responses.
    /// The response stream is completely independent and both side can be sending messages
    /// at the same time.
    pub fn duplex_streaming<P, Q>(
        &self,
        method: &Method<P, Q>,
        opt: CallOption,
    ) -> Result<(ClientDuplexSender<P>, ClientDuplexReceiver<Q>)> {
        Call::duplex_streaming(&self.channel, method, opt)
    }

    /// Spawn the future into current grpc poll thread.
    ///
    /// This can reduce a lot of context switching, but please make
    /// sure there is no heavy work in the future.
    pub fn spawn<F>(&self, f: F)
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        Executor::new(self.channel.cq()).spawn(f)
    }
}
