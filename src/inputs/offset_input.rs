use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};

use crate::BuilderContext;

#[derive(Clone, Debug)]
pub struct OffsetInput {
    pub offset: u64,
    pub limit: u64,
}

pub struct OffsetInputConfig {
    pub type_name: String,
    pub offset: String,
    pub limit: String,
}

impl std::default::Default for OffsetInputConfig {
    fn default() -> Self {
        Self {
            type_name: "OffsetInput".into(),
            offset: "offset".into(),
            limit: "limit".into(),
        }
    }
}

pub struct OffsetInputBuilder {
    pub context: &'static BuilderContext,
}

impl OffsetInputBuilder {
    pub fn type_name(&self) -> String {
        self.context.offset_input.type_name.clone()
    }

    pub fn input_object(&self) -> InputObject {
        InputObject::new(&self.context.offset_input.type_name)
            .field(InputValue::new(
                &self.context.offset_input.limit,
                TypeRef::named_nn(TypeRef::INT),
            ))
            .field(InputValue::new(
                &self.context.offset_input.offset,
                TypeRef::named_nn(TypeRef::INT),
            ))
    }

    pub fn parse_object(&self, object: &ObjectAccessor) -> OffsetInput {
        let offset = object
            .get(&self.context.offset_input.offset)
            .map_or_else(|| Ok(0), |v| v.u64())
            .unwrap();

        let limit = object
            .get(&self.context.offset_input.limit)
            .unwrap()
            .u64()
            .unwrap();

        OffsetInput { offset, limit }
    }
}
