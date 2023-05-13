use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, WrapFuture,
};
use actix_web_actors::ws;


use crate::server::messages::{Connect, Disconnect, ServerMessage};
use crate::server::{UserSession, WebSocketServer};
use std::collections::{HashMap};
use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::utils;
use crate::middleware::auth_extractor::UserAuthSession;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


pub type SessionContext<T> = ws::WebsocketContext<T>; 

pub struct WebSocketSession {
    pub id: usize,
    pub server: Addr<WebSocketServer>,
    pub hb: Instant,
    pub sessions: HashMap<String, UserSession>,
    pub user_auth_session: UserAuthSession
}

impl WebSocketSession {

    pub fn get_server_address (&self) -> Addr<WebSocketServer>{
        return self.server.to_owned();
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let addr = ctx.address();
        let sess = self.get_user_session_owned(addr);


        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // let sess = self.get_user_session(addr);
                let message_id  = match utils::generate_unique_id() {
                    Err(e)=> {
                        println!("Failed to generate uuid: {}", e.to_string());
                        return;
                    }
                    Ok(v) => v
                };

                if let Some(sess) = &sess {
                    act.server.do_send(ServerMessage {
                        message: Disconnect {
                        },
                        session: sess.to_owned(),
                        message_id: message_id
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

    pub fn new(srv: Addr<WebSocketServer>, user_auth_session: UserAuthSession) -> Self {
        Self {
            id: 0,
            server: srv,
            hb: Instant::now(),
            sessions: HashMap::new(),
            user_auth_session: user_auth_session
        }
    }

    pub fn get_user_session(&self, addr: Addr<WebSocketSession>) -> Option<&UserSession> {
        for (_sid, sess) in &self.sessions {
            if addr == sess.session {
                return Some(sess);
            }
        }

        return None;
    }

    pub fn get_user_session_owned(&self, addr: Addr<WebSocketSession>) -> Option<UserSession> {
        for (_sid, sess) in &self.sessions {
            if addr == sess.session {
                return Some(sess.to_owned());
            }
        }

        return None;
    }
}

impl Actor for WebSocketSession {
    type Context = SessionContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();

        let session_id = Uuid::new_v4().to_string();
        let user_session = UserSession {
            session: addr,
            session_id: session_id.clone(),
            auth_session: self.user_auth_session.clone()
        };

        self.sessions.insert(session_id, user_session.clone());
        let message_id  = match utils::generate_unique_id() {
            Err(e)=> {
                println!("Failed to generate uuid: {}", e.to_string());
                return;
            }
            Ok(v) => v
        };

        self.server
            .send(ServerMessage {
                message: Connect(user_session.to_owned()),
                session: user_session.to_owned(),
                message_id: message_id
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    Err(_e) => ctx.stop(),
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        let addr = ctx.address();
        let session = self.get_user_session(addr);
        let message_id  = match utils::generate_unique_id() {
            Err(e)=> {
                let v = "Failed to generate uuid".to_string();
                println!("{}: {}",v, e.to_string());
                v
            }
            Ok(v) => v
        };

        
        if let Some(session) = session {
            self.server.do_send(ServerMessage {
                message: Disconnect {
                },
                session: session.to_owned(),
                message_id: message_id
            });
        }

        actix::Running::Stop
    }
}
