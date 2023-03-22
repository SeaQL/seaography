use async_graphql::dynamic::ValueAccessor;
use sea_orm::{EntityTrait, Iterable, QueryOrder, Select};

use crate::{BuilderContext, EntityObjectBuilder};

/// used to parse order input object and apply it to statement
pub fn apply_order<T>(
    context: &'static BuilderContext,
    stmt: Select<T>,
    order_by: Option<ValueAccessor>,
) -> Select<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    if let Some(order_by) = order_by {
        let order_by = order_by
            .object().unwrap();

        let entity_object = EntityObjectBuilder { context };

        T::Column::iter().fold(stmt, |stmt, column: T::Column| {
            let column_name = entity_object.column_name::<T>(column);

            let order = order_by.get(&column_name);

            if let Some(order) = order {
                let order = order
                    .enum_name().unwrap();

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
