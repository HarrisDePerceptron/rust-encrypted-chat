use actix::{Message, Addr};

use crate::websocket_session::{WebSocketSession};



pub struct Connect(pub Addr<WebSocketSession>);
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
