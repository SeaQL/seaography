use proc_macro2::TokenStream;
use quote::quote;
use seaography_discoverer::SqlVersion;

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

    std::fs::write(file_name, tokens.to_string())?;

    Ok(())
}

pub fn write_cargo_toml<P: AsRef<std::path::Path>>(
    path: &P,
    crate_name: &str,
    sql_version: &SqlVersion,
) -> std::io::Result<()> {
    let file_path = path.as_ref().join("Cargo.toml");

    let content = std::fs::read_to_string("./generator/src/_Cargo.toml")?;

    let content = content.replace("<seaography-package-name>", crate_name);

    let sql_library = match sql_version {
        seaography_discoverer::SqlVersion::Sqlite => "sqlx-sqlite",
        seaography_discoverer::SqlVersion::Mysql => "sqlx-mysql",
        seaography_discoverer::SqlVersion::Postgres => "sqlx-postgres",
    };


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

    std::fs::write(file_name, tokens.to_string())?;

    Ok(())
}

///
/// Used to generate project/src/main.rs file content
///
pub fn generate_main(crate_name: &str) -> TokenStream {
    let crate_name_token: TokenStream = crate_name.parse().unwrap();

    quote! {
        use async_graphql::{
            dataloader::DataLoader,
            http::{playground_source, GraphQLPlaygroundConfig},
            EmptyMutation, EmptySubscription, Schema,
        };
        use async_graphql_poem::GraphQL;
        use dotenv::dotenv;
        use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
        use sea_orm::Database;
        use std::env;

        use #crate_name_token::*;

        #[handler]
        async fn graphql_playground() -> impl IntoResponse {
            Html(playground_source(GraphQLPlaygroundConfig::new("/")))
        }
        #[tokio::main]
        async fn main() {
            dotenv().ok();

            let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

            let depth_limit = env::var("DEPTH_LIMIT")
                .map(|data| data.parse::<usize>().expect("DEPTH_LIMIT is not a number"))
                .map_or(None, |data| Some(data));

            let complexity_limit = env::var("COMPLEXITY_LIMIT")
                .map(|data| {
                    data.parse::<usize>()
                        .expect("COMPLEXITY_LIMIT is not a number")
                })
                .map_or(None, |data| Some(data));

            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .with_test_writer()
                .init();
            let database = Database::connect(db_url).await.unwrap();
            let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
                OrmDataloader {
                    db: database.clone(),
                },
                tokio::spawn,
            );
            let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
                .data(database)
                .data(orm_dataloader);

            let schema = if let Some(depth) = depth_limit {
                schema.limit_depth(depth)
            } else {
                schema
            };

            let schema = if let Some(complexity) = complexity_limit {
                schema.limit_complexity(complexity)
            } else {
                schema
            };

            let schema = schema.finish();

            let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));
            println!("Playground: http://localhost:8000");
            Server::new(TcpListener::bind("0.0.0.0:8000"))
                .run(app)
                .await
                .unwrap();
        }

    }
}

pub fn write_main<P: AsRef<std::path::Path>>(
    path: &P,
    crate_name: &str,
) -> std::io::Result<()> {
    let tokens = generate_main(crate_name);

    let file_name = path.as_ref().join("main.rs");

    std::fs::write(file_name, tokens.to_string())?;

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

    let tokens = format!(r#"
    DATABASE_URL="{db_url}"
    # COMPLEXITY_LIMIT={depth_limit}
    # DEPTH_LIMIT={complexity_limit}
    "#);

    let file_name = path.as_ref().join(".env");

    std::fs::write(file_name, tokens.to_string())?;

    Ok(())
}
