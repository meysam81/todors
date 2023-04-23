use crate::errors::TodoErrors;
use crate::logging::{error, Logger};
use crate::traits::Controller;

pub use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

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

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello from Rust!")
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

async fn list_todos<T>(state: web::Data<AppState<T>>) -> HttpResponse
where
    T: Controller,
{
    match state.controller.list().await {
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
    cfg.service(index)
        .route("/api/v1/todos", web::post().to(create_todo::<T>))
        .route("/api/v1/todos/{id}", web::get().to(get_todo::<T>))
        .route("/api/v1/todos/{id}", web::delete().to(delete_todo::<T>))
        .route("/api/v1/todos", web::get().to(list_todos::<T>))
        .route("/api/v1/todos/{id}", web::patch().to(update_todo::<T>));
}
