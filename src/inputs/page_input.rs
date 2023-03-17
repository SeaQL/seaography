use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};

use crate::BuilderContext;

#[derive(Clone, Debug)]
pub struct PageInput {
    pub page: u64,
    pub limit: u64,
}

#[derive(Clone, Debug)]
pub struct PageInputConfig {
    pub type_name: String,
    pub page: String,
    pub limit: String,
}

impl std::default::Default for PageInputConfig {
    fn default() -> Self {
        PageInputConfig {
            type_name: "PageInput".into(),
            page: "page".into(),
            limit: "limit".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PageInputBuilder {
    pub context: &'static BuilderContext,
}

impl PageInputBuilder {
    pub fn type_name(&self) -> String {
        self.context.page_input.type_name.clone()
    }

    pub fn input_object(&self) -> InputObject {
        InputObject::new(&self.context.page_input.type_name)
            .field(InputValue::new(
                &self.context.page_input.limit,
                TypeRef::named_nn(TypeRef::INT),
            ))
            .field(InputValue::new(
                &self.context.page_input.page,
                TypeRef::named_nn(TypeRef::INT),
            ))
    }

    pub fn parse_object(&self, object: &ObjectAccessor) -> PageInput {
        let page = object
            .get(&self.context.page_input.page)
            .map_or_else(|| Ok(0), |v| v.u64())
            .unwrap();
        let limit = object
            .get(&self.context.page_input.limit)
            .unwrap()
            .u64()
            .unwrap();

        PageInput { page, limit }
    }
}
