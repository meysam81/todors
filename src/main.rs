use clap::Parser;
use cli::{Cli, Commands};
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
        Commands::List => {
            let todos = Todo::list(&conn).await?;
            slog::info!(logger, "{:?}", todos);
        }
        Commands::Create(cli::Create { title }) => {
            let mut todo = Todo::new(title);
            match todo.save(&conn).await {
                Ok(_) => slog::debug!(logger, "{:?}", todo),
                Err(err) => slog::error!(logger, "Failed to save: {:?}", err),
            };
        }
        Commands::Delete(cli::Delete { id }) => {
            match Todo::delete(id, &conn).await {
                Ok(_) => slog::info!(logger, "Deleted successfully!"),
                Err(err) => slog::error!(logger, "Failed to delete: {:?}", err),
            };
        }
        Commands::Update(cli::Update {
            id,
            title,
            done,
            undone,
        }) => {
            let done = if let Some(undone) = undone {
                Some(!undone)
            } else {
                done
            };
            match Todo::update(id, title, done, &conn).await {
                Ok(_) => slog::info!(logger, "Updated successfully!"),
                Err(err) => slog::error!(logger, "Failed to update: {:?}", err),
            };
        }
        Commands::Completion(cli::Completion { shell }) => {
            cli::print_completions(shell);
        }
        _ => {
            slog::error!(logger, "Not implemented yet");
        }
    };

    Ok(())
}
