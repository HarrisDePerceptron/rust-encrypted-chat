use std::fmt::{Debug};
use serde::{Serialize, Deserialize};




#[derive(Debug, Clone, Serialize)]
pub enum UserServiceError {
    SignupError(String),
    GetError(String),
    UpdateError(String),
    LoginError(String),
}

impl std::fmt::Display for UserServiceError
where
    Self: std::fmt::Debug + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std:: fmt::Result {
        let response = serde_json::to_string(&self)
            .map_err(|_e| std::fmt::Error::from(std::fmt::Error))?;

        f.write_str(&response)
    }
}

#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct SignupResponse {
    pub token: String,
    pub user_id: String,
    pub user_name: String
}



#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct LoginRequest {
    pub user_id: String,
    pub username: String,
    pub password: String,
}



#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String
}

