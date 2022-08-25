use proc_macro2::TokenStream;
use quote::{quote, format_ident};

pub fn enum_filter_fn(ident: syn::Ident) -> TokenStream {
    let name = format_ident!("{}EnumFilter", ident);

    quote!{
        #[derive(Debug, async_graphql::InputObject)]
        pub struct #name {
            pub eq: Option<#ident>,
            pub ne: Option<#ident>,
            pub gt: Option<#ident>,
            pub gte: Option<#ident>,
            pub lt: Option<#ident>,
            pub lte: Option<#ident>,
            pub is_in: Option<Vec<#ident>>,
            pub is_not_in: Option<Vec<#ident>>,
            pub is_null: Option<bool>,
        }
    }
}