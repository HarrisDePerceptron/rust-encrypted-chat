use std::time::Instant;

use actix::{Addr};
use actix_web::{Responder, HttpResponse, web, get, HttpRequest, Error };
use actix_web_actors::ws;


use crate::server::WebSocketServer;
use crate::session::WebSocketSession;


#[get("/ws/")]
pub async fn websocket_index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Addr<WebSocketServer>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        // WebSocketSession {
        //     server: state.get_ref().clone(),
        //     id: 0,
        //     hb: Instant::now()
        // },
        WebSocketSession::new(state.get_ref().clone()),
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}

