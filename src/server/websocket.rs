
use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient, Addr};

use crate::messages::websocket_session_messages::{TextMessage};
use crate::server::messages::{Connect,TextMessageAll,CountAll,Disconnect, Join};
use crate::server::{UserSession, Channel};

use super::messages::SendChannel;


pub struct WebSocketServer {
    pub index: usize,
    pub sessions: HashMap<String, UserSession>,
    pub channels: HashMap<String, Channel>,
    pub ch_id: usize
}



impl Actor for WebSocketServer {
    type Context = Context<Self>;
}

impl WebSocketServer {
    pub fn notify_all(&self, message: &str) {
        for (session_id, session) in &self.sessions {
            let res: Recipient<TextMessage> = session.session.clone().recipient();
            res.do_send(TextMessage {
                message: message.to_owned(),
            });
        }
    }

    pub fn new() -> Self {
        let mut channels = HashMap::new();
        channels.insert("default".to_string(), Channel::new("0", "default"));

        Self{
            index: 0,
            channels: channels,
            sessions: HashMap::new(),
            ch_id: 0
        }
    }

    fn join_channel_default(&mut self, user_session: &UserSession) {
        self.join_channel("default", user_session)
    }

    pub fn join_channel (&mut self, name: &str, user_session: &UserSession) -> (){
        let ch = self.channels.get_mut(name);

        if let Some(ch) = ch {
           ch.add_session(user_session);
        }else{
            let mut ch_default = Channel::new(&self.ch_id.to_string(), name);
            ch_default.add_session(user_session);
            self.channels.insert(name.to_string(), ch_default);
        }

    }


    pub fn get_channel(&self ,channel: &str) -> Option<&Channel> {
        return self.channels.get(channel);
    }

    pub fn register_session(&mut self,session: &UserSession){
        self.sessions.insert(session.session_id.clone(),  session.clone());

    }

    pub fn get_session(&self,session_id: &str) -> Option<&UserSession> {
        return self.sessions.get(session_id);
    }

    pub fn remove_session(&mut self,sess: &UserSession)-> Option<UserSession>{
        self.sessions.remove(&sess.session_id)
    }

    pub fn remove_session_channels(&mut self, session:  &UserSession){
        for (name, ch) in self.channels.iter_mut() {

            let res = ch.remove_session(session);
            if let Some(_) = res {
                println!("Removed session from channel: {}", name);
            }
        }
    }

    pub fn send_to_channel(&self, channel: &str, message: &str){
        let ch = self.get_channel(channel);
        if let Some(ch) = ch {
            ch.send(message);
        }
        
    }

}



impl Handler<Connect> for WebSocketServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        println!("Connecting new websocket session...");
        
        self.register_session(&msg.0);


        self.index += 1;

        self.join_channel_default(&msg.0);


        return self.index;
    }
}


impl Handler<TextMessageAll> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: TextMessageAll, ctx: &mut Self::Context) -> Self::Result {
        self.notify_all(&msg.message);
    }
}


impl Handler<CountAll> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: CountAll, ctx: &mut Self::Context) -> Self::Result {
        self.notify_all(self.index.to_string().as_str());
    }
}


impl Handler<Disconnect> for WebSocketServer {
    type Result=();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {


        println!("Chatserver: disconnecting with actor id: {}", msg.session.session_id);
        self.remove_session(&msg.session);
        self.remove_session_channels(&msg.session);

    }
}


impl Handler<Join> for WebSocketServer {
    type Result=();

    fn handle(&mut self, msg: Join, ctx: &mut Self::Context) -> Self::Result {
        println!("Joining channel...");

        let sess = self.get_session(&msg.session_id);

        if let Some(sess) = sess {
            self.join_channel(&msg.name, &sess.to_owned());

        }
        
    }
}


impl Handler<SendChannel> for WebSocketServer {
    type Result=();

    fn handle(&mut self, msg: SendChannel, ctx: &mut Self::Context) -> Self::Result {
        println!("Sending to channel...");
        self.send_to_channel(&msg.channel_name, &msg.message);
    }
}
