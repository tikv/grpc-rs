
use slog_async;
use slog::{Drain, Logger};
use slog_scope::{self, GlobalLoggerGuard};
use slog_stdlog;
use slog_term::{FullFormat, TermDecorator};

pub fn init_log() -> GlobalLoggerGuard {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, slog_o!());

    let guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init().unwrap();
    guard
}
