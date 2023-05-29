
use super::websocket::WebSocketServer;




use async_trait::async_trait;

use actix::{Context, Actor, WrapFuture, ActorFutureExt, ContextFutureSpawner, AsyncContext};
use  super::channel::Channel;

use crate::app::websocket::{WebsocketService, ChannelData,WebsocketServiceTrait};
use super::model;


#[async_trait]
pub trait WebsocketPersistence
where
    Self: Actor
{

    fn store_channel(channel: Channel,_self: &Self,  ctx: &mut Self::Context);
    fn load_channels(_self: &Self,  ctx: &mut Self::Context);
    fn on_load_channels(_self: &Self, ctx: &mut Self::Context) -> ();
    fn publish_to_channels(_self: &Self,  ctx: &mut Self::Context, channel: &str, message: &str);

}


// #[async_trait]
// impl WebsocketPersistence for WebSocketServer 
// {
//     fn store_channel(channel: Channel,_self: &Self,  ctx: &mut Context<Self>){

//         async {
//             let mut service = WebsocketService::new();
//             let data = ChannelData {
//                 name: channel.name
//             };

    
//             service.store(data)
//                 .await
//                 .map_err(|e| model::WebsocketServerError::ChannelStoreError(e.to_string()))

//         }.into_actor(_self)
//         .then(|res, _self, _ctx| {
            
//             if let Err(e) = res {
//                 println!("Failed to store channel: {}", e.to_string());
//             }
//             actix::fut::ready(())
//         }).wait(ctx);

//         ()
//     }
    
//     fn load_channels(_self: &Self,  ctx: &mut Context<Self>){

//         async {
//             let mut service = WebsocketService::new();
    
//             service.load()
//                 .await
//                 .map_err(|e| model::WebsocketServerError::ChannelStoreError(e.to_string()))

//         }.into_actor(_self)
//         .then(|res, _self, _ctx| {
         
            
//             let channels = match res {
//                 Err(e) =>{
//                     println!("Failed to store channel: {}", e.to_string());
//                     return  actix::fut::ready(())
//                 },
//                 Ok(v) => v
//             };
            
//             let channels: Vec<Channel> = channels.iter()
//                 .map(|e| Channel::new(&e.name))
//                 .collect();

//             for ch in channels{
//                 _self.add_channel(ch)
//                     .unwrap_or_else(|e| println!("Error adding channel: {}", e.to_string()));

//             }
//             Self::on_load_channels(_self, _ctx);
            
//             actix::fut::ready(())
//         }).wait(ctx);

//         ()
//     }

//     fn on_load_channels(_self: &Self, ctx: &mut Context<Self>) -> (){
        
//         let channels: Vec<ChannelData> = _self.channels
//             .iter()
//             .map(|(k , v)| ChannelData{
//                 name: v.name.to_string()
//             })
//             .collect();

//         println!("Loaded channels: {:?}", channels);
        
        
//         let current_channels = _self.channels.clone();

//         let (tx, rx): (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>) = std::sync::mpsc::channel();

//         let tx1 = tx.clone();
//         let addr = ctx.address();

//         let execution =  async move {
//                 let mut service = WebsocketService::new();


//                 let res = service.subscribe_channels(channels, addr)
//                     .await
//                     .map_err(|e| model::WebsocketServerError::ChannelStoreError(e.to_string()));

//                 if let Err(e) = res {
//                     println!("Subscription error: {}", e.to_string());
//                 }


//         };
    
//             // _self.arbiter_subscriber.spawn(execution);


//         ()
//     }

//     fn publish_to_channels(_self: &Self,  ctx: &mut Context<Self>, channel: &str, message: &str){

//         let ch = channel.to_string();
//         let msg = message.to_string();

//         async {
//             let mut service = WebsocketService::new();
    
//             service.publish_channel(ch, msg)
//                 .await
//                 .map_err(|e| model::WebsocketServerError::ChannelStoreError(e.to_string()))

//         }.into_actor(_self)
//         .then(|res, _self, _ctx| {
         
            
//             match res {
//                 Err(e) =>{
//                     println!("Failed to publish to channel: {}", e.to_string());
//                     return  actix::fut::ready(())
//                 },
//                 Ok(v) => v
//             };
            
        
//             actix::fut::ready(())
//         }).wait(ctx);

//         ()
//     }


// }