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


use encrypted_chat::auth;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect(".dot env file unable to load");
    env_logger::init_from_env(Env::default().default_filter_or("info"));


    let secret_key = Key::from(secrets::SESSION_KEY.as_bytes());
    println!("Secret key: {:?}", secret_key.master());

    let token = auth::generate_token(2).unwrap();
    println!("Token is: `{}`", token);


    let decode_claim = auth::decode_token(token).unwrap();

    println!("Decode claim: {:?}", decode_claim);

    // let mut validation = Validation::new(Algorithm::HS256);
    // validation.validate_exp = false;
    // validation.required_spec_claims.remove("exp");


    // let token = decode::<EmptyClaim>(&token, &DecodingKey::from_secret("secret".as_ref()), &validation).unwrap();
    // println!("token got back: {:?}", token);


    // let secret_key = Key::generate();

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
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(IdentityMiddleware::default())
            .wrap(session_mw)
            .app_data(state.clone())
            .configure(config_app)
    })
    .bind(("127.0.0.1", 8085))?
    .run();

    server.await
}
