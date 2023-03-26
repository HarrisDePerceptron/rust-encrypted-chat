use std::time::Instant;

use actix::Addr;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

use crate::server::WebSocketServer;
use crate::session::WebSocketSession;

#[get("/ws")]
pub async fn websocket_index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Addr<WebSocketServer>>,
) -> Result<HttpResponse, Error> {

    let headers = req.headers();

    let resp = ws::start(WebSocketSession::new(
        state.get_ref().clone()), &req, stream
    );
    println!("{:?}", resp);
    resp
}
