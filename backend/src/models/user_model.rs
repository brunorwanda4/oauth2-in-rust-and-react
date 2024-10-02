use bcrypt::hash;
use serde::{Deserialize, Serialize};

#[derive(Debug , Serialize ,Deserialize)]
pub struct UserModel {
    pub email : String,
    pub password : Option<String>,
}

impl UserModel {
    pub fn new(
        email : String,
        password : Option<String>,
    ) -> Self {
        let hash_pass = password.as_ref()
            .map(|pw| hash(pw, 10).unwrap());
        UserModel {
            email,
            password : hash_pass,
        }
    }
}