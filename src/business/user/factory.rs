


use super::service::UserService;
use crate::persistence::redis::RedisProvider;

use crate::business::service_redis::{RedisApplicationService, RedisFactory};
use crate::business::application_factory::FactoryTrait;



pub struct UserFactory<'a, 'b> 
where
    'a: 'b
{
    service: Option<UserService<'a>>,
    redis_factory: &'b mut RedisFactory<'b, 'b>

}

impl<'a, 'b> UserFactory<'a, 'b>
where 
    'b: 'a
{
    // pub fn new(service: &'b mut RedisApplicationService<'b>) -> Self{

    //     Self { service: None, redis_service:  service }
    // }

    pub fn new(redis_factory: &'b mut RedisFactory<'b,'b>) -> Self{

        Self { service: None, redis_factory: redis_factory }
    }
}


impl<'a, 'b> FactoryTrait<'a> for UserFactory<'a, 'b> 
    where 
        'b: 'a
{
    type Service = UserService<'a>;

    fn get(&'a mut self) -> &'a mut Self::Service  {
        if let None = self.service{
            let redis_service = self.redis_factory.get();

            let service = UserService::new(redis_service);
            self.service = Some(service);
            
            match &mut self.service {
                None => panic!("Should be set"),
                Some(v) => v
            }
        }else {
            match &mut self.service {
                None => panic!("Should be set"),
                Some(v) => v
            }
        }
    }

}
