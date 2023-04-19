use clap::Parser;
use cli::Cli;
use models::Todo;
use settings::Settings;

pub mod cli;
mod db;
mod logging;
mod models;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new().unwrap();
    let logger = logging::init(settings.log_level.as_str());

    slog::debug!(logger, "{:?}", settings);

    let conn = db::connect(&settings.db_url, None).await?;
    slog::debug!(logger, "{:?}", conn);

    let cli = Cli::parse();
    slog::debug!(logger, "{:?}", cli);

    match cli.command {
        cli::Commands::List => {
            let todos = Todo::list(&conn).await?;
            slog::debug!(logger, "{:?}", todos);
        }
        _ => {
            slog::error!(logger, "Not implemented yet");
        }
    };

    Ok(())
}
