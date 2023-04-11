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
use std::sync::{Mutex};



// use encrypted_chat::business::user::service::{UserService, UserServiceError};
// use encrypted_chat::business::user::model::{User, SignupRequest};
// use encrypted_chat::business::service_redis::{RedisApplicationService};


use encrypted_chat::business::user::factory::{UserFactory};
use encrypted_chat::business::user::model::{SignupRequest};
use encrypted_chat::business::service_redis::{RedisFactory};

use encrypted_chat::business::application_factory::{FactoryTrait};




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect(".dot env file unable to load");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let mut provider = persistence::redis::RedisProvider::new();

    // let mut redis_service = RedisApplicationService::new("user", &mut provider);
    // let mut u_f = UserFactory::new(&mut redis_service);
    

    let mut r_f = RedisFactory::new("user", &mut provider);
    let mut u_f = UserFactory::new(r_f.get());


    let user_service = u_f.get();


    
    // let mut user_service = UserService::new(&mut redis_service);

    let user = SignupRequest{
        username: "user1".to_string(),
        password: "password1".to_string()
    };
    

    let res = user_service.signup(user).await.unwrap();
    println!("application user created: {:?}", res);

    let res = user_service.get(20).await.unwrap();
    println!("application users get: {:?}", res);


    // let mut m = res[1].clone();
    // m.data.password = "new password 101".to_string();

    // m.id = Some("whdsdjask".to_string());

    // println!("udpating with id: {:?}", m.id);

    // let res = user_service.update(m).await.unwrap();
    // println!("application users update: {:?}", res);
   
    
    let secret_key = Key::from(secrets::SESSION_KEY.as_bytes());

    // let conn  = persistence::redis::connect().await.unwrap();

    let err = std::io::Error::new(
        std::io::ErrorKind::ConnectionReset,
        "Redis connection error".to_string(),
    );

    // let conn = provider.get_connection().await.map_err(|e| err)?;

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
