
use crate::app::user::factory::{UserFactory};

pub trait FactoryTrait {
    type Service;
    fn get(&self) -> Self::Service ;
 
}



pub struct ServiceFactory {
    pub user: UserFactory,
}
