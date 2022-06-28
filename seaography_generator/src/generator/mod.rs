use std::path::Path;

use quote::quote;
use seaography_types::table_meta::TableMeta;

use self::{type_filter::write_type_filter, orm_dataloader::write_orm_dataloader, entity::write_graphql_entity, root_node::write_root_node};

pub mod toml;
pub mod entity;
pub mod orm_dataloader;
pub mod root_node;
pub mod type_filter;

pub fn write_graphql<P: AsRef<Path>>(path: &P, tables_meta: &Vec<TableMeta>) -> std::io::Result<()> {

    for table_meta in tables_meta.iter() {
        write_graphql_entity(&path.as_ref().join("/entities"), table_meta)?;
    }
    write_root_node(path, tables_meta)?;

    write_orm_dataloader(path)?;

    write_type_filter(path)?;

    write_mod(path)?;

    Ok(())
}

pub fn write_mod<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    let mod_tokens = quote!{
        pub mod entities;
        pub mod query_root;
        pub mod type_filter;
        pub mod orm_data_loader;
        pub use query_root::QueryRoot;
        pub use type_filter::TypeFilter;
        pub use orm_data_loader::OrmDataLoader;
    };

    std::fs::write(path.as_ref().join("mod.rs"), mod_tokens.to_string())?;

    Ok(())
}

pub fn write_entities_mod<P: AsRef<Path>>(path: &P, tables_meta: &Vec<TableMeta>) -> std::io::Result<()> {
    let entity_names: Vec<proc_macro2::TokenStream> = tables_meta.iter().map(|table_meta| table_meta.snake_case_ident()).collect();

    let mod_tokens = quote!{
        #(pub use #entity_names;)*
    };

    std::fs::write(path.as_ref().join("entities/mod.rs"), mod_tokens.to_string())?;

    Ok(())
}