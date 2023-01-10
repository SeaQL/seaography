use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{util::add_line_break, WebFrameworkEnum};

pub struct EntityDefinition {
    pub name: TokenStream,
    pub relations: BTreeMap<String, TokenStream>,
}

pub fn generate_query_root(entities: &Vec<EntityDefinition>) -> TokenStream {
    let entities: Vec<TokenStream> = entities.iter().map(|entity| {
        let entity_path = &entity.name;

        let relations: Vec<TokenStream> = entity.relations.iter().filter(|(_relationship_name, related_path)| {
            if related_path.to_string().eq("Entity") {
                false
            } else {
                true
            }
        }).map(|(relationship_name, related_path)| {
            quote!{
                entity_object_relation::<#entity_path::Entity, #related_path>(#relationship_name, false)
            }
        }).collect();

        let self_relations: Vec<TokenStream> = entity.relations.iter()
            .filter(|(_relationship_name, related_path)| {
                if related_path.to_string().eq("Entity") {
                    true
                } else {
                    false
                }
            })
            .map(|(relationship_name, _related_path)| {
                let relationship_name = format!("{}Reverse", relationship_name);

                quote!{
                    entity_object_relation::<#entity_path::Entity, #entity_path::Entity>(#relationship_name, false),
                    entity_object_relation::<#entity_path::Entity, #entity_path::Entity>(#relationship_name, true)
                }
            })
            .collect();

        quote!{
            DynamicGraphqlEntity::from_entity::<#entity_path::Entity>(&pagination_input, vec![
                #(#relations),*
                #(#self_relations),*
            ])
        }
    }).collect();

    quote! {
        use async_graphql::{dataloader::DataLoader, dynamic::*};
        use sea_orm::DatabaseConnection;
        use seaography::{DynamicGraphqlEntity, entity_object_relation};

        use crate::OrmDataloader;

        pub fn schema(
            database: DatabaseConnection,
            orm_dataloader: DataLoader<OrmDataloader>,
            depth: Option<usize>,
            complexity: Option<usize>,
        ) -> Result<Schema, SchemaError> {
            let order_by_enum = seaography::get_order_by_enum();
            let cursor_input = seaography::get_cursor_input();
            let page_input = seaography::get_page_input();
            let pagination_input = seaography::get_pagination_input(&cursor_input, &page_input);

            let query = Object::new("Query");

            let entities = vec![
                #(#entities),*
            ];

            let schema = Schema::build(query.type_name(), None, None);

            let (schema, query) = entities
                .into_iter()
                .fold((schema, query), |(schema, query), object| {
                    (
                        schema
                            .register(object.filter_input)
                            .register(object.order_input)
                            .register(object.edge_object)
                            .register(object.connection_object)
                            .register(object.entity_object),
                        query.field(object.query),
                    )
                });

            let schema = if let Some(depth) = depth {
                schema.limit_depth(depth)
            } else {
                schema
            };

            let schema = seaography::get_filter_types()
                .into_iter()
                .fold(schema, |schema, object| schema.register(object));

            let schema = if let Some(complexity) = complexity {
                schema.limit_complexity(complexity)
            } else {
                schema
            };

            schema
                .register(seaography::PageInfo::to_object())
                .register(seaography::PaginationInfo::to_object())
                .register(cursor_input)
                .register(page_input)
                .register(pagination_input)
                .register(order_by_enum)
                .register(query)
                .data(database)
                .data(orm_dataloader)
                .finish()
        }
    }
}

pub fn write_query_root<P: AsRef<std::path::Path>>(
    path: &P,
    entities: &Vec<EntityDefinition>,
) -> Result<(), crate::error::Error> {
    let tokens = generate_query_root(entities);

    let file_name = path.as_ref().join("query_root.rs");

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
pub fn write_lib<P: AsRef<std::path::Path>>(path: &P) -> std::io::Result<()> {
    let tokens = quote! {
        use sea_orm::prelude::*;

        pub mod entities;
        pub mod query_root;

        pub struct OrmDataloader {
            pub db: DatabaseConnection,
        }

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
