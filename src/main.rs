#![allow(dead_code)]

use actix::Actor;
use actix_web::{web, App, HttpServer};

use encrypted_chat::app::config_app;

use encrypted_chat::server::WebSocketServer;

use actix_identity::{IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{Key};

use encrypted_chat::secrets;

use dotenv::dotenv;

use actix_web::middleware::Logger;
use env_logger::Env;

use encrypted_chat::persistence;
use std::sync::{Mutex,Arc};

use encrypted_chat::app::user::factory::{UserFactory};
use encrypted_chat::app::application_factory::{ServiceFactory};


use encrypted_chat::app::websocket::{WebsocketService, WebsocketServiceTrait};

use clap::Parser;

use log::{info, warn};


use encrypted_chat::server::websocket_redis_adpter::{RedisWebsocketAdapter, WebsocketAdapter};
use encrypted_chat::application_factory;
use encrypted_chat::app::chat::service::ChatService;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 8085)]
    port: u16,

    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,

    
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let port = args.port;
    let host = args.host;
    
    std::env::set_var("RUST_LOG", "debug");
    dotenv().ok().expect(".dot env file unable to load");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

   

    let secret_key = Key::from(secrets::SESSION_KEY.as_bytes());

    let redis_provider_m = Mutex::new(persistence::redis::RedisProvider::new());
    let redis_state = web::Data::new(redis_provider_m);

    let mut server = WebSocketServer::new();

    let redis_adpter = Arc::new(RedisWebsocketAdapter::new("encrypted",persistence::redis::RedisProvider::new() ));
    server.add_adapter(redis_adpter);


    let server_addr = server.start();

    let user_factory = UserFactory::new();


    let sf = ServiceFactory {
        user: user_factory,

    };

    let factory_state = web::Data::new(sf);

    let application_factory = application_factory::ApplicationFactory::new();
    let ch = ChatService::new(application_factory.mongo_provider.clone());

    let u = ch.create_user("harris").await.unwrap();
    println!("Created user: {}", u.to_string());

    // let us = ch.list_users(2).await.unwrap();
    // println!("Users: {:?}", us);
    
    let server = HttpServer::new(move || {
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)
                .session_lifecycle(PersistentSession::default().session_ttl(secrets::ONE_MINUTE))
                .build();

        let state = web::Data::new(server_addr.clone());

        App::new()
            // .wrap(auth_middleware::SayHi {})
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(IdentityMiddleware::default())
            .wrap(session_mw)
            .app_data(state.clone())
            .app_data(redis_state.clone())
            .app_data(factory_state.clone())
            .configure(config_app)
    })
    .bind((host.to_string(), port))?
    .run();


    info!("Using host {} and port {}", host, port);


    server.await
}
