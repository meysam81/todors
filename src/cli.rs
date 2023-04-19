//! Command line interface for the todo app
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
//! todors create "My first todo"
//!
//! todors delete 1
//!
//! todors list
//!
//! todors update 1 "My first todo updated"
//! ```
//! 
//! # Help
//! todo app with CLI, REST & gRPC interfaces
//!
//! ```bash
//! Usage: todors <COMMAND>
//!
//! Commands:
//!   serve   Serve either the gRPC or HTTP server
//!   create  Create a new TODO with a title
//!   delete  Delete a TODO by ID
//!   list    List all TODOs
//!   update  Update a TODO by ID
//!   help    Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help     Print help
//!   -V, --version  Print version
//!```
//! 

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Serve either the gRPC or HTTP server
    #[command(subcommand)]
    Serve(Serve),
    /// Create a new TODO with a title
    Create(Create),
    /// Delete a TODO by ID
    Delete(Delete),
    /// List all TODOs
    List,
    /// Update a TODO by ID
    Update(Update),
}

#[derive(Subcommand, Debug)]
pub enum Serve {
    /// Serve the gRPC server
    Grpc(ServerAddr),
    /// Serve the HTTP server
    Http(ServerAddr),
}

#[derive(Args, Debug)]
pub struct ServerAddr {
    #[arg(short, long, default_value_t = 8080)]
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
    title: String,
}

#[derive(Args, Debug)]
pub struct Delete {
    /// The ID of the TODO to delete
    id: u32,
}

#[derive(Args, Debug)]
pub struct Update {
    /// The ID of the TODO to update
    pub id: u32,
    /// The new title of the TODO
    #[arg(short, long)]
    pub title: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_bare_args() {
        Cli::command().debug_assert()
    }

    #[test]
    fn verify_create_subcommand_works() {
        let args = vec!["todors", "create", "Hello Rust!"];
        let c = Cli::parse_from(args);
        match c.command {
            Commands::Create(Create { title }) => {
                assert_eq!(title, "Hello Rust!");
            }
            _ => panic!("Expected a Create command"),
        }
    }

    #[test]
    fn verify_delete_subcommand_works() {
        let args = vec!["todors", "delete", "1"];
        let c = Cli::parse_from(args);
        match c.command {
            Commands::Delete(Delete { id }) => {
                assert_eq!(id, 1);
            }
            _ => panic!("Expected a Delete command"),
        }
    }

    #[test]
    fn verify_update_subcommand_works() {
        let args = vec!["todors", "update", "1", "--title", "Hello Rust!"];
        let c = Cli::parse_from(args);
        match c.command {
            Commands::Update(Update { id, title }) => {
                assert_eq!(id, 1);
                assert_eq!(title.unwrap(), "Hello Rust!");
            }
            _ => panic!("Expected a Update command"),
        }
    }

    #[test]
    fn verify_update_title_arg_is_optional() {
        let args = vec!["todors", "update", "1"];
        let c = Cli::parse_from(args);
        match c.command {
            Commands::Update(Update { id, title }) => {
                assert_eq!(id, 1);
                assert_eq!(title, None);
            }
            _ => panic!("Expected a Update command"),
        }
    }

    #[test]
    fn verify_cli_delete_errors_with_string_id() {
        let args = vec!["todors", "delete", "foo"];
        let c = Cli::try_parse_from(args);
        assert!(c.is_err());
    }

    #[test]
    fn update_subcommand_errors_with_non_int_id() {
        let args = vec!["todors", "update", "foo", "--title", "Hello Rust!"];
        let c = Cli::try_parse_from(args);
        assert!(c.is_err());
    }
}
