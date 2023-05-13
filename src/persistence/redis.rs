use redis;
use redis::aio;


pub struct RedisProvider {
    connection: Option<aio::Connection>,
}

pub struct RedisProviderError {
    pub reason: String,
}

impl RedisProvider {
    pub fn new() -> Self {
        Self { connection: None }
    }
    pub async fn connect(&self) -> redis::RedisResult<aio::Connection> {
        let client = redis::Client::open("redis://:87654321@localhost:6379").unwrap();
        let con = client.get_async_connection().await?;


        Ok(con)
    }

    pub async fn get_connection(&mut self) -> Result<&mut aio::Connection, RedisProviderError> {

        if let None  = self.connection {

            let connection = self.connect().await.map_err(|e| RedisProviderError {
                reason: e.to_string(),
            })?;
    
            self.connection = Some(connection);

            match &mut self.connection {
                None => panic!("should be set"),
                Some(v) => Ok(v)
            }
            
        }else{

            match &mut self.connection {
                None => panic!("should be set"),
                Some(v) => Ok(v)
            }
        }


    }
}
