use async_graphql::dynamic::ValueAccessor;
use heck::ToLowerCamelCase;
use sea_orm::{EntityTrait, IdenStatic, Iterable, QueryOrder, Select};

use crate::BuilderContext;

/// used to parse order input object and apply it to statement
pub fn apply_order<T>(
    context: &BuilderContext,
    stmt: Select<T>,
    order_by: Option<ValueAccessor>,
) -> Select<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    if let Some(order_by) = order_by {
        let order_by = order_by
            .object()
            .expect("We expect the ordering of entity to be object type");

        T::Column::iter().fold(stmt, |stmt, column: T::Column| {
            let order = order_by.get(column.as_str().to_lower_camel_case().as_str());

            if let Some(order) = order {
                let order = order
                    .enum_name()
                    .expect("We expect the order of a column to be enumeration type");

                let asc_variant = &context.order_by_enum.asc_variant;
                let desc_variant = &context.order_by_enum.desc_variant;

                if order.eq(asc_variant) {
                    stmt.order_by(column, sea_orm::Order::Asc)
                } else if order.eq(desc_variant) {
                    stmt.order_by(column, sea_orm::Order::Desc)
                } else {
                    panic!("Cannot map enumeration")
                }
            } else {
                stmt
            }
        })
    } else {
        stmt
    }
}
