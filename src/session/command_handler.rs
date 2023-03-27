
use actix::{Actor};
use crate::session::websocket_session::{SessionContext};
use crate::session::commands::{CommandRequest, CommandRequestError};

use super::ErrorCode;

pub trait CommandHandler
where 
    Self: Actor<Context = SessionContext<Self>>
 {
    fn handle_text(&self, command: &CommandRequest, ctx: &mut SessionContext<Self>);
    fn handle_error(&self, command: &CommandRequestError, ctx: &mut SessionContext<Self>);

    fn handle_error_default(&self, msg: &str, ctx: &mut SessionContext<Self>) {

        let error_request = CommandRequestError {
            code: ErrorCode::ZERO,
            message: msg.to_string()
        };

        self.handle_error(&error_request, ctx);

    }
}
