use async_graphql::dynamic::{ObjectAccessor, ValueAccessor};
use sea_orm::{ColumnTrait, ColumnType, Condition, EntityTrait, Iterable};

use crate::{
    prepare_enumeration_condition, prepare_float_condition, prepare_integer_condition,
    prepare_string_condition, prepare_unsigned_condition, BuilderContext, EntityObjectBuilder,
};

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

pub fn recursive_prepare_condition<T>(
    context: &'static BuilderContext,
    filters: ObjectAccessor,
) -> Condition
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let entity_object = EntityObjectBuilder { context };

    let condition = T::Column::iter().fold(Condition::all(), |condition, column: T::Column| {
        let column_name = entity_object.column_name::<T>(column);

        let filter = filters.get(&column_name);

        if let Some(filter) = filter {
            let filter = filter.object().unwrap();

            match column.def().get_column_type() {
                ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => {
                    prepare_string_condition(&filter, column, condition)
                }
                ColumnType::TinyInteger
                | ColumnType::SmallInteger
                | ColumnType::Integer
                | ColumnType::BigInteger => prepare_integer_condition(&filter, column, condition),
                ColumnType::TinyUnsigned
                | ColumnType::SmallUnsigned
                | ColumnType::Unsigned
                | ColumnType::BigUnsigned => prepare_unsigned_condition(&filter, column, condition),
                // FIXME: support f32 (different precision)
                ColumnType::Float | ColumnType::Double => {
                    prepare_float_condition(&filter, column, condition)
                }
                #[cfg(feature = "with-decimal")]
                ColumnType::Decimal(_) => crate::prepare_parsed_condition(
                    &filter,
                    column,
                    |v| {
                        use std::str::FromStr;

                        sea_orm::entity::prelude::Decimal::from_str(&v)
                            .expect("We expect value to be Decimal")
                    },
                    condition,
                ),
                // ColumnType::DateTime => {
                // FIXME
                // },
                // ColumnType::Timestamp => {
                // FIXME
                // },
                // ColumnType::TimestampWithTimeZone => {
                // FIXME
                // },
                // ColumnType::Time => {
                // FIXME
                // },
                // ColumnType::Date => {
                // FIXME
                // },
                // ColumnType::Year => {
                // FIXME
                // },
                // ColumnType::Interval => {
                // FIXME
                // },
                // ColumnType::Binary => {
                // FIXME
                // },
                // ColumnType::VarBinary => {
                // FIXME
                // },
                // ColumnType::Bit => {
                // FIXME
                // },
                // ColumnType::VarBit => {
                // FIXME
                // },
                // ColumnType::Boolean => {
                // FIXME
                // },
                // ColumnType::Money(_) => {
                // FIXME
                // },
                // ColumnType::Json => {
                // FIXME
                // },
                // ColumnType::JsonBinary => {
                // FIXME
                // },
                // ColumnType::Custom(_) => {
                // FIXME
                // },
                // ColumnType::Uuid => {
                // FIXME
                // },
                ColumnType::Enum { name: _, variants } => {
                    prepare_enumeration_condition(&filter, column, variants, condition)
                }
                // ColumnType::Array(_) => {
                // FIXME
                // },
                // ColumnType::Cidr => {
                // FIXME
                // },
                // ColumnType::Inet => {
                // FIXME
                // },
                // ColumnType::MacAddr => {
                // FIXME
                // },
                _ => panic!("Type is not supported"),
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
