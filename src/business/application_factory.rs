use crate::business::user::factory::{UserFactory};
use crate::business::user::service::{UserService};

use std::collections::HashMap;
use once_cell::sync::Lazy;


use std::cmp::Eq;

use crate::business::service_redis::RedisApplicationService;
use crate::persistence;



pub trait FactoryTrait<'a> {
    type Service;
    fn get(&'a mut self) -> &'a mut Self::Service ;
 
}


// #[derive(Hash)]
// #[derive(PartialEq, Eq)]
// enum FactoryKey {
//     USER
// }

// enum ApplicationFactory<'a> {
//     USER(Box<dyn FactoryTrait<'a, Service=UserService<'a>>>)
// }

// pub struct ApplicationServiceFactory<'a>{
//     factories: Vec<ApplicationFactory<'a>>
// }

// impl<'a> ApplicationServiceFactory<'a> {
//     fn get(&'a self, factory: ApplicationFactory) -> ApplicationFactory{
//         todo!()
//     }
// }


