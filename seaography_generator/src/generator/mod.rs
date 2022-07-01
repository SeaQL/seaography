use std::path::Path;

use quote::quote;
use seaography_types::enum_meta::EnumMeta;
use seaography_types::table_meta::TableMeta;

use self::{
    entity::write_graphql_entity, enumeration::write_graphql_enum,
    orm_dataloader::write_orm_dataloader, root_node::write_root_node,
    type_filter::write_type_filter,
};

pub mod entity;
pub mod enumeration;
pub mod orm_dataloader;
pub mod root_node;
pub mod toml;
pub mod type_filter;

pub fn write_graphql<P: AsRef<Path>>(
    path: &P,
    tables_meta: &Vec<TableMeta>,
    enums_meta: &Vec<EnumMeta>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&path.as_ref().join("enums"))?;
    for enum_meta in enums_meta.iter() {
        write_graphql_enum(&path.as_ref().join("enums"), enum_meta)?;
    }
    write_enums_mod(path, enums_meta)?;

    std::fs::create_dir_all(&path.as_ref().join("entities"))?;
    for table_meta in tables_meta.iter() {
        write_graphql_entity(&path.as_ref().join("entities"), table_meta)?;
    }
    write_entities_mod(path, tables_meta)?;

    write_orm_dataloader(path)?;

    write_root_node(path, tables_meta)?;

    write_type_filter(path)?;

    write_mod(path)?;

    Ok(())
}

pub fn write_mod<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    let mod_tokens = quote! {
        pub mod entities;
        pub mod root_node;
        pub mod type_filter;
        pub mod orm_dataloader;
        pub use root_node::QueryRoot;
        pub use type_filter::TypeFilter;
        pub use orm_dataloader::OrmDataloader;
    };

    std::fs::write(path.as_ref().join("mod.rs"), mod_tokens.to_string())?;

    Ok(())
}

pub fn write_enums_mod<P: AsRef<Path>>(
    path: &P,
    enums_meta: &Vec<EnumMeta>,
) -> std::io::Result<()> {
    let enum_names: Vec<proc_macro2::TokenStream> = enums_meta
        .iter()
        .map(|enumeration| enumeration.snake_case().parse().unwrap())
        .collect();

    let mod_tokens = quote! {
        #(pub mod #enum_names;)*

        #(pub use #enum_names::*;)*
    };

    std::fs::write(path.as_ref().join("enums/mod.rs"), mod_tokens.to_string())?;

    Ok(())
}

pub fn write_entities_mod<P: AsRef<Path>>(
    path: &P,
    tables_meta: &Vec<TableMeta>,
) -> std::io::Result<()> {
    let entity_names: Vec<proc_macro2::TokenStream> = tables_meta
        .iter()
        .map(|table_meta| table_meta.snake_case_ident())
        .collect();

    let mod_tokens = quote! {
        #(pub mod #entity_names;)*
    };

    std::fs::write(
        path.as_ref().join("entities/mod.rs"),
        mod_tokens.to_string(),
    )?;

    Ok(())
}
