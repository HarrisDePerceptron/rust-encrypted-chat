use mongodb::{Client, options::ClientOptions};
use anyhow::Result;

use log;

#[derive(Debug, Clone)]
pub struct MongoProvider {
    uri: String
}

impl MongoProvider {
    pub fn new(uri: &str) -> Self{
        Self {
            uri: uri.to_string()
        }
    }

    pub async fn connect(&self) -> Result<mongodb::Client> {
        log::info!("Connecting to mongo: {}", self.uri);

        let mut client_options = ClientOptions::parse(&self.uri)
            .await?;

        let client = Client::with_options(client_options)?;

        Ok(client)

    }
}

