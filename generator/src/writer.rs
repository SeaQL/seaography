use proc_macro2::TokenStream;
use quote::quote;

use crate::util::add_line_break;

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
) -> std::io::Result<()> {
    let file_path = path.as_ref().join("Cargo.toml");

    let content = include_str!("_Cargo.toml");

    let content = content.replace("<seaography-package-name>", crate_name);

    let content = content.replace("<seaography-sql-library>", sql_library);

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

///
/// Used to generate project/src/main.rs file content
///
pub fn generate_main(crate_name: &str) -> TokenStream {
    let crate_name_token: TokenStream = crate_name.replace('-', "_").parse().unwrap();

    quote! {
        use async_graphql::{
            dataloader::DataLoader,
            http::{playground_source, GraphQLPlaygroundConfig},
            EmptyMutation, EmptySubscription, Schema,
        };
        use async_graphql_poem::GraphQL;
        use dotenv::dotenv;
        use lazy_static::lazy_static;
        use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
        use sea_orm::Database;
        use #crate_name_token::*;
        use std::env;

        lazy_static! {
            static ref URL: String = env::var("URL").unwrap_or("0.0.0.0:8000".into());
            static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into());
            static ref DATABASE_URL: String =
                env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
            static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
                data.parse().expect("DEPTH_LIMIT is not a number")
            ));
            static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
                .map_or(None, |data| {
                    Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
                });
        }

        #[handler]
        async fn graphql_playground() -> impl IntoResponse {
            Html(playground_source(GraphQLPlaygroundConfig::new(&*ENDPOINT)))
        }

        #[tokio::main]
        async fn main() {
            dotenv().ok();
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO)
                .with_test_writer()
                .init();

            let database = Database::connect(&*DATABASE_URL)
                .await
                .expect("Fail to initialize database connection");
            let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
                OrmDataloader {
                    db: database.clone(),
                },
                tokio::spawn,
            );
            let mut schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
                .data(database)
                .data(orm_dataloader);
            if let Some(depth) = *DEPTH_LIMIT {
                schema = schema.limit_depth(depth);
            }
            if let Some(complexity) = *COMPLEXITY_LIMIT {
                schema = schema.limit_complexity(complexity);
            }
            let schema = schema.finish();
            let app = Route::new().at(
                &*ENDPOINT,
                get(graphql_playground).post(GraphQL::new(schema)),
            );

            println!("Visit GraphQL Playground at http://{}", *URL);
            Server::new(TcpListener::bind(&*URL))
                .run(app)
                .await
                .expect("Fail to start web server");
        }
    }
}

pub fn write_main<P: AsRef<std::path::Path>>(path: &P, crate_name: &str) -> std::io::Result<()> {
    let tokens = generate_main(crate_name);

    let file_name = path.as_ref().join("main.rs");

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
