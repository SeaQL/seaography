use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_type_filter() -> TokenStream {
    quote! {
        use sea_orm::prelude::*;

        #[derive(async_graphql::InputObject, Debug)]
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
        #[graphql(concrete(name = "DateTimeFilter", params(DateTime)))]
        #[graphql(concrete(name = "DateTimeUtcFilter", params(DateTimeUtc)))]
        // TODO #[graphql(concrete(name = "TimestampFilter", params()))]
        // TODO #[graphql(concrete(name = "TimestampWithTimeZoneFilter", params()))]
        #[graphql(concrete(name = "DecimalFilter", params(Decimal)))]
        // TODO #[graphql(concrete(name = "UuidFilter", params(uuid::Uuid)))]
        // TODO #[graphql(concrete(name = "BinaryFilter", params()))]
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
    }
}

pub fn write_type_filter<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    let file_name = path.as_ref().join("type_filter.rs");

    let data = generate_type_filter();

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}
