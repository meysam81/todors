use slog::{Drain, Logger};

pub fn init() -> Logger {
    let drain = slog_term::FullFormat::new(slog_term::TermDecorator::new().build())
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, slog::o!());
    log
}
