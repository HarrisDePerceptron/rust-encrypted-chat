
use crate::app::service_redis::RedisApplicationService;

use super::service_model::WebsocketServiceError;
use crate::app::application_service::{ApplicationServiceTrait};
use async_trait::async_trait;


use super::model;
use super::service_model;
use crate::app::application_model::ApplicationModel;
use crate::app::application_service;
use crate::auth;
use crate::secrets;
use super::service_trait;

pub struct WebsocketService {
    redis_service: RedisApplicationService,
}

impl WebsocketService {
    pub fn new(redis_service: RedisApplicationService) -> Self {
        Self {
            redis_service: redis_service,
        }
    }
}


impl application_service::ServiceTrait for WebsocketService {
    type Model = model::Websocket;
    type Persistence = RedisApplicationService;
    type Error =  WebsocketServiceError;

    fn get_persistence_service(&mut self) -> &mut Self::Persistence {
        &mut self.redis_service
    }
}


#[async_trait]
impl service_trait::WebsocketServiceTrait for WebsocketService {
    async fn store(
        &mut self,
    ) -> Result<(), Self::Error> {
        
        Ok(())
    }
}