use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ColumnType {
    String,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    Float,
    Double,
    Json,
    Date,
    Time,
    DateTime,
    Timestamp,
    TimestampWithTimeZone,
    Decimal,
    Uuid,
    Binary,
    Boolean,
    Enum(String),
}

pub fn map_sea_query_column_type(col_type: &sea_query::ColumnType) -> ColumnType {
    match col_type {
        sea_query::ColumnType::Char(_)
        | sea_query::ColumnType::String(_)
        | sea_query::ColumnType::Text
        | sea_query::ColumnType::Custom(_) => ColumnType::String,
        sea_query::ColumnType::TinyInteger(_) => ColumnType::Integer8,
        sea_query::ColumnType::SmallInteger(_) => ColumnType::Integer16,
        sea_query::ColumnType::Integer(_) => ColumnType::Integer32,
        sea_query::ColumnType::BigInteger(_) => ColumnType::Integer64,
        sea_query::ColumnType::TinyUnsigned(_) => ColumnType::Unsigned8,
        sea_query::ColumnType::SmallUnsigned(_) => ColumnType::Unsigned16,
        sea_query::ColumnType::Unsigned(_) => ColumnType::Unsigned32,
        sea_query::ColumnType::BigUnsigned(_) => ColumnType::Unsigned64,
        sea_query::ColumnType::Float(_) => ColumnType::Float,
        sea_query::ColumnType::Double(_) => ColumnType::Double,
        sea_query::ColumnType::Json | sea_query::ColumnType::JsonBinary => ColumnType::Json,
        sea_query::ColumnType::Date => ColumnType::Date,
        sea_query::ColumnType::Time(_) => ColumnType::Time,
        sea_query::ColumnType::DateTime(_) => ColumnType::DateTime,
        sea_query::ColumnType::Timestamp(_) => ColumnType::Timestamp,
        sea_query::ColumnType::TimestampWithTimeZone(_) => ColumnType::TimestampWithTimeZone,
        sea_query::ColumnType::Decimal(_) | sea_query::ColumnType::Money(_) => ColumnType::Decimal,
        sea_query::ColumnType::Uuid => ColumnType::Uuid,
        sea_query::ColumnType::Binary(_) => ColumnType::Binary,
        sea_query::ColumnType::Boolean => ColumnType::Boolean,
        sea_query::ColumnType::Enum(name, _) => ColumnType::Enum(name.clone()),
        _ => unimplemented!(),
    }
}
