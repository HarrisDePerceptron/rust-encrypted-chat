use crate::server::UserSession;
use crate::server::messages::{ServerMessage, Join,CountAll, SendChannel};
use crate::session::{WebSocketSession, ToChannelRequest};
// use crate::session::

use actix::{ActorContext, AsyncContext, StreamHandler, Handler, Actor};
use actix_web_actors::ws;
use serde::Serialize;
use std::time::Instant;

use crate::session::commands::{CommandRequest, CommandRequestError, ErrorCode};
use crate::session::command_parser::{JSONCommandParser};
use actix_web_actors::ws::{Message};

use crate::session::TextMessage;

use crate::session::websocket_session::{SessionContext};

use crate::session::command_handler::{CommandHandler};



impl CommandHandler for WebSocketSession{
    fn handle_text(&self, command: &CommandRequest, ctx: &mut SessionContext<Self>) {
        let server_address = self.get_server_address();
        let address = ctx.address();

        let user_session = match self.get_user_session(address) {
            Some(us) => us,
            None => {
                let msg = "Unable to get previous user session";
                let response_error = CommandRequestError{
                    code: ErrorCode::ZERO,
                    message: msg.to_owned()
                };
                
                self.handle_error(&response_error, ctx);

                return ;
            }
        };

        let session_id = user_session.session_id.to_owned();

        match command {
            CommandRequest::COUNT(_) => {
                server_address.do_send(ServerMessage {
                    message: CountAll {},
                    session: user_session.to_owned(),
                });
                
                
            },
            CommandRequest::JOIN(c) => {
              
                println!("joining 'hey' with session id: {}", session_id);
                server_address.do_send(ServerMessage {
                    message: Join {
                        name: c.channel_name.to_owned(),
                    },
                    session: user_session.to_owned(),
                });

                
            },

            CommandRequest::ToChannel(c) => {
                println!("sending message from session id: {}", session_id);
                server_address.do_send(SendChannel {
                        channel_name: c.channel_name.to_owned(),
                        msg: c.message.to_owned()
                });
                
            }
            _ => {
                let response_error = CommandRequestError{
                    code: ErrorCode::ZERO,
                    message: "Command  not implmented".to_owned()
                };

                println!("{:?}", response_error);
                
                self.handle_error(&response_error, ctx);
            }
        }
    }

    fn handle_error(&self, command: &CommandRequestError, ctx: &mut SessionContext<Self>) {
        let serialize = match serde_json::to_string(&command){
            Err(e)=> {println!("unbale to serialize error"); return;},
            Ok(ok) => ok
        };

        ctx.text(serialize);

    }
}

impl JSONCommandParser for WebSocketSession {}


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                println!("Got message error: {}", e.to_string());
                ctx.stop();
                return;
            }
        };

        let addr = ctx.address();

        let command = Self::parse_text_message(&msg);
        
        match command{
          Ok(c) => self.handle_text(&c, ctx),
          Err(e)=>  {
            if e.code != ErrorCode::NOT_TEXT_MESSAGE{
                self.handle_error(&e, ctx)
            }
          }

        };


        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                println!("ping");
                ctx.pong(&msg)
            }
            ws::Message::Pong(msg) => {
                self.hb = Instant::now();
            }

            ws::Message::Text(text) => {}
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(reason) => {
                if let Some(reason) = reason {
                    println!(
                        "closing connection...\nReason: {}",
                        reason.description.unwrap()
                    );
                }

                println!("closing connection...");
                ctx.stop();
            }
            ws::Message::Continuation(_) => ctx.stop(),
            ws::Message::Nop => (),
            _ => (),
        }
    }
}


impl Handler<TextMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {

        let msg_json = match serde_json::to_string(&msg){
            Ok(msg) => msg,
            Err(e)=> {
                println!("Json serialzation error: {}", e); 
                return;
            }
        };

        ctx.text(msg_json);
    }
}
