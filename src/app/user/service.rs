use crate::app::application_model::ApplicationModelTrait;
use crate::app::application_service::ServiceTrait;
use crate::app::service_redis::RedisApplicationService;

use super::model::User;
use super::service_model::UserServiceError;
use crate::app::application_service::{ApplicationServiceError, ApplicationServiceTrait};
use async_trait::async_trait;

use serde;
use serde::Serialize;
use serde_json;

use std::fmt;

use super::model;
use super::service_model;
use crate::app::application_factory;
use crate::app::application_model::ApplicationModel;
use crate::app::application_service;
use crate::auth;
use crate::secrets;
use super::service_trait;

pub struct UserService {
    redis_service: RedisApplicationService,
}

impl UserService {
    pub fn new(redis_service: RedisApplicationService) -> Self {
        Self {
            redis_service: redis_service,
        }
    }
}


impl application_service::ServiceTrait for UserService {
    type Model = model::User;
    type Persistence = RedisApplicationService;
    type Error =  UserServiceError;

    fn get_persistence_service(&mut self) -> &mut Self::Persistence {
        &mut self.redis_service
    }
}

#[async_trait]
impl service_trait::UserServiceTrait for UserService {
    async fn signup(
        &mut self,
        request: service_model::SignupRequest,
    ) -> Result<service_model::SignupResponse, service_model::UserServiceError> {
        let hash = auth::hash_password(&request.password)
            .map_err(|e| service_model::UserServiceError::SignupError(e))?;

        let user = User {
            username: request.username.to_owned(),
            password: hash,
        };

        let user_a = ApplicationModel {
            id: None,
            data: user,
        };

        let persistence_service = self.get_persistence_service();

        let result = persistence_service
            .create(user_a)
            .await
            .map_err(|e| service_model::UserServiceError::SignupError(format!("{:?}", e)))?;

        let user_id = result.id()
            .ok_or(service_model::UserServiceError::SignupError("user id not found".to_string()))?;

    

        let login_result = self.login(service_model::LoginRequest{
            user_id: user_id.to_owned(),
            password: request.password.to_owned(),
            username: request.username.to_owned()
        }).await?;


        let response = service_model::SignupResponse {
            token: login_result.token,
            user_id: user_id,
            user_name: request.username.to_owned()
        };

        Ok(response)
    }

    async fn get(
        &mut self,
        count: usize,
    ) -> Result<Vec<ApplicationModel<User>>, service_model::UserServiceError> {
        let result: Vec<ApplicationModel<User>> = self
            .redis_service
            .find(count)
            .await
            .map_err(|e| service_model::UserServiceError::GetError(format!("{:?}", e)))?;

        Ok(result)
    }

    async fn update(
        &mut self,
        model: ApplicationModel<User>,
    ) -> Result<ApplicationModel<User>, service_model::UserServiceError> {
        let result: ApplicationModel<User> = self
            .redis_service
            .update_by_id(model)
            .await
            .map_err(|e| service_model::UserServiceError::UpdateError(format!("{:?}", e)))?;

        Ok(result)
    }

    async fn login(
        &mut self,
        request: service_model::LoginRequest,
    ) -> Result<service_model::LoginResponse, service_model::UserServiceError> {
        let result: ApplicationModel<User> = self
            .redis_service
            .find_by_id(request.user_id.as_str())
            .await
            .map_err(|e| service_model::UserServiceError::LoginError(format!("{:?}", e)))?;

        let verify = auth::verify_password_hash(&request.password, &result.password).ok_or(
            service_model::UserServiceError::LoginError("Password hash do not match".to_string()),
        )?;

        if !verify {
            return Err(service_model::UserServiceError::LoginError(
                "Invalid password hash".to_string(),
            ));
        }

        if request.username != result.username {
            return Err(service_model::UserServiceError::LoginError(
                "Username or password do not match".to_string(),
            ));
        }

        let expiry: u64 = secrets::TOKEN_EXPIRY_DAYS
            .to_string()
            .parse()
            .map_err(|_| {
                service_model::UserServiceError::LoginError(
                    "Unable to parse expiry days".to_string(),
                )
            })?;

        let token = auth::generate_token(&request.user_id, &secrets::TOKEN_ISSUER, expiry)
            .map_err(|e| service_model::UserServiceError::LoginError(e.to_string()))?;

        let response = service_model::LoginResponse {
            token: token,
            user_id: request.user_id,
        };

        Ok(response)
    }

