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
        // It's safe to use swap here. If previous is false, then the lock
        // is taken, loop will break, set it to true is expected;
        // If previous is true, then the loop will go on until others swap
        // back a false, set it to true changes nothing.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::*;
    use std::sync::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_lock() {
        let lock1 = Arc::new(SpinLock::new(2));
        let lock2 = lock1.clone();
        let (tx, rx) = mpsc::channel();
        let guard = lock1.lock();
        thread::spawn(move || {
            let _guard = lock2.lock();
            tx.send(()).unwrap();
        });
        thread::sleep(Duration::from_millis(10));
        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
        drop(guard);
        assert_eq!(rx.recv(), Ok(()));
    }
}
