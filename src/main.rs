use models::Todo;
use settings::Settings;

mod db;
mod logging;
mod models;
mod settings;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let settings = Settings::new().unwrap();

    let logger = logging::init();
    slog::info!(logger, "Hello from Todors!");
    slog::debug!(logger, "{:?}", settings);

    let conn = db::connect(&settings.db_url, None).await?;
    slog::debug!(logger, "{:?}", conn);

    let mut todo = Todo::new("Hello Rust".to_string());
    todo.done();
    slog::debug!(logger, "{:?}", todo);

    // save to db
    todo.save(&conn).await?;

    Ok(())
}
