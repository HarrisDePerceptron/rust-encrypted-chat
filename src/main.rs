use std::collections::{HashMap, HashSet};

use actix::Actor;
use actix_web::{web, App, HttpServer};

use encrypted_chat::app::config_app;

use encrypted_chat::server::WebSocketServer;

use encrypted_chat::session::commands::CommandRequest;
use encrypted_chat::utils;

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware, config::PersistentSession};
use actix_web::{
    get, middleware::Logger, post, HttpMessage, HttpRequest, HttpResponse, Responder,
    cookie::{time::Duration, Key}
};



const ONE_MINUTE: Duration = Duration::minutes(1);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();

    let server = WebSocketServer::new();
    let server_addr = server.start();

    HttpServer::new(move || {
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)
                .session_lifecycle(PersistentSession::default().session_ttl(ONE_MINUTE))
                .build();
            
        let state = web::Data::new(server_addr.clone());

        App::new()
        .wrap(IdentityMiddleware::default())
        .wrap(session_mw)
        .app_data(state.clone())
        .configure(config_app)
        .service(index)
        .service(login)
        .service(logout)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}


#[get("/user")]
async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[post("/login")]
async fn login(request: HttpRequest) -> impl Responder {
    // Some kind of authentication should happen here -
    // e.g. password-based, biometric, etc.
    // [...]

    // Attached a verified user identity to the active
    // session.
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::NoContent()
}