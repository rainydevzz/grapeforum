use actix_session::Session;
use actix_web::{http::header::LOCATION, post, web, HttpRequest, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

use crate::entities;

#[post("/posts/{id}/delete")]
async fn edit_or_delete_post(session: Session, req: HttpRequest, conn: web::Data<DatabaseConnection>) -> impl Responder {
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
                post_data.delete(conn.get_ref()).await.unwrap();
                HttpResponse::Found().append_header((LOCATION, "/home")).finish()
            }
        }
        None => {
            HttpResponse::Found().append_header((LOCATION, "/home")).finish()
        }
    }
}