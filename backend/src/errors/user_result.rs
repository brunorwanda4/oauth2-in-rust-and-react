use serde::{Deserialize, Serialize};

#[derive(Debug , Serialize ,Deserialize)]
pub struct CreateUserResult {
    pub success : bool,
    pub message : String,
}
