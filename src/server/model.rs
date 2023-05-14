use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct ChannelUser {
    pub user_id: String,
    pub user_name: String
}



#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct ChannelListResponse{
    pub channel_id: String,
    pub channel_name: String,
    pub users: Vec<ChannelUser>

}



#[derive(Debug, Clone, Serialize)]
pub enum WebsocketServerError {
    ChannelNotFound(String),
    SessionChannellAddError(String),
    ChannelStoreError(String)
}

impl std::fmt::Display for WebsocketServerError
where
    Self: std::fmt::Debug + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std:: fmt::Result {
        let response = serde_json::to_string(&self)
            .map_err(|_e| std::fmt::Error::from(std::fmt::Error))?;

        f.write_str(&response)
    }
}
