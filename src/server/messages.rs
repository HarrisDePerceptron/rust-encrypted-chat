use actix::{Message};


use crate::server::UserSession;



use super::model;


pub struct ServerMessage<T> {
    pub message: T,
    pub session: UserSession,
    pub message_id: String
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
    pub msg: String
}
impl Message for SendChannel {
    type Result = ();
}


pub struct ListChannel {
    // pub channel_name: String
}


impl Message for ListChannel {
    type Result =  Vec<model::ChannelListResponse>;
}

#[derive(Message)]
#[rtype(result="()")]
pub struct ErrorMessage {
    pub error_message: String,
    pub error_code: i32
}