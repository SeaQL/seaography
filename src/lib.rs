//! Seaography - 🧭 A GraphQL framework and code generator for SeaORM

pub use seaography_derive as macros;

#[derive(Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum)]
pub enum OrderByEnum {
    Asc,
    Desc,
}

pub type BinaryVector = Vec<u8>;

#[derive(Debug, async_graphql::InputObject)]
#[graphql(concrete(name = "StringFilter", params(String)))]
#[graphql(concrete(name = "TinyIntegerFilter", params(i8)))]
#[graphql(concrete(name = "SmallIntegerFilter", params(i16)))]
#[graphql(concrete(name = "IntegerFilter", params(i32)))]
#[graphql(concrete(name = "BigIntegerFilter", params(i64)))]
#[graphql(concrete(name = "TinyUnsignedFilter", params(u8)))]
#[graphql(concrete(name = "SmallUnsignedFilter", params(u16)))]
#[graphql(concrete(name = "UnsignedFilter", params(u32)))]
#[graphql(concrete(name = "BigUnsignedFilter", params(u64)))]
#[graphql(concrete(name = "FloatFilter", params(f32)))]
#[graphql(concrete(name = "DoubleFilter", params(f64)))]
// TODO #[graphql(concrete(name = "JsonFilter", params()))]
// TODO #[graphql(concrete(name = "DateFilter", params()))]
// TODO #[graphql(concrete(name = "TimeFilter", params()))]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateFilter", params(sea_orm::prelude::Date)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateTimeFilter", params(sea_orm::prelude::DateTime)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateTimeUtcFilter", params(sea_orm::prelude::DateTimeUtc)))
)]
// TODO #[graphql(concrete(name = "TimestampFilter", params()))]
// TODO #[graphql(concrete(name = "TimestampWithTimeZoneFilter", params()))]
#[cfg_attr(
    feature = "with-decimal",
    graphql(concrete(name = "DecimalFilter", params(sea_orm::prelude::Decimal)))
)]
#[cfg_attr(
    feature = "with-uuid",
    graphql(concrete(name = "UuidFilter", params(sea_orm::prelude::Uuid)))
)]
#[graphql(concrete(name = "BinaryFilter", params(BinaryVector)))]
#[graphql(concrete(name = "BooleanFilter", params(bool)))]
pub struct TypeFilter<T: async_graphql::InputType> {
    pub eq: Option<T>,
    pub ne: Option<T>,
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
    pub is_in: Option<Vec<T>>,
    pub is_not_in: Option<Vec<T>>,
    pub is_null: Option<bool>,
}
