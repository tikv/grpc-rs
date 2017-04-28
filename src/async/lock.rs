use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

/// A simple spin lock for synchronization between Promise
/// and future.
pub struct SpinLock<T> {
    handle: UnsafeCell<T>,
    lock: AtomicBool,
}

// It's a lock, as long as the content can be sent between
// threads, it's Sync and Send.
unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    /// Create a lock with the given value.
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

/// A guard for `SpinLock`.
pub struct LockGuard<'a, T: 'a> {
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
