use async_graphql::dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, QueryTrait, TransactionTrait,
};

use crate::{
    get_filter_conditions, guard_error, prepare_active_model, BuilderContext, DatabaseContext,
    EntityInputBuilder, EntityObjectBuilder, EntityQueryFieldBuilder, FilterInputBuilder,
    GuardAction, OperationType, UserContext,
};

/// The configuration structure of EntityUpdateMutationBuilder
pub struct EntityUpdateMutationConfig {
    /// suffix that is appended on update mutations
    pub mutation_suffix: String,

    /// name for `data` field
    pub data_field: String,

    /// name for `filter` field
    pub filter_field: String,
}

impl std::default::Default for EntityUpdateMutationConfig {
    fn default() -> Self {
        Self {
            mutation_suffix: {
                if cfg!(feature = "field-snake-case") {
                    "_update"
                } else {
                    "Update"
                }
                .into()
            },
            data_field: "data".into(),
            filter_field: "filter".into(),
        }
    }
}

/// This builder produces the update mutation for an entity
pub struct EntityUpdateMutationBuilder {
    pub context: &'static BuilderContext,
}

impl EntityUpdateMutationBuilder {
    /// used to get mutation name for a SeaORM entity
    pub fn type_name<T>(&self) -> String
    where
        T: EntityTrait,
    {
        let entity_query_field_builder = EntityQueryFieldBuilder {
            context: self.context,
        };
        format!(
            "{}{}",
            entity_query_field_builder.type_name::<T>(),
            self.context.entity_update_mutation.mutation_suffix
        )
    }

    /// used to get the update mutation field for a SeaORM entity
    pub fn to_field<T, A>(&self) -> Field
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <T as EntityTrait>::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + Send,
    {
        let entity_input_builder = EntityInputBuilder {
            context: self.context,
        };
        let entity_filter_input_builder = FilterInputBuilder {
            context: self.context,
        };
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let object_name: String = entity_object_builder.type_name::<T>();
        let object_name_ = object_name.clone();

        let context = self.context;
        let hooks = &self.context.hooks;

        Field::new(
            self.type_name::<T>(),
            TypeRef::named_nn_list_nn(entity_object_builder.basic_type_name::<T>()),
            move |ctx| {
                let object_name = object_name.clone();
                FieldFuture::new(async move {
                    if let GuardAction::Block(reason) =
                        hooks.entity_guard(&ctx, &object_name, OperationType::Update)
                    {
                        return Err(guard_error(reason, "Entity guard triggered."));
                    }

                    let db = ctx
                        .data::<DatabaseConnection>()?
                        .restricted(ctx.data_opt::<UserContext>())?;

                    let transaction = db.begin().await?;

                    let entity_input_builder = EntityInputBuilder { context };
                    let entity_object_builder = EntityObjectBuilder { context };

                    let filters = ctx.args.get(&context.entity_update_mutation.filter_field);
                    let filter_condition = get_filter_conditions::<T>(context, filters)?;

                    let value_accessor = ctx
                        .args
                        .try_get(&context.entity_update_mutation.data_field)?;
                    let input_object = &value_accessor.object()?;

                    for (column, _) in input_object.iter() {
                        if let GuardAction::Block(reason) =
                            hooks.field_guard(&ctx, &object_name, column, OperationType::Update)
                        {
                            return Err(guard_error(reason, "Field guard triggered."));
                        }
                    }

                    let active_model = prepare_active_model::<T, A>(
                        &entity_input_builder,
                        &entity_object_builder,
                        input_object,
                    )?;

                    let entity_filter =
                        hooks.entity_filter(&ctx, &object_name, OperationType::Update);

                    let stmt = T::update_many()
                        .set(active_model)
                        .apply_if(entity_filter.clone(), |q, f| q.filter(f))
                        .filter(filter_condition.clone());

                    let result: Vec<T::Model> = if db.support_returning() {
                        stmt.exec_with_returning(&transaction).await?
                    } else {
                        stmt.exec(&transaction).await?;

                        T::find()
                            .apply_if(entity_filter, |q, f| q.filter(f))
                            .filter(filter_condition)
                            .all(&transaction)
                            .await?
                    };

                    for model in result.iter() {
                        A::after_save(model.clone(), &transaction, false).await?;
                    }

                    transaction.commit().await?;

                    hooks
                        .entity_watch(&ctx, &object_name, OperationType::Update)
                        .await;

                    Ok(Some(FieldValue::list(
                        result.into_iter().map(FieldValue::owned_any),
                    )))
                })
            },
        )
        .argument(InputValue::new(
            &context.entity_update_mutation.data_field,
            TypeRef::named_nn(entity_input_builder.update_type_name::<T>()),
        ))
        .argument(InputValue::new(
            &context.entity_update_mutation.filter_field,
            TypeRef::named(entity_filter_input_builder.type_name(&object_name_)),
        ))
    }
}
