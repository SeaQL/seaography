use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};

use crate::BuilderContext;

/// used to hold information about cursor pagination
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CursorInput {
    pub cursor: Option<String>,
    pub limit: u64,
}

/// The configuration structure for CursorInputBuilder
pub struct CursorInputConfig {
    /// name of the object
    pub type_name: String,
    /// name for 'cursor' field
    pub cursor: String,
    /// name for 'limit' field
    pub limit: String,
}

impl std::default::Default for CursorInputConfig {
    fn default() -> Self {
        Self {
            type_name: "CursorInput".into(),
            cursor: "cursor".into(),
            limit: "limit".into(),
        }
    }
}

/// This builder produces the cursor pagination options input object
pub struct CursorInputBuilder {
    pub context: &'static BuilderContext,
}

impl CursorInputBuilder {
    /// used to get type name
    pub fn type_name(&self) -> String {
        self.context.cursor_input.type_name.clone()
    }

    /// used to get cursor pagination options object
    pub fn input_object(&self) -> InputObject {
        InputObject::new(&self.context.cursor_input.type_name)
            .field(InputValue::new(
                &self.context.cursor_input.cursor,
                TypeRef::named(TypeRef::STRING),
            ))
            .field(InputValue::new(
                &self.context.cursor_input.limit,
                TypeRef::named_nn(TypeRef::INT),
            ))
    }

    /// used to parse query input to cursor pagination options struct
    pub fn parse_object(&self, object: &ObjectAccessor) -> CursorInput {
        let limit = object
            .get(&self.context.cursor_input.limit)
            .unwrap()
            .u64()
            .unwrap();

        let cursor = object.get(&self.context.cursor_input.cursor);
        let cursor: Option<String> = cursor.map(|cursor| cursor.string().unwrap().into());

        CursorInput { cursor, limit }
    }
}
