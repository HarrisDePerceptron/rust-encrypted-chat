// use crate::server::messages::{Join,TextMessageAll,CountAll,SendChannel};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    JOIN(JoinRequest),
    LEAVE(LeaveRequest),
    BROADCAST(BroadcastRequest),
    COUNT(CountRequest),
    ToChannel(ToChannelRequest)
    

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

// trait CommandT {}

// impl CommandT for Command {

// }


// trait CommandData<C> 
// where 
//     C: CommandT
// {
//     type Comm: 'static;
// }




// impl CommandData<Command> for JoinRequest {
//     type Comm = Command;
// }



// struct WebsocketRequest<C, D>
// where 
//     C: CommandT,
    
// {
//     command: Command<D>,
//     data: D 
// }


