use std::sync::Arc;
use std::time::Instant;

use tonic::transport::{server::Router, Server};
use tonic::{Request, Response, Status};

use proto::healthcheck::health_check_server::{HealthCheck, HealthCheckServer};
use proto::healthcheck::{Ping, Pong};

use proto::todo::todo_server::{Todo, TodoServer};
use proto::todo::{ListTodosRequest, ListTodosResponse};

use crate::errors::TodoErrors;
use crate::logging::{info, Logger};
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

impl From<proto::todo::CreateTodoRequest> for models::TodoWrite {
    fn from(request: proto::todo::CreateTodoRequest) -> Self {
        Self {
            title: request.title,
            done: request.done,
        }
    }
}

impl From<models::TodoRead> for proto::todo::TodoRead {
    fn from(todo: models::TodoRead) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            done: todo.done,
        }
    }
}

impl From<()> for proto::todo::Confirmation {
    fn from(_: ()) -> Self {
        Self {
            status: proto::todo::Status::Ok as i32,
        }
    }
}

impl From<TodoErrors> for Status {
    fn from(err: TodoErrors) -> Self {
        match err {
            TodoErrors::TodoNotFound => Status::not_found(err.to_string()),
            _ => Status::internal(err.to_string()),
        }
    }
}

impl From<entities::ListResponse<models::TodoRead>> for ListTodosResponse {
    fn from(response: entities::ListResponse<models::TodoRead>) -> Self {
        Self {
            data: response.data.into_iter().map(|x| x.into()).collect(),
            total: response.total,
            offset: response.offset,
            limit: response.limit,
        }
    }
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
        request: Request<proto::todo::CreateTodoRequest>,
    ) -> Result<Response<proto::todo::TodoRead>, Status> {
        let mut log = Log::new("todo.Todo/Create");

        let request = request.into_inner();
        log.args = Some(format!("{:?}", request));

        let start = Instant::now();
        let res = self
            .state
            .controller
            .create(models::TodoWrite::from(request))
            .await?;
        let elapsed = start.elapsed();

        log.latency = format!("{:?}", elapsed);

        info!(&self.state.logger, "{}", log);

        Ok(Response::new(proto::todo::TodoRead::from(res)))
    }
    async fn delete(
        &self,
        _request: Request<proto::todo::DeleteTodoRequest>,
    ) -> Result<Response<proto::todo::Confirmation>, Status> {
        let mut log = Log::new("todo.Todo/Delete");

        let request = _request.into_inner();
        log.args = Some(format!("{:?}", request));

        let start = Instant::now();
        let _res = self.state.controller.delete(request.id).await?;
        let elapsed = start.elapsed();

        log.latency = format!("{:?}", elapsed);

        info!(&self.state.logger, "{}", log);

        Ok(Response::new(proto::todo::Confirmation::from(())))
    }

    async fn get(
        &self,
        request: Request<proto::todo::GetTodoRequest>,
    ) -> Result<Response<proto::todo::TodoRead>, Status> {
        let mut log = Log::new("todo.Todo/Get");

        let request = request.into_inner();
        log.args = Some(format!("{:?}", request));

        let start = Instant::now();
        let res = self.state.controller.get(request.id).await?;
        let elapsed = start.elapsed();

        log.latency = format!("{:?}", elapsed);

        info!(&self.state.logger, "{}", log);

        Ok(Response::new(proto::todo::TodoRead::from(res)))
    }

    async fn list(
        &self,
        request: Request<ListTodosRequest>,
    ) -> Result<Response<ListTodosResponse>, Status> {
        let mut log = Log::new("todo.Todo/List");

        let request = request.into_inner();

        let request = entities::ListRequest {
            offset: request.offset,
            limit: request.limit,
        };
        log.args = Some(format!("{:?}", request));

        let start = Instant::now();
        let res = self.state.controller.list(request).await?;
        let elapsed = start.elapsed();

        log.latency = format!("{:?}", elapsed);

        info!(&self.state.logger, "{}", log);

        Ok(Response::new(proto::todo::ListTodosResponse::from(res)))
    }

    async fn update(
        &self,
        _request: Request<proto::todo::UpdateTodoRequest>,
    ) -> Result<Response<proto::todo::Confirmation>, Status> {
        todo!()
    }
}
