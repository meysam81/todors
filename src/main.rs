use clap::Parser;
use cli::{Cli, Commands};
use errors::TodoErrors;
use logging::{debug, error, info, trace, warn};
use models::{Todo, TodoController};
use serializers::to_json;
use settings::Settings;
use std::sync::Arc;
use tokio::sync::Mutex;

mod cli;
mod db;
mod errors;
mod http;
mod logging;
mod models;
mod serializers;
mod settings;
mod traits;

#[tokio::main]
async fn main() -> Result<(), TodoErrors> {
    let settings = Settings::new().unwrap();
    let logger = logging::init(settings.log_level.as_str());

    debug!(logger, "{:?}", settings);

    let conn = db::connect(&settings.db_url, None).await?;
    // let conn = Arc::new(Mutex::new(conn));
    trace!(logger, "{:?}", conn);

    let todo_controller = TodoController::new(conn);
    let web_state = http::AppState::new(todo_controller, logger.clone());

    let cli = Cli::parse();
    debug!(logger, "{:?}", cli);

    match cli.command {
        // Commands::List => {
        //     let todos = Todo::list(&conn).await?;
        //     let todos = to_json(&todos)?;
        //     println!("{}", todos);
        // }
        // Commands::Create(cli::Create { title }) => {
        //     let mut todo = Todo::new(title);
        //     match todo.save(&conn).await {
        //         Ok(_) => {
        //             let todo = to_json(&todo)?;
        //             println!("{}", todo)
        //         }
        //         Err(err) => error!(logger, "Failed to save: {:?}", err),
        //     };
        // }
        // Commands::Delete(cli::Delete { id }) => {
        //     match Todo::delete(id, &conn).await {
        //         Ok(_) => info!(logger, "Deleted successfully!"),
        //         Err(err) => error!(logger, "Failed to delete: {:?}", err),
        //     };
        // }
        // Commands::Update(cli::Update {
        //     id,
        //     title,
        //     done,
        //     undone,
        // }) => {
        //     let done = if let Some(undone) = undone {
        //         Some(!undone)
        //     } else {
        //         done
        //     };
        //     match Todo::update(id, title, done, &conn).await {
        //         Ok(_) => info!(logger, "Updated successfully!"),
        //         Err(err) => error!(logger, "Failed to update: {:?}", err),
        //     };
        // }
        // Commands::Get(cli::Get { id }) => {
        //     match Todo::get(id, &conn).await {
        //         Ok(todo) => {
        //             let todo = to_json(&todo)?;
        //             println!("{}", todo)
        //         }
        //         Err(err) => error!(logger, "Failed to get: {:?}", err),
        //     };
        // }
        Commands::Completion(cli::Completion { shell }) => {
            cli::print_completions(shell);
        }
        Commands::Serve(cli::Serve::Http(cli::ServerAddr { host, port })) => {
            info!(logger, "Starting server at {}:{}", host, port);
            let addr = format!("{}:{}", host, port);
            http::HttpServer::new(move || {
                http::App::new()
                    .app_data(web_state.clone())
                    .configure(http::configure::<TodoController>)
            })
            .bind(format!("{}:{}", host, port))?
            .run()
            .await?;
        }
        _ => {
            warn!(logger, "Not implemented yet");
        }
    };

    Ok(())
}
