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


use call::{BatchContext, Call};

/// A callback to wait for status for the aborted rpc call to be sent.
pub struct Abort {
    ctx: BatchContext,
    _call: Call,
}

impl Abort {
    pub fn new(call: Call) -> Abort {
        Abort {
            ctx: BatchContext::new(),
            _call: call,
        }
    }

    pub fn batch_ctx(&self) -> &BatchContext {
        &self.ctx
    }
}
