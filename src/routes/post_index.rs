use actix_session::Session;
use actix_web::{post, Responder, web::{self, Redirect}};
use crate::{entities, structures};
use sea_orm::{DatabaseConnection, EntityTrait};
use bcrypt::verify;

#[post("/login")]
pub async fn post_index(conn: web::Data<DatabaseConnection>, web::Form(form): web::Form<structures::Login>, session: Session) -> impl Responder {
    let user_result: Option<entities::users::Model> = entities::users::Entity::find_by_id(&form.user)
        .one(conn.get_ref())
        .await
        .unwrap();

    match user_result {
        Some(res) => {
            if verify(&form.password, &res.password).unwrap() {
                session.insert("authorization", &res.token).unwrap();
                session.insert("user", &res.name).unwrap();
                Redirect::to("/home").see_other()
            } else {
                Redirect::to("/invalid-user").see_other()
            }
        }
        None => {
            Redirect::to("/invalid-user").see_other()
        }
    }
}