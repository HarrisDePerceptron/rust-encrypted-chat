use actix::{Message};
use crate::server::server_response::{ServerResponse};
use serde::{Serialize};
use std::fmt::{Debug};

#[derive(Debug, Serialize)]
pub struct TextMessage {
    pub message: String,
    pub data: Option<ServerResponse>
}


impl Message for TextMessage {
    type Result = ();
}