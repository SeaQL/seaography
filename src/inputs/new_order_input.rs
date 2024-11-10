use async_graphql::dynamic::{InputObject, InputValue, TypeRef, ValueAccessor};
use sea_orm::{EntityTrait, Iterable};

use crate::{BuilderContext, EntityObjectBuilder};

/// The configuration structure for OrderInputBuilder
pub struct NewOrderInputConfig {
    /// used to format OrderInput object name
    pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for NewOrderInputConfig {
    fn default() -> Self {
        NewOrderInputConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{object_name}NewOrderInput")
            }),
        }
    }
}

/// This builder produces the OrderInput object of a SeaORM entity
pub struct NewOrderInputBuilder {
    pub context: &'static BuilderContext,
}

impl NewOrderInputBuilder {
    /// used to get type name
    pub fn type_name(&self, object_name: &str) -> String {
        self.context.new_order_input.type_name.as_ref()(object_name)
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

        InputObject::new(name)
            .field(InputValue::new(
                "asc",
                TypeRef::named(&(self.context.order_enum.type_name)(&object_name)),
            ))
            .field(InputValue::new(
                "desc",
                TypeRef::named(&(self.context.order_enum.type_name)(&object_name)),
            ))
    }

    pub fn parse_object<T>(
        &self,
        value: Option<ValueAccessor<'_>>,
    ) -> Vec<(T::Column, sea_orm::sea_query::Order)>
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        match value {
            Some(value) => {
                let mut data = Vec::new();
                let order = value.object().unwrap();
                let entity_object = EntityObjectBuilder {
                    context: self.context,
                };
                for col in T::Column::iter() {
                    let column_name = entity_object.column_name::<T>(&col);
                    if let Some(order) = order.get("asc") {
                        if column_name == order.enum_name().unwrap() {
                            data.push((col, sea_orm::Order::Asc));
                        }
                    }

                    if let Some(order) = order.get("desc") {
                        if column_name == order.enum_name().unwrap() {
                            data.push((col, sea_orm::Order::Desc));
                        }
                    }
                }
                data
            }
            None => Vec::new(),
        }
    }
}