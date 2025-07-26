use async_graphql::dynamic::{Field, FieldFuture, InputValue, TypeRef};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel, QueryFilter,
};

use crate::{
    apply_guard, get_filter_conditions, guard_error, BuilderContext, EntityObjectBuilder,
    EntityQueryFieldBuilder, FilterInputBuilder, GuardAction, OperationType,
};

/// The configuration structure of EntityDeleteMutationBuilder
pub struct EntityDeleteMutationConfig {
    /// suffix that is appended on delete mutations
    pub mutation_suffix: String,

    /// name for `filter` field
    pub filter_field: String,
}

impl std::default::Default for EntityDeleteMutationConfig {
    fn default() -> Self {
        Self {
            mutation_suffix: {
                if cfg!(feature = "field-snake-case") {
                    "_delete"
                } else {
                    "Delete"
                }
                .into()
            },
            filter_field: "filter".into(),
        }
    }
}

/// This builder produces the delete mutation for an entity
pub struct EntityDeleteMutationBuilder {
    pub context: &'static BuilderContext,
}

impl EntityDeleteMutationBuilder {
    /// used to get mutation name for a SeaORM entity
    pub fn type_name<T>(&self) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_query_field_builder = EntityQueryFieldBuilder {
            context: self.context,
        };
        format!(
            "{}{}",
            entity_query_field_builder.type_name::<T>(),
            self.context.entity_delete_mutation.mutation_suffix
        )
    }

    /// used to get the delete mutation field for a SeaORM entity
    pub fn to_field<T, A>(&self) -> Field
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <T as EntityTrait>::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
    {
        let entity_filter_input_builder = FilterInputBuilder {
            context: self.context,
        };
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let object_name: String = entity_object_builder.type_name::<T>();
        let object_name_ = object_name.clone();

        let context = self.context;

        let guard = self.context.guards.entity_guards.get(&object_name);
        let hooks = &self.context.hooks;

        Field::new(
            self.type_name::<T>(),
            TypeRef::named_nn(TypeRef::INT),
            move |ctx| {
                let object_name = object_name.clone();
                FieldFuture::new(async move {
                    if let GuardAction::Block(reason) = apply_guard(&ctx, guard) {
                        return Err(guard_error(reason, "Entity guard triggered."));
                    }
                    if let GuardAction::Block(reason) =
                        hooks.entity_guard(&ctx, &object_name, OperationType::Delete)
                    {
                        return Err(guard_error(reason, "Entity guard triggered."));
                    }

                    let db = ctx.data::<DatabaseConnection>()?;

                    let filters = ctx.args.get(&context.entity_delete_mutation.filter_field);
                    let filter_condition = get_filter_conditions::<T>(context, filters);

                    let res: DeleteResult =
                        T::delete_many().filter(filter_condition).exec(db).await?;

                    Ok(Some(async_graphql::Value::from(res.rows_affected)))
                })
            },
        )
        .argument(InputValue::new(
            &context.entity_delete_mutation.filter_field,
            TypeRef::named(entity_filter_input_builder.type_name(&object_name_)),
        ))
    }
}
