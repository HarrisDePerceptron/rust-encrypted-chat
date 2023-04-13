
use std::fmt::Debug;

use serde::{Serialize, Deserialize, de::DeserializeOwned};
use crate::app::application_model::{ApplicationModel, ApplicationModelTrait};

use std::future::Future;
use async_trait::async_trait;


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
    T: Debug + Serialize + Clone + DeserializeOwned + 'static
 {
    type Model: ApplicationModelTrait<T>;
    
    async fn create(&mut self, data: Self::Model) -> Result<Self::Model, ApplicationServiceError>;
    async fn find(&mut self, count: usize) ->  Result<Vec<Self::Model>, ApplicationServiceError>;
    async fn find_by_id(&mut self, id: &str) -> Result<Self::Model, ApplicationServiceError>;

    async fn update_by_id(&mut self, data: Self::Model) ->  Result<Self::Model, ApplicationServiceError>;
    
    async fn delete(&mut self, id:  &str) ->  Result<String, ApplicationServiceError>;
}


// struct ApplicationService{}


// impl<'a,T> ApplicationServiceTrait<'a,T> for ApplicationService 
// where 
//         T: Debug + Serialize + Clone + Deserialize<'a>
// {
//     type Model = ApplicationModel<T>;

//     fn create(&self, data: Self::Model) -> Result<Self::Model, ApplicationServiceError> {
//         todo!()
//     }

//     fn find(&self) -> Result<Vec<Self::Model>, ApplicationServiceError> {
//         todo!()
//     }

//     fn find_by_id(&self, id: &str) -> Result<Self::Model, ApplicationServiceError> {
//         todo!()
//     }

//     fn update_by_id(&self,data: Self::Model) -> Result<Self::Model, ApplicationServiceError> {
//         todo!()
//     }

//     fn delete(&self, id: &str) -> Result<String, ApplicationServiceError> {
//         todo!()
//     }
// }
