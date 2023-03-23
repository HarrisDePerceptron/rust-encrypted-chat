
use actix::{Actor};
use crate::session::websocket_session::{SessionContext};
use crate::session::commands::{CommandRequest, CommandRequestError};

pub trait CommandHandler
where 
    Self: Actor<Context = SessionContext<Self>>
 {
    fn handle_text(&self, command: &CommandRequest, ctx: &mut SessionContext<Self>);
    fn handle_error(&self, command: &CommandRequestError, ctx: &mut SessionContext<Self>);
}
