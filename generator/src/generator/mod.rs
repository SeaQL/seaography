use std::path::Path;
use proc_macro2::TokenStream;

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

/// Used to generate graphql folder with all entities, enums and module structure
pub fn write_graphql<P: AsRef<Path>>(
    path: &P,
    tables_meta: &Vec<TableMeta>,
    enums_meta: &Vec<EnumMeta>,
) -> std::io::Result<()> {
    if !enums_meta.is_empty() {
        std::fs::create_dir_all(&path.as_ref().join("enums"))?;
        for enum_meta in enums_meta.iter() {
            write_graphql_enum(&path.as_ref().join("enums"), enum_meta)?;
        }
        write_enums_mod(path, enums_meta)?;
    }

    std::fs::create_dir_all(&path.as_ref().join("entities"))?;
    for table_meta in tables_meta.iter() {
        write_graphql_entity(&path.as_ref().join("entities"), table_meta)?;
    }
    write_entities_mod(path, tables_meta)?;

    write_orm_dataloader(path)?;

    write_root_node(path, tables_meta)?;

    write_type_filter(path)?;

    write_mod(path, enums_meta)?;

    Ok(())
}

/// Used to write project/src/graphql/mod.rs
pub fn write_mod<P: AsRef<Path>>(path: &P, enums_meta: &Vec<EnumMeta>,) -> std::io::Result<()> {
    let mod_tokens = generate_graphql_mod(enums_meta.len());

    std::fs::write(path.as_ref().join("mod.rs"), mod_tokens.to_string())?;

    Ok(())
}

/// Used to generate project/src/graphql/mod.rs file content
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::generate_graphql_mod;
///
/// let left = generate_graphql_mod(0);
///
/// let right = quote!{
///         pub mod entities;
///         pub mod root_node;
///         pub mod type_filter;
///         pub mod orm_dataloader;
///         pub use root_node::QueryRoot;
///         pub use type_filter::TypeFilter;
///         pub use orm_dataloader::OrmDataloader;
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::generate_graphql_mod;
///
/// let left = generate_graphql_mod(1);
///
/// let right = quote!{
///         pub mod entities;
///         pub mod enums;
///         pub mod root_node;
///         pub mod type_filter;
///         pub mod orm_dataloader;
///         pub use root_node::QueryRoot;
///         pub use type_filter::TypeFilter;
///         pub use orm_dataloader::OrmDataloader;
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_graphql_mod(enums_meta_len: usize) -> TokenStream {
    let enums_mod = if enums_meta_len > 0 {
        quote!{
            pub mod enums;
        }
    } else {
        quote!{}
    };

    quote! {
        pub mod entities;
        #enums_mod
        pub mod root_node;
        pub mod type_filter;
        pub mod orm_dataloader;
        pub use root_node::QueryRoot;
        pub use type_filter::TypeFilter;
        pub use orm_dataloader::OrmDataloader;
    }
}

/// Used to write project/src/graphql/enums/mod.rs
pub fn write_enums_mod<P: AsRef<Path>>(
    path: &P,
    enums_meta: &Vec<EnumMeta>,
) -> std::io::Result<()> {
    let mod_tokens = generate_enums_mod(enums_meta);

    std::fs::write(path.as_ref().join("enums/mod.rs"), mod_tokens.to_string())?;

    Ok(())
}

/// Used to generate project/src/graphql/enums/mod.rs file content
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::generate_enums_mod;
/// use seaography_types::EnumMeta;
///
/// let left = generate_enums_mod(&vec![
///     EnumMeta {
///         enum_name: "BucketSize".into(),
///         enum_values: vec!["small".into(), "medium".into(), "large".into()]
///     }
/// ]);
///
/// let right = quote!{
///     pub mod bucket_size;
///     pub use bucket_size::*;
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_enums_mod(enums_meta: &[EnumMeta]) -> TokenStream {
    let enum_names: Vec<proc_macro2::TokenStream> = enums_meta
        .iter()
        .map(|enumeration| enumeration.snake_case().parse().unwrap())
        .collect();

    quote! {
        #(pub mod #enum_names;)*

        #(pub use #enum_names::*;)*
    }
}

/// Used to write project/src/graphql/entities/mod.rs
pub fn write_entities_mod<P: AsRef<Path>>(
    path: &P,
    tables_meta: &Vec<TableMeta>,
) -> std::io::Result<()> {
    let mod_tokens = generate_entities_mod(tables_meta);

    std::fs::write(
        path.as_ref().join("entities/mod.rs"),
        mod_tokens.to_string(),
    )?;

    Ok(())
}

/// Used to generate project/src/graphql/entities/mod.rs file content
///
/// ```
/// use quote::quote;
/// use seaography_generator::generator::generate_entities_mod;
/// use seaography_types::TableMeta;
///
/// let left = generate_entities_mod(&vec![
///     TableMeta {
///         table_name: "Users".into(),
///         relations: vec![],
///         columns: vec![]
///     },
///     TableMeta {
///         table_name: "ConsumerProducts".into(),
///         relations: vec![],
///         columns: vec![]
///     }
/// ]);
///
/// let right = quote!{
///     pub mod users;
///     pub mod consumer_products;
/// };
///
/// assert_eq!(left.to_string(), right.to_string());
/// ```
pub fn generate_entities_mod(tables_meta: &Vec<TableMeta>) -> TokenStream {
    let entity_names: Vec<proc_macro2::TokenStream> = tables_meta
        .iter()
        .map(|table_meta| table_meta.snake_case_ident())
        .collect();

    quote! {
        #(pub mod #entity_names;)*
    }
}
