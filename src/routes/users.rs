use actix_web::{get, HttpResponse, Responder, http::header::ContentType, HttpRequest, web};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entities::users;

#[get("/users/{username}")]
async fn user(req: HttpRequest, conn: web::Data<DatabaseConnection>) -> impl Responder {
    let username = req.match_info().get("username").unwrap();
    let user_res = users::Entity::find_by_id(username)
        .one(conn.get_ref())
        .await
        .unwrap();

    match user_res {
        Some(res) => {
            let hbs = handlebars::Handlebars::new();
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(
                    hbs.render_template(
                        include_str!(r"../static/templates/user.hbs"),
                        &serde_json::json!({
                            "nav": include_str!(r"../static/templates/nav.html"),
                            "name": &res.name,
                            "bio": &res.bio
                        })
                    ).unwrap()
                )
        }
        None => {
            HttpResponse::NotFound().body("couldn't find that user.")
        }
    }
}