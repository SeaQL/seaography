use crate::{
    BuilderContext, Connection, ConnectionObjectBuilder, CustomInputType, CustomOutputType,
    EntityObjectBuilder, GqlScalarValueType, PageInfo, PageInfoObjectBuilder, PaginationInfo,
    PaginationInfoObjectBuilder, PaginationInput, PaginationInputBuilder, SeaResult,
    SeaographyError,
};
use async_graphql::{
    dynamic::{FieldValue, TypeRef, ValueAccessor},
    Upload,
};
#[cfg(feature = "with-chrono")]
use chrono::{DateTime, Utc};
#[cfg(feature = "with-uuid")]
use uuid::Uuid;

macro_rules! impl_primitive {
    ($name:ty, $method:tt) => {
        impl CustomOutputType for $name {
            fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
                <$name as GqlScalarValueType>::gql_output_type_ref(ctx)
            }

            fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
                Some(FieldValue::value(value))
            }
        }

        impl CustomInputType for $name {
            fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
                <$name as GqlScalarValueType>::gql_input_type_ref(ctx)
            }

            fn parse_value(
                _context: &'static BuilderContext,
                value: Option<ValueAccessor<'_>>,
            ) -> SeaResult<Self> {
                match value {
                    None => Err(SeaographyError::AsyncGraphQLError("Value expected".into())),
                    Some(v) => Ok((v.$method()? as $name).to_owned()),
                }
            }
        }
    };
}

impl_primitive!(bool, boolean);
impl_primitive!(u8, u64);
impl_primitive!(u16, u64);
impl_primitive!(u32, u64);
impl_primitive!(u64, u64);
impl_primitive!(i8, i64);
impl_primitive!(i16, i64);
impl_primitive!(i32, i64);
impl_primitive!(i64, i64);
impl_primitive!(f32, f32);
impl_primitive!(f64, f64);

impl CustomInputType for String {
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        <String as GqlScalarValueType>::gql_input_type_ref(ctx)
    }

    fn parse_value(
        _context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        match value {
            None => Err(SeaographyError::AsyncGraphQLError("Value expected".into())),
            Some(v) => Ok(v.string()?.to_owned()),
        }
    }
}

impl CustomOutputType for String {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        <String as GqlScalarValueType>::gql_output_type_ref(ctx)
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::value(value))
    }
}

#[cfg(feature = "with-uuid")]
impl CustomInputType for Uuid {
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        <Uuid as GqlScalarValueType>::gql_input_type_ref(ctx)
    }

    fn parse_value(
        _context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        use std::str::FromStr;
        match value {
            None => Err(SeaographyError::AsyncGraphQLError("Value expected".into())),
            Some(v) => {
                let s = v.string()?;
                Ok(Uuid::from_str(s).map_err(|e| SeaographyError::AsyncGraphQLError(e.into()))?)
            }
        }
    }
}

#[cfg(feature = "with-uuid")]
impl CustomOutputType for Uuid {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        <Uuid as GqlScalarValueType>::gql_output_type_ref(ctx)
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::value(value.to_string()))
    }
}

#[cfg(feature = "with-chrono")]
impl CustomInputType for DateTime<Utc> {
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        <DateTime<Utc> as GqlScalarValueType>::gql_input_type_ref(ctx)
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        <DateTime<Utc> as GqlScalarValueType>::parse_value(context, value)
    }
}

#[cfg(feature = "with-chrono")]
impl CustomOutputType for DateTime<Utc> {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        <DateTime<Utc> as GqlScalarValueType>::gql_output_type_ref(ctx)
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::value(async_graphql::Value::String(
            value.to_string(),
        )))
    }
}

impl CustomInputType for serde_json::Value {
    fn gql_input_type_ref(_ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("Json")
    }

    fn parse_value(
        _context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        match value {
            None => Err(SeaographyError::AsyncGraphQLError("Value expected".into())),
            Some(value) => Ok(value.deserialize()?),
        }
    }
}

impl CustomOutputType for serde_json::Value {
    fn gql_output_type_ref(_ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("Json")
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        // TODO check that this can't fail
        Some(FieldValue::value(
            async_graphql::Value::from_json(value).unwrap(),
        ))
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

impl<T> CustomOutputType for Option<T>
where
    T: CustomOutputType,
{
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        match T::gql_output_type_ref(ctx) {
            TypeRef::Named(n) => TypeRef::Named(n),
            TypeRef::List(t) => TypeRef::List(t),
            TypeRef::NonNull(t) => TypeRef::clone(&*t),
        }
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        match value {
            Some(value) => T::gql_field_value(value),
            None => None,
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
                    format!("Expected a list, got {:?}", value).into(),
                )),
            },
        }
    }
}

impl<T> CustomOutputType for Vec<T>
where
    T: CustomOutputType,
{
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::NonNull(Box::new(TypeRef::List(Box::new(T::gql_output_type_ref(
            ctx,
        )))))
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        let mut items: Vec<FieldValue<'static>> = Vec::new();
        // TODO: Figure out what the right behaviour is here in the case where
        // T::gql_field_value returns None. For now, we just skip such values.
        for v in value.into_iter() {
            if let Some(item) = T::gql_field_value(v) {
                items.push(item);
            }
        }
        Some(FieldValue::list(items))
    }
}

impl CustomInputType for Upload {
    fn gql_input_type_ref(_ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("Upload")
    }

    fn parse_value(
        _context: &'static BuilderContext,
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

impl CustomOutputType for PageInfo {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        let page_info_object_builder = PageInfoObjectBuilder { context: ctx };
        TypeRef::named_nn(page_info_object_builder.type_name())
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(value))
    }
}

impl CustomOutputType for PaginationInfo {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        let page_info_object_builder = PaginationInfoObjectBuilder { context: ctx };
        TypeRef::named_nn(page_info_object_builder.type_name())
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(value))
    }
}

impl<T> CustomOutputType for Connection<T>
where
    T: sea_orm::EntityTrait,
    <T as sea_orm::EntityTrait>::Model: Sync + CustomOutputType,
{
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        let entity_object_builder = EntityObjectBuilder { context: ctx };
        let object_name: String = entity_object_builder.type_name::<T>();

        TypeRef::named_nn(ConnectionObjectBuilder { context: ctx }.type_name(&object_name))
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(value))
    }
}
