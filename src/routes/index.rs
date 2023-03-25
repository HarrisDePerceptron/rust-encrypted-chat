
use actix_web::{web, get, HttpResponse, Responder};

use actix::{Addr};
use crate::server::messages::{TextMessageAll, SendChannel};
use crate::server::WebSocketServer;





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


#[get("/channel/{channel}")]
pub async fn send_channel(path: web::Path<(String,)>,state: web::Data<Addr<WebSocketServer>>) -> impl Responder {
    let (channel, )=  path.into_inner();
    
    if let Err(e) = state
        .send(SendChannel {
            channel_name: channel,
            msg: "hi channel".to_owned(),
        })
        .await
    {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().body("sendding hi to conencted sockets")
}

