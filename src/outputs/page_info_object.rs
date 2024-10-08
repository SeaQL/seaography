use async_graphql::dynamic::{Field, FieldFuture, Object, TypeRef};
use async_graphql::Value;

use crate::BuilderContext;

/// used to hold pages pagination info
#[derive(Clone, Debug)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

/// The configuration structure for PageInfoObjectBuilder
pub struct PageInfoObjectConfig {
    /// type name
    pub type_name: String,
    /// name for 'hasPreviousPage' field
    pub has_previous_page: String,
    /// name for 'hasNextPage' field
    pub has_next_page: String,
    /// name for 'startCursor' field
    pub start_cursor: String,
    /// name for 'endCursor' field
    pub end_cursor: String,
}

impl std::default::Default for PageInfoObjectConfig {
    fn default() -> Self {
        PageInfoObjectConfig {
            type_name: "PageInfo".into(),
            has_previous_page: {
                if cfg!(feature = "snake-case-field") {
                    "has_previous_page"
                } else {
                    "hasPreviousPage"
                }
                .into()
            },
            has_next_page: {
                if cfg!(feature = "snake-case-field") {
                    "has_next_page"
                } else {
                    "hasNextPage"
                }
                .into()
            },
            start_cursor: {
                if cfg!(feature = "snake-case-field") {
                    "start_cursor"
                } else {
                    "startCursor"
                }
                .into()
            },
            end_cursor: {
                if cfg!(feature = "snake-case-field") {
                    "end_cursor"
                } else {
                    "endCursor"
                }
                .into()
            },
        }
    }
}

/// This builder produces the PageInfo object
/// that contains cursor pagination information
/// for a query
pub struct PageInfoObjectBuilder {
    pub context: &'static BuilderContext,
}

impl PageInfoObjectBuilder {
    /// used to get type name
    pub fn type_name(&self) -> String {
        self.context.page_info_object.type_name.clone()
    }

    /// used to get GraphQL object for PageInfo
    pub fn to_object(&self) -> Object {
        Object::new(&self.context.page_info_object.type_name)
            .field(Field::new(
                &self.context.page_info_object.has_previous_page,
                TypeRef::named_nn(TypeRef::BOOLEAN),
                |ctx| {
                    FieldFuture::new(async move {
                        let cursor_page_info = ctx.parent_value.try_downcast_ref::<PageInfo>()?;
                        Ok(Some(Value::from(cursor_page_info.has_previous_page)))
                    })
                },
            ))
            .field(Field::new(
                &self.context.page_info_object.has_next_page,
                TypeRef::named_nn(TypeRef::BOOLEAN),
                |ctx| {
                    FieldFuture::new(async move {
                        let cursor_page_info = ctx.parent_value.try_downcast_ref::<PageInfo>()?;
                        Ok(Some(Value::from(cursor_page_info.has_next_page)))
                    })
                },
            ))
            .field(Field::new(
                &self.context.page_info_object.start_cursor,
                TypeRef::named(TypeRef::STRING),
                |ctx| {
                    FieldFuture::new(async move {
                        let cursor_page_info = ctx.parent_value.try_downcast_ref::<PageInfo>()?;
                        let value = cursor_page_info
                            .start_cursor
                            .as_ref()
                            .map(|v| Value::from(v.as_str()))
                            .or_else(|| Some(Value::Null));
                        Ok(value)
                    })
                },
            ))
            .field(Field::new(
                &self.context.page_info_object.end_cursor,
                TypeRef::named(TypeRef::STRING),
                |ctx| {
                    FieldFuture::new(async move {
                        let cursor_page_info = ctx.parent_value.try_downcast_ref::<PageInfo>()?;
                        let value = cursor_page_info
                            .end_cursor
                            .as_ref()
                            .map(|v| Value::from(v.as_str()))
                            .or_else(|| Some(Value::Null));
                        Ok(value)
                    })
                },
            ))
    }
}
