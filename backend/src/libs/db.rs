use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::IndexOptions;
use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection, IndexModel,};

use crate::models::user_model::UserModel;
use crate::errors::db_error::{DbError, DbResult};
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
        let index_model = IndexModel::builder()
            .keys(doc! {
                "email" : 1
            })
            .options(
                IndexOptions::builder()
                    .unique(true)
                    .build()
            )
            .build();

        self.user.create_index(index_model).await;


        let new_user = UserModel::new(user.email, user.password);

        let res = self
            .user
            .insert_one(new_user)
            .await
            .ok()
            .expect("Failed to create index for user");
        Ok(res)
    }

    pub async fn get_user_by_id(&self, id:String) -> DbResult<UserModel> {
        let user_id = ObjectId::parse_str(id).map_err(|_| DbError::InvalidId)?;
        
        let user = self
            .user
            .find_one(doc! {"_id" : user_id})
            .await;

        match user {
            Ok(Some(res)) => Ok(res),
            Ok(None) => Err(DbError::UserNotFound),
            Err(_) => Err(DbError::UserNotFound),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> DbResult<UserModel>{
        let user = self
            .user
            .find_one(doc! {"email" : email})
            .await;

        match user {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(DbError::UserNotFound),
            Err(_) => Err(DbError::UserNotFound),  
        }

    }
}