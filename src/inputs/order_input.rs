use async_graphql::dynamic::{InputObject, InputValue, TypeRef};
use sea_orm::{EntityTrait, Iterable};

use crate::{BuilderContext, EntityObjectBuilder};

#[derive(Clone, Debug)]
pub struct OrderInputConfig {
    pub type_name: String,
}

impl std::default::Default for OrderInputConfig {
    fn default() -> Self {
        OrderInputConfig {
            type_name: "OrderInput".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OrderInputBuilder {
    pub context: &'static BuilderContext,
}

impl OrderInputBuilder {
    // FIXME: use context naming function
    pub fn type_name(&self, object_name: &String) -> String {
        format!("{}{}", object_name, self.context.order_input.type_name)
    }

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
