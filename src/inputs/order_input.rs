use async_graphql::dynamic::{InputObject, InputValue, TypeRef, ValueAccessor};
use sea_orm::{EntityTrait, Iterable};

use crate::{pluralize_unique, BuilderContext, EntityObjectBuilder};

/// The configuration structure for OrderInputBuilder
pub struct OrderInputConfig {
    /// used to format OrderInput object name
    pub type_name: crate::SimpleNamingFn,
}

impl std::default::Default for OrderInputConfig {
    fn default() -> Self {
        OrderInputConfig {
            type_name: Box::new(|object_name: &str| -> String {
                format!("{object_name}OrderInput")
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
        let object_name = pluralize_unique(&object_name, false);
        self.context.order_input.type_name.as_ref()(&object_name)
    }

    /// used to get the OrderInput object of a SeaORM entity
    pub fn to_object<T>(&self) -> InputObject
    where
        T: EntityTrait,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };

        let object_name = entity_object_builder.type_name::<T>();
        let name = self.type_name(&object_name);

        T::Column::iter().fold(InputObject::new(name), |object, column| {
            object.field(InputValue::new(
                entity_object_builder.column_name::<T>(&column),
                TypeRef::named(&self.context.order_by_enum.type_name),
            ))
        })
    }

    pub fn parse_object<T>(
        &self,
        value: Option<ValueAccessor<'_>>,
    ) -> Vec<(T::Column, sea_orm::sea_query::Order)>
    where
        T: EntityTrait,
    {
        match value {
            Some(value) => {
                let mut data = Vec::new();

                let order_by = value.object().unwrap();

                let entity_object = EntityObjectBuilder {
                    context: self.context,
                };

                for col in T::Column::iter() {
                    let column_name = entity_object.column_name::<T>(&col);
                    let order = order_by.get(&column_name);

                    if let Some(order) = order {
                        let order = order.enum_name().unwrap();

                        let asc_variant = &self.context.order_by_enum.asc_variant;
                        let desc_variant = &self.context.order_by_enum.desc_variant;

                        if order.eq(asc_variant) {
                            data.push((col, sea_orm::Order::Asc));
                        } else if order.eq(desc_variant) {
                            data.push((col, sea_orm::Order::Desc));
                        } else {
                            panic!("Cannot map enumeration")
                        }
                    }
                }

                data
            }
            None => Vec::new(),
        }
    }
}