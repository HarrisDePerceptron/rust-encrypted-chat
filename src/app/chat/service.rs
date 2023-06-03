use crate::persistence::mongo::MongoProvider;
use anyhow::Result;
use futures_util::TryStreamExt;
use  thiserror::Error;

use super::dao;


use mongodb;


#[derive(Error, Debug, Clone)]
pub enum ChatError {
    #[error("id not created")]
    IDNotCreated
}


pub struct ChatService {
    mongo_provider: MongoProvider,
    db: String, 
}


impl ChatService {
    pub fn new(provider: MongoProvider) -> Self {
        Self {
            mongo_provider: provider,
            db: "chat".to_string(),
        }
    }

    pub async fn create_room(&self, name: &str, user: dao::User) -> Result<String> {
        let client = self.mongo_provider
            .connect()
            .await?;
        let db = client.database(&self.db);
        let collection = String::from("room");

        let mut room = dao::Room::new(name, user);
        
        let col = db.collection::<dao::Room>(&collection);
        
        let result = col
            .insert_one(room, None)
            .await?;

        let id = result.inserted_id
            .as_str()
            .ok_or(ChatError::IDNotCreated)?
            .to_string();


        Ok(id)
        
    } 



    pub async fn create_user(&self, name: &str) -> Result<dao::DaoResponse<dao::DaoRequest<dao::User>>> {
        let client = self.mongo_provider
            .connect()
            .await?;
        let db = client.database(&self.db);
        let collection = String::from("chat_user");

        let mut user = dao::DaoRequest::new(dao::User::new(name));


        
        let col = db.collection::<dao::DaoRequest<dao::User>>(&collection);
        
        let result = col
            .insert_one(user.clone(), None)
            .await?;

        let oid = result.inserted_id.as_object_id()
            .ok_or(ChatError::IDNotCreated)?;

        let id = oid.to_string();


        let mut response = dao::DaoResponse::new(user);
        response._id =  Some(oid);
        response.id = Some(id);

        Ok(response)
        
    } 


    pub async fn list_users(&self, limit: i64) -> Result<dao::ResponseList<dao::User>>  {
        let client = self.mongo_provider
            .connect()
            .await?;
        let db = client.database(&self.db);
        let collection = String::from("chat_user");
        
        let col = db.collection::<dao::DaoResponse<dao::DaoRequest<dao::User>>>(&collection);
        
        let sort = mongodb::bson::Document::from(mongodb::bson::doc!{"created_at": 1});

        let find_options = mongodb::options::FindOptions::builder()
            .limit(limit)
            .sort(sort)
            .build();
        

        let mut cursor = col.find(None, find_options)
            .await?;

        let mut users: Vec<dao::DaoResponse<dao::DaoRequest<dao::User>>> = vec![];

        while let Some(mut u) = cursor.try_next().await? {
            if let Some(id) = u._id {
                u.id = Some(id.to_string());
            }
            
            users.push(u);
        }

        Ok(dao::ResponseList(users))
        
    } 


}