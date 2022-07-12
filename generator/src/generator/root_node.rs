use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use seaography_types::{TableMeta};

/// Use to generate project/src/graphql/root_node.rs file content
pub fn generate_root_node(tables_meta: &Vec<TableMeta>) -> TokenStream {
    let pagination_input = generate_pagination_input();

    let paginated_result = generate_paginated_result(tables_meta);

    let single_queries: Vec<TokenStream> = generate_single_queries(tables_meta);

    quote! {
        use super::entities;

        use sea_orm::prelude::*;

        #pagination_input

        #paginated_result

        pub struct QueryRoot;

        #[async_graphql::Object]
        impl QueryRoot {
            #(#single_queries)*
        }
    }
}

/// Used to gather all root_node queries for every entity
pub fn generate_single_queries(tables_meta: &Vec<TableMeta>) -> Vec<TokenStream> {
    tables_meta
        .iter()
        .map(|table_meta: &TableMeta| generate_table_query(table_meta))
        .collect()
}

/// Used to generate a root query for the current table_meta
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::root_node::generate_table_query;
/// use seaography_generator::test_cfg::get_char_table;
///
/// let char_table_meta = get_char_table();
///
/// let left = generate_table_query(&char_table_meta);
///
/// let right = quote!{
///     async fn char<'a>(
///         &self, ctx: &async_graphql::Context<'a>,
///         filters: Option<entities::char::Filter>,
///         pagination: Option<PaginationInput>,
///     ) -> PaginatedResult<entities::char::Model> {
///           println!("filters: {:?}", filters);
///
///           let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
///
///           let stmt = entities::char::Entity::find()
///               .filter(entities::char::filter_recursive(filters));
///
///           if let Some(pagination) = pagination {
///               let paginator = stmt
///                   .paginate(db, pagination.limit);
///
///               let data: Vec<entities::char::Model> = paginator
///                   .fetch_page(pagination.page)
///                   .await
///                   .unwrap();
///
///               let pages = paginator
///                   .num_pages()
///                   .await
///                   .unwrap();
///
///               PaginatedResult {
///                   data,
///                   pages,
///                   current: pagination.page
///               }
///           } else {
///               let data: Vec<entities::char::Model> = stmt
///                   .all(db)
///                   .await
///                   .unwrap();
///
///               PaginatedResult {
///                   data,
///                   pages: 1,
///                   current: 1
///               }
///           }
///       }
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_table_query(table_meta: &TableMeta) -> TokenStream {
    let entity_module = table_meta.snake_case_ident();

    quote! {
        async fn #entity_module<'a>(
            &self, ctx: &async_graphql::Context<'a>,
            filters: Option<entities::#entity_module::Filter>,
            pagination: Option<PaginationInput>,
        ) -> PaginatedResult<entities::#entity_module::Model> {
            println!("filters: {:?}", filters);

            let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();

            let stmt = entities::#entity_module::Entity::find()
                .filter(entities::#entity_module::filter_recursive(filters));

            if let Some(pagination) = pagination {
                let paginator = stmt
                    .paginate(db, pagination.limit);

                let data: Vec<entities::#entity_module::Model> = paginator
                    .fetch_page(pagination.page)
                    .await
                    .unwrap();

                let pages = paginator
                    .num_pages()
                    .await
                    .unwrap();

                PaginatedResult {
                    data,
                    pages,
                    current: pagination.page
                }
            } else {
                let data: Vec<entities::#entity_module::Model> = stmt
                    .all(db)
                    .await
                    .unwrap();

                PaginatedResult {
                    data,
                    pages: 1,
                    current: 1
                }
            }
        }
    }
}

/// Used to generate paginated input struct
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::root_node::generate_pagination_input;
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
/// use seaography_generator::generator::root_node::generate_paginated_result;
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
pub fn generate_paginated_result(tables_meta: &Vec<TableMeta>) -> TokenStream {
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

/// Used to write project/src/graphql/root_node.rs
pub fn write_root_node<P: AsRef<Path>>(
    path: &P,
    tables_meta: &Vec<TableMeta>,
) -> std::io::Result<()> {
    let file_name = path.as_ref().join("root_node.rs");

    let data = generate_root_node(tables_meta);

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}