    async fn get_by_id(
        &mut self,
        id: &str,
    ) -> Result<ApplicationModel<User>, service_model::UserServiceError> {
        let result: ApplicationModel<model::User> = self
            .redis_service
            .find_by_id(id)
            .await
            .map_err(|e| service_model::UserServiceError::GetError(format!("{:?}", e)))?;

        Ok(result)
    }
}

// impl UserService {
//     pub async fn signup(
//         &mut self,
//         request: service_model::SignupRequest,
//     ) -> Result<ApplicationModel<User>, service_model::UserServiceError> {
//         let hash = auth::hash_password(&request.password)
//             .map_err(|e| service_model::UserServiceError::SignupError(e))?;

//         let user = User {
//             username: request.username,
//             password: hash,
//         };

//         let user_a = ApplicationModel {
//             id: None,
//             data: user,
//         };

//         let result: ApplicationModel<User> = self
//             .redis_service
//             .create(user_a)
//             .await
//             .map_err(|e| service_model::UserServiceError::SignupError(format!("{:?}", e)))?;

//         Ok(result)
//     }

//     pub async fn get(
//         &mut self,
//         count: usize,
//     ) -> Result<Vec<ApplicationModel<User>>, service_model::UserServiceError> {
//         let result: Vec<ApplicationModel<User>> = self
//             .redis_service
//             .find(count)
//             .await
//             .map_err(|e| service_model::UserServiceError::GetError(format!("{:?}", e)))?;

//         Ok(result)
//     }

//     pub async fn update(
//         &mut self,
//         model: ApplicationModel<User>,
//     ) -> Result<ApplicationModel<User>, service_model::UserServiceError> {
//         let result: ApplicationModel<User> = self
//             .redis_service
//             .update_by_id(model)
//             .await
//             .map_err(|e| service_model::UserServiceError::UpdateError(format!("{:?}", e)))?;

//         Ok(result)
//     }

//     pub async fn login(
//         &mut self,
//         request: service_model::LoginRequest,
//     ) -> Result<service_model::LoginResponse, service_model::UserServiceError> {
//         let result: ApplicationModel<User> = self
//             .redis_service
//             .find_by_id(request.user_id.as_str())
//             .await
//             .map_err(|e| service_model::UserServiceError::LoginError(format!("{:?}", e)))?;

//         let verify = auth::verify_password_hash(&request.password, &result.password).ok_or(
//             service_model::UserServiceError::LoginError("Password hash do not match".to_string()),
//         )?;

//         if !verify {
//             return Err(service_model::UserServiceError::LoginError(
//                 "Invalid password hash".to_string(),
//             ));
//         }

//         if request.username != result.username {
//             return Err(service_model::UserServiceError::LoginError(
//                 "Username or password do not match".to_string(),
//             ));
//         }

//         let expiry: u64 = secrets::TOKEN_EXPIRY_DAYS
//             .to_string()
//             .parse()
//             .map_err(|_| {
//                 service_model::UserServiceError::LoginError(
//                     "Unable to parse expiry days".to_string(),
//                 )
//             })?;

//         let token = auth::generate_token(&request.user_id, &secrets::TOKEN_ISSUER, expiry)
//             .map_err(|e| service_model::UserServiceError::LoginError(e.to_string()))?;

//         let response = service_model::LoginResponse {
//             token: token,
//             user_id: request.user_id,
//         };

//         Ok(response)
//     }

//     pub async fn get_by_id(
//         &mut self,
//         id: &str,
//     ) -> Result<ApplicationModel<User>, service_model::UserServiceError> {
//         let result: ApplicationModel<model::User> = self
//             .redis_service
//             .find_by_id(id)
//             .await
//             .map_err(|e| service_model::UserServiceError::GetError(format!("{:?}", e)))?;

//         Ok(result)
//     }
// }
