use actix_session::Session;
use actix_web::{get, web::Data, HttpResponse};
use tera::{Context, Tera};

#[get("/")]
async fn index(tera: Data<Tera>, session: Session) -> HttpResponse {
    if let Some(true) = session.get::<bool>("logged_in").unwrap() {
        return HttpResponse::PermanentRedirect()
            .append_header(("Location", "/user"))
            .finish();
    }
    let body = tera.render("index.html.tera", &Context::new()).unwrap();

    HttpResponse::Ok().body(body)
}
