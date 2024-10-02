pub type DbResult<T> = core::result::Result<T , DbError>;

#[derive(Debug)]
pub enum DbError {
    ConnectionError,
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::ConnectionError => write!(f, "Failed to connect to the database"),
        }
    }
}