use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

#[derive(Debug, Eq, PartialEq, bae::FromAttributes, Clone)]
pub struct Seaography {
    entity: Option<syn::Lit>,
    object_config: Option<syn::Expr>,
}

pub fn root_query_fn(
    ident: &syn::Ident,
    attrs: &[Seaography],
) -> Result<TokenStream, crate::error::Error> {
    let paths = attrs
        .iter()
        .filter(|attribute| matches!(&attribute.entity, Some(_)))
        .map(
            |attribute| -> Result<(TokenStream, TokenStream), crate::error::Error> {
                let entity_name = if let syn::Lit::Str(item) = attribute.entity.as_ref().unwrap() {
                    Ok(item.value().parse::<TokenStream>()?)
                } else {
                    Err(crate::error::Error::Internal(
                        "Unreachable parse of query entities".into(),
                    ))
                }?;

                let config = if let Some(config) = &attribute.object_config {
                    quote! {
                        #[graphql(#config)]
                    }
                } else {
                    quote! {}
                };

                Ok((entity_name, config))
            },
        )
        .collect::<Result<Vec<(TokenStream, TokenStream)>, crate::error::Error>>()?;

    let object_config = attrs
        .iter()
        .find(|attribute| matches!(attribute.object_config, Some(_)))
        .map(|attribute| attribute.object_config.as_ref().unwrap());

    let implement_macros = match object_config {
        Some(object_config) => {
            quote! {
                #[async_graphql::Object(#object_config)]
            }
        }
        _ => {
            quote! {
                #[async_graphql::Object]
            }
        }
    };

    let queries: Vec<TokenStream> = paths
        .iter()
        .map(|(path, config)| {
            let name = format_ident!("{}", path.clone().into_iter().last().unwrap().to_string());

            let basic_query = basic_query(&name, path);

            quote! {
                #config
                #basic_query
            }
        })
        .collect();

    Ok(quote! {
        #implement_macros
        impl #ident {
            #(#queries)*
        }
    })
}

