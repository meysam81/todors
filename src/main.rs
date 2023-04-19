//! Command line interface for the todo app
//!
//! todo app with CLI, REST & gRPC interfaces
//!
//! # Usage
//!
//! The usage is as follows:
//!
//! ```bash
//! todors serve grpc -p 50051 -H 127.0.0.1
//!
//! todors serve http -p 8000 -H 127.0.0.1
//!
//! # Both port & host are optional, but ipv6 can also be used
//! todors serve grpc -H ::1
//!
//! todors create "My first todo"
//!
//! todors delete 1
//!
//! todors list
//!
//! todors update 1 "My first todo updated"
//!
//! todors completion bash | sudo tee /etc/bash_completion.d/todors
//! ```
//!
//! # Help
//!
//! ```bash
//! Usage: todors <COMMAND>
//!
//! Commands:
//!   serve       Serve either the gRPC or REST over HTTP server
//!   create      Create a new TODO with a title
//!   delete      Delete a TODO by ID
//!   list        List all TODOs
//!   update      Update a TODO by ID
//!   completion  Generate shell completion
//!   help        Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version
//!```
//!

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
        Commands::Get(cli::Get { id }) => {
            match Todo::get(id, &conn).await {
                Ok(todo) => slog::info!(logger, "{:?}", todo),
                Err(err) => slog::error!(logger, "Failed to get: {:?}", err),
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
