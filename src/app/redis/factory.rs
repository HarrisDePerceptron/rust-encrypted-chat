use crate::app::application_factory::FactoryTrait;
use super::model::DataStructure;
use super::service::RedisApplicationService;
use crate::persistence;




pub struct RedisFactory {
    schema_name: String,
    datastructure: DataStructure,
}

impl RedisFactory {
    pub fn new(schema_name: &str, datastructure: DataStructure) -> Self {
        Self {
            schema_name: schema_name.to_string(),
            datastructure: datastructure,
        }
    }
}

impl FactoryTrait for RedisFactory {
    type Service = RedisApplicationService;

    fn get(&self) -> Self::Service {
        let redis_provider = persistence::redis::RedisProvider::new();
        let service = Self::Service::new(
            &self.schema_name,
            redis_provider,
            self.datastructure.clone(),
        );
        service
    }
}
