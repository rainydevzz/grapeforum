use actix_session::Session;
use actix_web::{get, http::header::{ContentType, LOCATION}, HttpResponse, Responder};

use crate::utils::nav_builder;

#[get("/register")]
async fn register(session: Session) -> impl Responder {
    let hbs = handlebars::Handlebars::new();
    let auth = session.get("authorization").unwrap();
    if auth.is_some() {
        HttpResponse::Found().append_header((LOCATION, "/home")).finish()
    } else {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(
                hbs.render_template(
                    include_str!("../static/templates/register.hbs"),
                    &serde_json::json!({"nav": nav_builder(&hbs, auth), "footer": include_str!(r"../static/templates/footer.html")})
                ).unwrap()
            )
    }
}   