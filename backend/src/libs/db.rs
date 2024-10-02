use mongodb::results::{CreateIndexResult, InsertOneResult};
use mongodb::{Client, Collection,};

use crate::models::user_model::UserModel;
use crate::errors::db_error::DbResult;
pub struct Db {
    user : Collection<UserModel>
}

impl Db {
    pub async fn init () -> DbResult<Self>{
        let uri = "mongodb://localhost:27017/".to_string();
        let client = Client::with_uri_str(uri).await.unwrap();
        let database = client.database("oauth-rust");
        
        let user : Collection<UserModel> = database.collection("users");

        Ok(Self { user })
    }

    pub async fn create_user(&self, user: UserModel) -> DbResult<InsertOneResult>{
        let res = self
            .user
            .insert_one(user)
            .await
            .ok()
            .expect("Failed to create index for user");
        Ok(res)
    }
}