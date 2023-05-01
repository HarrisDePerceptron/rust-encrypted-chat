use std::fmt::{Debug};
use serde::{Serialize, Deserialize};
use crate::app::application_model::ApplicationModel;



#[derive(Debug, Clone, Serialize)]
pub enum WebsocketServiceError {
    SignupError(String),
}

impl std::fmt::Display for WebsocketServiceError
where
    Self: std::fmt::Debug + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std:: fmt::Result {
        let response = serde_json::to_string(&self)
            .map_err(|e| std::fmt::Error::from(std::fmt::Error))?;

        f.write_str(&response)
    }
}
