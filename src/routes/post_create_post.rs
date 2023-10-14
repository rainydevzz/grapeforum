use actix_session::Session;
use actix_web::{post, Responder, web::{self, Redirect}, HttpResponse, http::header::ContentType};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use crate::{entities::{posts, users}, structures};
use rand::distributions::{Alphanumeric, DistString};

#[post("/create-post")]
async fn create_post(
    web::Form(form): web::Form<structures::CreatePost>,
    conn: web::Data<DatabaseConnection>,
    session: Session
) -> impl Responder {
    let tok = session.get::<String>("authorization");
    let tok_result = users::Entity::find()
        .filter(users::Column::Token.contains(tok.unwrap().unwrap()))
        .one(conn.get_ref())
        .await
        .unwrap();

    match tok_result {
        Some(res) => {
            let new_post = posts::ActiveModel {
                title: Set(form.title),
                content: Set(form.content),
                owner: Set(res.name),
                id: Set(Alphanumeric.sample_string(&mut rand::thread_rng(), 12))
            };
            new_post.insert(conn.get_ref()).await.unwrap();
            Redirect::to("/home").see_other()
        }
        None => {
            Redirect::to("/").see_other()
        }
    }
}