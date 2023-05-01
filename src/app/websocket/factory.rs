


use super::service::WebsocketService;
use crate::app::service_redis::{RedisFactory, DataStructure};
use crate::app::application_factory::FactoryTrait;



pub struct WebsocketServiceFactory
{
    redis_factory: RedisFactory,

}

impl WebsocketServiceFactory
{
    pub fn new() -> Self{
        let  redis_factory = RedisFactory::new("websocket", DataStructure::KV);
        
        Self {redis_factory: redis_factory}
    }
}


impl FactoryTrait for WebsocketServiceFactory
{
    type Service = WebsocketService;

    fn get(&self) -> Self::Service  {
        let redis_service = self.redis_factory.get();
        let service = WebsocketService::new(redis_service);
        service
    }

}
