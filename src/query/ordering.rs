use sea_orm::{EntityTrait, QueryOrder, Select};

/// used to parse order input object and apply it to statement
pub fn apply_order<T>(
    stmt: Select<T>,
    order_by: Vec<(T::Column, sea_orm::sea_query::Order)>,
) -> Select<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    order_by
        .into_iter()
        .fold(stmt, |stmt, (col, ord)| stmt.order_by(col, ord))
}
