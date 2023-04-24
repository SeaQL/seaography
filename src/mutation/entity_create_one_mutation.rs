use async_graphql::dynamic::{Field, TypeRef, FieldFuture, FieldValue, InputValue};
use sea_orm::{EntityTrait, DatabaseConnection, ActiveModelTrait, Iterable, PrimaryKeyToColumn, PrimaryKeyTrait, IntoActiveModel};

use crate::{BuilderContext, EntityInputBuilder, EntityQueryFieldBuilder, EntityObjectBuilder};

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
        let entity_query_field_builder = EntityQueryFieldBuilder { context: self.context };
        format!("{}{}", entity_query_field_builder.type_name::<T>(), self.context.entity_create_mutation.mutation_suffix)
    }

    /// used to get the create mutation field for a SeaORM entity
    pub fn to_field<T, A>(&self) -> Field
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
        <T as EntityTrait>::Model: IntoActiveModel<A>,
        A: ActiveModelTrait<Entity = T> + sea_orm::ActiveModelBehavior + std::marker::Send,
    {
        let entity_input_builder = EntityInputBuilder { context: self.context };
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

                    let mut data = entity_input_builder.parse_object::<T>(&ctx.args.get(&context.entity_create_mutation.data_field).unwrap().object()?)?;

                    let mut model = A::default();

                    for column in T::Column::iter() {
                        let auto_increment = match <T::PrimaryKey as PrimaryKeyToColumn>::from_column(column) {
                            Some(_) => T::PrimaryKey::auto_increment(),
                            None => false,
                        };

                        if auto_increment {
                            continue;
                        }

                        match data.remove(&entity_object_builder.column_name::<T>(column)) {
                            Some(value) => {
                                model.set(column, value);
                            },
                            None => continue,
                        }
                    }

                    let db = ctx.data::<DatabaseConnection>()?;

                    let result = model.insert(db).await?;

                    Ok(Some(FieldValue::owned_any(result)))
                })
            }
        )
        .argument(InputValue::new(&context.entity_create_mutation.data_field, TypeRef::named_nn(entity_input_builder.insert_type_name::<T>())))
    }
}
