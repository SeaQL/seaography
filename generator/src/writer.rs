use proc_macro2::TokenStream;
use quote::quote;
use seaography_types::SqlVersion;

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
      #[derive(Debug, seaography_derive::QueryRoot)]
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

    let data = crate::toml::TomlStructure::new(crate_name, sql_version);

    std::fs::write(file_path, toml::to_string_pretty(&data).unwrap())?;

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

        #[derive(Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum)]
        pub enum OrderByEnum {
            Asc,
            Desc,
        }

        pub type BinaryVector = Vec<u8>;

        #[derive(async_graphql::InputObject, Debug)]
        #[graphql(concrete(name = "StringFilter", params(String)))]
        #[graphql(concrete(name = "TinyIntegerFilter", params(i8)))]
        #[graphql(concrete(name = "SmallIntegerFilter", params(i16)))]
        #[graphql(concrete(name = "IntegerFilter", params(i32)))]
        #[graphql(concrete(name = "BigIntegerFilter", params(i64)))]
        #[graphql(concrete(name = "TinyUnsignedFilter", params(u8)))]
        #[graphql(concrete(name = "SmallUnsignedFilter", params(u16)))]
        #[graphql(concrete(name = "UnsignedFilter", params(u32)))]
        #[graphql(concrete(name = "BigUnsignedFilter", params(u64)))]
        #[graphql(concrete(name = "FloatFilter", params(f32)))]
        #[graphql(concrete(name = "DoubleFilter", params(f64)))]
        // TODO #[graphql(concrete(name = "JsonFilter", params()))]
        // TODO #[graphql(concrete(name = "DateFilter", params()))]
        // TODO #[graphql(concrete(name = "TimeFilter", params()))]
        #[graphql(concrete(name = "DateFilter", params(Date)))]
        #[graphql(concrete(name = "DateTimeFilter", params(DateTime)))]
        #[graphql(concrete(name = "DateTimeUtcFilter", params(DateTimeUtc)))]
        // TODO #[graphql(concrete(name = "TimestampFilter", params()))]
        // TODO #[graphql(concrete(name = "TimestampWithTimeZoneFilter", params()))]
        #[graphql(concrete(name = "DecimalFilter", params(Decimal)))]
        // TODO #[graphql(concrete(name = "UuidFilter", params(uuid::Uuid)))]
        #[graphql(concrete(name = "BinaryFilter", params(BinaryVector)))]
        #[graphql(concrete(name = "BooleanFilter", params(bool)))]
        // TODO #[graphql(concrete(name = "EnumFilter", params()))]
        pub struct TypeFilter<T: async_graphql::InputType> {
            pub eq: Option<T>,
            pub ne: Option<T>,
            pub gt: Option<T>,
            pub gte: Option<T>,
            pub lt: Option<T>,
            pub lte: Option<T>,
            pub is_in: Option<Vec<T>>,
            pub is_not_in: Option<Vec<T>>,
            pub is_null: Option<bool>,
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
pub fn generate_main(db_url: &str, crate_name: &str) -> TokenStream {
    let crate_name_token: TokenStream = crate_name.parse().unwrap();

    quote! {
        use async_graphql::{
            http::{playground_source, GraphQLPlaygroundConfig},
            EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader
        };
        use async_graphql_poem::GraphQL;
        use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
        use sea_orm::Database;

        use #crate_name_token::*;

        #[handler]
        async fn graphql_playground() -> impl IntoResponse {
            Html(playground_source(GraphQLPlaygroundConfig::new("/")))
        }

        #[tokio::main]
        async fn main() {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .with_test_writer()
                .init();

            // TODO: use .env file to configure url
            let database = Database::connect(#db_url).await.unwrap();

            // TODO use environment variables to configure dataloader batch size
            let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
                OrmDataloader {
                    db: database.clone()
                },
                tokio::spawn
            ) ;

            let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
                .data(database)
                .data(orm_dataloader)
                .finish();

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
    db_url: &str,
    crate_name: &str,
) -> std::io::Result<()> {
    let tokens = generate_main(db_url, crate_name);

    let file_name = path.as_ref().join("main.rs");

    std::fs::write(file_name, tokens.to_string())?;

    Ok(())
}
