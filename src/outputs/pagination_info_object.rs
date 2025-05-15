use async_graphql::Value;
use async_graphql::dynamic::{Field, FieldFuture, Object, TypeRef};

use crate::BuilderContext;

/// used to hold offset pagination info
#[derive(Clone, Debug)]
pub struct PaginationInfo {
    pub pages: u64,
    pub current: u64,
    pub offset: u64,
    pub total: u64,
}

/// The configuration structure for PaginationInfoObjectBuilder
pub struct PaginationInfoObjectConfig {
    /// type name
    pub type_name: String,
    /// name for 'pages' field
    pub pages: String,
    /// name for 'current' field
    pub current: String,
    /// name for 'offset' field
    pub offset: String,
    /// name for 'total' field
    pub total: String,
}

impl std::default::Default for PaginationInfoObjectConfig {
    fn default() -> Self {
        PaginationInfoObjectConfig {
            type_name: "PaginationInfo".into(),
            pages: "pages".into(),
            current: "current".into(),
            offset: "offset".into(),
            total: "total".into(),
        }
    }
}

/// This builder produces the PaginationInfo object
/// that contains page/offset pagination information
/// for a query
pub struct PaginationInfoObjectBuilder {
    pub context: &'static BuilderContext,
}

impl PaginationInfoObjectBuilder {
    /// used to get type name
    pub fn type_name(&self) -> String {
        self.context.pagination_info_object.type_name.clone()
    }

    /// used to get GraphQL object for PaginationInfo
    pub fn to_object(&self) -> Object {
        Object::new(&self.context.pagination_info_object.type_name)
            .field(Field::new(
                &self.context.pagination_info_object.pages,
                TypeRef::named_nn(TypeRef::INT),
                |ctx| {
                    FieldFuture::new(async move {
                        let pagination_page_info =
                            ctx.parent_value.try_downcast_ref::<PaginationInfo>()?;
                        Ok(Some(Value::from(pagination_page_info.pages)))
                    })
                },
            ))
            .field(Field::new(
                &self.context.pagination_info_object.current,
                TypeRef::named_nn(TypeRef::INT),
                |ctx| {
                    FieldFuture::new(async move {
                        let pagination_page_info =
                            ctx.parent_value.try_downcast_ref::<PaginationInfo>()?;
                        Ok(Some(Value::from(pagination_page_info.current)))
                    })
                },
            ))
            .field(Field::new(
                &self.context.pagination_info_object.offset,
                TypeRef::named_nn(TypeRef::INT),
                |ctx| {
                    FieldFuture::new(async move {
                        let pagination_page_info =
                            ctx.parent_value.try_downcast_ref::<PaginationInfo>()?;
                        Ok(Some(Value::from(pagination_page_info.offset)))
                    })
                },
            ))
            .field(Field::new(
                &self.context.pagination_info_object.total,
                TypeRef::named_nn(TypeRef::INT),
                |ctx| {
                    FieldFuture::new(async move {
                        let pagination_page_info =
                            ctx.parent_value.try_downcast_ref::<PaginationInfo>()?;
                        Ok(Some(Value::from(pagination_page_info.total)))
                    })
                },
            ))
    }
}
