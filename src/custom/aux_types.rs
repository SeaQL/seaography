use crate::{BuilderContext, EntityInputBuilder, EntityObjectBuilder, SeaResult};
use async_graphql::dynamic::{FieldValue, ResolverContext, TypeRef, ValueAccessor};
use sea_orm::{EntityTrait, ModelTrait, TryIntoModel};

pub trait GqlModelHolderType: Sized + Send + Sync + 'static {
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
        ctx: &'static BuilderContext,
        value: Option<ValueAccessor<'_>>,
    ) -> SeaResult<Self>;

    fn gql_field_value(self, ctx: &'static BuilderContext) -> Option<FieldValue<'static>>;
}

impl<M> GqlModelHolderType for Option<M>
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

    fn gql_field_value(self, _ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        self.map(FieldValue::owned_any)
    }
}

impl<M> GqlModelHolderType for Vec<M>
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
                async_graphql::Error::new(format!("internal: field \"{type_name}\" not found"))
            })?
            .list()?
            .iter()
            .map(|item| entity_object_builder.parse_object::<M>(&item.object()?))
            .collect()
    }

    fn gql_field_value(self, _ctx: &'static BuilderContext) -> Option<FieldValue<'static>> {
        Some(FieldValue::list(
            self.into_iter().map(|value| FieldValue::owned_any(value)),
        ))
    }
}
