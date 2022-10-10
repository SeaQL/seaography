use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

#[derive(Debug, Eq, PartialEq, bae::FromAttributes)]
pub struct Seaography {
    entity: syn::Lit,
}

pub fn root_query_fn(
    ident: &syn::Ident,
    attrs: &[Seaography],
) -> Result<TokenStream, crate::error::Error> {
    let paths = attrs
        .iter()
        .map(|attribute| -> Result<TokenStream, crate::error::Error> {
            if let syn::Lit::Str(item) = &attribute.entity {
                Ok(item.value().parse::<TokenStream>()?)
            } else {
                Err(crate::error::Error::Internal(
                    "Unreachable parse of query entities".into(),
                ))
            }
        })
        .collect::<Result<Vec<TokenStream>, crate::error::Error>>()?;

    let queries: Vec<TokenStream> = paths
        .iter()
        .map(|path| {
            let name = format_ident!("{}", path.clone().into_iter().last().unwrap().to_string());

            let basic_query = basic_query(&name, path);

            quote! {
                #basic_query
            }
        })
        .collect();

    let basic_dependencies = basic_dependencies();

    Ok(quote! {
        #basic_dependencies

        #[async_graphql::Object]
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
            pagination: Option<Pagination>,
            order_by: Option<#path::OrderBy>,
        ) -> async_graphql::types::connection::Connection<String, #path::Model, ExtraPaginationFields, async_graphql::types::connection::EmptyFields> {
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
                pages: Option<usize>,
                current: Option<usize>
            ) -> async_graphql::types::connection::Connection<
                String,
                #path::Model,
                ExtraPaginationFields,
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

                        let cursor_string = CursorValues(values).encode_cursor();

                        async_graphql::types::connection::Edge::new(cursor_string, node)
                    })
                    .collect();

                let mut result = async_graphql::types::connection::Connection::<
                    String,
                    #path::Model,
                    ExtraPaginationFields,
                    async_graphql::types::connection::EmptyFields
                >::with_additional_fields(
                    has_previous_page,
                    has_next_page,
                    ExtraPaginationFields {
                        pages,
                        current
                    }
                );

                result.edges.extend(edges);

                result
            }

            if let Some(pagination) = pagination {

                match pagination {
                    Pagination::Pages(pagination) => {
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
                    Pagination::Cursor(cursor) => {
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
                            let values = CursorValues::decode_cursor(cursor_string.as_str()).unwrap();

                            let cursor_values: sea_orm::sea_query::value::ValueTuple = map_cursor_values(values.0);

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

                                let values = map_cursor_values(values);

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

                                let values = map_cursor_values(values);

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

pub fn basic_dependencies() -> TokenStream {
    quote! {
        #[derive(Debug, async_graphql::InputObject)]
        pub struct PageInput {
            pub limit: usize,
            pub page: usize,
        }

        #[derive(Debug, async_graphql::InputObject)]
        pub struct CursorInput {
            pub cursor: Option<String>,
            pub limit: u64,
        }

        #[derive(async_graphql::OneofObject)]
        pub enum Pagination {
            Pages(PageInput),
            Cursor(CursorInput),
        }


        #[derive(async_graphql::SimpleObject)]
        pub struct ExtraPaginationFields {
            pub pages: Option<usize>,
            pub current: Option<usize>,
        }

        #[derive(Debug)]
        pub enum DecodeMode {
            Type,
            Length,
            ColonSkip,
            Data,
        }

        pub fn map_cursor_values(values: Vec<sea_orm::Value>) -> sea_orm::sea_query::value::ValueTuple {
            use seaography::itertools::Itertools;

            if values.len() == 1 {
                sea_orm::sea_query::value::ValueTuple::One(values[0].clone())
            } else if values.len() == 2 {
                sea_orm::sea_query::value::ValueTuple::Two(values[0].clone(), values[1].clone())
            } else if values.len() == 3 {
                sea_orm::sea_query::value::ValueTuple::Three(values[0].clone(), values[1].clone(), values[2].clone())
            } else {
                panic!("seaography does not support cursors values with size greater than 3")
            }
        }

        #[derive(Debug)]
        pub struct CursorValues(pub Vec<sea_orm::Value>);

        impl async_graphql::types::connection::CursorType for CursorValues {
            type Error = String;

            fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
                let chars = s.chars();

                let mut values: Vec<sea_orm::Value> = vec![];

                let mut type_indicator = String::new();
                let mut length_indicator = String::new();
                let mut data_buffer = String::new();
                let mut length = -1;

                let mut mode: DecodeMode = DecodeMode::Type;
                for char in chars {
                    match mode {
                        DecodeMode::Type => {
                            if char.eq(&'[') {
                                mode = DecodeMode::Length;
                            } else if char.eq(&',') {
                                // SKIP
                            } else {
                                type_indicator.push(char);
                            }
                        },
                        DecodeMode::Length => {
                            if char.eq(&']') {
                                mode = DecodeMode::ColonSkip;
                                length = length_indicator.parse::<i64>().unwrap();
                            } else {
                                length_indicator.push(char);
                            }
                        },
                        DecodeMode::ColonSkip => {
                            // skips ':' char
                            mode = DecodeMode::Data;
                        },
                        DecodeMode::Data => {
                            if length > 0 {
                                data_buffer.push(char);
                                length -= 1;
                            }

                            if length <= 0{
                                let value: sea_orm::Value = match type_indicator.as_str() {
                                    "TinyInt" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::TinyInt(None)
                                        } else {
                                            sea_orm::Value::TinyInt(Some(data_buffer.parse::<i8>().unwrap()))
                                        }
                                    },
                                    "SmallInt" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::SmallInt(None)
                                        } else {
                                            sea_orm::Value::SmallInt(Some(data_buffer.parse::<i16>().unwrap()))
                                        }
                                    },
                                    "Int" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::Int(None)
                                        } else {
                                            sea_orm::Value::Int(Some(data_buffer.parse::<i32>().unwrap()))
                                        }
                                    },
                                    "BigInt" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::BigInt(None)
                                        } else {
                                            sea_orm::Value::BigInt(Some(data_buffer.parse::<i64>().unwrap()))
                                        }
                                    },
                                    "TinyUnsigned" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::TinyUnsigned(None)
                                        } else {
                                            sea_orm::Value::TinyUnsigned(Some(data_buffer.parse::<u8>().unwrap()))
                                        }
                                    },
                                    "SmallUnsigned" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::SmallUnsigned(None)
                                        } else {
                                            sea_orm::Value::SmallUnsigned(Some(data_buffer.parse::<u16>().unwrap()))
                                        }
                                    },
                                    "Unsigned" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::Unsigned(None)
                                        } else {
                                            sea_orm::Value::Unsigned(Some(data_buffer.parse::<u32>().unwrap()))
                                        }
                                    },
                                    "BigUnsigned" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::BigUnsigned(None)
                                        } else {
                                            sea_orm::Value::BigUnsigned(Some(data_buffer.parse::<u64>().unwrap()))
                                        }
                                    },
                                    "String" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::String(None)
                                        } else {
                                            sea_orm::Value::String(Some(Box::new(data_buffer.parse::<String>().unwrap())))
                                        }
                                    },
                                    "Uuid" => {
                                        if length.eq(&-1) {
                                            sea_orm::Value::Uuid(None)
                                        } else {
                                            sea_orm::Value::Uuid(Some(Box::new(data_buffer.parse::<sea_orm::prelude::Uuid>().unwrap())))
                                        }
                                    },
                                    _ => {
                                        // FIXME: missing value types
                                        panic!("cannot encode current type")
                                    },
                                };

                                values.push(value);

                                type_indicator = String::new();
                                length_indicator = String::new();
                                data_buffer = String::new();
                                length = -1;

                                mode = DecodeMode::Type;
                            }
                        }
                    }
                }

                Ok(Self(values))
            }

            fn encode_cursor(&self) -> String {
                use seaography::itertools::Itertools;

                self.0.iter().map(|value| -> String {
                    match value {
                        sea_orm::Value::TinyInt(value) => {
                            if let Some(value) = value {
                                let value = value.to_string();
                                format!("TinyInt[{}]:{}", value.len(), value)
                            } else {
                                format!("TinyInt[-1]:")
                            }
                        },
                        sea_orm::Value::SmallInt(value) => {
                            if let Some(value) = value {
                                let value = value.to_string();
                                format!("SmallInt[{}]:{}", value.len(), value)
                            } else {
                                format!("SmallInt[-1]:")
                            }
                        },
                        sea_orm::Value::Int(value) => {
                            if let Some(value) = value {
                                let value = value.to_string();
                                format!("Int[{}]:{}", value.len(), value)
                            } else {
                                format!("Int[-1]:")
                            }
                        },
                        sea_orm::Value::BigInt(value) => {
                            if let Some(value) = value {
                                let value = value.to_string();
                                format!("BigInt[{}]:{}", value.len(), value)
                            } else {
                                format!("BigInt[-1]:")
                            }
                        },
                        sea_orm::Value::TinyUnsigned(value) => {
                            if let Some(value) = value {
                                let value = value.to_string();
                                format!("TinyUnsigned[{}]:{}", value.len(), value)
                            } else {
                                format!("TinyUnsigned[-1]:")
                            }
                        },
                        sea_orm::Value::SmallUnsigned(value) => {
                            if let Some(value) = value {
                                let value = value.to_string();
                                format!("SmallUnsigned[{}]:{}", value.len(), value)
                            } else {
                                format!("SmallUnsigned[-1]:")
                            }
                        },
                        sea_orm::Value::Unsigned(value) => {
                            if let Some(value) = value {
                                let value = value.to_string();
                                format!("Unsigned[{}]:{}", value.len(), value)
                            } else {
                                format!("Unsigned[-1]:")
                            }
                        },
                        sea_orm::Value::BigUnsigned(value) => {
                            if let Some(value) = value {
                                let value = value.to_string();
                                format!("BigUnsigned[{}]:{}", value.len(), value)
                            } else {
                                format!("BigUnsigned[-1]:")
                            }
                        },
                        sea_orm::Value::String(value) => {
                            if let Some(value) = value {
                                let value = value.as_ref();
                                format!("String[{}]:{}", value.len(), value)
                            } else {
                                format!("String[-1]:")
                            }
                        },
                        sea_orm::Value::Uuid(value) => {
                            if let Some(value) = value {
                                let value = value.as_ref().to_string();
                                format!("Uuid[{}]:{}", value.len(), value)
                            } else {
                                format!("Uuid[-1]:")
                            }
                        },
                        _ => {
                            // FIXME: missing value types
                            panic!("cannot
                             current type")
                        },
                    }
                })
                .join(",")
            }
        }
    }
}