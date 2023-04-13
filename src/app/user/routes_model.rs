use std::fmt::{Debug};
use serde::{Serialize, Deserialize};
use crate::app::application_model::ApplicationModel;



#[derive(Debug, Clone, Serialize,Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}
