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

