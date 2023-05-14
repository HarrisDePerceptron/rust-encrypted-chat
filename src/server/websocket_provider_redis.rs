
use super::websocket::WebSocketServer;
use crate::app::application_factory::FactoryTrait;
use crate::app::application_service::{ApplicationServiceTrait};

use crate::app::redis::service::{RedisApplicationService};
use crate::app::redis::factory::{RedisFactory};
use crate::app::redis::model::{DataStructure};


use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Serialize, de::DeserializeOwned};
use async_trait::async_trait;

use crate::app::application_model::ApplicationModel;
use super::channel::Channel;

use crate::app::websocket::{ChannelData, WebsocketService, WebsocketServiceTrait};


#[derive(Debug, Serialize, Clone)]
pub enum WebsocketServerError {
    PersistenceLoadError(String),
    PersistenceSaveError(String)
}

#[async_trait]
pub trait WebsocketPersistence<T>
where 
    T: Debug + Clone + Serialize + DeserializeOwned + 'static
{
    type Persistence: ApplicationServiceTrait<T>;

    async fn load(&self)-> Result<(), WebsocketServerError>;
    async fn save (&self, channel: &Channel) -> Result<(), WebsocketServerError >;



}


#[async_trait]
impl<T> WebsocketPersistence<T> for WebSocketServer 
where
    T: Debug + Clone + Serialize + DeserializeOwned + 'static + std::marker::Send
{
    type Persistence = RedisApplicationService;

    async fn load(&self)-> Result<(), WebsocketServerError> {
        let mut service = WebsocketService::new();
        let result = service.load()
            .await
            .map_err(|e| WebsocketServerError::PersistenceLoadError(e.to_string()) )?;


        println!("Loading channels ...");
        for ch in result{
            println!("Channel: {:?}", ch)
            // self.channels.insert(k, v)
        }
        Ok(())
    }

    async fn save(&self, _channel: &Channel) -> Result<(), WebsocketServerError > {
        
        
        Ok(())
    }


}