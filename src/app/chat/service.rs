use crate::persistence::mongo::MongoProvider;
use anyhow::Result;
use futures_util::TryStreamExt;
use thiserror::Error;

use super::dao;

use mongodb;
use std::sync::Arc;


#[derive(Error, Debug, Clone)]
pub enum ChatError {
    #[error("id not created")]
    IDNotCreated,
    #[error("id {0} not found")]
    IDNotFound(String),

    #[error("user already has online status {0}")]
    AlreadyOnlineStatus(bool)
}

pub struct ChatService {
    // mongo_provider: MongoProvider,
    mongo_database: Arc<mongodb::Database>,
}

impl ChatService {
    pub fn new(database: Arc<mongodb::Database>) -> Self {
        Self {
           mongo_database: database.clone()
        }
    }

    pub async fn create_room(&self, name: &str, user: dao::User) -> Result<String> {
        // let client = self.mongo_provider.connect().await?;
        // let db = client.database(&self.db);
        let db = self.mongo_database.clone();

        let collection = String::from("room");

        let mut room = dao::Room::new(name, user);

        let col = db.collection::<dao::Room>(&collection);

        let result = col.insert_one(room, None).await?;

        let id = result
            .inserted_id
            .as_str()
            .ok_or(ChatError::IDNotCreated)?
            .to_string();

        Ok(id)
    }

    pub async fn create_user(
        &self,
        name: &str,
    ) -> Result<dao::DaoResponse<dao::DaoRequest<dao::User>>> {
        // let client = self.mongo_provider.connect().await?;
        // let db = client.database(&self.db);
        let db = self.mongo_database.clone();
        let collection = String::from("chat_user");

        let mut user = dao::DaoRequest::new(dao::User::new(name));

        let col = db.collection::<dao::DaoRequest<dao::User>>(&collection);

        let result = col.insert_one(user.clone(), None).await?;

        let oid = result
            .inserted_id
            .as_object_id()
            .ok_or(ChatError::IDNotCreated)?;

        let id = oid.to_string();

        let mut response = dao::DaoResponse::new(user);
        response._id = Some(oid);
        response.id = Some(id);

        Ok(response)
    }

    pub async fn list_users(&self, start: u64, limit: i64) -> Result<dao::ResponseList<dao::User>> {
        // let client = self.mongo_provider.connect().await?;
        // let db = client.database(&self.db);
        let db = self.mongo_database.clone();
        let collection = String::from("chat_user");

        let col = db.collection::<dao::DaoResponse<dao::DaoRequest<dao::User>>>(&collection);

        let sort = mongodb::bson::Document::from(mongodb::bson::doc! {"created_at": -1});

        let find_options = mongodb::options::FindOptions::builder()
            .skip(start)
            .limit(limit)
            .sort(sort)
            .build();


        let filter = mongodb::bson::doc! {"delete": false};
        let mut cursor = col.find(filter, find_options).await?;

        let mut users: Vec<dao::DaoResponse<dao::DaoRequest<dao::User>>> = vec![];

        while let Some(mut u) = cursor.try_next().await? {
            if let Some(id) = u._id {
                u.id = Some(id.to_string());
            }

            users.push(u);
        }

        Ok(dao::ResponseList(users))
    }

    pub async fn find_user_by_id(&self, id: &str) -> Result<dao::DaoResponse<dao::DaoRequest<dao::User>>> {
        // let client = self.mongo_provider.connect().await?;
        // let db = client.database(&self.db);
        let db = self.mongo_database.clone();
        let collection = String::from("chat_user");

        let col = db.collection::<dao::DaoResponse<dao::DaoRequest<dao::User>>>(&collection);

        let oid = mongodb::bson::oid::ObjectId::parse_str(id)?;

        let query = mongodb::bson::doc! {"_id":  oid};

        let result = col.find_one(query, None)
            .await?;

        let result = result.
            ok_or(ChatError::IDNotFound(id.to_string()))?;

        
        Ok(result)
    }

    pub async fn set_user_online(
        &self,
        id: &str,
        online: bool,
    ) -> Result<dao::DaoResponse<dao::DaoRequest<dao::User>>> {
        // let client = self.mongo_provider.connect().await?;
        // let db = client.database(&self.db);
        let db = self.mongo_database.clone();
        let collection = String::from("chat_user");

        let col = db.collection::<dao::DaoResponse<dao::DaoRequest<dao::User>>>(&collection);

        
        
        let mut user = self.find_user_by_id(id)
            .await?;

        
        
        if user.online == online {
            return Err(ChatError::AlreadyOnlineStatus(online).into());

        }

        let oid =  user._id.ok_or(ChatError::IDNotCreated)?;
        let query = mongodb::bson::doc! {"_id":  oid };
        user.data.data.online = online;

        let update_doc = mongodb::bson::to_document(&user.data)?;

        let update = mongodb::bson::doc!{"$set": update_doc};

        let result = col.update_one(query, update, None).await?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: &str) -> Result<String>{

        // let client = self.mongo_provider.connect().await?;
        // let db = client.database(&self.db);
        let db = self.mongo_database.clone();
        let collection = String::from("chat_user");

        let col = db.collection::<dao::DaoResponse<dao::DaoRequest<dao::User>>>(&collection);

    
        let mut user = self.find_user_by_id(id)
            .await?;

        

        let oid =  user._id.ok_or(ChatError::IDNotCreated)?;
        let query = mongodb::bson::doc! {"_id":  oid };
        

        user.data.delete = true;

        let update_doc = mongodb::bson::to_document(&user.data)?;

        let update = mongodb::bson::doc!{"$set": update_doc};

        let result = col.update_one(query, update, None).await?;
        

        Ok(id.to_string())        
    }
}
