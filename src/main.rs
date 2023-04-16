use models::Todo;
use settings::Settings;

mod db;
mod logging;
mod models;
mod settings;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let settings = Settings::new().unwrap();

    let logger = logging::init(settings.log_level.as_str());
    slog::info!(logger, "Hello from Todors!");
    slog::debug!(logger, "{:?}", settings);

    let conn = db::connect(&settings.db_url, None).await?;
    slog::debug!(logger, "{:?}", conn);

    let mut todo = Todo::new("Hello Rust".to_string());
    todo.done();
    slog::debug!(logger, "{:?}", todo);

    match todo.save(&conn).await {
        Ok(_) => slog::debug!(logger, "Todo saved!"),
        Err(e) => slog::error!(logger, "Error saving todo: {}", e),
    }

    Ok(())
}
