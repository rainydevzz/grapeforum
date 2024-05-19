use actix_session::Session;
use actix_web::{get, HttpRequest, Responder, web, HttpResponse, http::header::ContentType};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

use crate::{entities, utils::nav_builder};

#[get("/posts/{id}")]
async fn get_post(req: HttpRequest, conn: web::Data<DatabaseConnection>, session: Session) -> impl Responder {
    let user = session.get::<String>("user").unwrap();
    let mut can_reply = String::new();
    let mut can_control = String::new();
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let post_data = entities::posts::Entity::find_by_id(&id)
        .one(conn.get_ref())
        .await
        .unwrap();

    match post_data {
        Some(post_data) => {
            let comments = entities::comments::Entity::find()
                .filter(entities::comments::Column::PostId.contains(&id))
                .into_json()
                .all(conn.get_ref())
                .await
                .unwrap();

        match user {
            Some(u) => {
                if u == post_data.owner {
                    can_control = format!("<a href=\"/posts/{}/edit-or-delete\">Edit Or Delete</a>", id);
                }
                can_reply = "<a href=\"/posts/".to_owned() + &post_data.id + "/reply\">Reply</a>"
            }
            None => {}
        }

        let hbs = handlebars::Handlebars::new();
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(
                hbs.render_template(
                    include_str!(r"../static/templates/post.hbs"),
                    &serde_json::json!({
                        "nav": nav_builder(&hbs, session.get("authorization").unwrap()),
                        "title": post_data.title,
                        "content": post_data.content,
                        "can_reply": can_reply,
                        "comments": comments,
                        "footer": include_str!(r"../static/templates/footer.html"),
                        "can_control": can_control
                    })
                ).unwrap()
            )
        }
        None => {
            HttpResponse::NotFound().body("couldn't find that post.")
        }
    }
}