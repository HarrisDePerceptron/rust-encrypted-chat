use actix::{Message, Addr};

use crate::websocket_session::{WebSocketSession};
use crate::server::UserSession;



pub struct ServerMessage<T> {
    pub message: T,
    pub session: UserSession
}

impl<T> ServerMessage<T>
where
    T: Message
{
    fn session(&self)-> &UserSession{
        &self.session
    }
}

impl<T> std::ops::Deref for ServerMessage<T> 
where
    T: Message
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl<T> Message for ServerMessage<T> 
where 
    T: Message
{
    type Result = T::Result;
}



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
}
impl Message for Disconnect {
    type Result = ();
}



pub struct Join {
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
