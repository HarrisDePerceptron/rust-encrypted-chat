

use std::fmt::Debug;
use serde::{Serialize};


#[derive(Debug, Serialize, Clone)]
pub enum  ServerResponse {
    COUNT(ResponseBase<CountResponse>),
    JOIN(ResponseBase<JoinResponse>),
    SendChannel(ResponseBase<SendChannelResponse>),
    CONNECT(ResponseBase<ConnectResponse>),
    DISCONNECT(ResponseBase<DisconnectResponse>),
    ERROR(ResponseBase<ResponseError>)
}

#[derive(Debug, Serialize,Clone)]
pub struct ResponseBase<T>
where
    T: Debug + Serialize
{
    pub message: String,
    pub data: T,
    pub message_id: String
    
}



#[derive(Debug, Serialize, Clone)]
pub struct ResponseError {
    pub error_message: String,
    pub error_code: i32
}

#[derive(Debug, Serialize)]
pub enum ServerResponseType {
    ERROR(ResponseError),
    RESPONSE(ServerResponse)
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


