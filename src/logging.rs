use slog::Drain;
pub use slog::Logger;
pub use slog::{debug, error, info, trace, warn};

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
    match log_level.to_lowercase().as_str() {
        "debug" => slog::Level::Debug,
        "trace" => slog::Level::Trace,
        "info" => slog::Level::Info,
        "warn" => slog::Level::Warning,
        "error" => slog::Level::Error,
        _ => slog::Level::Info,
    }
}
