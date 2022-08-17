use proc_macro2::TokenStream;
use quote::quote;
use seaography_types::TableMeta;

/// Used to generate paginated input struct
///
/// ```
/// use quote::quote;
/// use seaography_generator::core::generate_pagination_input;
///
/// let left = generate_pagination_input();
///
/// let right = quote!{
///     #[derive(async_graphql::InputObject, Debug)]
///     pub struct PaginationInput {
///         pub limit: usize,
///         pub page: usize,
///     }
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_pagination_input() -> TokenStream {
    quote! {
        #[derive(async_graphql::InputObject, Debug)]
        pub struct PaginationInput {
            pub limit: usize,
            pub page: usize,
        }
    }
}

/// Used to generate paginated result struct for all entities
///
/// ```
/// use quote::quote;
/// use seaography_generator::core::generate_paginated_result;
/// use seaography_generator::test_cfg::get_tables_meta;
///
/// let left = generate_paginated_result(&get_tables_meta());
///
/// let right = quote!{
///     #[derive(async_graphql::SimpleObject, Debug)]
///     #[graphql(concrete(name = "PaginatedCharResult", params(entities::char::Model)))]
///     #[graphql(concrete(name = "PaginatedFontResult", params(entities::font::Model)))]
///     pub struct PaginatedResult<T: async_graphql::ObjectType> {
///         pub data: Vec<T>,
///         pub pages: usize,
///         pub current: usize,
///     }
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_paginated_result(tables_meta: &[TableMeta]) -> TokenStream {
    let derives: Vec<TokenStream> = tables_meta
        .iter()
        .map(|table_meta: &TableMeta| {
            let name = format!("Paginated{}Result", table_meta.camel_case());
            let module = table_meta.snake_case_ident();
            quote! {
                #[graphql(concrete(name = #name, params(entities::#module::Model)))]
            }
        })
        .collect();

    quote! {
        #[derive(async_graphql::SimpleObject, Debug)]
        #(#derives)*
        pub struct PaginatedResult<T: async_graphql::ObjectType> {
            pub data: Vec<T>,
            pub pages: usize,
            pub current: usize,
        }
    }
}