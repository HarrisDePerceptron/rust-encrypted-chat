use crate::persistence::mongo::MongoProvider;
use crate::secrets;
use std::sync::Arc;

use tokio;

pub struct ApplicationFactory {
    pub mongo_database: Arc<mongodb::Database>,
}

impl ApplicationFactory {
    pub async fn new() -> Self {
        let mongo_uri = secrets::MONGO_URI.to_string();
        let mongo_database = secrets::MONGO_DATABASE.to_string();

        let mongo_provider = MongoProvider::new(&mongo_uri);
        let client = mongo_provider
            .connect()
            .await
            .expect(&format!("Unable to connect to {}", mongo_uri));

        let db = client.database(&mongo_database);

        Self {
            mongo_database: Arc::new(db),
        }
    }
}
