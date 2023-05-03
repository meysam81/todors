use std::sync::Arc;

use tonic::transport::{server::Router, Server};
use tonic::{Request, Response, Status};

use proto::healthcheck::health_check_server::{HealthCheck, HealthCheckServer};
use proto::healthcheck::{Ping, Pong};

use proto::todo::todo_server::{Todo, TodoServer};
use proto::todo::{ListTodosRequest, ListTodosResponse, TodoRead};

use crate::logging::{error, info, Logger};
use crate::{entities, models};

use self::logging::Log;
use crate::traits::Controller;

pub fn build_server<T>(num_workers: usize, state: AppState<T>) -> Router
where
    T: Controller<
        Input = models::TodoWrite,
        Output = models::TodoRead,
        Id = models::Id,
        OptionalInput = models::TodoUpdate,
    >,
{
    Server::builder()
        .concurrency_limit_per_connection(num_workers)
        .add_service(HealthCheckServer::new(TodoHealthCheck::default()))
        .add_service(TodoServer::new(TodoService::new(state)))
}

mod logging;

mod proto {
    pub mod healthcheck {
        tonic::include_proto!("healthcheck");
    }

    pub mod todo {
        tonic::include_proto!("todo");
    }
}

impl<T> TodoService<T>
where
    T: Controller<
        Input = models::TodoWrite,
        Output = models::TodoRead,
        Id = models::Id,
        OptionalInput = models::TodoUpdate,
    >,
{
    pub fn new(state: AppState<T>) -> Self {
        Self { state }
    }
}

pub struct AppState<T>
where
    T: Controller,
{
    controller: T,
    logger: Arc<Logger>,
}

impl<T> AppState<T>
where
    T: Controller,
{
    pub fn new(controller: T, logger: Arc<Logger>) -> Self {
        Self { controller, logger }
    }
}

#[derive(Debug, Default)]
struct TodoHealthCheck {}

#[tonic::async_trait]
impl HealthCheck for TodoHealthCheck {
    async fn check(&self, request: Request<Ping>) -> Result<Response<Pong>, Status> {
        println!("Got a request: {:?}", request);

        let reply = Pong {
            message: "pong".to_string(),
        };

        Ok(Response::new(reply))
    }
}

struct TodoService<T>
where
    T: Controller<
        Input = models::TodoWrite,
        Output = models::TodoRead,
        Id = models::Id,
        OptionalInput = models::TodoUpdate,
    >,
{
    state: AppState<T>,
}

#[tonic::async_trait]
impl<T> Todo for TodoService<T>
where
    T: Controller<
        Input = models::TodoWrite,
        Output = models::TodoRead,
        Id = models::Id,
        OptionalInput = models::TodoUpdate,
    >,
{
    async fn create(
        &self,
        _request: Request<proto::todo::CreateTodoRequest>,
    ) -> Result<Response<proto::todo::TodoRead>, Status> {
        todo!()
    }

    async fn delete(
        &self,
        _request: Request<proto::todo::DeleteTodoRequest>,
    ) -> Result<Response<proto::todo::Confirmation>, Status> {
        todo!()
    }

    async fn get(
        &self,
        request: Request<proto::todo::GetTodoRequest>,
    ) -> Result<Response<proto::todo::TodoRead>, Status> {
        let mut log = Log {
            rpc: "todo.Todo/Get".to_string(),
            ..Default::default()
        };

        let request = request.into_inner();

        let start = std::time::Instant::now();
        let res = self.state.controller.get(request.id).await;
        let elapsed = start.elapsed();

        log.latency = format!("{:?}", elapsed);

        info!(&self.state.logger, "{}", log);

        match res {
            Ok(todo) => {
                let reply = proto::todo::TodoRead {
                    id: todo.id,
                    title: todo.title,
                    done: todo.done,
                };

                Ok(Response::new(reply))
            }
            Err(err) => {
                error! {&self.state.logger, "Error: {}", err};
                Err(Status::internal(err.to_string()))
            }
        }
    }

    async fn list(
        &self,
        request: Request<ListTodosRequest>,
    ) -> Result<Response<ListTodosResponse>, Status> {
        let mut log = Log {
            rpc: "todo.Todo/List".to_string(),
            ..Default::default()
        };

        let request = request.into_inner();

        let request = entities::ListRequest {
            offset: request.limit,
            limit: request.offset,
        };

        let start = std::time::Instant::now();
        let res = self.state.controller.list(request).await;
        let elapsed = start.elapsed();

        log.latency = format!("{:?}", elapsed);

        info!(&self.state.logger, "{}", log);

        match res {
            Ok(entities::ListResponse {
                data,
                offset,
                limit,
                total,
            }) => {
                let data = data
                    .into_iter()
                    .map(|todo| TodoRead {
                        id: todo.id,
                        title: todo.title,
                        done: todo.done,
                    })
                    .collect();

                let reply = ListTodosResponse {
                    data,
                    limit,
                    offset,
                    total,
                };

                Ok(Response::new(reply))
            }
            Err(err) => {
                error! {&self.state.logger, "Error: {}", err};
                Err(Status::internal(err.to_string()))
            }
        }
    }

    async fn update(
        &self,
        _request: Request<proto::todo::UpdateTodoRequest>,
    ) -> Result<Response<proto::todo::Confirmation>, Status> {
        todo!()
    }
}
