use actix_web::{post, HttpResponse, Responder, http::header::ContentType, web};
use crate::{users, structures};
use sea_orm::{DatabaseConnection, EntityTrait};
use bcrypt::verify;
use serde_json::json;

#[post("/")]
pub async fn post_index(conn: web::Data<DatabaseConnection>, web::Form(form): web::Form<structures::Login>) -> impl Responder {
    let user_result: Option<users::Model> = users::Entity::find_by_id(&form.user)
        .one(conn.get_ref())
        .await
        .unwrap();

    match user_result {
        Some(res) => {
            if verify(&form.password, &res.password).unwrap() {
                let hbs = handlebars::Handlebars::new();
                let tp1 = hbs.render_template(include_str!(r"../static/templates/home.hbs"), &json!({"user": &form.user})).unwrap();
                HttpResponse::Ok()
                    .content_type(ContentType::html())
                    .body(tp1)
            } else {
                return HttpResponse::Ok()
                    .content_type(ContentType::html())
                    .body(include_str!(r"../static/templates/invalid_user.html"));
            }
        }
        None => {
            return HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(include_str!(r"../static/templates/invalid_user.html"));
        }
    }
}