pub fn basic_query(name: &Ident, path: &TokenStream) -> TokenStream {
    quote! {
        pub async fn #name<'a>(
            &self,
            ctx: &async_graphql::Context<'a>,
            filters: Option<#path::Filter>,
            pagination: Option<seaography::Pagination>,
            order_by: Option<#path::OrderBy>,
        ) -> async_graphql::types::connection::Connection<String, #path::Model, seaography::ExtraPaginationFields, async_graphql::types::connection::EmptyFields> {
            use sea_orm::prelude::*;
            use sea_orm::Iterable;
            use seaography::itertools::Itertools;
            use async_graphql::types::connection::CursorType;

            println!("filters: {:?}", filters);

            let db: &crate::DatabaseConnection = ctx.data::<crate::DatabaseConnection>().unwrap();
            let stmt = #path::Entity::find()
                .filter(#path::filter_recursive(filters));

            let stmt = #path::order_by(stmt, order_by);

            fn get_result(
                data: Vec<#path::Model>,
                has_previous_page: bool,
                has_next_page: bool,
                pages: Option<u64>,
                current: Option<u64>
            ) -> async_graphql::types::connection::Connection<
                String,
                #path::Model,
                seaography::ExtraPaginationFields,
                async_graphql::types::connection::EmptyFields
            > {
                let edges: Vec<async_graphql::types::connection::Edge<String, #path::Model, async_graphql::types::connection::EmptyFields>> = data
                    .into_iter()
                    .map(|node| {
                        let values: Vec<sea_orm::Value> = #path::PrimaryKey::iter()
                            .map(|variant| {
                                node.get(variant.into_column())
                            })
                            .collect();

                        let cursor_string = seaography::CursorValues(values).encode_cursor();

                        async_graphql::types::connection::Edge::new(cursor_string, node)
                    })
                    .collect();

                let mut result = async_graphql::types::connection::Connection::<
                    String,
                    #path::Model,
                    seaography::ExtraPaginationFields,
                    async_graphql::types::connection::EmptyFields
                >::with_additional_fields(
                    has_previous_page,
                    has_next_page,
                    seaography::ExtraPaginationFields {
                        pages,
                        current
                    }
                );

                result.edges.extend(edges);

                result
            }

            if let Some(pagination) = pagination {

                match pagination {
                    seaography::Pagination::Pages(pagination) => {
                        let paginator = stmt.paginate(db, pagination.limit);

                        let data: Vec<#path::Model> = paginator
                            .fetch_page(pagination.page)
                            .await
                            .unwrap();

                        let pages = paginator
                            .num_pages()
                            .await
                            .unwrap();

                        get_result(data, pagination.page != 1, pagination.page < pages, Some(pages), Some(pagination.page))
                    },
                    seaography::Pagination::Cursor(cursor) => {
                        let next_stmt = stmt.clone();
                        let previous_stmt = stmt.clone();

                        fn apply_stmt_cursor_by(stmt: sea_orm::entity::prelude::Select<#path::Entity>) -> sea_orm::Cursor<sea_orm::SelectModel<#path::Model>> {
                            if #path::PrimaryKey::iter().len() == 1 {
                                let column = #path::PrimaryKey::iter().map(|variant| variant.into_column()).collect::<Vec<#path::Column>>()[0];
                                stmt.cursor_by(column)
                            } else if #path::PrimaryKey::iter().len() == 2 {
                                let columns = #path::PrimaryKey::iter().map(|variant| variant.into_column()).collect_tuple::<(#path::Column, #path::Column)>().unwrap();
                                stmt.cursor_by(columns)
                            } else if #path::PrimaryKey::iter().len() == 3 {
                                let columns = #path::PrimaryKey::iter().map(|variant| variant.into_column()).collect_tuple::<(#path::Column, #path::Column, #path::Column)>().unwrap();
                                stmt.cursor_by(columns)
                            } else {
                                panic!("seaography does not support cursors with size greater than 3")
                            }
                        }

                        let mut stmt = apply_stmt_cursor_by(stmt);

                        if let Some(cursor_string) = cursor.cursor {
                            let values = seaography::CursorValues::decode_cursor(cursor_string.as_str()).unwrap();

                            let cursor_values: sea_orm::sea_query::value::ValueTuple = seaography::map_cursor_values(values.0);

                            stmt.after(cursor_values);
                        }

                        let data = stmt
                            .first(cursor.limit)
                            .all(db)
                            .await
                            .unwrap();

                        let has_next_page: bool = {
                            let mut next_stmt = apply_stmt_cursor_by(next_stmt);

                            let last_node = data.last();

                            if let Some(node) = last_node {
                                let values: Vec<sea_orm::Value> = #path::PrimaryKey::iter()
                                    .map(|variant| {
                                        node.get(variant.into_column())
                                    })
                                    .collect();

                                let values = seaography::map_cursor_values(values);

                                let next_data = next_stmt
                                    .first(cursor.limit)
                                    .after(values)
                                    .all(db)
                                    .await
                                    .unwrap();

                                next_data.len() != 0
                            } else {
                                false
                            }
                        };

                        let has_previous_page: bool = {
                            let mut previous_stmt = apply_stmt_cursor_by(previous_stmt);

                            let first_node = data.first();

                            if let Some(node) = first_node {
                                let values: Vec<sea_orm::Value> = #path::PrimaryKey::iter()
                                    .map(|variant| {
                                        node.get(variant.into_column())
                                    })
                                    .collect();

                                let values = seaography::map_cursor_values(values);

                                let previous_data = previous_stmt
                                    .first(cursor.limit)
                                    .before(values)
                                    .all(db)
                                    .await
                                    .unwrap();

                                previous_data.len() != 0
                            } else {
                                false
                            }
                        };

                        get_result(data, has_previous_page, has_next_page, None, None)
                    }
                }
            } else {
                let data: Vec<#path::Model> = stmt.all(db).await.unwrap();

                get_result(data, false, false, Some(1), Some(1))
            }
        }
    }
}
