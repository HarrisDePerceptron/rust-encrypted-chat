
use crate::websocket_session::WebSocketSession;
use actix::{Actor, Addr, Context, Handler, Message, Recipient};

use crate::messages::websocket_session_messages::{TextMessage};
use crate::messages::websocket_server_messages::{Connect,TextMessageAll,CountAll};


pub struct WebSocketServer {
    pub index: usize,
    pub sessions: Vec<Addr<WebSocketSession>>,
}



impl Actor for WebSocketServer {
    type Context = Context<Self>;
}

impl WebSocketServer {
    pub fn notify_all(&self, message: &str) {
        for s in &self.sessions {
            let res: Recipient<TextMessage> = s.clone().recipient();
            res.do_send(TextMessage {
                message: message.to_owned(),
            });
        }
    }
}



impl Handler<Connect> for WebSocketServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        println!("Connecting new websocket session...");
        self.sessions.push(msg.0);

        self.index += 1;

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
