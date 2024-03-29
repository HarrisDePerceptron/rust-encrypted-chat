use std::fmt::{Debug};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

