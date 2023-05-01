use async_graphql::dynamic::{ObjectAccessor, ValueAccessor};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait, Iterable};

use crate::{
    prepare_enumeration_condition, BuilderContext, EntityObjectBuilder, FilterInputBuilder,
};

/// utility function used to create the query filter condition
/// for a SeaORM entity using query filter inputs
pub fn get_filter_conditions<T>(
    context: &'static BuilderContext,
    filters: Option<ValueAccessor>,
) -> Condition
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    if let Some(filters) = filters {
        let filters = filters.object().unwrap();

        recursive_prepare_condition::<T>(context, filters)
    } else {
        Condition::all()
    }
}

/// used to prepare recursively the query filtering condition
pub fn recursive_prepare_condition<T>(
    context: &'static BuilderContext,
    filters: ObjectAccessor,
) -> Condition
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let entity_object_builder = EntityObjectBuilder { context };
    let filter_input_builder = FilterInputBuilder { context };

    let condition = T::Column::iter().fold(Condition::all(), |condition, column: T::Column| {
        let column_name = entity_object_builder.column_name::<T>(&column);

        let filter = filters.get(&column_name);

        if let Some(filter) = filter {
            let filter = filter.object().unwrap();

            if let ColumnType::Enum { name: _, variants } = column.def().get_column_type() {
                prepare_enumeration_condition(&filter, column, variants, condition)
            } else {
                filter_input_builder
                    .prepare_column_condition::<T>(condition, &filter, &column)
                    .unwrap()
            }
        } else {
            condition
        }
    });

    let condition = if let Some(and) = filters.get("and") {
        let filters = and.list().unwrap();

        condition.add(
            filters
                .iter()
                .fold(Condition::all(), |condition, filters: ValueAccessor| {
                    let filters = filters.object().unwrap();
                    condition.add(recursive_prepare_condition::<T>(context, filters))
                }),
        )
    } else {
        condition
    };

    let condition = if let Some(or) = filters.get("or") {
        let filters = or.list().unwrap();

        condition.add(
            filters
                .iter()
                .fold(Condition::any(), |condition, filters: ValueAccessor| {
                    let filters = filters.object().unwrap();
                    condition.add(recursive_prepare_condition::<T>(context, filters))
                }),
        )
    } else {
        condition
    };

    condition
}
