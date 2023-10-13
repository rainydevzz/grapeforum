use actix_web::{get, HttpResponse, Responder, http::header::ContentType};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!(r"../static/templates/index.hbs"))
}