use actix_web::{get, HttpResponse};

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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn index_uri() {
        let app = test::init_service(App::new().service(index)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}
