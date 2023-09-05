use actix_session::Session;
use actix_web::{get, web::Data, HttpResponse};
use tera::{Context, Tera};

#[get("/user")]
pub async fn userarea(tera: Data<Tera>, session: Session) -> HttpResponse {
    let Some(true) = session.get::<bool>("logged_in").unwrap() else {
        return HttpResponse::Forbidden().finish();
    };

    let Some(username) = session.get::<String>("username").unwrap() else {
        return HttpResponse::InternalServerError().finish();
    };

    let mut context = Context::new();
    context.insert("username", &username);
    let body = tera.render("user_area.html.tera", &context).unwrap();

    HttpResponse::Ok().body(body)
}
