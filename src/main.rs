use actix_web::{get, App, HttpResponse, HttpServer, Responder, http::header::ContentType, post, web};
use bcrypt::{hash, verify};
use actix_governor::{Governor, GovernorConfigBuilder};
use sea_orm::{DatabaseConnection, Database, EntityTrait, Set, ActiveModelTrait, QueryFilter, ColumnTrait};
use std::env;
use rand::distributions::{Alphanumeric, DistString};

mod structures;
mod users;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!(r"../templates/index.html"))
}

#[post("/")]
async fn index(conn: web::Data<DatabaseConnection>, web::Form(form): web::Form<structures::Login>) -> impl Responder {
    let user_result: Option<users::Model> = users::Entity::find_by_id(&form.user)
        .one(conn.get_ref())
        .await
        .unwrap();
    match user_result {
        Some(res) => {
            println!("{:?}", &res);
            if verify(&form.password, &res.password).unwrap() {
                return HttpResponse::Ok().finish();
            } else {
                return HttpResponse::Unauthorized()
                    .content_type(ContentType::html())
                    .body(include_str!(r"../templates/invalid_user.html"));
            }
        }
        None => {
            let new_user = users::ActiveModel {
                name: Set(form.user),
                password: Set(hash(&form.password, 4).unwrap()),
                token: Set(hash(Alphanumeric.sample_string(&mut rand::thread_rng(), 16), 4).unwrap())
            };
            new_user.insert(conn.get_ref()).await.unwrap();
            return HttpResponse::Ok().finish();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let gov_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    dotenvy::dotenv().expect("not found");

    let url: &String = &env::vars().find(|v: &(String, String)| v.0 == "DATABASE_URL").unwrap().1;
    let db: DatabaseConnection = Database::connect(url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&gov_conf))
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}