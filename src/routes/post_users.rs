use actix_session::Session;
use actix_web::{post, Responder, web::{self}, HttpRequest, HttpResponse, http::header::LOCATION};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use crate::{entities::users, structures};

#[post("/users/{username}")]
async fn user(req: HttpRequest, conn: web::Data<DatabaseConnection>, session: Session, bio: web::Form<structures::Bio>) -> impl Responder {
    let tok = session.get::<String>("authorization").unwrap();
    match tok {
        Some(t) => {
            let user = users::Entity::find()
                .filter(users::Column::Token.contains(t))
                .one(conn.get_ref())
                .await
                .unwrap()
                .unwrap();
            let mut user: users::ActiveModel = user.into();
            user.bio = Set(bio.bio.clone());
            user.update(conn.get_ref()).await.unwrap();
            HttpResponse::Found().append_header((LOCATION, req.path())).finish()

        }
        None => {
            HttpResponse::Found().append_header((LOCATION, "/home")).finish()
        }
    }
}