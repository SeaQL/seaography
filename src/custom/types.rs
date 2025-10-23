use crate::{
    converted_null_to_sea_orm_value, converted_value_to_sea_orm_value, pluralize_unique,
    sea_query_value_to_graphql_value, BuilderContext, Connection, EntityInputBuilder,
    EntityObjectBuilder, SeaResult, TypesMapHelper,
};
use async_graphql::dynamic::{Enum, Field, FieldValue, TypeRef, Union, ValueAccessor};
use sea_orm::{EntityTrait, ModelTrait, TryIntoModel};
#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub use seaography_macros::{CustomEnum, CustomFields};

/// ```
/// use seaography::{async_graphql, CustomFields, CustomInputType};
/// use async_graphql::Context;
///
/// pub struct Operations;
///
/// #[CustomFields]
/// impl Operations {
///     async fn foo(_ctx: &Context<'_>, username: String) -> async_graphql::Result<String> {
///         Ok(format!("Hello, {}!", username))
///     }
///
///     async fn bar(_ctx: &Context<'_>, x: i32, y: i32) -> async_graphql::Result<i32> {
///         Ok(x + y)
///     }
/// }
///
/// #[derive(Clone, CustomInputType)]
/// pub struct Circle {
///     pub center: Point,
///     pub radius: f64,
/// }
///
/// #[derive(Clone, Copy, CustomInputType)]
/// pub struct Point {
///     pub x: f64,
///     pub y: f64,
/// }
///
/// #[CustomFields]
/// impl Circle {
///     pub async fn area(&self) -> async_graphql::Result<f64> {
///         Ok(std::f64::consts::PI * self.radius * self.radius)
///     }
/// }
/// ```
pub trait CustomFields {
    fn to_fields(context: &'static BuilderContext) -> Vec<Field>;
}

/// ```
/// use seaography::CustomEnum;
///
/// #[derive(CustomEnum)]
/// pub enum Status {
///     Available,
///     BackOrdering,
///     Unavailable,
/// }
/// ```
pub trait CustomEnum {
    fn to_enum() -> Enum;
}

pub trait CustomUnion {
    fn to_union() -> Union;
}

pub trait GqlScalarValueType: Sized {
    fn gql_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        Self::gql_type_ref(ctx)
    }
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        Self::gql_type_ref(ctx)
    }

    fn parse_value(
        ctx: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;

    fn to_graphql_value(self, ctx: &'static BuilderContext) -> Option<async_graphql::Value>;

    fn gql_field_value(self, ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        Self::to_graphql_value(self, ctx).map(FieldValue::value)
    }
}

pub trait GqlModelType: Sized + Send + Sync + 'static {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef;
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn parse_value(
        ctx: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;

    fn gql_field_value(self, _ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(self))
    }
}

impl<T> GqlScalarValueType for T
where
    T: sea_orm::sea_query::ValueType + Into<sea_orm::Value>,
{
    fn gql_type_ref(context: &'static BuilderContext) -> TypeRef {
        let ty = T::column_type();
        let not_null = !T::is_option();
        let enum_type_name = T::enum_type_name();
        let types_map_helper = TypesMapHelper { context };
        match types_map_helper.sea_orm_column_type_to_graphql_type(&ty, not_null, enum_type_name) {
            Some(type_ref) => type_ref,
            None => unreachable!("{} is not handled", T::type_name()),
        }
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let types_map_helper = TypesMapHelper { context };
        let column_type = T::column_type();
        let column_type =
            types_map_helper.sea_orm_column_type_to_converted_type(None, &column_type);

        if value.is_none() {
            // this is Value::unwrap and should not panic
            Ok(converted_null_to_sea_orm_value(&column_type)?.unwrap())
        } else {
            let value = converted_value_to_sea_orm_value(
                &column_type,
                &value.expect("Checked not null"),
                "",
                "",
            )?;
            // this is Value::unwrap and should not panic
            Ok(value.unwrap())
        }
    }

    fn to_graphql_value(self, ctx: &'static BuilderContext) -> Option<async_graphql::Value> {
        Some(
            sea_query_value_to_graphql_value(ctx, self.into(), false)
                .unwrap_or(async_graphql::Value::Null),
        )
    }
}

impl<M> GqlModelType for M
where
    M: ModelTrait + Sync + 'static,
    <<M as ModelTrait>::Entity as EntityTrait>::ActiveModel: TryIntoModel<M>,
{
    fn gql_output_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_object_builder = EntityObjectBuilder { context };
        let type_name = entity_object_builder.type_name::<M::Entity>();
        TypeRef::named_nn(type_name)
    }

    fn gql_input_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_input_builder = EntityInputBuilder { context };
        let type_name = entity_input_builder.insert_type_name::<M::Entity>();
        TypeRef::named_nn(type_name)
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let entity_object_builder = EntityObjectBuilder { context };

        entity_object_builder.parse_object::<M>(&value.expect("Checked not null").object()?)
    }
}

impl<E> GqlModelType for Connection<E>
where
    E: EntityTrait,
    E::Model: Send + Sync,
{
    fn gql_output_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_object_builder = EntityObjectBuilder { context };
        let entity_name = pluralize_unique(&entity_object_builder.type_name::<E>(), true);
        let type_name = context.connection_object.type_name.as_ref()(&entity_name);
        TypeRef::named_nn(type_name)
    }

    fn gql_input_type_ref(_: &'static BuilderContext) -> TypeRef {
        todo!()
    }

    fn parse_value(
        context: &'static BuilderContext,
        _value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let entity_object_builder = EntityObjectBuilder { context };
        let object_name = entity_object_builder.type_name::<E>();
        todo!("Not supporting complex type {object_name}")
    }
}
