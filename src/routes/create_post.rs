use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, http::header::ContentType};

use crate::utils::nav_builder;

#[get("/create-post")]
async fn create_post(session: Session) -> impl Responder {
    let hbs = handlebars::Handlebars::new();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body( 
            hbs.render_template(
                include_str!(r"../static/templates/create_post.hbs"),
                &serde_json::json!({"nav": nav_builder(&hbs, session.get("authorization").unwrap()), "footer": include_str!(r"../static/templates/footer.html")})
            ).unwrap()
        )
}