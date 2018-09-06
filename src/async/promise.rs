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

use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

use super::{BatchMessage, Inner};
use call::{BatchContext, RpcStatusCode};
use error::Error;

/// Batch job type.
#[derive(PartialEq, Debug)]
pub enum BatchType {
    /// Finish without reading any message.
    Finish,
    /// Extract one message when finish.
    Read,
    /// Check the rpc code and then extract one message.
    CheckRead,
}

/// A promise used to resolve batch jobs.
pub struct Batch {
    ty: BatchType,
    ctx: BatchContext,
    inner: Arc<Inner<BatchMessage>>,
}

impl Batch {
    pub fn new(ty: BatchType, inner: Arc<Inner<BatchMessage>>) -> Batch {
        Batch {
            ty,
            ctx: BatchContext::new(),
            inner,
        }
    }

    pub fn context(&self) -> &BatchContext {
        &self.ctx
    }

    fn read_one_msg(&mut self, success: bool) {
        let task = {
            let mut guard = self.inner.lock();
            if success {
                guard.set_result(Ok(self.ctx.recv_message()))
            } else {
                // rely on C core to handle the failed read (e.g. deliver approriate
                // statusCode on the clientside).
                guard.set_result(Ok(None))
            }
        };
        task.map(|t| t.notify());
    }

    fn finish_response(&mut self, succeed: bool) {
        let task = {
            let mut guard = self.inner.lock();
            if succeed {
                let status = self.ctx.rpc_status();
                if status.status == RpcStatusCode::Ok {
                    guard.set_result(Ok(None))
                } else {
                    guard.set_result(Err(Error::RpcFailure(status)))
                }
            } else {
                guard.set_result(Err(Error::RemoteStopped))
            }
        };
        task.map(|t| t.notify());
    }

    fn handle_unary_response(&mut self) {
        let task = {
            let mut guard = self.inner.lock();
            let status = self.ctx.rpc_status();
            if status.status == RpcStatusCode::Ok {
                guard.set_result(Ok(self.ctx.recv_message()))
            } else {
                guard.set_result(Err(Error::RpcFailure(status)))
            }
        };
        task.map(|t| t.notify());
    }

    pub fn resolve(mut self, success: bool) {
        match self.ty {
            BatchType::CheckRead => {
                assert!(success);
                self.handle_unary_response();
            }
            BatchType::Finish => {
                self.finish_response(success);
            }
            BatchType::Read => {
                self.read_one_msg(success);
            }
        }
    }
}

impl Debug for Batch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Batch [{:?}]", self.ty)
    }
}

/// A promise used to resolve async shutdown result.
pub struct Shutdown {
    inner: Arc<Inner<()>>,
}

impl Shutdown {
    pub fn new(inner: Arc<Inner<()>>) -> Shutdown {
        Shutdown { inner }
    }

    pub fn resolve(self, success: bool) {
        let task = {
            let mut guard = self.inner.lock();
            if success {
                guard.set_result(Ok(()))
            } else {
                guard.set_result(Err(Error::ShutdownFailed))
            }
        };
        task.map(|t| t.notify());
    }
}
