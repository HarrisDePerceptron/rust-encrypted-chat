use futures_util::Future;
use redis;
use redis::AsyncCommands;
use redis::Commands;

use redis::aio;

use actix::fut::{ready, Ready};
use actix_web::web;

pub struct RedisProvider {
    connection: Option<aio::Connection>,
}

pub struct RedisProviderError {
    reason: String,
}

impl RedisProvider {
    pub fn new() -> Self {
        Self { connection: None }
    }
    pub async fn connect(&self) -> redis::RedisResult<aio::Connection> {
        let client = redis::Client::open("redis://:87654321@localhost:6379").unwrap();
        let con = client.get_async_connection().await?;

        // con.set("key1", b"foo").await?;

        // redis::cmd("SET")
        //     .arg(&["key2", "bar"])
        //     .query_async(&mut con)
        //     .await?;

        // let result = redis::cmd("MGET")
        //     .arg(&["key1", "key2"])
        //     .query_async(&mut con)
        //     .await;
        // assert_eq!(result, Ok(("foo".to_string(), b"bar".to_vec())));

        // Ok(())

        Ok(con)
    }

    pub async fn get_connection(&mut self) -> Result<&mut aio::Connection, RedisProviderError> {
        // if let Some(connection) = &self.connection {
        //     return Ok(connection);
        // }

        if let None  = self.connection {

            let connection = self.connect().await.map_err(|e| RedisProviderError {
                reason: e.to_string(),
            })?;
    
            self.connection = Some(connection);
    
            return match &mut self.connection {
                Some(v) => Ok(v),
                None => panic!("Connection should be set to class instance"),
            };
        }else{

            match &mut self.connection {
                Some(v)=> Ok(v),
                None => panic!("Connection should be set to class instance")
            }
        }


    }
}
