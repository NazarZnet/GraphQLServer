pub mod database;

use crate::api::logout;
pub use crate::api::{graphql, login, playground, signup,health_check};
pub use crate::schemas::AppData;

use std::net::TcpListener;
use actix_web::dev::Server;
pub use database::*;

use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{middleware, web, App, HttpServer};

use dotenv::dotenv;
use time::Duration;

pub fn create_server(listener:TcpListener) -> std::io::Result<Server> {
    dotenv().ok();

    let app_state = AppData::init()
        .expect("Can not establish DB connection");

    log::info!("Starting HTTP server on {}", listener.local_addr().unwrap());

    let secret_key = Key::generate();

    let server=HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(health_check)
            .service(graphql)
            .service(playground)
            .service(login)
            .service(logout)
            .service(signup)
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .cookie_http_only(true)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::minutes(5)),
                    )
                    .build(),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)

}
