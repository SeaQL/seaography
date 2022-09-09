pub use seaography_derive as macros;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_parser)]
    pub database_url: String,

    #[clap(value_parser)]
    pub crate_name: String,

    #[clap(value_parser)]
    pub destination: String,

    #[clap(short, long)]
    pub expanded_format: Option<bool>,

    #[clap(short, long)]
    pub depth_limit: Option<usize>,

    #[clap(short, long)]
    pub complexity_limit: Option<usize>,
}

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
#[cfg_attr(feature = "with-chrono", graphql(concrete(name = "DateFilter", params(sea_orm::prelude::Date))))]
#[cfg_attr(feature = "with-chrono", graphql(concrete(name = "DateTimeFilter", params(sea_orm::prelude::DateTime))))]
#[cfg_attr(feature = "with-chrono", graphql(concrete(name = "DateTimeUtcFilter", params(sea_orm::prelude::DateTimeUtc))))]
// TODO #[graphql(concrete(name = "TimestampFilter", params()))]
// TODO #[graphql(concrete(name = "TimestampWithTimeZoneFilter", params()))]
#[cfg_attr(feature = "with-decimal", graphql(concrete(name = "DecimalFilter", params(sea_orm::prelude::Decimal))))]
// TODO #[graphql(concrete(name = "UuidFilter", params(uuid::Uuid)))]
#[graphql(concrete(name = "BinaryFilter", params(BinaryVector)))]
#[graphql(concrete(name = "BooleanFilter", params(bool)))]
// TODO #[graphql(concrete(name = "EnumFilter", params()))]
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
