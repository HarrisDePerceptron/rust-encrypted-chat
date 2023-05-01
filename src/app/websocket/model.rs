use std::fmt::{Debug};
use serde::{Serialize, Deserialize};
use crate::app::application_model::ApplicationModel;



#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct WebsocketChannel {
    channel_id: String,
    channel_name: String
}


#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct WebsocketUser {
    user_id: String,
    user_name: String
}

#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct Websocket {
    channel: WebsocketChannel,
    users: Vec<WebsocketUser>

}


