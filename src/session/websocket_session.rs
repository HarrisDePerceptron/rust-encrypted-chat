use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    StreamHandler, WrapFuture,
};
use actix_web_actors::ws;

use crate::session::TextMessage;
use crate::server::messages::{Connect, CountAll, Disconnect, Join, ServerMessage};
use crate::server::{usersession, UserSession, WebSocketServer};
use crate::session::commands::Command;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocketSession {
    pub id: usize,
    pub server: Addr<WebSocketServer>,
    pub hb: Instant,
    pub sessions: HashMap<String, UserSession>,
}

impl WebSocketSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let addr = ctx.address();
        let sess = self.get_user_session_owned(addr);

        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // let sess = self.get_user_session(addr);

                if let Some(sess) = &sess {
                    act.server.do_send(ServerMessage {
                        message: Disconnect {
                        },
                        session: sess.to_owned(),
                    });
                }
                // notify chat server

                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    pub fn new(srv: Addr<WebSocketServer>) -> Self {
        Self {
            id: 0,
            server: srv,
            hb: Instant::now(),
            sessions: HashMap::new(),
        }
    }

    pub fn get_user_session(&self, addr: Addr<WebSocketSession>) -> Option<&UserSession> {
        for (sid, sess) in &self.sessions {
            if addr == sess.session {
                return Some(sess);
            }
        }

        return None;
    }

    pub fn get_user_session_owned(&self, addr: Addr<WebSocketSession>) -> Option<UserSession> {
        for (sid, sess) in &self.sessions {
            if addr == sess.session {
                return Some(sess.to_owned());
            }
        }

        return None;
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();

        let session_id = Uuid::new_v4().to_string();
        let user_session = UserSession {
            session: addr,
            session_id: session_id.clone(),
        };

        self.sessions.insert(session_id, user_session.clone());

        self.server
            .send(ServerMessage {
                message: Connect(user_session.to_owned()),
                session: user_session.to_owned(),
            })
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

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        let addr = ctx.address();
        let session = self.get_user_session(addr);
        if let Some(session) = session {
            self.server.do_send(ServerMessage {
                message: Disconnect {
                },
                session: session.to_owned(),
            });
        }

        actix::Running::Stop
    }
}

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
        let user_session = self.get_user_session(addr).expect("user session not found");

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                println!("ping");
                ctx.pong(&msg)
            }
            ws::Message::Pong(msg) => {
                self.hb = Instant::now();
            }

            ws::Message::Text(text) => {
                let msg = text.to_string();


                let command: Result<Command, _> = serde_json::from_str(&msg);

                if let Err(e) = command {
                    let error_msg = e.to_string();
                    ctx.text(error_msg);
                    return;
                }

                println!("got msg: {}", msg);

                match msg.as_str() {
                    "count" => {
                        self.server.do_send(ServerMessage {
                            message: CountAll {},
                            session: user_session.to_owned(),
                        });
                    }
                    "join" => {
                        let session_id = user_session.session_id.to_owned();
                        println!("joining 'hey' with session id: {}", session_id);
                        self.server.do_send(ServerMessage {
                            message: Join {
                                name: "hey".to_string()
                            },
                            session: user_session.to_owned(),
                        });
                    }
                    _ => {
                        ctx.text(text);
                    }
                }
            }
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
        ctx.text(msg.message);
    }
}
