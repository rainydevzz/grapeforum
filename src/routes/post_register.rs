use actix_web::{post, Responder, web::{self}, HttpResponse, http::header::ContentType};
use base64::Engine;
use crate::{entities, structures};
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait};
use bcrypt::hash;
use rand::distributions::{Alphanumeric, DistString};

#[post("/register")]
pub async fn post_register(conn: web::Data<DatabaseConnection>, web::Form(form): web::Form<structures::Register>) -> impl Responder {
    let user_result: Option<entities::users::Model> = entities::users::Entity::find_by_id(&form.user)
        .one(conn.get_ref())
        .await
        .unwrap();

    match user_result {
        Some(_) => {
            HttpResponse::Ok().body("User already exists!")
        }
        None => {
            let mut b64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(&form.user);
            let r_str = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
            b64.push_str(&r_str);
            println!("{}", &b64);
            let new_user = entities::users::ActiveModel {
                name: Set(form.user),
                password: Set(hash(form.password, 4).unwrap()),
                token: Set(b64)
            };
            new_user.insert(conn.get_ref()).await.unwrap();
            HttpResponse::Ok().content_type(ContentType::html()).body("You have registered! Please visit the <a href=\"/\">login</a> page to login.")
        }
    }
}