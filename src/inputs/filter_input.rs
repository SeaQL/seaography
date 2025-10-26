use async_graphql::dynamic::{InputObject, InputValue, TypeRef};
use sea_orm::{EntityTrait, Iterable};

use crate::{pluralize_unique, BuilderContext, EntityObjectBuilder, FilterTypesMapHelper};

/// The configuration structure for FilterInputBuilder
pub struct FilterInputConfig {
    /// the filter input type name formatter function
    pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for FilterInputConfig {
    fn default() -> Self {
        FilterInputConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{object_name}FilterInput")
            }),
        }
    }
}

/// This builder is used to produce the filter input object of a SeaORM entity
pub struct FilterInputBuilder {
    pub context: &'static BuilderContext,
}

impl FilterInputBuilder {
    /// used to get the filter input object name
    /// object_name is the name of the SeaORM Entity GraphQL object
    pub fn type_name(&self, object_name: &str) -> String {
        let object_name = pluralize_unique(object_name, false);
        self.context.filter_input.type_name.as_ref()(&object_name)
    }

    /// used to produce the filter input object of a SeaORM entity
    pub fn to_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
    {
        let filter_types_map_helper = FilterTypesMapHelper {
            context: self.context,
        };

        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let entity_name = entity_object_builder.type_name::<T>();
        let filter_name = self.type_name(&entity_name);

        let object = T::Column::iter().fold(InputObject::new(&filter_name), |object, column| {
            match filter_types_map_helper.get_column_filter_input_value::<T>(&column) {
                Some(field) => object.field(field),
                None => object,
            }
        });

        object
            .field(InputValue::new("and", TypeRef::named_nn_list(&filter_name)))
            .field(InputValue::new("or", TypeRef::named_nn_list(&filter_name)))
            .field(InputValue::new("not", TypeRef::named(&filter_name)))
    }
}
