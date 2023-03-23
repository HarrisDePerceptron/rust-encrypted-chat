

use std::fmt::Debug;
use serde::{Serialize};


enum  Response {
    COUNT(ResponseBase<CountResponse>),
    JOIN(ResponseBase<JoinResponse>),
    SendChannel(ResponseBase<SendChannelResponse>),
    Connect(ResponseBase<ConnectResponse>),
    Disconnect(ResponseBase<DisconnectResponse>)
}

#[derive(Debug, Serialize)]
pub struct ResponseBase<T>
where
    T: Debug + Serialize
{
    pub message: String,
    pub data: T,
    pub error: Option<String>    
}



#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub error_message: String,
    pub error_code: i32
}



#[derive(Debug, Serialize)]
pub struct CountResponse{
    count: i32

}


#[derive(Debug, Serialize)]
pub struct JoinResponse{
    
}


#[derive(Debug, Serialize)]
pub struct SendChannelResponse{
    
}

#[derive(Debug, Serialize)]
pub struct DisconnectResponse{
    
}


#[derive(Debug, Serialize)]
pub struct ConnectResponse{
    
}


