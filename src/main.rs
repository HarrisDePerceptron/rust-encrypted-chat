use actix::Actor;
use actix_web::{web, App, HttpServer};

use encrypted_chat::app::config_app;

use encrypted_chat::server::WebSocketServer;

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{time::Duration, Key};

use encrypted_chat::secrets;

use dotenv::dotenv;

use actix_web::middleware::Logger;
use env_logger::Env;

use encrypted_chat::middleware::auth_middleware;

use encrypted_chat::persistence;
// use futures_util::lock::Mutex;
use std::sync::Mutex;

// use encrypted_chat::business::user::factory::{UserFactory};
// use encrypted_chat::business::user::model::{SignupRequest};
// use encrypted_chat::business::service_redis::{RedisFactory};

// use encrypted_chat::business::application_factory::{FactoryTrait};

use encrypted_chat::auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect(".dot env file unable to load");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let secret_key = Key::from(secrets::SESSION_KEY.as_bytes());

    let pass_hash = auth::hash_password("mypass").unwrap();

    println!("original hash: {}", pass_hash);

    let pass_verify = auth::verify_password_hash("mypass", &pass_hash).unwrap();

    println!("pass_verify: {}", pass_verify);

    let redis_provider_m = Mutex::new(persistence::redis::RedisProvider::new());
    let redis_state = web::Data::new(redis_provider_m);

    let server = WebSocketServer::new();
    let server_addr = server.start();

    let server = HttpServer::new(move || {
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)
                .session_lifecycle(PersistentSession::default().session_ttl(secrets::ONE_MINUTE))
                .build();

        let state = web::Data::new(server_addr.clone());

        App::new()
            .wrap(auth_middleware::SayHi {})
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(IdentityMiddleware::default())
            .wrap(session_mw)
            .app_data(state.clone())
            .app_data(redis_state.clone())
            .configure(config_app)
    })
    .bind(("127.0.0.1", 8085))?
    .run();

    server.await
}
