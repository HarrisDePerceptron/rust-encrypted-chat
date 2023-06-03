pub mod user;
pub mod application_model;
pub mod application_service;
pub mod application_factory;
pub mod  application_utils;
pub mod websocket;
pub mod chat;
pub mod redis;


use actix_web::{web};


use crate::routes::index::{say_hi,send_channel};
use crate::routes::websocket_route::websocket_index;
use crate::routes::index;


// use crate::routes::auth::{index, login, logout, self};
// use crate::business::user::routes::{index, login, logout, verify};



pub fn config_app(cfg: &mut web::ServiceConfig){
    cfg
        // .app_data(state.clone())
        .service(websocket_index)
        .service(say_hi)
        .service(index::list_channels)
        .service(send_channel)
        .service(user::routes::index)
        .service(user::routes::login)
        .service(user::routes::logout)
        .service(user::routes::verify)
        .service(user::routes::signup);

}
