use actix_web::{get, HttpResponse, Responder, http::header::ContentType};

#[get("/create-post")]
async fn create_post() -> impl Responder {
    let hbs = handlebars::Handlebars::new();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body( 
            hbs.render_template(
                include_str!(r"../static/templates/create_post.hbs"),
                &serde_json::json!({"nav": include_str!(r"../static/templates/nav.html"), "footer": include_str!(r"../static/templates/footer.html")})
            ).unwrap()
        )
}