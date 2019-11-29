// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::fmt::{self, Debug, Formatter};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use super::Inner;
use crate::call::{BatchContext, MessageReader, RpcStatusCode};
use crate::error::Error;
use crate::task::CqFuture;

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
    inner: Arc<Inner<Option<MessageReader>>>,
    ref_count: AtomicUsize,
}

impl Batch {
    pub fn new(ty: BatchType, inner: Arc<Inner<Option<MessageReader>>>) -> Batch {
        Batch {
            ty,
            ctx: BatchContext::new(),
            inner,
            ref_count: AtomicUsize::new(1),
        }
    }

    pub fn context(&self) -> &BatchContext {
        &self.ctx
    }

    /// Create a future which will be notified after the batch is resolved.
    pub fn cq_future(&self) -> CqFuture<Option<MessageReader>> {
        let mut guard = self.inner.lock();
        guard.reset();
        CqFuture::new(self.inner.clone())
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
                if status.status == RpcStatusCode::OK {
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
            if status.status == RpcStatusCode::OK {
                guard.set_result(Ok(self.ctx.recv_message()))
            } else {
                guard.set_result(Err(Error::RpcFailure(status)))
            }
        };
        task.map(|t| t.notify());
    }

    /// Return `true` means the tag can be reused.
    pub fn resolve(&mut self, success: bool) -> bool {
        match self.ty {
            BatchType::CheckRead => {
                assert!(success);
                self.handle_unary_response();
            }
            BatchType::Finish => {
                self.finish_response(success);
                drop(self.ctx.take_send_message());
                return self.unref_batch();
            }
            BatchType::Read => {
                self.read_one_msg(success);
                return self.unref_batch();
            }
        }
        false
    }

    /// Ref the `Batch` before call `grpc_call_start_batch`.
    pub fn ref_batch(&self) {
        self.ref_count.fetch_add(1, Ordering::Release);
    }

    /// Return `true` means the tag can be reused.
    pub fn unref_batch(&self) -> bool {
        self.ref_count.fetch_sub(1, Ordering::AcqRel) > 1
    }
}

impl Drop for Batch {
    fn drop(&mut self) {
        self.unref_batch();
    }
}

impl Debug for Batch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
