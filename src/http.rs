use crate::serializers::Serialize;
use crate::traits::Controller;

pub use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub struct AppState<T>
where
    T: Controller,
{
    controller: T,
}

impl<T> AppState<T>
where
    T: Controller,
{
    pub fn new(controller: T) -> web::Data<AppState<T>> {
        web::Data::new(AppState { controller })
    }
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn list_todos<T>(state: web::Data<AppState<T>>) -> impl Responder
where
    T: Controller,
{
    match state.controller.list().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => {
            // TODO: print error log
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn configure<T>(cfg: &mut web::ServiceConfig)
where
    T: Controller + 'static,
{
    cfg.service(index)
        .route("/api/v1/todos", web::get().to(list_todos::<T>));
}
