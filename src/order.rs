use async_graphql::dynamic::*;
use sea_orm::{prelude::*, Iterable};

pub fn entity_to_order<T>(entity_object: &Object) -> InputObject
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let name = format!("{}OrderInput", entity_object.type_name());

    T::Column::iter().fold(InputObject::new(name), |object, column| {
        object.field(InputValue::new(
            column.as_str(),
            TypeRef::named("OrderByEnum"),
        ))
    })
}

pub fn get_order_by_enum() -> Enum {
    Enum::new("OrderByEnum")
        .item(EnumItem::new("ASC"))
        .item(EnumItem::new("DESC"))
}
