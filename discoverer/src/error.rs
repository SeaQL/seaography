use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    SqlxError(sqlx::Error),
    #[error("Discovery error: {0}")]
    Error(String),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::SqlxError(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::Error(err.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
