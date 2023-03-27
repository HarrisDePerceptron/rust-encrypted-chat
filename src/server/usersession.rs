
use crate::session::WebSocketSession;
use actix::{Addr};

#[derive(Clone)]
pub struct UserSession {
    pub session_id: String,
    pub session: Addr<WebSocketSession>
}


