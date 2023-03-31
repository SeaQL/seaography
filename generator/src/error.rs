use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Code generator: {0}")]
    Error(String),
    #[error("IO: {0}")]
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
