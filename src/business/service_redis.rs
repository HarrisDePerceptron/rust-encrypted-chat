
use std::fmt::Debug;

use serde::{Serialize, Deserialize};
use crate::business::application_model::ApplicationModel;
use crate::business::application_service::{ApplicationServiceTrait, ApplicationServiceError};


use async_trait::async_trait;

use crate::persistence::redis::{RedisProvider, RedisProviderError};

struct RedisApplicationService<'a>{
    provider: &'a mut RedisProvider
}

#[async_trait]
impl<'a,T> ApplicationServiceTrait<'a,T> for RedisApplicationService<'a>
where 
        T: Debug + Serialize + Clone + Deserialize<'a> + Send + 'static
{
    type Model = ApplicationModel<T> ;

    async fn create(&mut self, data: Self::Model) -> Result<Self::Model, ApplicationServiceError> {
        let conn = self.provider.get_connection()
                                                   .await
                                                   .map_err(|e|  ApplicationServiceError::CreateError(e.reason))?;
        
        
        
        todo!()
    }

   async fn find(&mut self) -> Result<Vec<Self::Model>, ApplicationServiceError> {
        todo!()
    }

    async fn find_by_id(&mut self, id: &str) -> Result<Self::Model, ApplicationServiceError> {
        todo!()
    }

    async fn update_by_id(&mut self,data: Self::Model) -> Result<Self::Model, ApplicationServiceError> {
        todo!()
    }

    async fn delete(&mut self, id: &str) -> Result<String, ApplicationServiceError> {
        todo!()
    }
}
