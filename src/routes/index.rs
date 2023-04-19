
use actix_web::{web, get, HttpResponse, Responder, Result};

use actix::{Addr};
use crate::server::messages::{TextMessageAll, SendChannel};
use crate::server::WebSocketServer;


use crate::app::application_model::{RouteResponseOk, RouteResponse, RouteResponseError, RouteResponseErrorDefault};

use crate::server::messages;



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

#[get("/channel")]
pub async fn list_channels(state: web::Data<Addr<WebSocketServer>>) -> Result<impl Responder> {
    let channel_list =  state.send(messages::ListChannel{})
        .await
        .map_err(|e| RouteResponseErrorDefault(e.to_string()))?;

    return Ok(RouteResponse::Ok(channel_list));


    
}

