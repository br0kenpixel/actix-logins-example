use actix_session::Session;
use actix_web::{get, HttpResponse};

#[get("/logout")]
pub async fn logout(session: Session) -> HttpResponse {
    if let Some(true) = session.get::<bool>("logged_in").unwrap() {
        session.purge();
        HttpResponse::PermanentRedirect()
            .append_header(("Location", "/"))
            .finish()
    } else {
        HttpResponse::BadRequest().finish()
    }
}
