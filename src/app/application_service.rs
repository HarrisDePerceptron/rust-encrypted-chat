use serde::{Serialize, Deserialize, de::DeserializeOwned};
use crate::app::application_model::{ApplicationModel, ApplicationModelTrait};

use std::future::Future;
use async_trait::async_trait;

use std::fmt;



#[derive(Debug, Clone, Serialize)]
pub enum ApplicationServiceError{
    CreateError(String),
    FindError(String),
    FindAllError(String),
    UpdateError(String),
    DeleteError(String),
}

#[async_trait]
pub trait ApplicationServiceTrait <T>
where 
    T: fmt::Debug + Serialize + Clone + DeserializeOwned + 'static
 {
    type Model: ApplicationModelTrait<T>;
    
    async fn create(&mut self, data: Self::Model) -> Result<Self::Model, ApplicationServiceError>;
    async fn find(&mut self, query: String, count: usize) ->  Result<Vec<Self::Model>, ApplicationServiceError>;
    async fn find_by_id(&mut self, id: &str) -> Result<Self::Model, ApplicationServiceError>;

    async fn update_by_id(&mut self, data: Self::Model) ->  Result<Self::Model, ApplicationServiceError>;
    
    async fn delete(&mut self, id:  &str) ->  Result<String, ApplicationServiceError>;
}




#[async_trait]
pub trait ServiceTrait  {

    type Error;

    type Model: fmt::Debug + Serialize + Clone  + serde::de::DeserializeOwned + 'static;

    type Persistence: self::ApplicationServiceTrait<Self::Model, Model = ApplicationModel<Self::Model>>; 
    
    
    fn get_persistence_service(&mut self) -> &mut Self::Persistence;
}