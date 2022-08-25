#[derive(Debug)]
pub enum Error {
    Error(String),
    SeaOrmCodegenError(sea_orm_codegen::Error),
    IoError(std::io::Error),
    ParseError(url::ParseError),
    DiscovererError(seaography_discoverer::Error),
}

impl From<sea_orm_codegen::Error> for Error {
    fn from(err: sea_orm_codegen::Error) -> Self {
        Self::SeaOrmCodegenError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<seaography_discoverer::Error> for Error {
    fn from(err: seaography_discoverer::Error) -> Self {
        Self::DiscovererError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
