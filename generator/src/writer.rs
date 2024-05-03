use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    parser::{parse_entity, parse_enumerations, EntityDefinition},
    util::add_line_break,
    WebFrameworkEnum,
};

pub fn generate_query_root<P: AsRef<Path>>(entities_path: &P) -> TokenStream {
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

    let entities: Vec<EntityDefinition> = entities_paths
        .into_iter()
        .map(|path| {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            parse_entity(file_name.into())
        })
        .collect();

    let entities: Vec<TokenStream> = entities
        .iter()
        .map(|entity| {
            let entity_path = &entity.name;

            quote! {
                #entity_path
            }
        })
        .collect();

    let enumerations = std::fs::read_dir(entities_path)
        .unwrap()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .find(|r| {
            let name = r.file_stem();

            if let Some(v) = name {
                v.eq(std::ffi::OsStr::new("sea_orm_active_enums"))
            } else {
                false
            }
        });

    let enumerations = match enumerations {
        Some(_) => {
            let file_content =
                std::fs::read_to_string(entities_path.as_ref().join("sea_orm_active_enums.rs"))
                    .unwrap();

            parse_enumerations(file_content)
        }
        None => vec![],
    };

    let enumerations = enumerations.iter().map(|definition| {
        let name = &definition.name;

        quote! {
            builder.register_enumeration::<crate::entities::sea_orm_active_enums::#name>();
        }
    });

    quote! {
        use crate::entities::*;
        use async_graphql::dynamic::*;
        use sea_orm::DatabaseConnection;
        use seaography::{Builder, BuilderContext};

        lazy_static::lazy_static! {
            static ref CONTEXT: BuilderContext = BuilderContext::default();
        }

        pub fn schema(
            database: DatabaseConnection,
            depth: Option<usize>,
            complexity: Option<usize>,
        ) -> Result<Schema, SchemaError> {
            let mut builder = Builder::new(&CONTEXT, database.clone());

            seaography::register_entities!(
                builder,
                [
                    #(#entities,)*
                ]
            );

            #(#enumerations)*

            let schema = builder.schema_builder();

            let schema = if let Some(depth) = depth {
                schema.limit_depth(depth)
            } else {
                schema
            };

            let schema = if let Some(complexity) = complexity {
                schema.limit_complexity(complexity)
            } else {
                schema
            };

            schema.data(database).finish()
        }
    }
}

pub fn write_query_root<P: AsRef<std::path::Path>, T: AsRef<std::path::Path>>(
    src_path: &P,
    entities_path: &T,
) -> Result<(), crate::error::Error> {
    let tokens = generate_query_root(entities_path);

    let file_name = src_path.as_ref().join("query_root.rs");

    std::fs::write(file_name, add_line_break(tokens))?;

    Ok(())
}

///
/// Used to generate project/Cargo.toml file content
///
pub fn write_cargo_toml<P: AsRef<std::path::Path>>(
    path: &P,
    crate_name: &str,
    sql_library: &str,
    framework: WebFrameworkEnum,
) -> std::io::Result<()> {
    let file_path = path.as_ref().join("Cargo.toml");

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
