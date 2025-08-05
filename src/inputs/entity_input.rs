use std::collections::BTreeMap;

use async_graphql::dynamic::{InputObject, InputValue, ObjectAccessor};
use sea_orm::{ColumnTrait, EntityTrait, Iterable, PrimaryKeyToColumn, PrimaryKeyTrait};

use crate::{BuilderContext, EntityObjectBuilder, SeaResult, TypesMapHelper};

/// The configuration structure of EntityInputBuilder
pub struct EntityInputConfig {
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
            insert_suffix: "InsertInput".into(),
            insert_skips: Vec::new(),
            update_suffix: "UpdateInput".into(),
            update_skips: Vec::new(),
        }
    }
}

/// Used to create the entity create/update input object
pub struct EntityInputBuilder {
    pub context: &'static BuilderContext,
}

impl EntityInputBuilder {
    /// used to get SeaORM entity insert input object name
    pub fn insert_type_name<T>(&self) -> String
    where
        T: EntityTrait,
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
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let object_name = entity_object_builder.type_name::<T>();
        format!("{}{}", object_name, self.context.entity_input.update_suffix)
    }

    /// used to produce the SeaORM entity input object
    fn input_object<T>(&self, is_insert: bool) -> InputObject
    where
        T: EntityTrait,
    {
        let name = if is_insert {
            self.insert_type_name::<T>()
        } else {
            self.update_type_name::<T>()
        };

        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let types_map_helper = TypesMapHelper {
            context: self.context,
        };

        T::Column::iter().fold(InputObject::new(name), |object, column| {
            let column_name = entity_object_builder.column_name::<T>(&column);

            let full_name = format!("{}.{}", entity_object_builder.type_name::<T>(), column_name);

            let skip = if is_insert {
                self.context.entity_input.insert_skips.contains(&full_name)
            } else {
                self.context.entity_input.update_skips.contains(&full_name)
            };

            if skip {
                return object;
            }

            let column_def = column.def();
            let enum_type_name = column.enum_type_name();

            let auto_increment = match <T::PrimaryKey as PrimaryKeyToColumn>::from_column(column) {
                Some(_) => T::PrimaryKey::auto_increment(),
                None => false,
            };
            let has_default_expr = column_def.get_column_default().is_some();
            let is_insert_not_nullable =
                is_insert && !(column_def.is_null() || auto_increment || has_default_expr);

            let graphql_type = match types_map_helper.sea_orm_column_type_to_graphql_type(
                column_def.get_column_type(),
                is_insert_not_nullable,
                enum_type_name,
            ) {
                Some(type_name) => type_name,
                None => return object,
            };

            object.field(InputValue::new(column_name, graphql_type))
        })
    }

    /// used to produce the SeaORM entity insert input object
    pub fn insert_input_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
    {
        self.input_object::<T>(true)
    }

    /// used to produce the SeaORM entity update input object
    pub fn update_input_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
    {
        self.input_object::<T>(false)
    }

    pub fn parse_object<T>(
        &self,
        object: &ObjectAccessor,
    ) -> SeaResult<BTreeMap<String, sea_orm::Value>>
    where
        T: EntityTrait,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let types_map_helper = TypesMapHelper {
            context: self.context,
        };

        let mut map = BTreeMap::<String, sea_orm::Value>::new();

        for column in T::Column::iter() {
            let column_name = entity_object_builder.column_name::<T>(&column);

            let value = match object.get(&column_name) {
                Some(value) => value,
                None => continue,
            };

            let result =
                types_map_helper.async_graphql_value_to_sea_orm_value::<T>(&column, &value)?;

            map.insert(column_name, result);
        }

        Ok(map)
    }
}
