use proc_macro2::TokenStream;
use quote::quote;

use crate::{util::add_line_break, WebFrameworkEnum};

pub fn generate_query_root(
    entities_hashmap: &crate::sea_orm_codegen::EntityHashMap,
) -> Result<TokenStream, crate::error::Error> {
    let items: Vec<_> = entities_hashmap
        .keys()
        .into_iter()
        .filter(|entity| {
            entity.ne(&&"mod.rs".to_string())
                && entity.ne(&&"prelude.rs".to_string())
                && entity.ne(&&"sea_orm_active_enums.rs".to_string())
        })
        .map(|entity| {
            let entity = &entity.as_str()[..entity.len() - 3];
            format!("crate::entities::{}", entity)
        })
        .collect();

    Ok(quote! {
      #[derive(Debug, seaography::macros::QueryRoot)]
      #(#[seaography(entity = #items)])*
      pub struct QueryRoot;
    })
}

pub fn write_query_root<P: AsRef<std::path::Path>>(
    path: &P,
    entities_hashmap: &crate::sea_orm_codegen::EntityHashMap,
) -> Result<(), crate::error::Error> {
    let tokens = generate_query_root(entities_hashmap)?;

    let file_name = path.as_ref().join("query_root.rs");

    std::fs::write(file_name, add_line_break(tokens))?;

    Ok(())
}

pub fn write_cargo_toml<P: AsRef<std::path::Path>>(
    path: &P,
    crate_name: &str,
    sql_library: &str,
    framework: WebFrameworkEnum,
) -> std::io::Result<()> {
    let file_path = path.as_ref().join("Cargo.toml");

    let ver = format!(
        "^{}.{}.0",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR")
    );

    let content = match framework {
        WebFrameworkEnum::Actix => include_str!("./templates/actix_cargo.toml"),
        WebFrameworkEnum::Poem => include_str!("./templates/poem_cargo.toml"),
    };

    let content = content
        .replace("<seaography-package-name>", crate_name)
        .replace("<seaography-sql-library>", sql_library)
        .replace("<seaography-version>", &ver);

    std::fs::write(file_path, content.as_bytes())?;

    Ok(())
}

///
/// Used to generate project/src/lib.rs file content
///
pub fn generate_lib() -> TokenStream {
    quote! {
        use sea_orm::prelude::*;

        pub mod entities;
        pub mod query_root;

        pub use query_root::QueryRoot;

        pub struct OrmDataloader {
            pub db: DatabaseConnection,
        }
    }
}

pub fn write_lib<P: AsRef<std::path::Path>>(path: &P) -> std::io::Result<()> {
    let tokens = generate_lib();

    let file_name = path.as_ref().join("lib.rs");

    std::fs::write(file_name, add_line_break(tokens))?;

    Ok(())
}

pub fn write_env<P: AsRef<std::path::Path>>(
    path: &P,
    db_url: &str,
    depth_limit: Option<usize>,
    complexity_limit: Option<usize>,
) -> std::io::Result<()> {
    let depth_limit = depth_limit.map_or("".into(), |value| value.to_string());
    let complexity_limit = complexity_limit.map_or("".into(), |value| value.to_string());

    let tokens = [
        format!(r#"DATABASE_URL="{}""#, db_url),
        format!(r#"# COMPLEXITY_LIMIT={}"#, depth_limit),
        format!(r#"# DEPTH_LIMIT={}"#, complexity_limit),
    ]
    .join("\n");

    let file_name = path.as_ref().join(".env");

    std::fs::write(file_name, tokens)?;

    Ok(())
}
