use crate::entities::ListRequest;
use crate::errors::TodoErrors;
use crate::logging::{debug, error, Logger};
use crate::models;
use crate::traits::Controller;

use actix_web::{get, web, HttpResponse};
pub use actix_web::{App, HttpServer};

use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

pub struct AppState<T>
where
    T: Controller,
{
    controller: T,
    logger: Logger,
}

impl<T> AppState<T>
where
    T: Controller,
{
    pub fn new(controller: T, logger: Logger) -> web::Data<AppState<T>> {
        web::Data::new(AppState { controller, logger })
    }
}

mod index {
    use super::*;

    #[utoipa::path(
        operation_id = "Index page responding with Hello world!",
        responses(
            (status = 200, content_type = "text/plain", description = "Index URI responding with hello world.", example = json!("Hello world!"), body = String),
        ),
        tag = "index"
    )]
    #[get("/")]
    pub async fn index() -> HttpResponse {
        HttpResponse::Ok()
            .content_type("text/plain".as_bytes())
            .body("Hello from Todors written in Rust!")
    }
}

mod todo {
    use super::*;

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
    pub async fn create_todo<T>(
        state: web::Data<AppState<T>>,
        todo: web::Json<T::Input>,
    ) -> HttpResponse
    where
        T: Controller,
    {
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
    pub async fn create_batch<T>(
        state: web::Data<AppState<T>>,
        todos: web::Json<Vec<T::Input>>,
    ) -> HttpResponse
    where
        T: Controller,
    {
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
    pub async fn delete_todo<T>(state: web::Data<AppState<T>>, id: web::Path<T::Id>) -> HttpResponse
    where
        T: Controller,
    {
        match state.controller.delete(id.into_inner()).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(TodoErrors::TodoNotFound) => HttpResponse::NotFound().finish(),
            Err(err) => {
                error!(state.logger, "Failed to delete todo: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

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
    pub async fn get_todo<T>(state: web::Data<AppState<T>>, id: web::Path<T::Id>) -> HttpResponse
    where
        T: Controller,
    {
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

    impl From<web::Query<ListRequest>> for ListRequest {
        fn from(query: web::Query<ListRequest>) -> Self {
            query.into_inner()
        }
    }

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
    pub async fn list_todos<T>(
        state: web::Data<AppState<T>>,
        pagination: web::Query<ListRequest>,
    ) -> HttpResponse
    where
        T: Controller,
    {
        debug!(state.logger, "Listing todos with request: {:?}", pagination);
        match state.controller.list(pagination.into()).await {
            Ok(todos) => HttpResponse::Ok().json(todos),
            Err(err) => {
                error!(state.logger, "Failed to list todos: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

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
    pub async fn update_todo<T>(
        state: web::Data<AppState<T>>,
        id: web::Path<T::Id>,
        todo: web::Json<T::OptionalInput>,
    ) -> HttpResponse
    where
        T: Controller,
    {
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
pub fn configure<T>(cfg: &mut web::ServiceConfig)
where
    T: Controller + 'static,
{
    cfg.service(index::index)
        .service(
            web::scope("/api/v1")
                .route("/todos", web::post().to(todo::create_todo::<T>))
                .route("/todos", web::put().to(todo::create_batch::<T>))
                .route("/todos/{id}", web::get().to(todo::get_todo::<T>))
                .route("/todos/{id}", web::delete().to(todo::delete_todo::<T>))
                .route("/todos", web::get().to(todo::list_todos::<T>))
                .route("/todos/{id}", web::patch().to(todo::update_todo::<T>)),
        )
        .service(
            SwaggerUi::new("/docs/{_:.*}")
                .url("/openapi.json", build_apidoc())
                .config(build_apidoc_config()),
        );
}

fn build_apidoc() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            index::index,
            todo::create_todo,
            todo::create_batch,
            todo::delete_todo,
            todo::get_todo,
            todo::list_todos,
            todo::update_todo,
        ),
        components(
            schemas(models::TodoRead, models::TodoWrite, models::TodoUpdate),
        ),
        tags(
            (name = "todo", description = "Todo management endpoints."),
            (name = "index", description = "Index endpoint."),
        ),
    )]
    struct ApiDoc;

    ApiDoc::openapi()
}

fn build_apidoc_config<'a>() -> Config<'a> {
    Config::default()
        .try_it_out_enabled(true)
        .default_models_expand_depth(2)
        .display_request_duration(true)
        .filter(true)
        .show_common_extensions(true)
        .request_snippets_enabled(true)
        .persist_authorization(true)
        .display_operation_id(true)
}
