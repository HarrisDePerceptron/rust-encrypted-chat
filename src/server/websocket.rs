use std::collections::HashMap;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, Recipient};
use actix_web::cookie::time::util;

use crate::server::messages::{
    Connect, CountAll, Disconnect, Join, SendChannel, ServerMessage, TextMessageAll,
};
use crate::server::{Channel, UserSession};
use crate::session::{ErrorCode, TextMessage, WebSocketSession};

use super::messages::ErrorMessage;
use super::server_response::{self, ResponseError};
use super::server_response::{
    ConnectResponse, CountResponse, ResponseBase, SendChannelResponse, ServerResponse,
};
use crate::server::channel;

use crate::session::websocket_session::SessionContext;
use crate::utils;

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
        for (session_id, session) in &self.sessions {
            let res: Recipient<TextMessage> = session.session.clone().recipient();
            res.do_send(TextMessage {
                message: message.to_owned(),
                data: None,
            });
        }
    }

    pub fn new() -> Self {
        let mut channels = HashMap::new();
        channels.insert("default".to_string(), Channel::new("0", "default"));

        Self {
            index: 0,
            channels: channels,
            sessions: HashMap::new(),
            ch_id: 0,
        }
    }

    fn join_channel_default(
        &mut self,
        user_session: &UserSession,
    ) -> Result<(), channel::ChannelError> {
        self.join_channel("default", user_session)
    }

    pub fn join_channel(
        &mut self,
        name: &str,
        user_session: &UserSession,
    ) -> Result<(), channel::ChannelError> {
        let ch = self.channels.get_mut(name);

        if let Some(ch) = ch {
            match ch.add_session(user_session) {
                Err(e) => return Err(e),
                _ => (),
            }
        } else {
            let mut ch_default = Channel::new(&self.ch_id.to_string(), name);
            match ch_default.add_session(user_session) {
                Err(e) => return Err(e),
                _ => (),
            };

            self.channels.insert(name.to_string(), ch_default);
        }

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
}

impl Handler<ServerMessage<Connect>> for WebSocketServer {
    type Result = usize;

    fn handle(&mut self, msg: ServerMessage<Connect>, ctx: &mut Self::Context) -> Self::Result {
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

    fn handle(&mut self, msg: TextMessageAll, ctx: &mut Self::Context) -> Self::Result {
        self.notify_all(&msg.message);
    }
}

impl Handler<ServerMessage<CountAll>> for WebSocketServer {
    type Result = <ServerMessage<CountAll> as Message>::Result;

    fn handle(&mut self, msg: ServerMessage<CountAll>, ctx: &mut Self::Context) -> Self::Result {
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

    fn handle(&mut self, msg: ServerMessage<Disconnect>, ctx: &mut Self::Context) -> Self::Result {
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

    fn handle(&mut self, msg: ServerMessage<Join>, ctx: &mut Self::Context) -> Self::Result {
        println!("Joining channel...");

        let sess = self.get_session(&msg.session.session_id);

        if let Some(sess) = sess {
            let user_session = sess.to_owned();

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
        }
    }
}

impl Handler<SendChannel> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: SendChannel, ctx: &mut Self::Context) -> Self::Result {
        println!("Sending to channel...");

        self.send_to_channel(&msg.channel_name, &msg.msg, None);
    }
}

impl Handler<ServerMessage<SendChannel>> for WebSocketServer {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage<SendChannel>, ctx: &mut Self::Context) -> Self::Result {
        println!("Sending to channel...");

        let response_message = format!("Sent to channel {}", msg.channel_name);
        let data = server_response::ServerResponse::SendChannel(server_response::ResponseBase {
            message: response_message.to_owned(),
            message_id: msg.message_id.to_owned(),
            data: server_response::SendChannelResponse {},
        });
        self.send_to_channel(&msg.channel_name, &msg.msg, Some(data));
    }
}

// impl Handler<ServerMessage<ErrorMessage>> for WebSocketServer {
//     type Result=();

//     fn handle(&mut self, msg: ServerMessage<ErrorMessage>, ctx: &mut Self::Context) -> Self::Result {
//         println!("Sending to channel...");

//         let response = server_response::ServerResponse::ERROR(
//             server_response::ResponseBase{
//                 message: "error".to_string(),
//                 message_id: msg.message_id.to_owned(),
//                 data: server_response::ResponseError{
//                     error_code: msg.error_code.to_owned(),
//                     error_message: msg.error_message.to_owned()
//                 }
//             }
//         );

//         self.send_to_session(&msg.session,"error", Some(response));

//     }
// }
