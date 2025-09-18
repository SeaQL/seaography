use crate::{BuilderContext, SeaResult};
use async_graphql::dynamic::{InputObject, TypeRef, ValueAccessor};
#[cfg(feature = "macros")]
pub use seaography_macros::CustomInputType;

pub trait CustomInputType: Sized {
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;
}

pub trait CustomInputObject {
    fn input_object(context: &'static BuilderContext) -> InputObject;
}
