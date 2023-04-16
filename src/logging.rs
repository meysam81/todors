use slog::{Drain, Logger};

pub fn init(log_level: &str) -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let level = get_log_level(log_level);
    let drain = slog::LevelFilter::new(drain, level).fuse();
    let logger = Logger::root(drain.fuse(), slog::o!());
    logger
}

fn get_log_level(log_level: &str) -> slog::Level {
    match log_level {
        "debug" => slog::Level::Debug,
        "info" => slog::Level::Info,
        "warn" => slog::Level::Warning,
        "error" => slog::Level::Error,
        _ => slog::Level::Debug,
    }
}
