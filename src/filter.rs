use async_graphql::dynamic::*;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use sea_orm::{prelude::*, Iterable};

/// used to create filter input for SeaORM Entity
pub fn entity_to_filter<T>(entity_object: &Object) -> InputObject
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let name = format!("{}FilterInput", entity_object.type_name());

    let object = T::Column::iter().fold(InputObject::new(&name), |object, column| {
        let name = column.as_str().to_lower_camel_case();

        let field = match column.def().get_column_type() {
            ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => {
                Some(InputValue::new(name, TypeRef::named("StringFilterInput")))
            }
            ColumnType::TinyInteger
            | ColumnType::SmallInteger
            | ColumnType::Integer
            | ColumnType::BigInteger
            | ColumnType::TinyUnsigned
            | ColumnType::SmallUnsigned
            | ColumnType::Unsigned
            | ColumnType::BigUnsigned => {
                Some(InputValue::new(name, TypeRef::named("IntegerFilterInput")))
            }
            ColumnType::Float | ColumnType::Double => {
                Some(InputValue::new(name, TypeRef::named("FloatFilterInput")))
            }
            ColumnType::Decimal(_) | ColumnType::Money(_) => {
                Some(InputValue::new(name, TypeRef::named("TextFilterInput")))
            }
            ColumnType::DateTime
            | ColumnType::Timestamp
            | ColumnType::TimestampWithTimeZone
            | ColumnType::Time
            | ColumnType::Date => Some(InputValue::new(name, TypeRef::named("TextFilterInput"))),
            ColumnType::Year(_) => {
                Some(InputValue::new(name, TypeRef::named("IntegerFilterInput")))
            }
            ColumnType::Boolean => {
                Some(InputValue::new(name, TypeRef::named("BooleanFilterInput")))
            }
            ColumnType::Uuid => Some(InputValue::new(name, TypeRef::named("TextFilterInput"))),
            // FIXME
            // ColumnType::Custom(_) => {
            // },
            ColumnType::Enum { name: enum_name, variants: _ } => {
                Some(InputValue::new(name, TypeRef::named(format!("{}EnumFilterInput", enum_name.to_string().to_upper_camel_case()))))
            },
            // FIXME
            // ColumnType::Array => {
            // },
            ColumnType::Cidr | ColumnType::Inet | ColumnType::MacAddr => {
                Some(InputValue::new(name, TypeRef::named("TextFilterInput")))
            }
            _ => None,
        };

        match field {
            Some(field) => object.field(field),
            None => object,
        }
    });

    object
        .field(InputValue::new("and", TypeRef::named_nn_list(&name)))
        .field(InputValue::new("or", TypeRef::named_nn_list(&name)))
}
