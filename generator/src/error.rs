#[derive(Debug)]
pub enum Error {
    Error(String),
    SeaOrmCodegenError(sea_orm_codegen::Error),
    IoError(std::io::Error),
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

pub type Result<T> = std::result::Result<T, Error>;
