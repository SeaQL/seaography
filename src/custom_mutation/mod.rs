use sea_orm::ModelTrait;
use crate::{EntityObjectBuilder,TypesMapHelper, converted_value_to_sea_orm_value, SeaResult, BuilderContext};

pub trait AsyncGqlValueType {
    type Data;

    fn gql_type_ref(ctx: &'static BuilderContext) -> async_graphql::dynamic::TypeRef;

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &async_graphql::dynamic::ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self::Data>;
}

impl<T> AsyncGqlValueType for T
where
    T: sea_orm::sea_query::ValueType,
{
    type Data = T;

    fn gql_type_ref(ctx: &'static BuilderContext) -> async_graphql::dynamic::TypeRef {
        let ty = T::column_type();
        let not_null = true;
        let enum_type_name = T::enum_type_name();
        let types_map_helper = TypesMapHelper { context: ctx };
        match types_map_helper.sea_orm_column_type_to_graphql_type(&ty, not_null, enum_type_name) {
            Some(type_ref) => type_ref,
            None => unreachable!("{} is not handled", T::type_name()),
        }
    }

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &async_graphql::dynamic::ResolverContext<'_>,
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

/*
impl<M> AsyncGqlValueType for M
where
    M: sea_orm::ModelTrait,
    <<M as ModelTrait>::Entity as sea_orm::EntityTrait>::Model: Sync,
{
    fn gql_type_ref(ctx: &'static BuilderContext) -> async_graphql::dynamic::TypeRef {
        let entity_object_builder = EntityObjectBuilder { context: ctx };
        let type_name = entity_object_builder.type_name::<M::Entity>();
        async_graphql::dynamic::TypeRef::named_nn(type_name)
    }
}
*/

pub struct SeaOrmModel<M>(pub M);

impl<M> AsyncGqlValueType for SeaOrmModel<M>
where
    M: sea_orm::ModelTrait,
    <<M as ModelTrait>::Entity as sea_orm::EntityTrait>::Model: Sync,
{
    type Data = M;

    fn gql_type_ref(ctx: &'static BuilderContext) -> async_graphql::dynamic::TypeRef {
        let entity_object_builder = EntityObjectBuilder { context: ctx };
        let type_name = entity_object_builder.type_name::<M::Entity>();
        async_graphql::dynamic::TypeRef::named_nn(type_name)
    }

    fn try_get_arg(
        context: &'static BuilderContext,
        ctx: &async_graphql::dynamic::ResolverContext<'_>,
        name: &str,
    ) -> SeaResult<Self::Data> {
        todo!()
    }
}
