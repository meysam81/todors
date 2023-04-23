use clap::Parser;
use cli::{handle_local, Cli, Commands};
use errors::TodoErrors;
use grpc::{Channel, HealthCheckServer, Server as GrpcServer, TodoHealthCheck};
use logging::{debug, error, info, trace, warn};
use models::TodoController;
use settings::Settings;

mod cli;
mod db;
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

    debug!(logger, "{:?}", settings);

    let conn = db::connect(&settings.db_url, None).await?;
    trace!(logger, "{:?}", conn);

    let todo_controller = TodoController::new(conn);

    let cli = Cli::parse();
    debug!(logger, "{:?}", cli);

    match cli.command {
        Commands::Local(local) => {
            let cli_state = cli::CliState::new(todo_controller, logger);
            handle_local(local, cli_state).await;
        }
        Commands::Serve(cli::Serve::Http(cli::HttpServerAddr { host, port })) => {
            info!(
                logger,
                "Starting server at {}:{} with {} threads...", &host, &port, &settings.num_workers
            );
            let web_state = http::AppState::new(todo_controller, logger.clone());
            let addr = format!("{}:{}", host, port);
            let r = http::HttpServer::new(move || {
                http::App::new()
                    .app_data(web_state.clone())
                    .configure(http::configure::<TodoController>)
            })
            .workers(settings.num_workers)
            .bind(addr)?
            .run()
            .await;

            match r {
                Ok(_) => info!(logger, "Server stopped"),
                Err(err) => error!(logger, "Server failed: {:?}", err),
            };
        }
        Commands::Serve(cli::Serve::Grpc(cli::GrpcServerAddr { host, port })) => {
            info!(
                logger,
                "Starting server at {}:{} with {} threads...", &host, &port, &settings.num_workers
            );
            let addr = format!("{}:{}", host, port);
            let todo_greeter = TodoHealthCheck::default();
            let r = GrpcServer::builder()
                .concurrency_limit_per_connection(settings.num_workers)
                .add_service(HealthCheckServer::new(todo_greeter))
                .serve(addr.parse().unwrap())
                .await;

            match r {
                Ok(_) => info!(logger, "Server stopped"),
                Err(err) => error!(logger, "Server failed: {:?}", err),
            };
        }
    };

    Ok(())
}
