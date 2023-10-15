use actix_web::{get, HttpResponse, Responder, http::header::ContentType};

#[get("/register")]
async fn register() -> impl Responder {
    let hbs = handlebars::Handlebars::new();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(
            hbs.render_template(
                include_str!("../static/templates/register.hbs"),
                &serde_json::json!({"nav": include_str!("../static/templates/nav.html"), "footer": include_str!(r"../static/templates/footer.html")})
            ).unwrap()
        )
}