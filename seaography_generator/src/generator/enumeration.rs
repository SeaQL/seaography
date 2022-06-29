use std::path::Path;
use proc_macro2::TokenStream;
use quote::quote;
use seaography_types::enum_meta::EnumMeta;

pub fn generate_enumeration(enum_meta: &EnumMeta) -> TokenStream {
    let enum_ident: TokenStream = enum_meta
        .camel_case()
        .parse()
        .unwrap();

    let enum_values: Vec<TokenStream> = enum_meta
        .enums()
        .iter()
        .map(|value| value.parse().unwrap() )
        .collect();

    quote! {
        use crate::orm::sea_orm_active_enums::*;

        impl async_graphql::resolver_utils::EnumType for #enum_ident {
            fn items() -> &'static [async_graphql::resolver_utils::EnumItem<Self>] {
                [
                    #(Self::#enum_values),*
                ]
            }
        }
    }
}

pub fn write_graphql_enum<P: AsRef<Path>>(path: &P, enum_meta: &EnumMeta) -> std::io::Result<()> {
    let file_name = path.as_ref().join(format!("{}.rs", enum_meta.snake_case()));

    let data = generate_enumeration(enum_meta);

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}