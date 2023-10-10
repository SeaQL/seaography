use async_graphql::dynamic::{InputObject, InputValue, TypeRef, ValueAccessor};

use crate::{BuilderContext, CursorInputBuilder, OffsetInputBuilder, PageInputBuilder};

use super::{CursorInput, OffsetInput, PageInput};

/// used to hold information about which pagination
/// strategy will be applied on the query
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaginationInput {
    pub cursor: Option<CursorInput>,
    pub page: Option<PageInput>,
    pub offset: Option<OffsetInput>,
}

/// The configuration structure for PaginationInputBuilder
pub struct PaginationInputConfig {
    /// name of the object
    pub type_name: String,
    /// name for 'cursor' field
    pub cursor: String,
    /// name for 'page' field
    pub page: String,
    /// name for 'offset' field
    pub offset: String,
}

impl std::default::Default for PaginationInputConfig {
    fn default() -> Self {
        PaginationInputConfig {
            type_name: "PaginationInput".into(),
            cursor: "cursor".into(),
            page: "page".into(),
            offset: "offset".into(),
        }
    }
}

pub struct PaginationInputBuilder {
    pub context: &'static BuilderContext,
}

impl PaginationInputBuilder {
    /// used to get type name
    pub fn type_name(&self) -> String {
        self.context.pagination_input.type_name.clone()
    }

    /// used to get pagination input object
    pub fn input_object(&self) -> InputObject {
        InputObject::new(&self.context.pagination_input.type_name)
            .field(InputValue::new(
                &self.context.pagination_input.cursor,
                TypeRef::named(&self.context.cursor_input.type_name),
            ))
            .field(InputValue::new(
                &self.context.pagination_input.page,
                TypeRef::named(&self.context.page_input.type_name),
            ))
            .field(InputValue::new(
                &self.context.pagination_input.offset,
                TypeRef::named(&self.context.offset_input.type_name),
            ))
            .oneof()
    }

    /// used to parse query input to pagination information structure
    pub fn parse_object(&self, value: Option<ValueAccessor<'_>>) -> PaginationInput {
        if value.is_none() {
            return PaginationInput {
                cursor: None,
                offset: None,
                page: None,
            };
        }

        let binding = value.unwrap();
        let object = binding.object().unwrap();

        let cursor_input_builder = CursorInputBuilder {
            context: self.context,
        };
        let page_input_builder = PageInputBuilder {
            context: self.context,
        };
        let offset_input_builder = OffsetInputBuilder {
            context: self.context,
        };

        let cursor = if let Some(cursor) = object.get(&self.context.pagination_input.cursor) {
            let object = cursor.object().unwrap();
            Some(cursor_input_builder.parse_object(&object))
        } else {
            None
        };

        let page = if let Some(page) = object.get(&self.context.pagination_input.page) {
            let object = page.object().unwrap();
            Some(page_input_builder.parse_object(&object))
        } else {
            None
        };

        let offset = if let Some(offset) = object.get(&self.context.pagination_input.offset) {
            let object = offset.object().unwrap();
            Some(offset_input_builder.parse_object(&object))
        } else {
            None
        };

        PaginationInput {
            cursor,
            page,
            offset,
        }
    }
}
