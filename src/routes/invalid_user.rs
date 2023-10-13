use actix_web::{get, HttpResponse, Responder, http::header::ContentType};

#[get("/invalid-user")]
pub async fn invalid_user() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!(r"../static/templates/invalid_user.html"))
}