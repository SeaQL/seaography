#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GuardAction {
    Block(Option<String>),
    Allow,
}

pub fn guard_error(reason: Option<String>, fallback: &str) -> async_graphql::Error {
    match reason {
        Some(reason) => async_graphql::Error::new(reason),
        None => async_graphql::Error::new(fallback),
    }
}
