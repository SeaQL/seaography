use async_graphql::dynamic::{InputObject, InputValue, TypeRef};
use sea_orm::EntityTrait;

use crate::{pluralize_unique, BuilderContext, EntityObjectBuilder, RelatedEntityFilter};

/// The configuration structure for HavingInputBuilder
pub struct HavingInputConfig {
    /// the filter input type name formatter function
    pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for HavingInputConfig {
    fn default() -> Self {
        HavingInputConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{object_name}HavingInput")
            }),
        }
    }
}

/// This builder is used to produce the filter input object of a SeaORM entity
pub struct HavingInputBuilder {
    pub context: &'static BuilderContext,
}

impl HavingInputBuilder {
    /// used to get the filter input object name
    /// object_name is the name of the SeaORM Entity GraphQL object
    pub fn type_name(&self, object_name: &str) -> String {
        let object_name = pluralize_unique(object_name, false);
        self.context.having_input.type_name.as_ref()(&object_name)
    }

    /// used to produce the filter input object of a SeaORM entity
    pub fn to_object<T>(&self, related_entity_filter: &RelatedEntityFilter<T>) -> InputObject
    where
        T: EntityTrait,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let entity_name = entity_object_builder.type_name::<T>();
        let field_name = self.type_name(&entity_name);

        let related_fields = related_entity_filter.field_names();

        let mut object = InputObject::new(&field_name);

        if related_fields.is_empty() {
            object = object.field(InputValue::new("_", TypeRef::named(TypeRef::BOOLEAN)));
        }

        for (field_name, filter_input) in related_fields {
            object = object.field(InputValue::new(field_name, TypeRef::named(filter_input)));
        }

        object
    }
}
