
use std::fmt::Debug;

use crate::app::application_model::ApplicationModel;
use crate::app::application_service::{ApplicationServiceError, ApplicationServiceTrait};

use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use crate::persistence;
use async_trait::async_trait;

use crate::persistence::redis::{RedisProvider};
use redis::AsyncCommands;
use redis::Commands;

use crate::utils;
use super::model::DataStructure;



pub struct RedisApplicationService {
    provider: RedisProvider,
    name: String,
    datastructure: DataStructure,
}

impl RedisApplicationService {
    pub fn new(name: &str, provider: RedisProvider, datastructure: DataStructure) -> Self {
        Self {
            provider: provider,
            name: name.to_string(),
            datastructure: datastructure,
        }
    }

    pub fn get_datastructure(&self) -> DataStructure {
        self.datastructure.clone()
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
                Err(_e) => continue,
                Ok(v) => v,
            };

            data.push(item);
        }

        Ok(data)
    }

    async fn create_unordered_set(
        conn: &mut redis::aio::Connection,
        key: &str,
        value: &str,
    ) -> Result<(), ApplicationServiceError> {
        conn.sadd(key.to_string(), value.to_string())
            .await
            .map_err(|e| ApplicationServiceError::CreateError(e.to_string()))?;

        Ok(())
    }

    async fn create_kv(
        conn: &mut redis::aio::Connection,
        key: &str,
        value: &str,
    ) -> Result<(), ApplicationServiceError> {
        conn.set(key.to_string(), value.to_string())
            .await
            .map_err(|e| ApplicationServiceError::CreateError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id_kv(
        conn: &mut redis::aio::Connection,
        id: &str,
    ) -> Result<String, ApplicationServiceError> {
        let result_str: String = conn.get(id).await.map_err(|e| {
            ApplicationServiceError::FindError(format!("Unable to find key: {}", e.to_string()))
        })?;

        Ok(result_str)
    }

    async fn find_by_id_unordered_set(
        conn: &mut redis::aio::Connection,
        id: &str,
    ) -> Result<String, ApplicationServiceError> {
        let result_str: String = conn.smembers(id).await.map_err(|e| {
            ApplicationServiceError::FindError(format!("Unable to find key: {}", e.to_string()))
        })?;

        Ok(result_str)
    }

    async fn create_(&mut self, key: &str, value: &str) -> Result<(), ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::CreateError(e.reason))?;

        match self.datastructure {
            DataStructure::UnorderedSet => Self::create_unordered_set(conn, key, value).await,
            DataStructure::KV => Self::create_kv(conn, key, value).await,
        }
    }

    async fn find_by_id_(&mut self, key: &str) -> Result<String, ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::CreateError(e.reason))?;

        match self.datastructure {
            DataStructure::UnorderedSet => Self::find_by_id_unordered_set(conn, key).await,
            DataStructure::KV => Self::find_by_id_kv(conn, key).await,
        }
    }

    async fn delete_kv(
        conn: &mut redis::aio::Connection,
        id: &str,
    ) -> Result<String, ApplicationServiceError> {
        let result_str: String = conn.del(id).await.map_err(|e| {
            ApplicationServiceError::DeleteError(format!("Unable to find key: {}", e.to_string()))
        })?;

        Ok(result_str)
    }

    async fn delete_any(
        conn: &mut redis::aio::Connection,
        id: &str,
    ) -> Result<(), ApplicationServiceError> {
        conn.del(id).await.map_err(|e| {
            ApplicationServiceError::DeleteError(format!("Unable to find key: {}", e.to_string()))
        })?;

        Ok(())
    }

    async fn delete_(&mut self, key: &str) -> Result<(), ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::CreateError(e.reason))?;

        match self.datastructure {
            DataStructure::UnorderedSet => Self::delete_any(conn, key).await,
            DataStructure::KV => Self::delete_any(conn, key).await,
        }
    }

    async fn find_kv(
        conn: &mut redis::aio::Connection,
        query: &str,
        count: usize,
    ) -> Result<Vec<String>, ApplicationServiceError> {
        let mut results: Vec<String> = Vec::new();

        {
            let mut res = conn
                .scan_match(query)
                .await
                .map_err(|e| ApplicationServiceError::FindAllError(e.to_string()))?;
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
        }

        let mut values: Vec<String> = Vec::new();

        for k in &results {
            let _v: String = conn
                .get(k)
                .await
                .map_err(|e| ApplicationServiceError::FindAllError(e.to_string()))?;

            values.push(k.to_string());
            
        }

        Ok(results)

    }

    async fn find_unordered_set(
        conn: &mut redis::aio::Connection,
        query: &str,
        count: usize,
    ) -> Result<Vec<String>, ApplicationServiceError> {
        let mut res = conn
            .sscan_match(query, "*")
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

    async fn find_(
        &mut self,
        query: &str,
        count: usize,
    ) -> Result<Vec<String>, ApplicationServiceError> {
        let conn = self
            .provider
            .get_connection()
            .await
            .map_err(|e| ApplicationServiceError::CreateError(e.reason))?;

        match self.datastructure {
            DataStructure::UnorderedSet => Self::find_unordered_set(conn, query, count).await,
            DataStructure::KV => Self::find_kv(conn, query, count).await,
        }
    }
}

#[async_trait]
impl<T> ApplicationServiceTrait<T> for RedisApplicationService
where
    T: Debug + Serialize + Clone + DeserializeOwned + Send + 'static,
{
    type Model = ApplicationModel<T>;

    async fn create(
        &mut self,
        mut data: Self::Model,
    ) -> Result<Self::Model, ApplicationServiceError> {
        let key = utils::generate_unique_id()
            .map_err(|e| ApplicationServiceError::CreateError(e.to_string()))?;

        let key = format!("{}:{}", self.name, key);

        data.id = Some(key.to_string());

        let data_str = serde_json::to_string(&data)
            .map_err(|e| ApplicationServiceError::CreateError(e.to_string()))?;

        self.create_(&key, &data_str).await?;

        return Ok(data);
    }

    async fn find(
        &mut self,
        query: String,
        count: usize,
    ) -> Result<Vec<Self::Model>, ApplicationServiceError> {
        let mut data: Vec<Self::Model> = Vec::new();

        let values = self.find_(&query, count).await?;


        for v in &values {
            let model: Self::Model = serde_json::from_str(&v)
                .map_err(|e| ApplicationServiceError::FindAllError(e.to_string()))?;


            data.push(model);
        }

        Ok(data)
    }

    async fn find_by_id(&mut self, id: &str) -> Result<Self::Model, ApplicationServiceError> {
        let result_str = self.find_by_id_(id).await?;

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

        self.create_(&id, &data_str).await?;

        Ok(model)
    }

    async fn delete(&mut self, id: &str) -> Result<String, ApplicationServiceError> {

        self.delete_(id).await?;

        Ok(id.to_string())
    }
}