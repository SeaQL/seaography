use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeaographyError {
    #[error("[async_graphql] {0:?}")]
    AsyncGraphQLError(async_graphql::Error),
    #[error("[int conversion] {0}")]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("[parsing] {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("[type conversion: {1}] {0}")]
    TypeConversionError(String, String),
    #[error("[array conversion] postgres array can not be nested type of array")]
    NestedArrayConversionError
}

impl From<async_graphql::Error> for SeaographyError {
    fn from(value: async_graphql::Error) -> Self {
        SeaographyError::AsyncGraphQLError(value)
    }
}

pub type SeaResult<T> = Result<T, SeaographyError>;