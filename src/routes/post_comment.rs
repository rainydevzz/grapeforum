use actix_session::Session;
use actix_web::{HttpRequest, Responder, web::{self, Redirect}, post};
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::{entities, structures};
use rand::distributions::{Alphanumeric, DistString};

#[post("/posts/{id}/reply")]
async fn post_comment(req: HttpRequest, conn: web::Data<DatabaseConnection>, session: Session, web::Form(form): web::Form<structures::Comment>) -> impl Responder {
    let user = session.get::<String>("user").unwrap().unwrap();
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let post_data = entities::posts::Entity::find_by_id(&id)
        .one(conn.get_ref())
        .await
        .unwrap()
        .unwrap();

    let comment = entities::comments::ActiveModel {
        id: Set(Alphanumeric.sample_string(&mut rand::thread_rng(), 16)),
        post_id: Set(post_data.id),
        owner: Set(user),
        content: Set(form.comment)
    };
    comment.insert(conn.get_ref()).await.unwrap();

    let rd = "/posts/".to_owned() + &id; 
    Redirect::to(rd).see_other()
}