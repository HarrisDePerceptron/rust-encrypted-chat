use std::fmt::{Debug};
use serde::{Serialize, Deserialize};
use crate::business::application_model::ApplicationModel;

#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}


pub type UserModel = ApplicationModel<User>;
