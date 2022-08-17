use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use seaography_types::enum_meta::EnumMeta;
use std::path::Path;


// TODO: DEPRECATE FILE
// TODO: inject derive into sea orm cli generated code

/// Used to generate graphql enumeration
///
/// ```
/// use quote::quote;
/// use seaography_generator::files::enumeration::generate_enumeration;
/// use seaography_types::EnumMeta;
/// let enum_meta = EnumMeta {
///     enum_name: "Size".into(),
///     enum_values: vec!["Small".into(), "Medium".into(), "Large".into(), "Extra-Large".into()]
/// };
///
/// let left = generate_enumeration(&enum_meta);
/// let right = quote!{
///     use crate::orm::sea_orm_active_enums;
///     use async_graphql::*;
///     use sea_orm::entity::prelude::*;
///     #[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Enum)]
///     #[graphql(remote = "sea_orm_active_enums::Size")]
///     #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "Size")]
///     pub enum Size {
///         #[sea_orm(string_value = "Small")]
///         Small,
///         #[sea_orm(string_value = "Medium")]
///         Medium,
///         #[sea_orm(string_value = "Large")]
///         Large,
///         #[sea_orm(string_value = "Extra-Large")]
///         ExtraLarge,
///     }
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
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
        use crate::orm::sea_orm_active_enums;
        use async_graphql::*;
        use sea_orm::entity::prelude::*;

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
