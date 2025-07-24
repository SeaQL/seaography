use crate::{
    converted_value_to_sea_orm_value, BuilderContext, Connection, EntityObjectBuilder,
    PaginationInput, PaginationInputBuilder, SeaResult, TypesMapHelper,
};
use async_graphql::dynamic::{FieldValue, ResolverContext, TypeRef};
use sea_orm::{EntityTrait, ModelTrait};

pub trait GqlScalarValueType: Sized {
    fn gql_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self>;

    fn gql_field_value(value: Self) -> FieldValue<'static>
    where
        async_graphql::Value: From<Self>,
    {
        FieldValue::value(value)
    }
}

pub trait GqlModelType: Sized + Send + Sync + 'static {
    fn gql_type_ref(ctx: &'static BuilderContext) -> TypeRef;

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self>;

    fn gql_field_value(value: Self) -> FieldValue<'static> {
        FieldValue::owned_any(value)
    }
}

impl<T> GqlScalarValueType for T
where
    T: sea_orm::sea_query::ValueType,
{
    fn gql_type_ref(context: &'static BuilderContext) -> TypeRef {
        let ty = T::column_type();
        let not_null = true;
        let enum_type_name = T::enum_type_name();
        let types_map_helper = TypesMapHelper { context };
        match types_map_helper.sea_orm_column_type_to_graphql_type(&ty, not_null, enum_type_name) {
            Some(type_ref) => type_ref,
            None => unreachable!("{} is not handled", T::type_name()),
        }
    }

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<T> {
        let ty = T::column_type();
        let types_map_helper = TypesMapHelper { context };
        let column_type = types_map_helper.get_column_type_helper("", "", &ty);
        let value = ctx.args.try_get(name)?;
        let val = converted_value_to_sea_orm_value(&column_type, &value, "", "")?;
        Ok(val.unwrap())
    }
}

impl<M> GqlModelType for M
where
    M: ModelTrait + Sync + 'static,
    <<M as ModelTrait>::Entity as EntityTrait>::Model: Sync,
{
    fn gql_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_object_builder = EntityObjectBuilder { context };
        let type_name = entity_object_builder.type_name::<M::Entity>();
        TypeRef::named_nn(type_name)
    }

    fn try_get_arg(
        context: &'static BuilderContext,
        _ctx: &ResolverContext<'_>,
        _name: &str,
    ) -> SeaResult<Self> {
        let entity_object_builder = EntityObjectBuilder { context };
        let object_name = entity_object_builder.type_name::<M::Entity>();
        todo!("Not supporting complex type {object_name}")
    }
}

impl<E> GqlModelType for Connection<E>
where
    E: EntityTrait,
    E::Model: Send + Sync,
{
    fn gql_type_ref(context: &'static BuilderContext) -> TypeRef {
        let entity_object_builder = EntityObjectBuilder { context };
        let entity_name = entity_object_builder.type_name::<E>();
        let type_name = context.connection_object.type_name.as_ref()(&entity_name);
        TypeRef::named_nn(type_name)
    }

    fn try_get_arg(
        context: &'static BuilderContext,
        _ctx: &ResolverContext<'_>,
        _name: &str,
    ) -> SeaResult<Self> {
        let entity_object_builder = EntityObjectBuilder { context };
        let object_name = entity_object_builder.type_name::<E>();
        todo!("Not supporting complex type {object_name}")
    }
}

impl GqlModelType for PaginationInput {
    fn gql_type_ref(ctx: &'static BuilderContext) -> TypeRef {
        TypeRef::named(ctx.pagination_input.type_name.to_owned())
    }

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self> {
        let pagination = ctx.args.get(name);
        Ok(PaginationInputBuilder { context }.parse_object(pagination))
    }
}
