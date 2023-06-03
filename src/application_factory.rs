
use crate::persistence::mongo::MongoProvider;
use crate::secrets;

pub struct ApplicationFactory {
    pub mongo_provider: MongoProvider
}

impl ApplicationFactory {
    pub fn new() -> Self {
        Self {
            mongo_provider: MongoProvider::new(secrets::MONGO_URI.to_string().as_str())
        }
    }
}
