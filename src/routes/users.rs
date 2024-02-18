use actix_session::Session;
use actix_web::{get, HttpResponse, Responder, http::header::ContentType, HttpRequest, web};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::{entities::users, utils::nav_builder};

#[get("/users/{username}")]
async fn user(req: HttpRequest, conn: web::Data<DatabaseConnection>, session: Session) -> impl Responder {
    let username = req.match_info().get("username").unwrap();
    let user_res = users::Entity::find_by_id(username)
        .one(conn.get_ref())
        .await
        .unwrap();

    match user_res {
        Some(res) => {
            let hbs = handlebars::Handlebars::new();
            let can_edit: String;
            let tok = session.get::<String>("authorization").unwrap();
                if !tok.is_none() {
                    if tok.unwrap() == res.token {
                        can_edit = include_str!(r"../static/templates/bio_form.html").to_owned();
                    } else {
                        can_edit = "".to_owned();
                    }
                } else {
                    can_edit = "".to_owned();
                }
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(
                    hbs.render_template(
                        include_str!(r"../static/templates/user.hbs"),
                        &serde_json::json!({
                            "nav": nav_builder(&hbs, session.get("authorization").unwrap()),
                            "name": &res.name,
                            "bio": &res.bio,
                            "bio_form": can_edit
                        })
                    ).unwrap()
                )
        }
        None => {
            HttpResponse::NotFound().body("couldn't find that user.")
        }
    }
}