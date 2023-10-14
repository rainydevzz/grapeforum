use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, http::header::ContentType, web};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entities;

#[get("/home")]
pub async fn home(conn: web::Data<DatabaseConnection>, session: Session) -> impl Responder {
    let mut home_data = entities::posts::Entity::find().into_json().all(conn.get_ref()).await.unwrap();
    let mut reversed_data = Vec::new();
    while home_data.len() > 0 {
        reversed_data.push(home_data.pop().unwrap())
    }
    let hbs = handlebars::Handlebars::new();
    let mut post_ask = String::new();
    let mut user = session.get::<String>("user").unwrap();
    match &user {
        Some(_) => {
            post_ask = "No posts? <a href=\"/create-post\">Create One!</a>".to_string()
        }
        None => user = Some("Guest".to_string())
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(
            hbs.render_template(
                include_str!(r"../static/templates/home.hbs"),
                &serde_json::json!({
                    "nav": include_str!(r"../static/templates/nav.html"),
                    "user": user.unwrap(),
                    "posts": reversed_data,
                    "post_ask": post_ask
                })
            ).unwrap()
        )
}