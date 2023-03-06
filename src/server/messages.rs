use actix::{Message, Addr};

use crate::websocket_session::{WebSocketSession};
use crate::server::UserSession;



pub struct Connect (pub UserSession);

impl Message for Connect {
    type Result = usize;
}



pub struct TextMessageAll {
    pub message: String,
}
impl Message for TextMessageAll {
    type Result = ();
}



pub struct CountAll {}
impl Message for CountAll {
    type Result = ();
}


pub struct Disconnect {
    pub session: UserSession
}
impl Message for Disconnect {
    type Result = ();
}



pub struct Join {
    pub session_id: String,
    pub name: String
}
impl Message for Join {
    type Result = ();
}



pub struct SendChannel {
    pub channel_name: String,
    pub message: String
}
impl Message for SendChannel {
    type Result = ();
}
