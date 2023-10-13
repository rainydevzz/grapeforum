use actix_web::{App, HttpServer, web, cookie::Key};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_governor::{Governor, GovernorConfigBuilder};
use sea_orm::{DatabaseConnection, Database};
use std::env;
use routes::*;

mod routes;
mod structures;
mod users;

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

    let secret = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&gov_conf))
            .wrap(
                SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret.clone()
                )
            )
            .app_data(web::Data::new(db.clone()))
            .service(actix_files::Files::new("/static", "./src/static"))
            .service(index::hello)
            .service(post_index::post_index)
            .service(register::register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}