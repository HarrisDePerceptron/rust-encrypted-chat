

use actix::Addr;

use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::server::WebSocketServer;
use crate::session::WebSocketSession;


use actix_identity::Identity;

use crate::middleware::auth_extractor;


#[get("/ws")]
pub async fn websocket_index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Addr<WebSocketServer>>,
    _cookie: Option<Identity>,
    auth: auth_extractor::UserAuthSession
) -> Result<HttpResponse, Error> {
    let _headers = req.headers();


    let resp = ws::start(WebSocketSession::new(state.get_ref().clone(), auth), &req, stream);

    println!("{:?}", resp);
    resp
}

