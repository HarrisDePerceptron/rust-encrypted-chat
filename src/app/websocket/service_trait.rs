use async_trait::async_trait;
use crate::app::application_service;


#[async_trait]
pub trait WebsocketServiceTrait
where
    Self: application_service::ServiceTrait,
{
    async fn store(
        &mut self,
    ) -> Result<(), Self::Error>;

}