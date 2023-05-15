use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message, Recipient, ResponseFuture, WrapFuture};
use crate::server::messages::{
    Connect, CountAll, Disconnect, Join, SendChannel, ServerMessage, TextMessageAll,
};
use crate::server::{Channel, UserSession};
use crate::session::{TextMessage, WebSocketSession};

use super::messages::ErrorMessage;

use crate::server::server_response;
use super::server_response::{
    ConnectResponse, CountResponse, ResponseBase, ServerResponse
};
use crate::server::channel;


use crate::utils;
use super::messages;
use super::model;
use super::websocket_provider_redis::WebsocketPersistence;

use crate::app::websocket::{WebsocketService,WebsocketServiceTrait, ChannelData};

use actix::AsyncContext;
use actix::prelude::*;


pub struct WebSocketServer {
    pub index: usize,
    pub sessions: HashMap<String, UserSession>,
    pub channels: HashMap<String, Channel>,
    pub ch_id: usize,
    
}

impl Actor for WebSocketServer {
    type Context = Context<Self>;
}

impl WebSocketServer {
    pub fn notify_all(&self, message: &str) {
        for (_session_id, session) in &self.sessions {
            let res: Recipient<TextMessage> = session.session.clone().recipient();
            res.do_send(TextMessage {
                message: message.to_owned(),
                data: None,
            });
        }
    }

    pub fn new() -> Self {
        let mut channels = HashMap::new();
        channels.insert("default".to_string(), Channel::new("default"));       
        
        
        Self {
            index: 0,
            channels: channels,
            sessions: HashMap::new(),
            ch_id: 0,
        }
    }

    fn add_channel(&mut self, ch: Channel) -> Result<(), String>{

        if self.channels.contains_key(&ch.name) {
           return Err(format!("channel {} already exisits", ch.name));
        }

        self.channels.insert(ch.name.to_string(), ch);

        Ok(())
        
    }

    async fn join_channel_default(
        &mut self,
        user_session: &UserSession,
    ) -> Result<(), model::WebsocketServerError> 
    {
        self.join_channel("default", user_session)
    }

    pub fn join_channel(
        &mut self,
        name: &str,
        user_session: &UserSession,
    ) 
    -> Result<(), model::WebsocketServerError>
        // -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), model::WebsocketServerError>>>>
     {
        let ch = self.channels.get_mut(name);


        let channel =  match ch{
            Some(v) => {
                v.add_session(user_session)
                .map_err(|e|model::WebsocketServerError::SessionChannellAddError(format!("{:?}", e)))?;
                v.clone()
            },
            None =>  {
                let mut ch_default = Channel::new(name);
                ch_default.add_session(user_session)
                .map_err(|e|model::WebsocketServerError::SessionChannellAddError(format!("{:?}", e)))?;


                self.channels.insert(name.to_string(), ch_default.clone());
                ch_default
            }
        };

            Ok(())
                
        
    }

    pub fn get_channel(&self, channel: &str) -> Option<&Channel> {
        return self.channels.get(channel);
    }

    pub fn register_session(&mut self, session: &UserSession) {
        self.sessions
            .insert(session.session_id.clone(), session.clone());
    }

    pub fn get_session(&self, session_id: &str) -> Option<&UserSession> {
        return self.sessions.get(session_id);
    }

    pub fn get_session_address(&self, session: &UserSession) -> Option<UserSession> {
        for (_, sess) in &self.sessions {
            if sess.session == session.session {
                return Some(sess.to_owned());
            }
        }

        None
    }

    pub fn get_address_session(&self, addr: Addr<WebSocketSession>) -> Option<UserSession> {
        for (_, sess) in &self.sessions {
            if sess.session == addr {
                return Some(sess.to_owned());
            }
        }

        None
    }

    pub fn remove_session(&mut self, sess: &UserSession) -> Option<UserSession> {
        self.sessions.remove(&sess.session_id)
    }

    pub fn remove_session_channels(&mut self, session: &UserSession) {
        for (name, ch) in self.channels.iter_mut() {
            let res = ch.remove_session(session);
            if let Some(_) = res {
                println!("Removed session from channel: {}", name);
            }
        }
    }

