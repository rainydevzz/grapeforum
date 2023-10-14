use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, http::header::{ContentType, LOCATION}};

#[get("/")]
pub async fn hello(session: Session) -> impl Responder {
    let hbs = handlebars::Handlebars::new();
    if session.get::<String>("user").unwrap().is_some() {
        HttpResponse::Found().append_header((LOCATION, "/home")).finish()
    } else {
        HttpResponse::Ok()
        .content_type(ContentType::html())
        .body( 
            hbs.render_template(
                include_str!(r"../static/templates/index.hbs"),
                &serde_json::json!({"nav": include_str!(r"../static/templates/nav.html")})
            ).unwrap()
        )
    }
}