use actix::{Actor};
use actix_web_actors::ws::{Message};


use serde_json::{self, error::Category};

use crate::session::commands::{CommandRequest, CommandRequestError, ErrorCode};

use crate::session::websocket_session::{SessionContext};



pub trait JSONCommandParser
where 
    Self: Actor<Context = SessionContext<Self>>
{

    fn parse_command_request(
        s: &str,
    ) -> Result<CommandRequest, CommandRequestError> {

        
        let command: Result<CommandRequest, _> = serde_json::from_str(s);

        let command = match command{

            Err (e) => {
                let error_msg = e.to_string();
                
                let msg = match e.classify() {
                    Category::Syntax => format!("JSON deserialzation error: {}", error_msg),
                    _ => format!("{}", error_msg)
                };
                Err(CommandRequestError {message:  msg, code: ErrorCode::ZERO })
            },
            
            Ok(res) => Ok(res)
        };


        return command;
    }

    
    fn parse_text_message(message: &Message) -> Result<CommandRequest, CommandRequestError>{
        let text = match message {
            Message::Text(msg) => msg,
            _ => return Err(
                CommandRequestError { 
                    message: "Not a text message".to_owned(), 
                    code: ErrorCode::NOT_TEXT_MESSAGE
                }),
        };

        let msg = text.to_string();

        let command = Self::parse_command_request(&msg) ;
        return command;       
    }


}

