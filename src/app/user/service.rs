use crate::app::service_redis::RedisApplicationService;

use super::model::{User};
use crate::app::application_service::{ApplicationServiceError, ApplicationServiceTrait};
use async_trait::async_trait;

use serde::Serialize;
use serde_json;


use crate::app::application_model::ApplicationModel;
use super::service_model;
use crate::auth;
use crate::secrets;



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





impl UserService {
    pub async fn signup(
        &mut self,
        request: service_model::SignupRequest,
    ) -> Result<ApplicationModel<User>, service_model::UserServiceError> {

        let hash = auth::hash_password(&request.password)
            .map_err(|e| service_model::UserServiceError::SignupError(e))?;

        
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
            .map_err(|e| service_model::UserServiceError::SignupError(format!("{:?}", e)))?;

        Ok(result)
    }


    pub async fn get(
        &mut self,
        count: usize,
    ) -> Result<Vec<ApplicationModel<User>>, service_model::UserServiceError> {
        

        let result: Vec<ApplicationModel<User>> = self.redis_service.find(count)
            .await
            .map_err(|e| service_model::UserServiceError::GetError(format!("{:?}", e)) )?;

        Ok(result)
    }


    pub async fn update(
        &mut self,
        model: ApplicationModel<User>,
    ) -> Result<ApplicationModel<User>, service_model::UserServiceError> {
        

        let result: ApplicationModel<User> = self.redis_service.update_by_id(model)
            .await
            .map_err(|e| service_model::UserServiceError::UpdateError(format!("{:?}", e)) )?;

        Ok(result)
    }


    pub async fn login(
        &mut self,
        request: service_model::LoginRequest,
    ) -> Result<service_model::LoginResponse, service_model::UserServiceError> {
        
        let result: ApplicationModel<User> = self.redis_service
            .find_by_id(request.user_id.as_str())
            .await
            .map_err(|e| service_model::UserServiceError::LoginError(format!("{:?}", e)))?;


        let verify = auth::verify_password_hash(&request.password, &result.password)
            .ok_or(service_model::UserServiceError::LoginError("Password hash do not match".to_string()))?;
        
        if !verify {
            return Err(service_model::UserServiceError::LoginError("Invalid password hash".to_string()));
        }

        if request.username != result.username {
            return Err(service_model::UserServiceError::LoginError("Username or password do not match".to_string()));
        }

        let expiry: u64 = secrets::TOKEN_EXPIRY_DAYS.to_string().parse()
            .map_err(|_| service_model::UserServiceError::LoginError("Unable to parse expiry days".to_string()))?;

        

        let token = auth::generate_token(&request.user_id, &secrets::TOKEN_ISSUER, expiry )
            .map_err(|e| service_model::UserServiceError::LoginError(e.to_string()))?;
        
        let response = service_model::LoginResponse {
            token: token,
            user_id: request.user_id
        };

        
        Ok(response)
    }

}
