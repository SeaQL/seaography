use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};

use crate::BuilderContext;

#[derive(Clone, Debug)]
pub struct CursorInput {
    pub cursor: Option<String>,
    pub limit: u64,
}

#[derive(Clone, Debug)]
pub struct CursorInputConfig {
    pub type_name: String,
    pub cursor: String,
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

#[derive(Clone, Debug)]
pub struct CursorInputBuilder {
    pub context: &'static BuilderContext,
}

impl CursorInputBuilder {
    pub fn type_name(&self) -> String {
        self.context.cursor_input.type_name.clone()
    }

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

    pub fn parse_object(&self, object: &ObjectAccessor) -> CursorInput {
        let limit = object
            .get(&self.context.cursor_input.limit)
            .unwrap()
            .u64()
            .unwrap();

        let cursor = object.get("cursor");
        let cursor: Option<String> = if let Some(cursor) = cursor {
            Some(cursor.string().unwrap().into())
        } else {
            None
        };

        CursorInput { cursor, limit }
    }
}
