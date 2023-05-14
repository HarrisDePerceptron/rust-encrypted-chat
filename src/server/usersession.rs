
use crate::session::WebSocketSession;
use actix::{Addr};
use crate::middleware::auth_extractor::UserAuthSession;


#[derive(Debug, Clone)]
pub struct UserSession {
    pub session_id: String,
    pub session: Addr<WebSocketSession>,
    pub auth_session: UserAuthSession
}


