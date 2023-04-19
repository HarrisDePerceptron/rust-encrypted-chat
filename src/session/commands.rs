use std::fmt::Debug;

// use crate::server::messages::{Join,TextMessageAll,CountAll,SendChannel};
use serde::{Serialize, Deserialize};

pub struct ErrorCode;


impl ErrorCode {
    pub const  ZERO:i32  = 0;
    pub const  NOT_TEXT_MESSAGE:i32  = 1;

}


#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRequestError {
    pub message: String,
    pub code: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub enum CommandRequest {
    JOIN(JoinRequest),
    LEAVE(LeaveRequest),
    BROADCAST(BroadcastRequest),
    COUNT(CountRequest),
    ToChannel(ToChannelRequest),
    ListChannels(ListChannelRequest)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListChannelRequest {
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ToChannelRequest {
    pub channel_name: String,
    pub message: String
}



#[derive(Serialize, Deserialize, Debug)]
pub struct JoinRequest {
    pub channel_name: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveRequest {
    pub channel_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastRequest {
    pub message: String
}




#[derive(Serialize, Deserialize, Debug)]
pub struct CountRequest {

}