use heck::ToUpperCamelCase;
use std::path::Path;

use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use seaography_types::{
    column_meta::ColumnMeta, column_type::ColumnType, relationship_meta::RelationshipMeta,
    table_meta::TableMeta,
};

pub fn generate_entity(table_meta: &TableMeta) -> TokenStream {
    quote!{}
    // TODO: inject derive into sea orm cli generated code
}

pub fn write_graphql_entity<P: AsRef<Path>>(
    path: &P,
    table_meta: &TableMeta,
) -> std::io::Result<()> {
    let file_name = path
        .as_ref()
        .join(format!("{}.rs", table_meta.snake_case()));

    let data = generate_entity(table_meta);

    std::fs::write(file_name, data.to_string())?;

    Ok(())
}
