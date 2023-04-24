use tonic::transport::{server::Router, Server};
use tonic::{Request, Response, Status};

pub use proto::healthcheck::health_check_server::{HealthCheck, HealthCheckServer};
use proto::healthcheck::{Ping, Pong};

pub use proto::todo::todo_server::{Todo, TodoServer};
use proto::todo::{ListTodosRequest, ListTodosResponse, TodoRead};

mod proto {
    pub mod healthcheck {
        tonic::include_proto!("healthcheck");
    }

    pub mod todo {
        tonic::include_proto!("todo");
    }
}

pub fn build_server(num_workers: usize) -> Router {
    Server::builder()
        .concurrency_limit_per_connection(num_workers)
        .add_service(HealthCheckServer::new(TodoHealthCheck::default()))
        .add_service(TodoServer::new(TodoService::default()))
}

#[derive(Debug, Default)]
pub struct TodoHealthCheck {}

#[tonic::async_trait]
impl HealthCheck for TodoHealthCheck {
    async fn check(&self, request: Request<Ping>) -> Result<Response<Pong>, Status> {
        println!("Got a request: {:?}", request);

        let reply = proto::healthcheck::Pong {
            message: "pong".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
pub struct TodoService {}

#[tonic::async_trait]
impl Todo for TodoService {
    async fn list_todos(
        &self,
        request: Request<ListTodosRequest>,
    ) -> Result<Response<ListTodosResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ListTodosResponse {
            todos: vec![TodoRead {
                id: 1,
                title: "todo 1".to_string(),
                done: true,
            }],
        };

        Ok(Response::new(reply))
    }
}
