

use serde::Serialize;

use crate::{server::UserSession, session::TextMessage};

use crate::server::server_response::ServerResponse;
use crate::utils;


pub struct Channel {
    pub id: String,
    pub name: String,
    pub sessions: Vec<UserSession>,
}



#[derive(Debug, Serialize)]
pub enum ChannelError{
    AlreadyAdded(String)
}


impl Channel {
    pub fn new(name: &str) -> Self{
        let id = utils::generate_unique_id().expect("Unable to generate uuid for channel");

        let channel_id = format!("channel:{}", id);

        Self { 
            id: channel_id , 
            name: name.to_string(), 
            sessions: vec![] 
        }
    }


    pub fn add_session(&mut self, session:  &UserSession) -> Result<(), ChannelError>{
        let result = self.sessions.iter().position(|x| x.session_id==session.session_id);

        if let Some(_) = result {
            let msg = format!("Session'{}' already added to channel {}", session.session_id, self.name);
            return Err(ChannelError::AlreadyAdded(msg));
            
        }else{

            self.sessions.push(session.clone());
        }

        Ok(())
    }

    pub fn remove_session(&mut self, session: &UserSession) -> Option<UserSession>{

        let mut sess: Option<UserSession> = None;
        let idx= self.sessions.iter().position(|x| x.session_id==session.session_id);
        if let Some(idx) = idx {
            sess = Some(self.sessions.remove(idx));

        }

        return sess;
    }

    pub fn send(&self,message: &str, data: Option<ServerResponse>) {
        for sess in &self.sessions {
            sess.session.do_send(TextMessage{
                message: message.to_string(),
                data: data.clone()
            });
        }
    }
}

