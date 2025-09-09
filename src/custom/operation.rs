use crate::{
    converted_null_to_sea_orm_value, converted_value_to_sea_orm_value, pluralize_unique,
    sea_query_value_to_graphql_value, BuilderContext, Connection, EntityInputBuilder,
    EntityObjectBuilder, PaginationInput, PaginationInputBuilder, SeaResult, TypesMapHelper,
};
use async_graphql::{
    dynamic::{Field, FieldValue, ResolverContext, TypeRef, ValueAccessor},
    InputType, Upload,
};
use sea_orm::{EntityTrait, ModelTrait, TryIntoModel};

pub trait CustomOperation {
    fn to_fields() -> Vec<Field>;
}

pub trait GqlScalarValueType: Sized {
    fn gql_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        Self::gql_type_ref(ctx)
    }
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        Self::gql_type_ref(ctx)
    }

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self> {
        Self::parse_value(context, ctx.args.get(name))
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;

    fn to_graphql_value(self) -> Option<async_graphql::Value>;

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Self::to_graphql_value(value).map(FieldValue::value)
    }
}

pub trait GqlInputModelType: Sized + Send + Sync + 'static {
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self> {
        Self::parse_value(context, ctx.args.get(name))
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;
}

pub trait GqlOutputModelType: Sized + Send + Sync + 'static {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(value))
    }
}

pub trait GqlModelType: Sized + Send + Sync + 'static {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef;
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self> {
        Self::parse_value(context, Some(ctx.args.try_get(name)?))
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::owned_any(value))
    }
}

pub trait GqlModelOptionType: Sized + Send + Sync + 'static {
    fn gql_output_type_ref(ctx: &'static BuilderContext) -> TypeRef;
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self> {
        Self::parse_value(context, ctx.args.get(name))
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>>;
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

    fn to_graphql_value(self) -> Option<async_graphql::Value> {
        Some(
            sea_query_value_to_graphql_value(self.into(), false)
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

impl<M> GqlModelOptionType for Option<M>
where
    M: ModelTrait + Sync + 'static,
    <<M as ModelTrait>::Entity as EntityTrait>::ActiveModel: TryIntoModel<M>,
{
    fn gql_output_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_object_builder = EntityObjectBuilder { context };
        let type_name = entity_object_builder.type_name::<M::Entity>();
        TypeRef::named(type_name)
    }

    fn gql_input_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_input_builder = EntityInputBuilder { context };
        let type_name = entity_input_builder.insert_type_name::<M::Entity>();
        TypeRef::named(type_name)
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let entity_object_builder = EntityObjectBuilder { context };

        match value {
            Some(input) => Ok(Some(
                entity_object_builder.parse_object::<M>(&input.object()?)?,
            )),
            None => Ok(None),
        }
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        value.map(FieldValue::owned_any)
    }
}

impl<M> GqlModelOptionType for Vec<M>
where
    M: ModelTrait + Sync + 'static,
    <<M as ModelTrait>::Entity as EntityTrait>::ActiveModel: TryIntoModel<M>,
{
    fn gql_output_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_object_builder = EntityObjectBuilder { context };
        let type_name = entity_object_builder.type_name::<M::Entity>();
        TypeRef::named_nn_list_nn(type_name)
    }

    fn gql_input_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_input_builder = EntityInputBuilder { context };
        let type_name = entity_input_builder.insert_type_name::<M::Entity>();
        TypeRef::named_nn_list_nn(type_name)
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        let entity_object_builder = EntityObjectBuilder { context };

        value
            .ok_or_else(|| {
                let type_name = entity_object_builder.type_name::<M::Entity>();
                async_graphql::Error::new(format!("internal: field \"{}\" not found", type_name))
            })?
            .list()?
            .iter()
            .map(|item| entity_object_builder.parse_object::<M>(&item.object()?))
            .collect()
    }

    fn gql_field_value(values: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::list(
            values.into_iter().map(|value| FieldValue::owned_any(value)),
        ))
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

impl GqlInputModelType for PaginationInput {
    fn gql_input_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named(ctx.pagination_input.type_name.to_owned())
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        PaginationInputBuilder { context }.parse_object(value)
    }
}

impl GqlInputModelType for Upload {
    fn gql_input_type_ref(_context: &'static BuilderContext) -> TypeRef {
        TypeRef::named_nn("Upload")
    }

    fn parse_value(
        _context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        Ok(Upload::parse(value.map(|v| v.as_value()).cloned())?)
    }
}

impl<T> GqlInputModelType for Option<T>
where
    T: GqlInputModelType,
{
    fn gql_input_type_ref(context: &'static BuilderContext) -> TypeRef {
        match T::gql_input_type_ref(context) {
            TypeRef::NonNull(ty) => ty.as_ref().to_owned(),
            _ => unimplemented!("Cannot be used as optional input"),
        }
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        match value {
            Some(v) => Ok(Some(T::parse_value(context, Some(v))?)),
            None => Ok(None),
        }
    }
}

impl<T> GqlInputModelType for Vec<T>
where
    T: GqlInputModelType,
{
    fn gql_input_type_ref(context: &'static BuilderContext) -> TypeRef {
        TypeRef::List(T::gql_input_type_ref(context).into())
    }

    fn parse_value(
        context: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self> {
        match value {
            Some(value) => value
                .list()?
                .iter()
                .map(|v| T::parse_value(context, Some(v)))
                .collect(),
            None => Ok(Vec::new()),
        }
    }
}

impl<M> GqlOutputModelType for Option<M>
where
    M: GqlOutputModelType,
{
    fn gql_output_type_ref(context: &'static BuilderContext) -> TypeRef {
        match M::gql_output_type_ref(context) {
            TypeRef::NonNull(ty) => ty.as_ref().to_owned(),
            _ => unimplemented!("Cannot be used as optional output"),
        }
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        value.map(FieldValue::owned_any)
    }
}

impl<M> GqlOutputModelType for Vec<M>
where
    M: GqlOutputModelType,
{
    fn gql_output_type_ref(context: &'static BuilderContext) -> TypeRef {
        TypeRef::List(M::gql_output_type_ref(context).into())
    }

    fn gql_field_value(values: Self) -> Option<FieldValue<'static>> {
        Some(FieldValue::list(
            values.into_iter().map(|value| FieldValue::owned_any(value)),
        ))
    }
}

impl<M> GqlOutputModelType for Box<M>
where
    M: GqlOutputModelType + Clone,
{
    fn gql_output_type_ref(context: &'static BuilderContext) -> TypeRef {
        M::gql_output_type_ref(context)
    }

    fn gql_field_value(value: Self) -> Option<FieldValue<'static>> {
        M::gql_field_value(*value.clone())
    }
}
