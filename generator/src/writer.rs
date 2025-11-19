use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{util::add_line_break, WebFrameworkEnum};

pub fn generate_query_root(entities_path: &Path) -> TokenStream {
    let mut entities_paths: Vec<_> = std::fs::read_dir(entities_path)
        .unwrap()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_file())
        .filter(|r| {
            let name = r.file_stem();

            if let Some(v) = name {
                !v.eq(std::ffi::OsStr::new("prelude"))
                    && !v.eq(std::ffi::OsStr::new("sea_orm_active_enums"))
                    && !v.eq(std::ffi::OsStr::new("mod"))
            } else {
                false
            }
        })
        .collect();
    entities_paths.sort();


    quote! {
        use crate::entities::*;
        use async_graphql::dynamic::*;
        use sea_orm::DatabaseConnection;
        use seaography::{async_graphql, lazy_static::lazy_static, Builder, BuilderContext};

        lazy_static! {
            static ref CONTEXT: BuilderContext = BuilderContext::default();
        }

        pub fn schema(
            database: DatabaseConnection,
            depth: Option<usize>,
            complexity: Option<usize>,
        ) -> Result<Schema, SchemaError> {
            let  builder = Builder::new(&CONTEXT, database.clone());
            let  builder = register_entity_modules(builder);
            let  builder = register_active_enums(builder);

            builder
                .set_depth_limit(depth)
                .set_complexity_limit(complexity)
                .schema_builder()
                .data(database)
        }
    }
}

pub fn write_query_root(src_path: &Path, entities_path: &Path) -> Result<(), crate::error::Error> {
    let tokens = generate_query_root(entities_path);

    let file_name = src_path.join("query_root.rs");

    std::fs::write(file_name, add_line_break(tokens))?;

    Ok(())
}

///
/// Used to generate project/Cargo.toml file content
///
pub fn write_cargo_toml(
    path: &Path,
    crate_name: &str,
    sql_library: &str,
    framework: WebFrameworkEnum,
) -> std::io::Result<()> {
    let file_path = path.join("Cargo.toml");

    let ver = env!("CARGO_PKG_VERSION");

    let content = match framework {
        WebFrameworkEnum::Actix => include_str!("./templates/actix_cargo.toml"),
        WebFrameworkEnum::Poem => include_str!("./templates/poem_cargo.toml"),
        WebFrameworkEnum::Axum => include_str!("./templates/axum_cargo.toml"),
    };

    let content = content
        .replace("<seaography-package-name>", crate_name)
        .replace("<seaography-sql-library>", sql_library)
        .replace("<seaography-version>", ver);

    std::fs::write(file_path, content.as_bytes())?;

    Ok(())
}

///
/// Used to generate project/src/lib.rs file content
///
pub fn write_lib<P: AsRef<std::path::Path>>(path: &P) -> std::io::Result<()> {
    let tokens = quote! {
        pub mod entities;
        pub mod query_root;
    };

    let file_name = path.as_ref().join("lib.rs");

    std::fs::write(file_name, add_line_break(tokens))?;

    Ok(())
}

///
/// Used to generate project/.env file content
///
pub fn write_env(
    path: &Path,
    db_url: &str,
    depth_limit: Option<usize>,
    complexity_limit: Option<usize>,
) -> std::io::Result<()> {
    let depth_limit = depth_limit.map_or("".into(), |value| value.to_string());
    let complexity_limit = complexity_limit.map_or("".into(), |value| value.to_string());

    let tokens = [
        format!(r#"DATABASE_URL="{db_url}""#),
        format!(r#"# COMPLEXITY_LIMIT={complexity_limit}"#),
        format!(r#"# DEPTH_LIMIT={depth_limit}"#),
    ]
    .join("\n");

    let file_name = path.join(".env");

    std::fs::write(file_name, tokens)?;

    Ok(())
}