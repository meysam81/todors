use std::sync::Arc;

use tonic::transport::{server::Router, Server};
use tonic::{Request, Response, Status};

use proto::healthcheck::health_check_server::{HealthCheck, HealthCheckServer};
use proto::healthcheck::{Ping, Pong};

use proto::todo::todo_server::{Todo, TodoServer};
use proto::todo::{ListTodosRequest, ListTodosResponse, TodoRead};

use crate::logging::{error, info, Logger};
use crate::{entities, models};

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
        request: Request<proto::todo::TodoWrite>,
    ) -> Result<Response<proto::todo::TodoRead>, Status> {
        println!("Got a request: {:?}", request);

        let reply = proto::todo::TodoRead {
            id: 1,
            title: "todo 1".to_string(),
            done: true,
        };

        Ok(Response::new(reply))
    }

    async fn delete(
        &self,
        request: Request<proto::todo::TodoId>,
    ) -> Result<Response<proto::todo::Confirmation>, Status> {
        println!("Got a request: {:?}", request);

        let reply = proto::todo::Confirmation {
            status: proto::todo::Status::Ok.into(),
        };

        Ok(Response::new(reply))
    }

    async fn get(
        &self,
        request: Request<proto::todo::TodoId>,
    ) -> Result<Response<proto::todo::TodoRead>, Status> {
        println!("Got a request: {:?}", request);

        let reply = proto::todo::TodoRead {
            id: 1,
            title: "todo 1".to_string(),
            done: true,
        };

        Ok(Response::new(reply))
    }

    async fn list(
        &self,
        request: Request<ListTodosRequest>,
    ) -> Result<Response<ListTodosResponse>, Status> {
        info!(&self.state.logger, "Got a request: {:?}", request);

        let request = request.into_inner();

        let request = entities::ListRequest {
            offset: request.limit,
            limit: request.offset,
        };

        let res = self.state.controller.list(request).await;

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
        request: Request<proto::todo::TodoUpdate>,
    ) -> Result<Response<proto::todo::Confirmation>, Status> {
        println!("Got a request: {:?}", request);

        let reply = proto::todo::Confirmation {
            status: proto::todo::Status::Ok.into(),
        };

        Ok(Response::new(reply))
    }
}
