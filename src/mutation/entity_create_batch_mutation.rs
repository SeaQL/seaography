use async_graphql::dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, TransactionTrait,
};

use crate::{
    prepare_active_model, BuilderContext, EntityInputBuilder, EntityObjectBuilder,
    EntityQueryFieldBuilder,
};

/// The configuration structure of EntityCreateBatchMutationBuilder
pub struct EntityCreateBatchMutationConfig {
    /// suffix that is appended on create mutations
    pub mutation_suffix: String,
    /// name for `data` field
    pub data_field: String,
}

impl std::default::Default for EntityCreateBatchMutationConfig {
    fn default() -> Self {
        EntityCreateBatchMutationConfig {
            mutation_suffix: "CreateBatch".into(),
            data_field: "data".into(),
        }
    }
}

/// This builder produces the create batch mutation for an entity
pub struct EntityCreateBatchMutationBuilder {
    pub context: &'static BuilderContext,
}

impl EntityCreateBatchMutationBuilder {
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
            self.context.entity_create_batch_mutation.mutation_suffix
        )
    }

    /// used to get the create mutation field for a SeaORM entity
    pub fn to_field<T, A>(&self) -> Field
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <T as EntityTrait>::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
    {
        let entity_input_builder = EntityInputBuilder {
            context: self.context,
        };
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };

        let context = self.context;

        Field::new(
            self.type_name::<T>(),
            TypeRef::named_nn_list_nn(entity_object_builder.basic_type_name::<T>()),
            move |ctx| {
                FieldFuture::new(async move {
                    let db = ctx.data::<DatabaseConnection>()?;
                    let transaction = db.begin().await?;

                    let entity_input_builder = EntityInputBuilder { context };
                    let entity_object_builder = EntityObjectBuilder { context };

                    let mut results: Vec<_> = Vec::new();
                    for input_object in ctx
                        .args
                        .get(&context.entity_create_batch_mutation.data_field)
                        .unwrap()
                        .list()?
                        .iter()
                    {
                        let active_model = prepare_active_model::<T, A>(
                            &entity_input_builder,
                            &entity_object_builder,
                            &(input_object.object()?),
                        )?;
                        let result = active_model.insert(&transaction).await?;
                        results.push(result);
                    }

                    transaction.commit().await?;

                    Ok(Some(FieldValue::list(
                        results
                            .into_iter()
                            .map(FieldValue::owned_any),
                    )))
                })
            },
        )
        .argument(InputValue::new(
            &context.entity_create_batch_mutation.data_field,
            TypeRef::named_nn_list_nn(entity_input_builder.insert_type_name::<T>()),
        ))
    }
}
