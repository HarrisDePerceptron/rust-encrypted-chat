use super::model::User;
use async_trait::async_trait;
use super::service_model;
use crate::app::application_model::ApplicationModel;
use crate::app::application_service;


#[async_trait]
pub trait UserServiceTrait
where
    Self: application_service::ServiceTrait,
{
    async fn signup(
        &mut self,
        request: service_model::SignupRequest,
    ) -> Result<service_model::SignupResponse, Self::Error>;

    async fn get(
        &mut self,
        count: usize,
    ) -> Result<Vec<ApplicationModel<User>>, Self::Error>;

    async fn update(
        &mut self,
        model: ApplicationModel<User>,
    ) -> Result<ApplicationModel<User>, Self::Error>;

    async fn login(
        &mut self,
        request: service_model::LoginRequest,
    ) -> Result<service_model::LoginResponse, Self::Error>;


    async fn get_by_id(
        &mut self,
        id: &str,
    ) -> Result<ApplicationModel<User>, Self::Error>;



}