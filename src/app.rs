pub mod user;
pub mod application_model;
pub mod application_service;


pub mod service_redis;
pub mod application_factory;




use actix::Actor;
use actix_web::{web, App};

use crate::server::WebSocketServer;

use crate::routes::index::{say_hi,send_channel};
use crate::routes::websocket_route::websocket_index;


// use crate::routes::auth::{index, login, logout, self};
// use crate::business::user::routes::{index, login, logout, verify};



pub fn config_app(cfg: &mut web::ServiceConfig){
    cfg
        // .app_data(state.clone())
        .service(websocket_index)
        .service(say_hi)
        .service(send_channel)
        .service(user::routes::index)
        .service(user::routes::login)
        .service(user::routes::logout)
        .service(user::routes::verify)
        .service(user::routes::signup);

}
