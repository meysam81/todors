use crate::errors::TodoErrors;
use crate::logging::{debug, error, Logger};
use crate::traits::{Controller, ListRequest};

use actix_web::{get, web, HttpResponse};
pub use actix_web::{App, HttpServer};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

#[utoipa::path(
    responses(
        (status = 200, content_type = "application/json", description = "Index URI responding with hello world.", example = json!("Hello world!"), body = String),
    ),
    tag = "index"
)]
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain".as_bytes())
        .body("Hello from Todors written in Rust!")
}

async fn create_todo<T>(state: web::Data<AppState<T>>, todo: web::Json<T::Input>) -> HttpResponse
where
    T: Controller,
{
    match state.controller.create(&todo.into_inner()).await {
        Ok(todo) => HttpResponse::Created().json(todo),
        Err(err) => {
            error!(state.logger, "Failed to create todo: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn delete_todo<T>(state: web::Data<AppState<T>>, id: web::Path<T::Id>) -> HttpResponse
where
    T: Controller,
{
    match state.controller.delete(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            error!(state.logger, "Failed to delete todo: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_todo<T>(state: web::Data<AppState<T>>, id: web::Path<T::Id>) -> HttpResponse
where
    T: Controller,
{
    match state.controller.get(id.into_inner()).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(TodoErrors::TodoNotFound) => HttpResponse::NotFound().finish(),
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

async fn list_todos<T>(
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

async fn update_todo<T>(
    state: web::Data<AppState<T>>,
    id: web::Path<T::Id>,
    todo: web::Json<T::OptionalInput>,
) -> HttpResponse
where
    T: Controller,
{
    match state
        .controller
        .update(id.into_inner(), &todo.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(err) => {
            error!(state.logger, "Failed to update todo: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn configure<T>(cfg: &mut web::ServiceConfig)
where
    T: Controller + 'static,
{
    use crate::models;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            index,
            // create_todo,
            // delete_todo,
            // get_todo,
            // list_todos,
            // update_todo,
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

    cfg.service(index)
        .service(
            web::scope("/api/v1")
                .route("/todos", web::post().to(create_todo::<T>))
                .route("/todos/{id}", web::get().to(get_todo::<T>))
                .route("/todos/{id}", web::delete().to(delete_todo::<T>))
                .route("/todos", web::get().to(list_todos::<T>))
                .route("/todos/{id}", web::patch().to(update_todo::<T>)),
        )
        .service(SwaggerUi::new("/docs/{_:.*}").url("/openapi.json", ApiDoc::openapi()));
}
