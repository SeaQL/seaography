use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use seaography_types::enum_meta::EnumMeta;
use std::path::Path;
use heck::ToUpperCamelCase;

pub fn generate_enumeration(enum_meta: &EnumMeta) -> TokenStream {
    let enum_proxy_name = format!("sea_orm_active_enums::{}", enum_meta.camel_case());
    let enum_name = &enum_meta.enum_name;
    let enum_ident: TokenStream = enum_meta.camel_case().parse().unwrap();

    let enum_values = &enum_meta.enum_values;
    let values_variants = enum_values.iter().map(|v| v.trim()).map(|v| {
        if v.chars().all(|c| c.is_numeric()) {
            format_ident!("_{}", v)
        } else {
            format_ident!("{}", v.to_upper_camel_case())
        }
    });

    quote! {
        use sea_orm::entity::prelude::*;
        use async_graphql::*;

        use crate::orm::sea_orm_active_enums;

        #[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Enum)]
        #[graphql(remote = #enum_proxy_name)]
        #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = #enum_name)]
        pub enum #enum_ident {
            #(
                #[sea_orm(string_value = #enum_values)]
                #values_variants,
            )*
        }
    }
}

pub fn write_graphql_enum<P: AsRef<Path>>(path: &P, enum_meta: &EnumMeta) -> std::io::Result<()> {
    let file_name = path.as_ref().join(format!("{}.rs", enum_meta.snake_case()));

    let data = generate_enumeration(enum_meta);

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}
