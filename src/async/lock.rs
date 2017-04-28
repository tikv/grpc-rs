

use call::BatchContext;
use call::server::{RequestContext, UnaryRequestContext};
use cq::CompletionQueue;
use error::{Error, Result};
use futures::{Async, Poll};

use futures::task::{self, Task};
use grpc_sys::GrpcStatusCode;
use protobuf::{self, MessageStatic};
use server::{self, Inner as ServerInner};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    handle: UnsafeCell<T>,
    lock: AtomicBool,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(t: T) -> SpinLock<T> {
        SpinLock {
            handle: UnsafeCell::new(t),
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> LockGuard<T> {
        // TODO: what if poison?
        while self.lock.swap(true, Ordering::SeqCst) {}
        LockGuard { inner: self }
    }
}

struct LockGuard<'a, T: 'a> {
    inner: &'a SpinLock<T>,
}

impl<'a, T> Deref for LockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.inner.handle.get() }
    }
}

impl<'a, T> DerefMut for LockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner.handle.get() }
    }
}

impl<'a, T> Drop for LockGuard<'a, T> {
    fn drop(&mut self) {
        self.inner.lock.swap(false, Ordering::SeqCst);
    }
}
