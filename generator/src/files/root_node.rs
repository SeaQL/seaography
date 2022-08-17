use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;
use seaography_types::TableMeta;

/// Use to generate project/src/graphql/root_node.rs file content
pub fn generate_root_node_file(tables_meta: &[TableMeta]) -> TokenStream {
    let pagination_input = crate::core::generate_pagination_input();

    let paginated_result = crate::core::generate_paginated_result(tables_meta);

    let root_node = generate_root_node(tables_meta);

    quote! {
        use super::entities;

        use sea_orm::prelude::*;

        #pagination_input

        #paginated_result

        #root_node
    }
}

pub fn generate_root_node(tables_meta: &[TableMeta]) -> TokenStream {
    let queries: Vec<TokenStream> = tables_meta
        .iter()
        .map(|table_meta| {
            let entity_module = table_meta.snake_case_ident();

            quote!{
                seaography_derive::register_entity(#entity_module);
            }
        })
        .collect();

    quote! {
        pub struct QueryRoot;

        #[async_graphql::Object]
        impl QueryRoot {
            #(#queries)*
        }
    }
}


/// Used to write project/src/graphql/root_node.rs
pub fn write_root_node<P: AsRef<Path>>(path: &P, tables_meta: &[TableMeta]) -> std::io::Result<()> {
    let file_name = path.as_ref().join("root_node.rs");

    let data = generate_root_node_file(tables_meta);

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}
