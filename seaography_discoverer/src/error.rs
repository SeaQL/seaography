#[derive(Debug)]
pub enum Error {
    SqlxError(sqlx::Error),
    Error(String),
    SerdeJson(serde_json::Error),
    UrlParseError(url::ParseError)
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParseError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
