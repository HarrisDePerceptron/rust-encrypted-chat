use crate::app::service_redis::RedisApplicationService;

use super::model::{User};
use crate::app::application_service::{ApplicationServiceError, ApplicationServiceTrait};
use async_trait::async_trait;

use serde::Serialize;
use serde_json;


use crate::app::application_model::ApplicationModel;
use super::service_model;



use crate::auth;



pub struct UserService {
    redis_service: RedisApplicationService,
}

impl UserService {
    pub fn new(redis_service: RedisApplicationService) -> Self {
        Self { 
            redis_service:  redis_service
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum UserServiceError {
    SignupError(String),
    GetError(String),
    UpdateError(String)
}

impl UserService {
    pub async fn signup(
        &mut self,
        request: service_model::SignupRequest,
    ) -> Result<ApplicationModel<User>, UserServiceError> {

        let hash = auth::hash_password(&request.password)
            .map_err(|e| UserServiceError::SignupError(e))?;

        
        let user = User {
            username: request.username,
            password: hash,
        };

        let user_a = ApplicationModel {
            id: None,
            data: user,
        };

        let result: ApplicationModel<User> = self
            .redis_service
            .create(user_a)
            .await
            .map_err(|e| UserServiceError::SignupError(format!("{:?}", e)))?;

        Ok(result)
    }


    pub async fn get(
        &mut self,
        count: usize,
    ) -> Result<Vec<ApplicationModel<User>>, UserServiceError> {
        

        let result: Vec<ApplicationModel<User>> = self.redis_service.find(count)
            .await
            .map_err(|e| UserServiceError::GetError(format!("{:?}", e)) )?;

        Ok(result)
    }


    pub async fn update(
        &mut self,
        model: ApplicationModel<User>,
    ) -> Result<ApplicationModel<User>, UserServiceError> {
        

        let result: ApplicationModel<User> = self.redis_service.update_by_id(model)
            .await
            .map_err(|e| UserServiceError::UpdateError(format!("{:?}", e)) )?;

        Ok(result)
    }

}
