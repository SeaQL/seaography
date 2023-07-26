use async_graphql::dynamic::{Field, FieldFuture, FieldValue, InputValue, ObjectAccessor, TypeRef};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Iterable,
    PrimaryKeyToColumn, PrimaryKeyTrait,
};

use crate::{BuilderContext, EntityInputBuilder, EntityObjectBuilder, EntityQueryFieldBuilder};

/// The configuration structure of EntityCreateOneMutationBuilder
pub struct EntityCreateOneMutationConfig {
    /// suffix that is appended on create mutations
    pub mutation_suffix: String,
    /// name for `data` field
    pub data_field: String,
}

impl std::default::Default for EntityCreateOneMutationConfig {
    fn default() -> Self {
        EntityCreateOneMutationConfig {
            mutation_suffix: "CreateOne".into(),
            data_field: "data".into(),
        }
    }
}

/// This builder produces the create one mutation for an entity
pub struct EntityCreateOneMutationBuilder {
    pub context: &'static BuilderContext,
}

impl EntityCreateOneMutationBuilder {
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
            self.context.entity_create_one_mutation.mutation_suffix
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
            TypeRef::named_nn(entity_object_builder.basic_type_name::<T>()),
            move |ctx| {
                FieldFuture::new(async move {
                    let entity_input_builder = EntityInputBuilder { context };
                    let entity_object_builder = EntityObjectBuilder { context };
                    let db = ctx.data::<DatabaseConnection>()?;
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

                    let result = active_model.insert(db).await?;

                    Ok(Some(FieldValue::owned_any(result)))
                })
            },
        )
        .argument(InputValue::new(
            &context.entity_create_one_mutation.data_field,
            TypeRef::named_nn(entity_input_builder.insert_type_name::<T>()),
        ))
    }
}

pub fn prepare_active_model<'a, T, A>(
    entity_input_builder: &EntityInputBuilder,
    entity_object_builder: &EntityObjectBuilder,
    input_object: &ObjectAccessor<'_>,
) -> async_graphql::Result<A>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
    <T as EntityTrait>::Model: IntoActiveModel<A>,
    A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
{
    let mut data = entity_input_builder.parse_object::<T>(input_object)?;

    let mut active_model = A::default();

    for column in T::Column::iter() {
        // used to skip auto created primary keys
        let auto_increment = match <T::PrimaryKey as PrimaryKeyToColumn>::from_column(column) {
            Some(_) => T::PrimaryKey::auto_increment(),
            None => false,
        };

        if auto_increment {
            continue;
        }

        match data.remove(&entity_object_builder.column_name::<T>(&column)) {
            Some(value) => {
                active_model.set(column, value);
            }
            None => continue,
        }
    }

    Ok(active_model)
}
