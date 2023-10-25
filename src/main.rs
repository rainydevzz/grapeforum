use actix_web::{App, HttpServer, web, cookie::Key};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_governor::{Governor, GovernorConfigBuilder};
use sea_orm::{DatabaseConnection, Database, ConnectionTrait};
use std::env;
use routes::{*, users::user};

mod routes;
mod structures;
mod entities;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let gov_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(8)
        .finish()
        .unwrap();

    dotenvy::dotenv().expect("not found");

    let url: &String = &env::vars().find(|v: &(String, String)| v.0 == "DATABASE_URL").unwrap().1;
    let db: DatabaseConnection = Database::connect(url).await.unwrap();

    let _ = &db.execute_unprepared(include_str!(r"./in.sql")).await.unwrap();

    let secret = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&gov_conf))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new(db.clone()))
            .service(actix_files::Files::new("/static", "./src/static").show_files_listing())
            .service(login::login)
            .service(post_index::post_index)
            .service(register::register)
            .service(invalid_user::invalid_user)
            .service(home::home)
            .service(post_register::post_register)
            .service(create_post::create_post)
            .service(post_create_post::create_post)
            .service(posts::get_post)
            .service(comment::comment)
            .service(post_comment::post_comment)
            .service(index::index)
            .service(user)
            .service(post_users::user)
            .service(logout::logout)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(1)
    .run()
    .await
}
