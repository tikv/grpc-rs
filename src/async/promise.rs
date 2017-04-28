use call::BatchContext;
use error::Error;
use grpc_sys::GrpcStatusCode;
use std::sync::Arc;
use super::Inner;

#[derive(PartialEq, Debug)]
pub enum BatchType {
    Finish,
    ReadOne,
    FinishUnary,
}

pub struct Batch {
    ty: BatchType,
    ctx: BatchContext,
    inner: Arc<Inner<Vec<u8>>>,
}

impl Batch {
    pub fn new(ty: BatchType, inner: Arc<Inner<Vec<u8>>>) -> Batch {
        Batch {
            ty: ty,
            ctx: BatchContext::new(),
            inner: inner,
        }
    }

    pub fn context(&self) -> &BatchContext {
        &self.ctx
    }

    fn read_one_msg(&mut self) {
        let mut guard = self.inner.lock();
        guard.result(Ok(self.ctx.recv_message()));
    }

    fn finish_response(&mut self, succeed: bool) {
        let mut guard = self.inner.lock();
        let status = self.ctx.rpc_status();
        if status.status != GrpcStatusCode::Ok || !succeed {
            guard.result(Err(Error::RpcFailure(status)));
            return;
        }

        guard.result(Ok(vec![]))
    }

    fn handle_unary_response(&mut self) {
        let mut guard = self.inner.lock();
        let status = self.ctx.rpc_status();
        if status.status != GrpcStatusCode::Ok {
            guard.result(Err(Error::RpcFailure(status)));
            return;
        }

        guard.result(Ok(self.ctx.recv_message()))
    }

    pub fn resolve(mut self, success: bool) {
        match self.ty {
            BatchType::FinishUnary => {
                assert!(success);
                self.handle_unary_response();
            }
            BatchType::Finish => {
                self.finish_response(success);
            }
            BatchType::ReadOne => {
                assert!(success);
                self.read_one_msg();
            }
        }
    }
}

pub struct Shutdown {
    inner: Arc<Inner<()>>,
}

impl Shutdown {
    pub fn new(inner: Arc<Inner<()>>) -> Shutdown {
        Shutdown { inner: inner }
    }

    pub fn resolve(self, _: bool) {
        let mut guard = self.inner.lock();
        guard.result(Ok(()))
    }
}
