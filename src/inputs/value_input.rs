use async_graphql::dynamic::ValueAccessor;

pub trait InputValueHelper {
    /// Return None if Value is null
    fn maybe_string(&self) -> Result<Option<String>, async_graphql::Error>;
}

impl InputValueHelper for Option<ValueAccessor<'_>> {
    fn maybe_string(&self) -> Result<Option<String>, async_graphql::Error> {
        Ok(match self {
            Some(value) => {
                if value.is_null() {
                    None
                } else {
                    Some(value.string()?.to_owned())
                }
            }
            None => None,
        })
    }
}
