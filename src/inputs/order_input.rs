use async_graphql::dynamic::{InputObject, InputValue, TypeRef};
use sea_orm::{EntityTrait, Iterable};

use crate::{BuilderContext, EntityObjectBuilder};

/// The configuration structure for OrderInputBuilder
pub struct OrderInputConfig {
    /// used to format OrderInput object name
    pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for OrderInputConfig {
    fn default() -> Self {
        OrderInputConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{}OrderInput", object_name)
            }),
        }
    }
}

/// This builder produces the OrderInput object of a SeaORM entity
pub struct OrderInputBuilder {
    pub context: &'static BuilderContext,
}

impl OrderInputBuilder {
    /// used to get type name
    pub fn type_name(&self, object_name: &str) -> String {
        self.context.order_input.type_name.as_ref()(object_name)
    }

    /// used to get the OrderInput object of a SeaORM entity
    pub fn to_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };

        let object_name = entity_object_builder.type_name::<T>();
        let name = self.type_name(&object_name);

        T::Column::iter().fold(InputObject::new(name), |object, column| {
            object.field(InputValue::new(
                entity_object_builder.column_name::<T>(column),
                TypeRef::named(&self.context.order_by_enum.type_name),
            ))
        })
    }
}
