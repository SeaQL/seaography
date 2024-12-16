use async_graphql::dynamic::{ObjectAccessor, ValueAccessor};

pub fn get_cascade_conditions(cascades: Option<ValueAccessor>) -> Option<ObjectAccessor> {
    if let Some(cascades) = cascades {
        Some(cascades.object().unwrap())
    } else {
        None
    }
}
