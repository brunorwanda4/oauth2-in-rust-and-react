use serde::{Deserialize, Serialize};

pub type DbResult<T> = core::result::Result<T , DbError>;

#[derive(Debug , Deserialize , Serialize)]
pub enum DbError {
    // database error  (generic)
    ConnectionError,
    DatabaseError,
    InvalidId,
    // users error
    CreateUserError,
    UserNotFound,
    UserEmailIsReadyExit {email : String},
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::ConnectionError => write!(f, "Failed to connect to the database"),
            DbError::DatabaseError => write!(f, "Some thing went wrong with database"),
            DbError::InvalidId => write!(f, "Invalid id"),
            // users error
            DbError::CreateUserError => write!(f, "Failed to create user"),
            DbError::UserNotFound => write!(f, "User not found"),
            DbError::UserEmailIsReadyExit {email}=> write!(f, "User with this email already exists : {}", email),
        }
    }
}