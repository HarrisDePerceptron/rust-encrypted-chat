use actix_web::{web, App, HttpServer};
use actix::{Actor};

use encrypted_chat::websocket_server::WebSocketServer;

use encrypted_chat::routes::websocket_route::{websocket_index};
use encrypted_chat::routes::index::{say_hi};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = WebSocketServer {
        index: 0,
        sessions: vec![],
    };

    let server_addr = server.start();

    let state = web::Data::new(server_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(websocket_index)
            .service(say_hi)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
