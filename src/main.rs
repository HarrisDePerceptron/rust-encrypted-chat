use std::collections::{HashSet, HashMap};

use actix_web::{web, App, HttpServer};
use actix::{Actor};


use encrypted_chat::app::config_app;

use encrypted_chat::server::WebSocketServer;

use encrypted_chat::session::commands::{CommandRequest};
use encrypted_chat::utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let uid = utils::generate_unique_id().expect("unable to generate uuid");
    println!("Your uuid is {}", uid );


    let server =  WebSocketServer::new();
    let server_addr = server.start();
   

   HttpServer::new(move || {
    
        let state = web::Data::new(server_addr.clone());

        App::new()
            .app_data(state.clone())
            .configure(config_app)

    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
