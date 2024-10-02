use serde::{Deserialize, Serialize};

#[derive(Debug , Serialize ,Deserialize)]
pub struct UserModel {
    pub email : String,
    pub password : Option<String>,
}