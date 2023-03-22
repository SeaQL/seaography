use async_graphql::dynamic::{InputObject, InputValue, TypeRef};
use sea_orm::{EntityTrait, Iterable};

use crate::{BuilderContext, EntityObjectBuilder};

pub struct OrderInputConfig {
    pub type_name: Box<dyn Fn(&String) -> String + Sync>,
}

impl std::default::Default for OrderInputConfig {
    fn default() -> Self {
        OrderInputConfig {
            type_name: Box::new(|name: &String| -> String {
                format!("{}OrderInput", name)
            }),
        }
    }
}

pub struct OrderInputBuilder {
    pub context: &'static BuilderContext,
}

impl OrderInputBuilder {
    pub fn type_name(&self, object_name: &String) -> String {
        self.context.order_input.type_name.as_ref()(object_name)
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
