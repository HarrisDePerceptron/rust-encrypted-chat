


use super::service::UserService;
use crate::persistence::redis::RedisProvider;

use crate::app::service_redis::{RedisApplicationService, RedisFactory};
use crate::app::application_factory::FactoryTrait;



pub struct UserFactory
{
    redis_factory: RedisFactory,

}

impl UserFactory
{
    pub fn new() -> Self{
        let  redis_factory = RedisFactory::new("user");
        
        Self {redis_factory: redis_factory}
    }
}


impl FactoryTrait for UserFactory
{
    type Service = UserService;

    fn get(&self) -> Self::Service  {
        let redis_service = self.redis_factory.get();
        let service = UserService::new(redis_service);
        service
    }

}
