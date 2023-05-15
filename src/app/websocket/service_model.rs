use std::fmt::{Debug};
use serde::{Serialize, Deserialize};




#[derive(Debug, Clone, Serialize)]
pub enum WebsocketServiceError {
    SignupError(String),
    IncorrectDataStructure(String),
    StoreError(String),
    LoadError(String)
}

impl std::fmt::Display for WebsocketServiceError
where
    Self: std::fmt::Debug + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std:: fmt::Result {
        let response = serde_json::to_string(&self)
            .map_err(|_e| std::fmt::Error::from(std::fmt::Error))?;

        f.write_str(&response)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelData {
    pub name: String,
}

