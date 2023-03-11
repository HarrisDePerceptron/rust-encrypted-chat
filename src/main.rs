use std::collections::{HashSet, HashMap};

use actix_web::{web, App, HttpServer};
use actix::{Actor};


use encrypted_chat::app::config_app;

use encrypted_chat::server::WebSocketServer;

use encrypted_chat::session::commands::{Command, JoinRequest};

use serde_json;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let c = Command::JOIN(
        JoinRequest { channel_name: "hello-channel".to_owned() }
    );


    // let cs: String = serde_json::to_string(&c).unwrap();

    // println!("Serialized: {}", cs);

    // let dc: Command= serde_json::from_str(&cs).unwrap();


    // if let Command::JOIN(dd) = dc {
    //     println!("command join deserialized: {:?}", dd);
    // }


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
