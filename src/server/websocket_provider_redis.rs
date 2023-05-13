
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
// use crate::app::websocket::model::Websocket;

use super::channel::Channel;

#[derive(Debug, Serialize, Clone)]
enum WebsocketServerError {
    PersistenceLoadError(String),
    PersistenceSaveError(String)
}

#[async_trait]
trait WebsocketPersistence<T>
where 
    T: Debug + Clone + Serialize + DeserializeOwned + 'static
{
    type Persistence: ApplicationServiceTrait<T>;

    async fn load(&self)-> Result<(), WebsocketServerError>;
    async fn save (&self, channel: &Channel) -> Result<(), WebsocketServerError >;
    fn get_persistence(&self) -> Self::Persistence;

    async fn sync_channels(&self) ->  HashMap<String, Channel>;


}


// #[async_trait]
// impl<T> WebsocketPersistence<T> for WebSocketServer 
// where
//     T: Debug + Clone + Serialize + DeserializeOwned + 'static + std::marker::Send
// {
//     type Persistence = RedisApplicationService;

//     async fn load(&self)-> Result<(), WebsocketServerError> {
//         let mut persistence = <WebSocketServer as WebsocketPersistence<T>>::get_persistence(self);
//         let _result: ApplicationModel<Websocket> = persistence.find_by_id("websocket:mapping")
//             .await
//             .map_err(|e| WebsocketServerError::PersistenceLoadError(format!("{:?}",e)))?;
   

//         Ok(())
//     }

//     async fn save(&self, _channel: &Channel) -> Result<(), WebsocketServerError > {
//         let mut persistence = <WebSocketServer as WebsocketPersistence<T>>::get_persistence(self);
//         let _result: ApplicationModel<Websocket> = persistence.find_by_id("websocket:mapping")
//             .await
//             .map_err(|e| WebsocketServerError::PersistenceLoadError(format!("{:?}",e)))?;

        
//         Ok(())
//     }


//     fn get_persistence(&self) -> Self::Persistence {
//         let redis = RedisFactory::new("channel", DataStructure::UnorderedSet);
//         let redis_service = redis.get();

//         return redis_service;

//     }


//     async fn sync_channels(&self) ->  HashMap<String, Channel> {
//         // let persistence = self.get_persistence();
        
//         todo!()
        
//     }
// }