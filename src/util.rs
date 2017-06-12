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


use std::cell::RefCell;

thread_local!(
    /// ID of current poll worker.
    ///
    /// Not using `ThreadId` because it's a nightly-only API (for now).
    static WORKER_ID: RefCell<usize> = RefCell::new(0)
);

/// Set an id to `WORKER_ID`.
pub fn set_worker_id(id: usize) {
    WORKER_ID.with(|tid| {
        *tid.borrow_mut() = id;
    })
}

/// Get the `WORKER_ID`.
pub fn get_worker_id() -> usize {
    WORKER_ID.with(|tid| {
        *tid.borrow()
    })
}

#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn test_worker_id() {
        assert_eq!(super::get_worker_id(), 0);
        super::set_worker_id(1);
        assert_eq!(super::get_worker_id(), 1);

        thread::spawn(|| {
            assert_eq!(super::get_worker_id(), 0);
            super::set_worker_id(2);
            assert_eq!(super::get_worker_id(), 2);
        }).join().unwrap();

        assert_eq!(super::get_worker_id(), 1);
    }
}
