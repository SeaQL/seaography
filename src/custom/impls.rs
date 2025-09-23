use crate::{
    BuilderContext, CustomInputType, CustomOutputType, GqlScalarValueType, SeaResult,
    SeaographyError,
};
use async_graphql::dynamic::{FieldValue, TypeRef, ValueAccessor};

macro_rules! impl_primitive {
    ($name:ty, $method:tt) => {
        impl CustomOutputType for $name {
            fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
                <$name as GqlScalarValueType>::gql_output_type_ref(ctx)
            }

            fn gql_field_value(self, _ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
                Some(FieldValue::value(self))
            }
        }

        impl CustomInputType for $name {
            fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
                <$name as GqlScalarValueType>::gql_input_type_ref(ctx)
            }

            fn parse_value(
                _ctx: &'static BuilderContext,
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

macro_rules! impl_scalar_type {
    ($type:ty) => {
        impl CustomInputType for $type {
            fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
                <$type as GqlScalarValueType>::gql_input_type_ref(ctx)
            }

            fn parse_value(
                context: &'static BuilderContext,
                value: Option<ValueAccessor<'_>>,
            ) -> SeaResult<Self> {
                <$type as GqlScalarValueType>::parse_value(context, value)
            }
        }

        impl CustomOutputType for $type {
            fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
                <$type as GqlScalarValueType>::gql_output_type_ref(ctx)
            }

            fn gql_field_value(self, ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
                <$type as GqlScalarValueType>::gql_field_value(self, ctx)
            }
        }
    };
}

impl_scalar_type!(String);

#[cfg(feature = "with-chrono")]
impl_scalar_type!(sea_orm::entity::prelude::ChronoDate);

#[cfg(feature = "with-chrono")]
impl_scalar_type!(sea_orm::entity::prelude::ChronoTime);

#[cfg(feature = "with-chrono")]
impl_scalar_type!(sea_orm::entity::prelude::ChronoDateTime);

#[cfg(feature = "with-chrono")]
impl_scalar_type!(sea_orm::entity::prelude::ChronoDateTimeWithTimeZone);

#[cfg(feature = "with-chrono")]
impl_scalar_type!(sea_orm::entity::prelude::ChronoDateTimeUtc);

#[cfg(feature = "with-chrono")]
impl_scalar_type!(sea_orm::entity::prelude::ChronoDateTimeLocal);

#[cfg(feature = "with-time")]
impl_scalar_type!(sea_orm::entity::prelude::TimeDate);

#[cfg(feature = "with-time")]
impl_scalar_type!(sea_orm::entity::prelude::TimeTime);

#[cfg(feature = "with-time")]
impl_scalar_type!(sea_orm::entity::prelude::TimeDateTime);

#[cfg(feature = "with-time")]
impl_scalar_type!(sea_orm::entity::prelude::TimeDateTimeWithTimeZone);

#[cfg(feature = "with-decimal")]
impl_scalar_type!(sea_orm::entity::prelude::Decimal);

#[cfg(feature = "with-bigdecimal")]
impl_scalar_type!(sea_orm::entity::prelude::BigDecimal);

#[cfg(feature = "with-uuid")]
impl_scalar_type!(sea_orm::entity::prelude::Uuid);

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

    fn gql_field_value(self, _ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        // TODO check that this can't fail
        Some(FieldValue::value(
            async_graphql::Value::from_json(self).unwrap(),
        ))
    }
}
