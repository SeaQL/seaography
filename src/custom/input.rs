use crate::{BuilderContext, PaginationInput, PaginationInputBuilder, SeaResult, SeaographyError};
use async_graphql::{
    dynamic::{InputObject, TypeRef, ValueAccessor},
    Upload,
};
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

impl CustomInputType for Upload {
    fn gql_input_type_ref(_ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("Upload")
    }

    fn parse_value(
        _ctx: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        Ok(<Upload as async_graphql::InputType>::parse(
            value.map(|v| v.as_value()).cloned(),
        )?)
    }
}

impl CustomInputType for PaginationInput {
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named(PaginationInputBuilder { context: ctx }.type_name())
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        PaginationInputBuilder { context }.parse_object(value)
    }
}

impl<T> CustomInputType for Option<T>
where
    T: CustomInputType,
{
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        match T::gql_input_type_ref(ctx) {
            TypeRef::Named(n) => TypeRef::Named(n),
            TypeRef::List(t) => TypeRef::List(t),
            TypeRef::NonNull(t) => TypeRef::clone(&*t),
        }
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        match value {
            None => Ok(None),
            Some(v) => match v.as_value() {
                async_graphql::Value::Null => Ok(None),
                _ => Ok(Some(T::parse_value(context, Some(v))?)),
            },
        }
    }
}

impl<T> CustomInputType for Vec<T>
where
    T: CustomInputType,
{
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::NonNull(Box::new(TypeRef::List(Box::new(T::gql_input_type_ref(
            ctx,
        )))))
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        match value {
            None => Err(SeaographyError::AsyncGraphQLError(
                "Expected a list, got missing value".into(),
            )),
            Some(v) => match v.as_value() {
                async_graphql::Value::List(_) => {
                    let list = v.list()?;
                    let mut res: Vec<T> = Vec::new();
                    for item in list.iter() {
                        res.push(T::parse_value(context, Some(item))?);
                    }
                    Ok(res)
                }
                value => Err(SeaographyError::AsyncGraphQLError(
                    format!("Expected a list, got {value:?}").into(),
                )),
            },
        }
    }
}
