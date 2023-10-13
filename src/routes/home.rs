use actix_web::{get, HttpResponse, Responder, http::header::ContentType};

#[get("/home")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!(r"../static/templates/home.hbs"))
}