    pub fn send_to_channel(&self, channel: &str, message: &str, data: Option<ServerResponse>) {
        let ch = self.get_channel(channel);
        if let Some(ch) = ch {
            ch.send(message, data);
        }
    }

    pub fn send_to_session(
        &self,
        session: &UserSession,
        message: &str,
        data: Option<ServerResponse>,
    ) {
        let sess = self.get_session_address(session);
        if let Some(sess) = sess {
            sess.session.do_send(TextMessage {
                message: message.to_string(),
                data: data,
            });
        }
    }

    pub fn send_error(&self, session: &UserSession, message: ErrorMessage) -> Result<(), String> {
        let sess = match self.get_session_address(session) {
            Some(v) => v,
            None => return Err("Session not found".to_string())
        };

        let message_id = match utils::generate_unique_id(){
            Err(e)=> return Err(e.to_string()),
            Ok(v)=> v
        };

        let response = server_response::ServerResponse::ERROR(server_response::ResponseBase {
            message: "error".to_string(),
            message_id: message_id, 
            data: server_response::ResponseError {
                error_code: message.error_code.to_owned(),
                error_message: message.error_message.to_owned(),
            },
        });


        self.send_to_session(&sess,&message.error_message, Some(response));


        Ok(())
    }
    
    pub fn list_channels(&self) -> Vec<model::ChannelListResponse>{
        let mut  chs: Vec<model::ChannelListResponse> = Vec::new();
        for (_key, ch) in &self.channels {
            let channel_name= ch.name.to_owned();
            let channel_id = ch.id.to_owned();

            let mut channel_users: Vec<model::ChannelUser> = Vec::new();

            for sess in &ch.sessions {
                let channel_user = model::ChannelUser {
                    user_id: sess.auth_session.user_id.to_owned(),
                    user_name: sess.auth_session.username.to_owned()
                };

                channel_users.push(channel_user);

            }

            let channel_list_respose = model::ChannelListResponse {
                channel_id: channel_id,
                channel_name: channel_name,
                users: channel_users
            };


            chs.push(channel_list_respose);

        }

        return chs;
        
    }

    pub fn store_channel(channel: Channel,_self: &Self,  ctx: &mut Context<Self>){

        async {
            let mut service = WebsocketService::new();
            let data = ChannelData {
                name: channel.name
            };
    
            service.store(data)
                .await
                .map_err(|e| model::WebsocketServerError::ChannelStoreError(e.to_string()))

        }.into_actor(_self)
        .then(|res, _self, ctx| {
            
            if let Err(e) = res {
                print!("Failed to store channel: {}", e.to_string());
            }
            actix::fut::ready(())
        }).wait(ctx);

        ()
    } 
}

impl Handler<ServerMessage<Connect>> for WebSocketServer {
    type Result = usize;

    fn handle(&mut self, msg: ServerMessage<Connect>, _ctx: &mut Self::Context) -> Self::Result {
        println!("Connecting new websocket session...");

        self.register_session(&msg.0);

        self.index += 1;

        self.join_channel_default(&msg.0);

        let connect_response = ServerResponse::CONNECT(ResponseBase {
            message: "connected".to_string(),
            message_id: msg.message_id,
            data: ConnectResponse {},
        });

        self.send_to_session(&msg.session, "connected", Some(connect_response));

        return self.index;
    }
}

impl Handler<TextMessageAll> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: TextMessageAll, _ctx: &mut Self::Context) -> Self::Result {
        self.notify_all(&msg.message);
    }
}

impl Handler<ServerMessage<CountAll>> for WebSocketServer {
    type Result = <ServerMessage<CountAll> as Message>::Result;

    fn handle(&mut self, msg: ServerMessage<CountAll>, _ctx: &mut Self::Context) -> Self::Result {
        let count: usize = self.sessions.keys().len();
        let data = ServerResponse::COUNT(ResponseBase {
            message_id: msg.message_id,
            message: "the count".to_owned(),
            data: CountResponse { count: count },
        });

        self.send_to_session(&msg.session, "the count", Some(data));
    }
}

