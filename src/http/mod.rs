use crate::logging::Logger;
use crate::models;
use crate::traits::Controller;

pub use actix_web::dev::Server;

use actix_web::web;
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

mod index;
mod logging;
mod todo;

pub fn build_server<T>(state: web::Data<AppState<T>>, addr: String, num_workers: usize) -> Server
where
    T: Controller + 'static + Sync + Send,
{
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(configure::<T>)
            .wrap(logging::LogMiddleware::default())
    })
    .bind(addr)
    .unwrap()
    .workers(num_workers)
    .run()
}

fn configure<T>(cfg: &mut web::ServiceConfig)
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
