
use actix::{fut, Actor, Addr, AsyncContext, StreamHandler, Handler,WrapFuture, ActorFutureExt, ActorContext,ContextFutureSpawner};
use actix_web_actors::ws;

use crate::websocket_server::WebSocketServer;
use crate::messages::websocket_session_messages::{TextMessage};
use crate::messages::websocket_server_messages::{Connect,CountAll};

pub struct WebSocketSession {
    pub id: usize,
    pub server: Addr<WebSocketServer>,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        self.server
            .send(Connect(addr))
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    Err(e) => ctx.stop(),
                }

                fut::ready(())
            })
            .wait(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            // Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Text(text)) => match text.to_string().as_str() {
                "count" => {
                    self.server.do_send(CountAll {});
                }
                _ => {
                    ctx.text(text);
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}



impl Handler<TextMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.message);
    }
}