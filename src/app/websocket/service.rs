
use crate::{app::{redis::service::RedisApplicationService, application_service::ServiceTrait, application_model::ApplicationModel}, persistence};

use async_trait::async_trait;
use crate::app::application_service;
use crate::app::redis::model::{DataStructure};
use super::service_model::{WebsocketServiceError};
use crate::persistence::redis::RedisProvider;


use serde_json;
use serde::{Serialize, Deserialize};

use redis::AsyncCommands;


use super::service_model;



#[async_trait]
pub trait WebsocketServiceTrait
{
    async fn store(
        &mut self,
        channel: service_model::ChannelData
    ) -> Result<(), WebsocketServiceError>;

    async fn load(
        &mut self,
    ) -> Result<Vec<service_model::ChannelData>, WebsocketServiceError>;

}

pub struct WebsocketService {
    redis_provider: RedisProvider,
    max_channels: usize

}

impl WebsocketService {
    pub fn new() -> Self {

        let provider = RedisProvider::new();
        
        Self {
            redis_provider: provider,
            max_channels: 10000
        }
    }
}




#[async_trait]
impl WebsocketServiceTrait for WebsocketService {
    async fn store(
        &mut self,
        channel: service_model::ChannelData
    ) -> Result<(),WebsocketServiceError> {
        
        let conn = self.redis_provider.get_connection()
            .await
            .map_err(|e| WebsocketServiceError::StoreError(format!("{}",e.reason)))?;


        let data_str = serde_json::to_string(&channel)
            .map_err(|e| WebsocketServiceError::SignupError(e.to_string()))?;

        conn.sadd("channels",data_str)
            .await
            .map_err(|e| WebsocketServiceError::StoreError(e.to_string()))?;        
        
        Ok(())
    }


    async fn load(
        &mut self,
    ) -> Result<Vec<service_model::ChannelData>, WebsocketServiceError> {

        let conn = self.redis_provider.get_connection()
            .await
            .map_err(|e| WebsocketServiceError::StoreError(format!("{}",e.reason)))?;

        let mut res = conn.sscan_match("channels", "*")
            .await
            .map_err(|e| WebsocketServiceError::LoadError(e.to_string()) )?;

        let  mut channels: Vec<service_model::ChannelData> = Vec::new();

        for i in 0..self.max_channels {
            let item: Option<String>= res.next_item()
                .await;

            let v = match item {
                Some(v) => v,
                None => break
            };
            
            let data: service_model::ChannelData = match serde_json::from_str(&v) {
                Err(e) => continue,
                Ok(v) => v
            };
            
            channels.push(data);
            
        }


        Ok(channels)

    }
}