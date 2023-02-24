
use actix_web::{web, get, HttpResponse, Responder};

use actix::{Addr};
use crate::messages::websocket_server_messages::{TextMessageAll};
use crate::websocket_server::WebSocketServer;



#[get("/hi")]
pub async fn say_hi(state: web::Data<Addr<WebSocketServer>>) -> impl Responder {
    if let Err(e) = state
        .send(TextMessageAll {
            message: "hiya everyone".to_owned(),
        })
        .await
    {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().body("sendding hi to conencted sockets")
}

