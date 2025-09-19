use crate::BuilderContext;
use async_graphql::dynamic::{FieldValue, Object, TypeRef};
#[cfg(feature = "macros")]
pub use seaography_macros::CustomOutputType;

pub trait CustomOutputType {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef;
    fn gql_field_value(self, ctx: &'static BuilderContext) -> Option<FieldValue<'static>>;
}

pub trait CustomOutputObject {
    fn basic_object(context: &'static BuilderContext) -> Object;
}

pub trait ConvertOutput {
    fn convert_output(
        value: &sea_orm::sea_query::Value,
    ) -> async_graphql::Result<Option<FieldValue<'static>>>;
}
