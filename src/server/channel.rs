use crate::server::UserSession;


pub struct Channel {
    pub id: String,
    pub name: String,
    pub users: Vec<UserSession>,
}


impl Channel {
    pub fn new(id: &str, name: &str) -> Self{
        Self { 
            id: id.to_string() , 
            name: name.to_string(), 
            users: vec![] 
        }
    }


    pub fn add_user(&mut self, session:  &UserSession) -> (){
        self.users.push(session.clone());
    }
}

