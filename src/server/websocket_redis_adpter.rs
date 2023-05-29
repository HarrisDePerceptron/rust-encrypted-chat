
use super::websocket::{WebSocketServer};




use async_trait::async_trait;
use serde::Serialize;


use crate::app::websocket::{WebsocketService, ChannelData,WebsocketServiceTrait, WebsocketServiceError};
use super::model;
use crate::persistence::redis::{RedisProvider, RedisProviderError};

use thiserror::Error;
use anyhow::Result;

use log;


use futures_util::StreamExt;

use std::{sync::mpsc, fmt::format};





#[derive(Error, Debug)]
enum WebsocketAdpterError {
    
}


#[derive(Debug, Clone, Serialize)]
pub struct AdapterMessage {
    pub channel: String,
    pub message: String
}

#[async_trait]
pub trait WebsocketAdapter
{
    async fn subscribe(&self, pattern: &str, sender: mpsc::Sender<AdapterMessage>) -> Result<()>;
    async fn publish_to_channel(&self, channel: &str, message: &str) -> Result<()>;

}


pub struct RedisWebsocketAdapter {
    redis_provider: RedisProvider,
    prefix: String

}

impl RedisWebsocketAdapter {
    pub fn  new(prefix: &str,  provider: RedisProvider) -> Self{
        
        Self {
            prefix: prefix.to_string(),
            redis_provider: provider
        }
    }
}

#[async_trait]
impl WebsocketAdapter for RedisWebsocketAdapter 
{
    async fn publish_to_channel(&self, channel: &str, message: &str) -> Result<()>{

        // let ch = channel.to_string();
        let msg = message.to_string();
        let mut service = WebsocketService::new();
        
        let channel_name = format!("{}:{}", self.prefix, channel);

        log::debug!("publishing: {}: {}", channel_name, msg);
        service.publish_channel(channel_name, msg)
            .await?;
            
        Ok(())
    }

    async fn subscribe(&self, pattern: &str,  sender: mpsc::Sender<AdapterMessage>) ->  Result<()> {


        let conn = self.redis_provider
            .connect()
            .await
            .map_err(|e| WebsocketServiceError::RedisConnectionError(e.to_string()))?;

        let mut pubsub = conn.into_pubsub();

        let pattern = format!("{}:{}", self.prefix, pattern);


        log::info!("subscriping to pattern: {}", pattern);

        pubsub.psubscribe(pattern.to_string()).await?;

    
        let mut msg_stream = pubsub.into_on_message();

        loop{
            println!("hi in the loop guys");
            let msg = msg_stream.next().await;

            let msg = match msg {
                Some(v) => v,
                None => continue
            };
            
           
            let payload: String =  match msg.get_payload(){
                Ok(v) => v,
                Err(e)=> {
                    log::error!("{}", e.to_string());
                    continue;
                }
            };

            
            let mut channel_name = msg.get_channel_name().to_string();

            let pattern = format!("{}:", self.prefix);
            channel_name = channel_name.replace(&pattern, "");


            log::info!("Before: {}, After: {}", msg.get_channel_name() ,channel_name);

            let message = AdapterMessage {
                channel: channel_name,
                message: payload
            };
            
            match sender.send(message){
                Ok(v) => v,
                Err(e)=> {
                    log::error!("{}", e.to_string());
                    continue;
                }
            };               

        };

        Ok(())
    }


}