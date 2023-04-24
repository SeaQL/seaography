use std::collections::BTreeMap;

use async_graphql::dynamic::{InputObject, InputValue, TypeRef, ObjectAccessor};
use sea_orm::{ColumnTrait, EntityTrait, Iterable};

use crate::{map_sea_orm_column_type_to_graphql_type, BuilderContext, EntityObjectBuilder, map_graphql_value_to_sea_orm_value};

/// The configuration structure of EntityInputBuilder
pub struct EntityInputConfig {
    /// if true both insert and update are the same input object
    pub unified: bool,
    /// suffix that is appended on insert input objects
    pub insert_suffix: String,
    /// names of "{entity}.{column}" you want to skip the insert input to be generated
    pub insert_skips: Vec<String>,
    /// suffix that is appended on update input objects
    pub update_suffix: String,
    /// names of "{entity}.{column}" you want to skip the update input to be generated
    pub update_skips: Vec<String>,
}

impl std::default::Default for EntityInputConfig {
    fn default() -> Self {
        EntityInputConfig {
            unified: true,
            insert_suffix: "InsertInput".into(),
            insert_skips: Vec::new(),
            update_suffix: "UpdateInput".into(),
            update_skips: Vec::new(),
        }
    }
}

pub struct EntityInputBuilder {
    pub context: &'static BuilderContext,
}

impl EntityInputBuilder {
    /// used to get SeaORM entity insert input object name
    pub fn insert_type_name<T>(&self) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = entity_object_builder.type_name::<T>();
        format!("{}{}", object_name, self.context.entity_input.insert_suffix)
    }

    /// used to get SeaORM entity update input object name
    pub fn update_type_name<T>(&self) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        if self.context.entity_input.unified {
            return self.insert_type_name::<T>();
        }
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = entity_object_builder.type_name::<T>();
        format!("{}{}", object_name, self.context.entity_input.update_suffix)
    }

    /// used to produce the SeaORM entity input object
    fn input_object<T>(&self, insert: bool) -> InputObject
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let name = if insert {
            self.insert_type_name::<T>()
        } else {
            self.update_type_name::<T>()
        };

        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };

        T::Column::iter().fold(InputObject::new(name), |object, column| {
            let column_name = entity_object_builder.column_name::<T>(column);

            let full_name = format!("{}.{}", entity_object_builder.type_name::<T>(), column_name);

            let skip = if insert {
                self.context.entity_input.insert_skips.contains(&full_name)
            } else {
                self.context.entity_input.update_skips.contains(&full_name)
            };

            if skip {
                return object
            }

            let column_def = column.def();

            let type_name = match map_sea_orm_column_type_to_graphql_type(
                self.context,
                column_def.get_column_type(),
            ) {
                Some(type_name) => type_name,
                None => return object,
            };

            let graphql_type = if column_def.is_null() {
                TypeRef::named(type_name)
            } else {
                TypeRef::named_nn(type_name)
            };

            object.field(InputValue::new(column_name, graphql_type))
        })
    }

    /// used to produce the SeaORM entity insert input object
    pub fn insert_input_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        self.input_object::<T>(true)
    }

    /// used to produce the SeaORM entity update input object
    pub fn update_input_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        self.input_object::<T>(self.context.entity_input.unified)
    }

    pub fn parse_object<T>(&self, object: &ObjectAccessor) -> Result<BTreeMap<String, sea_orm::Value>, async_graphql::Error>
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let mut map = BTreeMap::<String, sea_orm::Value>::new();

        for column in T::Column::iter() {
            let column_name = entity_object_builder.column_name::<T>(column);

            let value = match object.get(&column_name) {
                Some(value) => value,
                None => continue,
            };

            match map_graphql_value_to_sea_orm_value(self.context, column.def().get_column_type(), value) {
                Some(result) => {
                    map.insert(column_name, result?);
                },
                None => continue,
            };
        }

        Ok(map)
    }
}
