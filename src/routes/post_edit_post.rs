use actix_session::Session;
use actix_web::{http::header::LOCATION, post, web, HttpRequest, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

use crate::{entities, structures};

#[post("/posts/{id}/edit-or-delete")]
async fn edit_or_delete_post(session: Session, req: HttpRequest, conn: web::Data<DatabaseConnection>, web::Form(form): web::Form<structures::UpdateForm>,) -> impl Responder {
    let user: Option<String> = session.get("user").unwrap().unwrap();
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
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
                let mut to_update = post_data.into_active_model();
                to_update.content = Set(form.content);
                to_update.update(conn.get_ref()).await.unwrap();
                HttpResponse::Found().append_header((LOCATION, format!("/posts/{}", id))).finish()
            }
        }
        None => {
            HttpResponse::Found().append_header((LOCATION, "/home")).finish()
        }
    }
}