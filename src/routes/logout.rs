use actix_session::Session;
use actix_web::{get, Responder, web::Redirect};

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    Redirect::to("/home")
}