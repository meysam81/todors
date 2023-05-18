use clap::Parser;
use cli::{handle_local, Cli, Commands};
use errors::TodoErrors;
use grpc::build_server;
use logging::{debug, error, info, trace};
use models::TodoController;
use settings::Settings;
use std::sync::Arc;

mod apidoc;
mod cli;
mod consts;
mod db;
mod entities;
mod errors;
mod grpc;
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
    let logger = Arc::new(logger);

    debug!(logger, "{:?}", settings);

    let conn = db::connect(&settings.db_url, None).await?;
    trace!(logger, "{:?}", conn);

    let todo_controller = TodoController::new(
        conn,
        Some(settings.pagination_limit),
        Some(settings.pagination_hard_limit),
        Some(settings.create_batch_hard_limit),
    );

    let cli = Cli::parse();
    debug!(logger, "{:?}", cli);

    match cli.command {
        Commands::Local(local) => {
            let cli_state = cli::CliState::new(todo_controller);
            handle_local(local, cli_state).await;
        }
        Commands::Serve(cli::Serve::Http(cli::HttpServerAddr { host, port })) => {
            let addr = format!("{}:{}", host, port);
            info!(
                logger,
                "Starting server at {} with {} threads...", &addr, &settings.num_workers
            );
            let web_state = http::AppState::new(todo_controller, logger.clone());
            let r = http::build_server(web_state, addr, settings.num_workers).await;

            match r {
                Ok(_) => debug!(logger, "Server stopped"),
                Err(err) => error!(logger, "Server failed: {:?}", err),
            };
        }
        Commands::Serve(cli::Serve::Grpc(cli::GrpcServerAddr { host, port })) => {
            let state = grpc::AppState::new(todo_controller, logger.clone());

            info!(
                logger,
                "Starting server at {}:{} with {} threads...", &host, &port, &settings.num_workers
            );
            let addr = format!("{}:{}", host, port);
            let r = build_server(settings.num_workers, state)
                .serve(addr.parse().unwrap())
                .await;

            match r {
                Ok(_) => debug!(logger, "Server stopped"),
                Err(err) => error!(logger, "Server failed: {:?}", err),
            };
        }
    };

    Ok(())
}
