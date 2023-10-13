use actix_web::{get, HttpResponse, Responder, http::header::ContentType};

#[get("/")]
pub async fn hello() -> impl Responder {
    let hbs = handlebars::Handlebars::new();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body( 
            hbs.render_template(
                include_str!(r"../static/templates/index.hbs"),
                &serde_json::json!({"nav": include_str!(r"../static/templates/nav.html")})
            ).unwrap()
        )
}