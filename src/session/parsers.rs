use actix_web_actors::ws::{Message, WebsocketContext};
use serde_json;

use crate::session::commands::Command;
use crate::session::WebSocketSession;

type SessionContext = WebsocketContext<WebSocketSession>;





fn parse_command(s: &str, ctx: &SessionContext) -> Result<Command, serde_json::Error> {
    let command: Result<Command, _> = serde_json::from_str(s);

    if let Err(e) = command {
        let error_msg = e.to_string();
        ctx.text(error_msg);
        return Err(e);
    }


    return command;

}

fn parse_text_message(message: Message, ctx: &SessionContext) -> () {
    let text = match message {
        Message::Text(msg) => msg,
        _ => return,
    };

    let msg = text.to_string();

    let command = parse_command(&msg, &ctx).unwrap_or_else(|_|{
        println!("error parsing command");
        return;
    });
    


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
                    name: "hey".to_string(),
                },
                session: user_session.to_owned(),
            });
        }
        _ => {
            ctx.text(text);
        }
    }
}
