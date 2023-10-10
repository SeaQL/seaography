use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor, TypeRef};

use crate::BuilderContext;

/// used to hold information about page pagination
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageInput {
    pub page: u64,
    pub limit: u64,
}

/// The configuration structure for PageInputBuilder
pub struct PageInputConfig {
    /// name of the object
    pub type_name: String,
    /// name for 'page' field
    pub page: String,
    /// name for 'limit' field
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

/// This builder produces the page pagination options input object
pub struct PageInputBuilder {
    pub context: &'static BuilderContext,
}

impl PageInputBuilder {
    /// used to get type name
    pub fn type_name(&self) -> String {
        self.context.page_input.type_name.clone()
    }

    /// used to get page pagination options object
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

    /// used to parse query input to page pagination options struct
    pub fn parse_object(&self, object: &ObjectAccessor) -> PageInput {
        let page = object
            .get(&self.context.page_input.page)
            .map_or_else(|| Ok(0), |v| v.u64())
            .unwrap_or(0);
        let limit = object
            .get(&self.context.page_input.limit)
            .unwrap()
            .u64()
            .unwrap();

        PageInput { page, limit }
    }
}
