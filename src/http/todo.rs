use crate::entities::ListRequest;
use crate::errors::TodoErrors;
use crate::logging::{debug, error};
use crate::models;
use crate::traits::Controller;

use actix_web::{web, HttpResponse};
pub use actix_web::{App, HttpServer};

use super::AppState;

impl From<web::Query<ListRequest>> for ListRequest {
    fn from(query: web::Query<ListRequest>) -> Self {
        query.into_inner()
    }
}

pub struct Api<T>
where
    T: Controller<
        Input = models::TodoWrite,
        Output = models::TodoRead,
        Id = models::Id,
        OptionalInput = models::TodoUpdate,
    >,
{
    _controller: T,
}

impl<T> Api<T>
where
    T: Controller<
        Input = models::TodoWrite,
        Output = models::TodoRead,
        Id = models::Id,
        OptionalInput = models::TodoUpdate,
    >,
{
    pub async fn create_todo(
        state: web::Data<AppState<T>>,
        todo: web::Json<T::Input>,
    ) -> HttpResponse {
        match state.controller.create(todo.into_inner()).await {
            Ok(todo) => HttpResponse::Created().json(todo),
            Err(TodoErrors::DatabaseError(err)) => HttpResponse::Conflict()
                .content_type("text/plain")
                .body(err.to_string()),
            Err(err) => {
                error!(state.logger, "Failed to create todo: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub async fn create_batch(
        state: web::Data<AppState<T>>,
        todos: web::Json<Vec<T::Input>>,
    ) -> HttpResponse {
        match state.controller.create_batch(todos.into_inner()).await {
            Ok(ids) => HttpResponse::Created().json(ids),
            Err(TodoErrors::BatchTooLarge { max_size }) => HttpResponse::PayloadTooLarge()
                .content_type("text/plain")
                .body(format!("Batch too large, max batch size is {}", max_size)),
            Err(TodoErrors::DatabaseError(err)) => HttpResponse::Conflict()
                .content_type("text/plain")
                .body(err.to_string()),
            Err(err) => {
                error!(state.logger, "Failed to create todo: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub async fn delete_todo(state: web::Data<AppState<T>>, id: web::Path<T::Id>) -> HttpResponse {
        match state.controller.delete(id.into_inner()).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(TodoErrors::TodoNotFound) => HttpResponse::NotFound().finish(),
            Err(err) => {
                error!(state.logger, "Failed to delete todo: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub async fn get_todo(state: web::Data<AppState<T>>, id: web::Path<T::Id>) -> HttpResponse {
        match state.controller.get(id.into_inner()).await {
            Ok(todo) => HttpResponse::Ok().json(todo),
            Err(TodoErrors::TodoNotFound) => HttpResponse::NotFound()
                .content_type("text/plain")
                .body("TODO not found"),
            Err(err) => {
                error!(state.logger, "Failed to get todo: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub async fn list_todos(
        state: web::Data<AppState<T>>,
        pagination: web::Query<ListRequest>,
    ) -> HttpResponse {
        debug!(state.logger, "Listing todos with request: {:?}", pagination);
        match state.controller.list(pagination.into()).await {
            Ok(todos) => HttpResponse::Ok().json(todos),
            Err(err) => {
                error!(state.logger, "Failed to list todos: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub async fn update_todo(
        state: web::Data<AppState<T>>,
        id: web::Path<T::Id>,
        todo: web::Json<T::OptionalInput>,
    ) -> HttpResponse {
        match state
            .controller
            .update(id.into_inner(), todo.into_inner())
            .await
        {
            Ok(_) => HttpResponse::Accepted()
                .content_type("text/plain")
                .body("TODO updated"),
            Err(TodoErrors::TodoNotFound) => HttpResponse::NotFound().finish(),
            Err(err) => {
                error!(state.logger, "Failed to update todo: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/todos",
    operation_id = "Create a new TODO",
    context_path = "/api/v1",
    request_body = TodoWrite,
    responses(
        (status = 201, content_type = "application/json", example = json!({"id": 1, "title": "Todo title", "description": "Todo description", "done": false}), body = TodoRead),
        (status = 409, description = "a TODO with the same title exists", content_type = "text/plain", example = json!("Invalid input"), body = String),
        (status = 500, description = "Internal server error", content_type = "text/plain", example = json!("Internal server error"), body = String),
    ),
    tag = "todo",
)]
fn create_todo() {}

#[allow(dead_code)]
#[utoipa::path(
    put,
    path = "/todos",
    operation_id = "Create a batch of TODOs",
    context_path = "/api/v1",
    request_body(content = Vec<TodoWrite>, example = json!([{"title": "Call Jack", "done": false}, {"title": "Buy milk", "done": false}, {"title": "Go to gym", "done": false}])),
    responses(
        (status = 201, content_type = "application/json", example = json!([1, 2, 3]), body = Vec<u32>),
        (status = 409, description = "a TODO with the same title exists", content_type = "text/plain", example = json!("Todo already exists"), body = String),
        (status = 500, description = "Internal server error", content_type = "text/plain", example = json!("Internal server error"), body = String),
    ),
)]
pub fn create_batch() {}

#[allow(dead_code)]
#[utoipa::path(
    delete,
    path = "/todos/{id}",
    operation_id = "Delete a TODO by id",
    context_path = "/api/v1",
    params(
        ("id" = u32, description = "Todo identifier", example = json!(1), nullable = false, minimum = 1),
    ),
    responses(
        (status = 204, description = "TODO deleted", content_type = "text/plain", example = json!("TODO deleted"), body = String),
        (status = 404, description = "TODO not found", content_type = "text/plain", example = json!("TODO not found"), body = String),
        (status = 500, description = "Internal server error", content_type = "text/plain", example = json!("Internal server error"), body = String),
    ),
    tag = "todo",
)]
pub fn delete_todo() {}

#[allow(dead_code)]
#[utoipa::path(
        get,
        path = "/todos/{id}",
        operation_id = "Get a TODO by id",
        context_path = "/api/v1",
        params(
            ("id" = u32, description = "Todo identifier", example = json!(1), nullable = false, minimum = 1),
        ),
        responses(
            (status = 200, content_type = "application/json", example = json!({"id": 1, "title": "Todo title", "description": "Todo description", "done": false}), body = TodoRead),
            (status = 404, description = "TODO not found", content_type = "text/plain", example = json!("TODO not found"), body = String),
        ),
        tag = "todo",
    )]
pub fn get_todo() {}

#[allow(dead_code)]
#[utoipa::path(
        get,
        path = "/todos",
        operation_id = "List TODOs",
        context_path = "/api/v1",
        params(
            ("offset" = Option<u32>, Query, description = "Row offset", example = json!(1), nullable = true, minimum = 0, example = 0),
            ("limit" = Option<u32>, Query, description = "Number of items per page", example = json!(10), nullable = true, minimum = 1, maximum = 1000, example = 100),
        ),
        responses(
            (status = 200, content_type = "application/json", example = json!([{"id": 1, "title": "Todo title", "description": "Todo description", "done": false}]), body = Vec<TodoRead>),
            (status = 500, description = "Internal server error", content_type = "text/plain", example = json!("Internal server error"), body = String),
        ),
        tag = "todo",
    )]
pub fn list_todos() {}

#[allow(dead_code)]
#[utoipa::path(
        patch,
        path = "/todos/{id}",
        operation_id = "Update a TODO by id",
        request_body = TodoUpdate,
        context_path = "/api/v1",
        params(
            ("id" = u32, description = "Todo identifier", example = json!(1), nullable = false, minimum = 1),
        ),
        responses(
            (status = 202, description = "TODO updated", content_type = "text/plain", example = json!("TODO updated"), body = String),
            (status = 404, description = "TODO not found", content_type = "text/plain", example = json!("TODO not found"), body = String),
            (status = 500, description = "Internal server error", content_type = "text/plain", example = json!("Internal server error"), body = String),
        ),
        tag = "todo",
    )]
pub fn update_todo() {}
