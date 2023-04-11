use std::borrow::BorrowMut;
use std::fmt::Debug;

use crate::business::application_model::ApplicationModel;
use crate::business::application_service::{ApplicationServiceError, ApplicationServiceTrait};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

use async_trait::async_trait;

use crate::persistence::redis::{RedisProvider, RedisProviderError};
use redis::AsyncCommands;
use redis::Commands;

use crate::utils;

pub struct RedisApplicationService<'a> {
    provider: &'a mut RedisProvider,
    name: String,
}

impl<'a> RedisApplicationService<'a> {
    pub fn new(name: &str, provider: &'a mut RedisProvider) -> Self {
        Self {
            provider: provider,
            name: name.to_string(),
        }
    }

    async fn get_keys(
        &mut self,
        pattern: &str,
        count: usize,
    ) -> Result<Vec<String>, ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::FindError(e.reason))?;

        let mut res = conn
            .scan_match(pattern)
            .await
            .map_err(|e| ApplicationServiceError::FindAllError(e.to_string()))?;

        let mut results: Vec<String> = Vec::new();

        let mut i = 0;
        while i < count {
            let item: Option<String> = res.next_item().await;

            if let None = item {
                break;
            }

            let key_item = item.ok_or(ApplicationServiceError::FindAllError(
                "Item not found".to_string(),
            ))?;
            results.push(key_item);
            i += 1;
        }

        Ok(results)
    }

    async fn get_values(
        &mut self,
        keys: Vec<String>,
    ) -> Result<Vec<String>, ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::FindError(e.reason))?;

        let mut data: Vec<String> = Vec::new();

        for k in &keys {
            let item: String = match conn.get(k).await {
                Err(e) => continue,
                Ok(v) => v,
            };

            data.push(item);
        }

        Ok(data)
    }
}

#[async_trait]
impl<'a, T> ApplicationServiceTrait<T> for RedisApplicationService<'a>
where
    T: Debug + Serialize + Clone + DeserializeOwned + Send + 'static,
{
    type Model = ApplicationModel<T>;

    async fn create(
        &mut self,
        mut data: Self::Model,
    ) -> Result<Self::Model, ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::CreateError(e.reason))?;

        let key = utils::generate_unique_id()
            .map_err(|e| ApplicationServiceError::CreateError(e.to_string()))?;

        let key = format!("{}:{}", self.name, key);

        data.id = Some(key.to_string());

        let data_str = serde_json::to_string(&data)
            .map_err(|e| ApplicationServiceError::CreateError(e.to_string()))?;

        conn.set(key.to_string(), data_str)
            .await
            .map_err(|e| ApplicationServiceError::CreateError(e.to_string()))?;

        return Ok(data);
    }

    async fn find(&mut self, count: usize) -> Result<Vec<Self::Model>, ApplicationServiceError> {
        let pattern = format!("{}:*", self.name);

        let mut data: Vec<Self::Model> = Vec::new();

        let keys = self.get_keys(&pattern, count).await?;

        let values = self.get_values(keys).await?;

        for v in &values {
            let model: Self::Model = match serde_json::from_str(&v) {
                Err(e) => {
                    // println!("error converting model: {}: {}", e.to_string(), v);
                    continue;
                }
                Ok(v) => v,
            };

            data.push(model);
        }

        Ok(data)
    }

    async fn find_by_id(&mut self, id: &str) -> Result<Self::Model, ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::FindError(e.reason))?;

        let result_str: String = conn
            .get(id)
            .await
            .map_err(|e| ApplicationServiceError::FindError(format!("Unable to find key: {}",e.to_string())))?;

        let result: Self::Model = serde_json::from_str(&result_str)
            .map_err(|e| ApplicationServiceError::FindError(e.to_string()))?;

        return Ok(result);
    }

    async fn update_by_id(
        &mut self,
        data: Self::Model,
    ) -> Result<Self::Model, ApplicationServiceError> {
        let id = data.id.ok_or(ApplicationServiceError::UpdateError(
            "Id is none".to_string(),
        ))?;

        let mut model: Self::Model = self.find_by_id(&id).await?;
        model.data = data.data;

        let data_str = serde_json::to_string(&model)
            .map_err(|e| ApplicationServiceError::UpdateError(e.to_string()))?;

        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::UpdateError(e.reason))?;

        conn.set(id, data_str)
            .await
            .map_err(|e| ApplicationServiceError::UpdateError(e.to_string()))?;

        Ok(model)
    }

    async fn delete(&mut self, id: &str) -> Result<String, ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::UpdateError(e.reason))?;

        conn.del(id)
            .await
            .map_err(|e| ApplicationServiceError::UpdateError(e.to_string()))?;

        Ok(id.to_string())
    }
}
