use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, http::header::{ContentType, LOCATION}};
use super::super::utils::nav_builder;

#[get("/login")]
pub async fn login(session: Session) -> impl Responder {
    let hbs = handlebars::Handlebars::new();
    if session.get::<String>("user").unwrap().is_some() {
        HttpResponse::Found().append_header((LOCATION, "/home")).finish()
    } else {
        HttpResponse::Ok()
        .content_type(ContentType::html())
        .body( 
            hbs.render_template(
                include_str!(r"../static/templates/login.hbs"),
                &serde_json::json!({"nav": nav_builder(&hbs, None), "footer": include_str!(r"../static/templates/footer.html")})
            ).unwrap()
        )
    }
}