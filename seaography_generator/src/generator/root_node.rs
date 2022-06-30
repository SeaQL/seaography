use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use seaography_types::{column_meta::ColumnMeta, table_meta::TableMeta};

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

pub fn generate_single_queries(tables_meta: &Vec<TableMeta>) -> Vec<TokenStream> {
    tables_meta
        .iter()
        .map(|table_meta: &TableMeta| {
            let entity_module = table_meta.snake_case_ident();

            let filter_recursive = generate_recursive_filter_fn(table_meta);

            quote! {
                async fn #entity_module<'a>(
                    &self, ctx: &async_graphql::Context<'a>,
                    filters: Option<entities::#entity_module::Filter>,
                    pagination: Option<PaginationInput>,
                ) -> PaginatedResult<entities::#entity_module::Model> {
                    println!("filters: {:?}", filters);

                    #filter_recursive

                    let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();

                    let stmt = entities::#entity_module::Entity::find()
                        .filter(filter_recursive(filters));

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
        })
        .collect()
}

pub fn generate_recursive_filter_fn(table_meta: &TableMeta) -> TokenStream {
    let entity_module = table_meta.snake_case_ident();

    let columns_filters: Vec<TokenStream> = table_meta
        .columns
        .iter()
        .map(|column: &ColumnMeta| {
            let column_name = column.snake_case_ident();
            let column_enum_name = column.camel_case_ident();

            quote! {
                if let Some(#column_name) = current_filter.#column_name {
                    if let Some(eq_value) = #column_name.eq {
                        condition = condition.add(entities::#entity_module::Column::#column_enum_name.eq(eq_value))
                    }

                    if let Some(ne_value) = #column_name.ne {
                        condition = condition.add(entities::#entity_module::Column::#column_enum_name.ne(ne_value))
                    }
                }
            }
        })
        .collect();

    quote! {
        fn filter_recursive(root_filter: Option<entities::#entity_module::Filter>) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();

            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters
                        .into_iter()
                        .fold(
                            sea_orm::Condition::any(),
                            |fold_condition, filter| fold_condition.add(filter_recursive(Some(*filter)))
                        );
                    condition = condition.add(or_condition);
                }

                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters
                        .into_iter()
                        .fold(
                            sea_orm::Condition::all(),
                            |fold_condition, filter| fold_condition.add(filter_recursive(Some(*filter)))
                        );
                    condition = condition.add(and_condition);
                }

                #(#columns_filters)*
            }

            condition
        }
    }
}

pub fn generate_pagination_input() -> TokenStream {
    quote! {
        #[derive(async_graphql::InputObject, Debug)]
        pub struct PaginationInput {
            pub limit: usize,
            pub page: usize,
        }
    }
}

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

pub fn write_root_node<P: AsRef<Path>>(
    path: &P,
    tables_meta: &Vec<TableMeta>,
) -> std::io::Result<()> {
    let file_name = path.as_ref().join("root_node.rs");

    let data = generate_root_node(tables_meta);

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}
