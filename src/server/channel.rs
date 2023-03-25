

use crate::{server::UserSession, session::TextMessage};

use crate::server::server_response::ServerResponse;


pub struct Channel {
    pub id: String,
    pub name: String,
    pub sessions: Vec<UserSession>,
}


impl Channel {
    pub fn new(id: &str, name: &str) -> Self{
        Self { 
            id: id.to_string() , 
            name: name.to_string(), 
            sessions: vec![] 
        }
    }


    pub fn add_session(&mut self, session:  &UserSession) -> (){
        let result = self.sessions.iter().position(|x| x.session_id==session.session_id);

        if let Some(_) = result {
            println!("Session'{}' already added to channel {}", session.session_id, self.name);
        }else{

            self.sessions.push(session.clone());
        }
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

