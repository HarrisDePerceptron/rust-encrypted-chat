use actix::Actor;
use actix_web::{web, App};

use crate::server::WebSocketServer;

use crate::routes::index::{say_hi,send_channel};
use crate::routes::websocket_route::websocket_index;



use crate::routes::auth::{index, login, logout, self};

pub fn config_app(cfg: &mut web::ServiceConfig){
    cfg
        // .app_data(state.clone())
        .service(websocket_index)
        .service(say_hi)
        .service(send_channel)
        .service(index)
        .service(login)
        .service(logout)
        .service(auth::verify)
        .service(auth::signup);

}
