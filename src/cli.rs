use crate::errors::TodoErrors;
use crate::logging::{error, info, Logger};
use crate::models::{TodoRead, TodoUpdate, TodoWrite};
use crate::serializers::{to_json, to_pretty_json};
use crate::traits::{Controller, ListRequest};
use clap::{Args, Command, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io;

pub struct CliState<T>
where
    T: Controller,
{
    controller: T,
    logger: Logger,
}

impl<T> CliState<T>
where
    T: Controller,
{
    pub fn new(controller: T, logger: Logger) -> Self {
        Self { controller, logger }
    }
}

pub async fn handle_local<T>(local: Local, state: CliState<T>)
where
    T: Controller<Input = TodoWrite, Output = TodoRead, Id = u32, OptionalInput = TodoUpdate>,
{
    match local {
        Local::Create(Create { title, done }) => {
            let todos = title
                .into_iter()
                .map(|title| TodoWrite::new(title, done))
                .collect::<Vec<_>>();

            match state.controller.create_batch(todos).await {
                Ok(todo) => {
                    let todo = to_json(&todo).unwrap();
                    println!("Inserted ids: {}", todo);
                }
                Err(TodoErrors::BatchTooLarge { max_size }) => {
                    error!(
                        state.logger,
                        "Batch too large, max batch size is {}", max_size
                    );
                }
                Err(err) => {
                    error!(state.logger, "Failed to create todo: {:?}", err);
                }
            }
        }
        Local::Delete(Delete { id }) => match state.controller.delete(id).await {
            Ok(_) => info!(state.logger, "Successfully deleted: {}", id),
            Err(err) => {
                error!(state.logger, "Failed to delete todo: {:?}", err);
            }
        },
        Local::Get(Get { id, pretty }) => match state.controller.get(id).await {
            Ok(todo) => {
                let printer = if pretty { to_pretty_json } else { to_json };
                let todo = printer(&todo).unwrap();
                println!("{}", todo)
            }
            Err(err) => {
                error!(state.logger, "Failed to get todo: {:?}", err);
            }
        },
        Local::List(List {
            offset,
            limit,
            pretty,
        }) => match state.controller.list(ListRequest { offset, limit }).await {
            Ok(todos) => {
                let printer = if pretty { to_pretty_json } else { to_json };
                let todos = printer(&todos).unwrap();
                println!("{}", todos)
            }
            Err(err) => {
                error!(state.logger, "Failed to list todos: {:?}", err);
            }
        },
        Local::Update(Update {
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
            let todo = TodoUpdate::new(title, done);
            match state.controller.update(id, todo).await {
                Ok(_) => info!(state.logger, "Successfully updated: {}", id),
                Err(err) => {
                    error!(state.logger, "Failed to update todo: {:?}", err);
                }
            }
        }
        Local::Completion(Completion { shell }) => {
            print_completions(shell);
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Serve either the gRPC or REST over HTTP server
    #[command(subcommand)]
    Serve(Serve),
    #[command(flatten)]
    Local(Local),
}

#[derive(Subcommand, Debug)]
pub enum Local {
    /// Create a new TODO with a title
    Create(Create),
    /// Delete a TODO by ID
    Delete(Delete),
    /// List all TODOs
    List(List),
    /// Update a TODO by ID
    Update(Update),
    /// Get a TODO by ID
    Get(Get),
    /// Generate shell completion
    Completion(Completion),
}

#[derive(Subcommand, Debug)]
pub enum Serve {
    /// Serve gRPC over HTTP server
    Grpc(GrpcServerAddr),
    /// Serve REST over HTTP server
    Http(HttpServerAddr),
}

#[derive(Args, Debug)]
pub struct HttpServerAddr {
    #[arg(short, long, default_value_t = 8080)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub port: u16,
    #[arg(short = 'H')]
    #[arg(long, default_value = "127.0.0.1")]
    #[arg(value_parser = clap::value_parser!(std::net::IpAddr))]
    pub host: std::net::IpAddr,
}

#[derive(Args, Debug)]
pub struct GrpcServerAddr {
    #[arg(short, long, default_value_t = 50051)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub port: u16,
    #[arg(short = 'H')]
    #[arg(long, default_value = "127.0.0.1")]
    #[arg(value_parser = clap::value_parser!(std::net::IpAddr))]
    pub host: std::net::IpAddr,
}

#[derive(Args, Debug)]
pub struct Create {
    /// The title of the TODO
    #[arg(action = clap::ArgAction::Append)]
    pub title: Vec<String>,
    /// Whether or not the provided TODOs are done
    #[arg(short, long)]
    #[arg(default_value = "false")]
    #[arg(action = clap::ArgAction::SetTrue)]
    pub done: Option<bool>,
}

#[derive(Args, Debug)]
pub struct Delete {
    /// The ID of the TODO to delete
    pub id: u32,
}

#[derive(Args, Debug)]
pub struct List {
    /// Start of the query
    #[arg(short, long)]
    pub offset: Option<u32>,
    /// Number of results to return
    #[arg(short, long)]
    pub limit: Option<u32>,
    /// Whether or not to print indented JSON
    #[arg(short, long)]
    #[arg(action = clap::ArgAction::SetTrue)]
    pub pretty: bool,
}

#[derive(Args, Debug)]
pub struct Update {
    /// The ID of the TODO to update
    pub id: u32,
    /// The new title of the TODO
    #[arg(short, long)]
    pub title: Option<String>,
    #[arg(short, long)]
    #[arg(group = "finished")]
    #[arg(action = clap::ArgAction::SetTrue)]
    pub done: Option<bool>,
    #[arg(short, long)]
    #[arg(group = "finished")]
    #[arg(action = clap::ArgAction::SetTrue)]
    pub undone: Option<bool>,
}

#[derive(Args, Debug)]
pub struct Get {
    /// The ID of the TODO to get
    pub id: u32,
    /// Whether or not to print indented JSON
    #[arg(short, long)]
    #[arg(action = clap::ArgAction::SetTrue)]
    pub pretty: bool,
}

#[derive(Args, Debug)]
pub struct Completion {
    /// Generate completion scripts for your shell
    pub shell: Shell,
}

fn generate_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn print_completions(shell: Shell) {
    use clap::CommandFactory;
    let mut cmd = Cli::command();
    generate_completions(shell, &mut cmd);
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::CommandFactory;

    #[tokio::test]
    async fn verify_cli_bare_args() {
        Cli::command().debug_assert()
    }

    #[tokio::test]
    async fn verify_create_subcommand_works() {
        let args = vec!["todors", "create", "Hello Rust!"];
        let c = Cli::parse_from(args);
        // let done: Option<bool> = false;
        match c.command {
            Commands::Local(Local::Create(Create {
                title,
                done: Some(false),
            })) => {
                assert_eq!(title[0], "Hello Rust!");
            }
            _ => panic!("Expected a Create command"),
        }
    }

    #[tokio::test]
    async fn verify_delete_subcommand_works() {
        let args = vec!["todors", "delete", "1"];
        let c = Cli::parse_from(args);
        match c.command {
            Commands::Local(Local::Delete(Delete { id })) => {
                assert_eq!(id, 1);
            }
            _ => panic!("Expected a Delete command"),
        }
    }

    #[tokio::test]
    async fn verify_update_subcommand_works() {
        let args = vec!["todors", "update", "1", "--title", "Hello Rust!"];
        let c = Cli::parse_from(args);
        match c.command {
            Commands::Local(Local::Update(Update { id, title, .. })) => {
                assert_eq!(id, 1);
                assert_eq!(title.unwrap(), "Hello Rust!");
            }
            _ => panic!("Expected a Update command"),
        }
    }

    #[tokio::test]
    async fn verify_update_title_arg_is_optional() {
        let args = vec!["todors", "update", "1"];
        let c = Cli::parse_from(args);
        match c.command {
            Commands::Local(Local::Update(Update { id, title, .. })) => {
                assert_eq!(id, 1);
                assert_eq!(title, None);
            }
            _ => panic!("Expected a Update command"),
        }
    }

    #[tokio::test]
    async fn verify_cli_delete_errors_with_string_id() {
        let args = vec!["todors", "delete", "foo"];
        let c = Cli::try_parse_from(args);
        assert!(c.is_err());
    }

    #[tokio::test]
    async fn update_subcommand_errors_with_non_int_id() {
        let args = vec!["todors", "update", "foo", "--title", "Hello Rust!"];
        let c = Cli::try_parse_from(args);
        assert!(c.is_err());
    }
}
