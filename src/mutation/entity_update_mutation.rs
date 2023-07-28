use async_graphql::dynamic::{Field, FieldFuture, FieldValue, InputValue, TypeRef};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    TransactionTrait,
};

use crate::{
    get_filter_conditions, prepare_active_model, BuilderContext, EntityInputBuilder,
    EntityObjectBuilder, EntityQueryFieldBuilder, FilterInputBuilder,
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
            mutation_suffix: "Update".into(),
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
        <T as EntityTrait>::Model: Sync,
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
        let entity_filter_input_builder = FilterInputBuilder {
            context: self.context,
        };
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let object_name: String = entity_object_builder.type_name::<T>();

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

                    let filters = ctx.args.get(&context.entity_update_mutation.filter_field);
                    let filter_condition = get_filter_conditions::<T>(context, filters);
                    println!("{:?}", filter_condition);

                    let value_accessor = ctx
                        .args
                        .get(&context.entity_create_one_mutation.data_field)
                        .unwrap();
                    let input_object = &value_accessor.object()?;
                    let active_model = prepare_active_model::<T, A>(
                        &entity_input_builder,
                        &entity_object_builder,
                        input_object,
                    )?;

                    T::update_many()
                        .set(active_model)
                        .filter(filter_condition.clone())
                        .exec(&transaction)
                        .await?;

                    let result: Vec<T::Model> =
                        T::find().filter(filter_condition).all(&transaction).await?;

                    transaction.commit().await?;

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
            TypeRef::named(entity_filter_input_builder.type_name(&object_name)),
        ))
    }
}
