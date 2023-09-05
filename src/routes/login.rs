use crate::types::LoginEntry;
use actix_session::Session;
use actix_web::{
    get, post,
    web::{Data, Form},
    HttpResponse,
};
use tera::{Context, Tera};

#[get("/login")]
pub async fn login_page(tera: Data<Tera>, session: Session) -> HttpResponse {
    if let Some(true) = session.get::<bool>("logged_in").unwrap() {
        return HttpResponse::PermanentRedirect()
            .append_header(("Location", "/user"))
            .finish();
    }

    HttpResponse::Ok().body(tera.render("login.html.tera", &Context::new()).unwrap())
}

#[post("/login")]
pub async fn login_request(
    params: Form<LoginEntry>,
    tera: Data<Tera>,
    session: Session,
) -> HttpResponse {
    if let Some(true) = session.get::<bool>("logged_in").unwrap() {
        return HttpResponse::Forbidden().finish();
    }

    if params.username == "admin" && params.password == "admin" {
        session.insert("logged_in", true).unwrap();
        session.insert("username", params.username.clone()).unwrap();
        return HttpResponse::SeeOther()
            .append_header(("Location", "/user"))
            .finish();
    }

    HttpResponse::Ok().body(tera.render("bad_login.html.tera", &Context::new()).unwrap())
}
