use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, http::header::ContentType, web};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entities;

#[get("/home")]
pub async fn home(conn: web::Data<DatabaseConnection>, session: Session) -> impl Responder {
    let home_data = entities::posts::Entity::find().into_json().all(conn.get_ref()).await.unwrap();
    let hbs = handlebars::Handlebars::new();
    let user = session.get::<String>("user").unwrap().unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(
            hbs.render_template(
                include_str!(r"../static/templates/home.hbs"),
                &serde_json::json!({
                    "nav": include_str!(r"../static/templates/nav.html"),
                    "user": user,
                    "posts": home_data
                })
            ).unwrap()
        )

}