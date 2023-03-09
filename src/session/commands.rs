// use crate::server::messages::{Join,TextMessageAll,CountAll,SendChannel};

pub enum Command {
    JOIN(JoinRequest),
    LEAVE(LeaveRequest),
    BROADCAST(BroadcastRequest),
    COUNT(CountRequest),
    ToChannel(ToChannelRequest)
    

}

pub struct ToChannelRequest {
    pub channel_name: String,
    pub message: String
}


pub struct JoinRequest {
    pub channel_name: String
}

pub struct LeaveRequest {
    pub channel_name: String
}

pub struct BroadcastRequest {
    pub message: String
}

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