impl Handler<ServerMessage<Disconnect>> for WebSocketServer {
    type Result = <ServerMessage<Disconnect> as Message>::Result;

    fn handle(&mut self, msg: ServerMessage<Disconnect>, _ctx: &mut Self::Context) -> Self::Result {
        println!(
            "Chatserver: disconnecting with actor id: {}",
            msg.session.session_id
        );
        self.remove_session(&msg.session);
        self.remove_session_channels(&msg.session);
    }
}

impl Handler<ServerMessage<Join>> for WebSocketServer {
    type Result = <ServerMessage<Join> as Message>::Result;

    fn handle(&mut self, msg: ServerMessage<Join>, _ctx: &mut Self::Context) -> Self::Result {
        println!("Joining channel...");
        let sess = self.get_session(&msg.session.session_id);
        let channel_name = msg.name.to_string();


        let user_session = match sess {
            Some(v) => v.to_owned(),
            None => {println!("Unable to fetch user serssion"); return}
        };

        match self.join_channel(&msg.name, &user_session) {
            Err(e) => {
                let msg = format!("{:?}", e);
                // self.send_to_session(&user_session, &msg, None);
                match self.send_error(&user_session, ErrorMessage { error_message: msg, error_code: 0 }){
                    Err(e) => {println!("Join error: {}", e); return;},
                    _ => ()
                };
                return;
            }
            Ok(_) => (),
        }

        let response_message = format!("Joined channel {}", msg.name);
        let data = server_response::ServerResponse::JOIN(server_response::ResponseBase {
            message: response_message.to_owned(),
            message_id: msg.message_id,
            data: server_response::JoinResponse {},
        });

        self.send_to_session(&msg.session, &response_message, Some(data));
        

        let channel = match self.get_channel(&channel_name){
            None => {
                self.send_error(&user_session, ErrorMessage { error_message: format!("Failed to fetch channel {}", &channel_name), error_code: 0 })
                    .unwrap_or_else(|e| println!("Unable to send error: {}", e));

                return;
            },
            Some(v) => v.to_owned()
        };

        Self::store_channel(channel, &self, _ctx);

        
    }
}

impl Handler<SendChannel> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: SendChannel, _ctx: &mut Self::Context) -> Self::Result {
        println!("Sending to channel...");

        self.send_to_channel(&msg.channel_name, &msg.msg, None);
    }
}

impl Handler<ServerMessage<SendChannel>> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage<SendChannel>, _ctx: &mut Self::Context) -> Self::Result {
        println!("Sending to channel...");

        let response_message = format!("Sent to channel {}", msg.channel_name);
        let data = server_response::ServerResponse::SendChannel(server_response::ResponseBase {
            message: msg.msg.to_owned(),
            message_id: msg.message_id.to_owned(),
            data: server_response::SendChannelResponse {},
        });
        self.send_to_channel(&msg.channel_name, &response_message, Some(data));
    }
}

impl Handler<messages::ListChannel> for WebSocketServer {
    type Result =  ResponseFuture<Vec<model::ChannelListResponse>>;

    fn handle(&mut self, _msg: messages::ListChannel, _ctx: &mut Self::Context) -> Self::Result {
        let response = self.list_channels();

        Box::pin (
            async move {
                return response;
            }
        )
    }
}


impl Handler<ServerMessage<messages::ListChannel>> for WebSocketServer {
    type Result =  ResponseFuture<Vec<model::ChannelListResponse>>;

    fn handle(&mut self, msg: ServerMessage<messages::ListChannel>, _ctx: &mut Self::Context) -> Self::Result {
        let response = self.list_channels();

        let data = ServerResponse::ListChannels(ResponseBase {
            message_id: msg.message_id,
            message: "Channel listing".to_owned(),
            data: response.clone(),
        });

        self.send_to_session(&msg.session, "All channel list", Some(data));


        Box::pin (
            async move {
                return response;
            }
        )
    }
}
