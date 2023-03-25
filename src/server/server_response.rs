

use std::fmt::Debug;
use serde::{Serialize};


#[derive(Debug, Serialize, Clone)]
pub enum  ServerResponse {
    COUNT(ResponseBase<CountResponse>),
    JOIN(ResponseBase<JoinResponse>),
    SendChannel(ResponseBase<SendChannelResponse>),
    Connect(ResponseBase<ConnectResponse>),
    Disconnect(ResponseBase<DisconnectResponse>)
}

#[derive(Debug, Serialize,Clone)]
pub struct ResponseBase<T>
where
    T: Debug + Serialize
{
    pub message: String,
    pub data: T,
}



#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub error_message: String,
    pub error_code: i32
}



#[derive(Debug, Serialize, Clone)]
pub struct CountResponse{
    pub count: usize

}


#[derive(Debug, Serialize, Clone)]
pub struct JoinResponse{
    
}


#[derive(Debug, Serialize, Clone)]
pub struct SendChannelResponse{
    
}

#[derive(Debug, Serialize, Clone)]
pub struct DisconnectResponse{
    
}


#[derive(Debug, Serialize, Clone)]
pub struct ConnectResponse{
    
}


