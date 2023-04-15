use std::fmt::{Debug};
use serde::{Serialize, Deserialize};
use crate::app::application_model::ApplicationModel;



#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}



#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct LoginRequest {
    pub user_id: String,
    pub username: String,
    pub password: String,
}



#[derive(Clone, Debug, Deserialize)]
pub struct VerifyRequest {
    pub token: String,
}

