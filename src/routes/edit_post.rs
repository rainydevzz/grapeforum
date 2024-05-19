use actix_session::Session;
use actix_web::{get, http::header::{ContentType, LOCATION}, web, HttpRequest, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{utils::nav_builder, entities};

#[get("/posts/{id}/edit-or-delete")]
async fn edit_or_delete_post(session: Session, req: HttpRequest, conn: web::Data<DatabaseConnection>) -> impl Responder {
    let user: Option<String> = session.get("user").unwrap().unwrap();
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let hbs = handlebars::Handlebars::new();
    let post_data = entities::posts::Entity::find_by_id(&id)
        .one(conn.get_ref())
        .await
        .unwrap()
        .unwrap();
    match user {
        Some(u) => {
            if u != post_data.owner {
                HttpResponse::Found().append_header((LOCATION, "/home")).finish()
            } else {
                HttpResponse::Ok()
                    .content_type(ContentType::html())
                    .body( 
                        hbs.render_template(
                            include_str!(r"../static/templates/edit_post.hbs"),
                            &serde_json::json!({"nav": nav_builder(&hbs, session.get("authorization").unwrap()), "footer": include_str!(r"../static/templates/footer.html"), "id": id})
                        ).unwrap()
                    )
            }
        }
        None => {
            HttpResponse::Found().append_header((LOCATION, "/home")).finish()
        }
    }
}