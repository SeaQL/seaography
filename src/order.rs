use async_graphql::dynamic::*;
use heck::ToLowerCamelCase;
use sea_orm::{prelude::*, Iterable};

/// used to create order input for SeaORM entity
pub fn entity_to_order<T>(entity_object: &Object) -> InputObject
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let name = format!("{}OrderInput", entity_object.type_name());

    T::Column::iter().fold(InputObject::new(name), |object, column| {
        object.field(InputValue::new(
            column.as_str().to_lower_camel_case(),
            TypeRef::named("OrderByEnum"),
        ))
    })
}

pub fn get_order_by_enum() -> Enum {
    Enum::new("OrderByEnum")
        .item(EnumItem::new("ASC"))
        .item(EnumItem::new("DESC"))
}
