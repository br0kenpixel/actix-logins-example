use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time, Key},
    web::Data,
    App, HttpServer,
};
use tera::Tera;

mod routes;
pub mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Loading templates...");
    let tera = Data::new(Tera::new("templates/**").unwrap());

    let secret_key = Key::generate();

    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    // Customise session length!
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(time::Duration::days(5)),
                    )
                    .build(),
            )
            .service(routes::index::index)
            .service(routes::login::login_page)
            .service(routes::login::login_request)
            .service(routes::logout::logout)
            .service(routes::userarea::userarea)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
