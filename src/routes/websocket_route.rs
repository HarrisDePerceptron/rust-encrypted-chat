use std::{fmt::format, time::Instant};

use actix::Addr;
use actix_web::http::header::HeaderMap;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web_actors::ws;

use crate::server::WebSocketServer;
use crate::session::WebSocketSession;

use crate::auth;
use actix_identity::Identity;
use crate::middleware::util_middleware;



#[get("/ws")]
pub async fn websocket_index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Addr<WebSocketServer>>,
    cookie: Option<Identity>,
) -> Result<HttpResponse, Error> {
    let headers = req.headers();

    let authorization = headers.get("Authorization");

    let mut token_jwt: String = String::new();

    let mut cookie_available = false;

    let mut auth_header_available = false;

    let mut token_jwt_cookie: String = String::new();
    let mut token_jwt_header: String = String::new();

    let mut cookie_errors = String::new();


    match util_middleware::parse_cookie(&cookie) {
        Err(e) => {
            cookie_errors = e;
        }
        Ok(v) => {
            token_jwt_cookie = v;
            cookie_available = true;
        }
    };


    let mut auth_headers_errors = String::new();

    match util_middleware::parse_header_token(&headers) {
        Err(e) => {
            auth_headers_errors = e;
        }
        Ok(v) => {
            token_jwt_header = v;
            auth_header_available = true;
        }
    };

    if !cookie_available && !auth_header_available {

        let errors = format!("Unauthorized. auth not found in cookie or auth header: {}, {}", cookie_errors, auth_headers_errors);
        return Ok(HttpResponse::Unauthorized()
            .body(errors));
    }

    if cookie_available {
        token_jwt = token_jwt_cookie;
    } else if auth_header_available {
        token_jwt = token_jwt_header;
    }

    let token_decode = match auth::decode_token(token_jwt) {
        Err(e) => return Ok(HttpResponse::Unauthorized().body(e.to_string())),
        Ok(v) => v,
    };

    let user_id = token_decode.sub;

    println!("Got token: {:?}", user_id);

    let resp = ws::start(WebSocketSession::new(state.get_ref().clone()), &req, stream);

    println!("{:?}", resp);
    resp
}
