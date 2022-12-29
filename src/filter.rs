use async_graphql::dynamic::*;
use sea_orm::{prelude::*, Iterable};

pub fn entity_to_filter<T>(entity_object: &Object) -> InputObject
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let name = format!("{}FilterInput", entity_object.type_name());

    let object = T::Column::iter().fold(InputObject::new(&name), |object, column| {
        let field = match column.def().get_column_type() {
            ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => {
                InputValue::new(column.as_str(), TypeRef::named("StringFilterInput"))
            }
            ColumnType::TinyInteger
            | ColumnType::SmallInteger
            | ColumnType::Integer
            | ColumnType::BigInteger
            | ColumnType::TinyUnsigned
            | ColumnType::SmallUnsigned
            | ColumnType::Unsigned
            | ColumnType::BigUnsigned => {
                InputValue::new(column.as_str(), TypeRef::named("IntegerFilterInput"))
            }
            ColumnType::Float | ColumnType::Double => {
                InputValue::new(column.as_str(), TypeRef::named("FloatFilterInput"))
            }
            ColumnType::Decimal(_) => {
                InputValue::new(column.as_str(), TypeRef::named("TextFilterInput"))
            }
            // ColumnType::DateTime => {

            // },
            // ColumnType::Timestamp => {

            // },
            // ColumnType::TimestampWithTimeZone => {

            // },
            // ColumnType::Time => {

            // },
            // ColumnType::Date => {

            // },
            // ColumnType::Binary => {

            // },
            // ColumnType::TinyBinary => {

            // },
            // ColumnType::MediumBinary => {

            // },
            // ColumnType::LongBinary => {

            // },
            // ColumnType::Boolean => {

            // },
            // ColumnType::Money(_) => {

            // },
            // ColumnType::Json => {

            // },
            // ColumnType::JsonBinary => {

            // },
            // ColumnType::Custom(_) => {

            // },
            // ColumnType::Uuid => {

            // },
            // ColumnType::Enum { name, variants } => {

            // },
            // ColumnType::Array(_) => {

            // },
            _ => InputValue::new(column.as_str(), TypeRef::named("TextFilterInput")),
        };

        object.field(field)
    });

    object
        .field(InputValue::new("and", TypeRef::named_nn_list(&name)))
        .field(InputValue::new("or", TypeRef::named_nn_list(&name)))
}
