
use std::sync::{Arc, Mutex};

use crate::{app::{redis::service::RedisApplicationService, application_service::ServiceTrait, application_model::ApplicationModel}, persistence};

use async_trait::async_trait;
use crate::app::application_service;
use crate::app::redis::model::{DataStructure};
use super::service_model::{WebsocketServiceError};
use crate::persistence::redis::RedisProvider;


use serde_json;
use redis::AsyncCommands;


use super::service_model;
use futures_util::StreamExt;

use actix;


use crate::server::WebSocketServer;
use crate::server::messages as ServerMessage;

use thiserror::Error;
use anyhow::Result;




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

    async fn subscribe_channels(&mut self,  channel: Vec<service_model::ChannelData>, addr: actix::Addr<WebSocketServer>) 
        -> Result<(), WebsocketServiceError>;


    async fn publish_channel(&mut self,  channel: String,  message: String) 
        -> Result<(), WebsocketServiceError>;


    fn stop(&mut self) -> ();
    


}

pub struct WebsocketService {
    redis_provider: RedisProvider,
    max_channels: usize,
    running: Arc<Mutex<bool>>

}

impl WebsocketService {
    pub fn new() -> Self {

        let provider = RedisProvider::new();
        
        Self {
            redis_provider: provider,
            max_channels: 10000,
            running: Arc::new(Mutex::new(true))
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
            .map_err(|e| WebsocketServiceError::StoreError(e.to_string()))?;

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


    async fn subscribe_channels(&mut self, channels: Vec<service_model::ChannelData>, addr: actix::Addr<WebSocketServer>) -> Result<(), WebsocketServiceError>{
        
        let conn = self.redis_provider
            .connect()
            .await
            .map_err(|e| WebsocketServiceError::RedisConnectionError(e.to_string()))?;

        let mut pubsub = conn.into_pubsub();
        
        for ch in  channels {
            let sub_result = pubsub.subscribe(ch.name).await;
            
            if let Err(e) = sub_result {
                println!("Got subscribing error: {}", e.to_string());
                continue;
            }
        }
        let mut msg_stream = pubsub.into_on_message();
        
        let running = self.running.clone();

        loop{
            println!("hi in the loop guys");
            let msg = msg_stream.next().await;
            
            if let Some(msg) = msg{
                let payload: Result<String, redis::RedisError> = msg.get_payload();
                let payload = match payload {
                    Err(e) => continue,
                    Ok(v) => v
                };

                println!("got payload: {}", payload);

                addr.do_send(ServerMessage::SendChannel{
                    msg: payload,
                    channel_name: msg.get_channel_name().to_string()
                });

               

            }

        };
       

        Ok(())
    }


    fn stop(&mut self){
        let mut running = self.running.lock().expect("unable to lock the running mutex");
        *running = false;
    }


    async fn publish_channel(&mut self,  channel: String,  message: String) 
        -> Result<(), WebsocketServiceError> {
    

        let conn = self.redis_provider
            .get_connection()
            .await
            .map_err(|e|WebsocketServiceError::RedisConnectionError(e.reason))?;
        
        let res:Result<(), redis::RedisError>= conn
            .publish(channel, message)
            .await;
        
        

        Ok(())
    }

}