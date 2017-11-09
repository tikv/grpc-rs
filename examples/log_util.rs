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


extern crate slog;
extern crate slog_async;
extern crate slog_stdlog;
extern crate slog_scope;
extern crate slog_term;

use self::slog::{Drain, Logger, OwnedKV};
use self::slog_scope::GlobalLoggerGuard;
use self::slog_term::{FullFormat, TermDecorator};

pub fn init_log() -> GlobalLoggerGuard {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, OwnedKV(()));

    let guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    guard
}
