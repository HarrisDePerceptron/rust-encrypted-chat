
use std::fmt::Display;
use std::ops::Deref;

use serde;
use serde::{Serialize, Deserialize};

use chrono;
use mongodb::bson::oid::ObjectId;

use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoResponse<T> 
{   
    pub _id: Option<ObjectId>,
    pub id: Option<String>,
    #[serde(flatten)]
    pub data: T,

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseList<T>(pub Vec<DaoResponse<DaoRequest<T>>>);



impl<T> DaoResponse<DaoRequest<T>> {

    pub fn new(data: DaoRequest<T>) -> Self {
        Self { 
            _id: None, 
            id: None,
            data: data
        }
    }

}


impl<T> Deref for DaoResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


impl<T> Display for DaoResponse<DaoRequest<T>> 
where
    Self: Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(self)
            .expect("Unable to serialze dao response ");

        f.write_str(&s)
    }
}


impl<T> Display for ResponseList<T>
where
    Self: Serialize,
    T: Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(&self.0)
            .expect("Unable to serialze dao list response ");

        f.write_str(&s)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoRequest<T> 
{   
    #[serde(flatten)]
    pub data: T,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub delete: bool

}

impl<T> DaoRequest<T> {
    pub fn new(data: T) -> Self {
        Self { 
            data: data, 
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            delete: false
        }
    }
}

impl<T> Deref for DaoRequest<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub avatar: Option<String>, 
    pub online: bool,
    pub blocked_rooms: Vec<Room>,
    pub blocked_users: Vec<User>,

}


impl User {
    pub fn new(name: &str) -> Self {
        Self {
            avatar: None,
            online: false,
            name: name.to_string(),
            blocked_rooms: vec![],
            blocked_users: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRequest {
    room: Room,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub users: Vec<User>,
    pub online: Vec<User>,
    pub owner: User,
    pub typing: Vec<User>,
    pub blocked: Vec<User>,
    pub deleted: bool,
}

impl Room {
    pub fn new(name: &str, owner: User) -> Self {
        Self { 
            name: name.to_string(), 
            users: vec![], 
            online: vec![], 
            owner: owner, 
            typing: vec![], 
            blocked: vec![], 
            deleted: false, 
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    TEXT,
    FILE,
    IMAGE
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub from: User,
    pub body: String,
    pub message_type: MessageType,
    pub seen: Vec<User>,
    pub deleted: bool,
    pub room: Room,

}


impl TextMessage {
    pub fn new(body: &str, from: User, room: Room) -> Self {
        Self { 
            body: body.to_string(),
            from: from,
            message_type: MessageType::TEXT,
            room: room,
            seen: vec![],
            deleted: false,
        }
    }
}



