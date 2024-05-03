use proc_macro2::TokenStream;
use quote::quote;

use crate::util::add_line_break;

///
/// Used to generate project/src/main.rs file content
///
pub fn generate_main(crate_name: &str) -> TokenStream {
    let crate_name_token: TokenStream = crate_name.replace('-', "_").parse().unwrap();

    quote! {
        use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
        use async_graphql_axum::GraphQL;
        use axum::{
            response::{self, IntoResponse},
            routing::get,
            Router,
        };
        use dotenv::dotenv;
        use lazy_static::lazy_static;
        use sea_orm::Database;
        use std::env;
        use tokio::net::TcpListener;

        lazy_static! {
            static ref URL: String = env::var("URL").unwrap_or("localhost:8000".into());
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

        async fn graphiql() -> impl IntoResponse {
            response::Html(playground_source(GraphQLPlaygroundConfig::new(&*ENDPOINT)))
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
            let schema = #crate_name_token::query_root::schema(database, *DEPTH_LIMIT, *COMPLEXITY_LIMIT).unwrap();
            let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
            println!("Visit GraphQL Playground at http://{}", *URL);
            axum::serve(TcpListener::bind(&*URL).await.unwrap(), app)
                .await
                .unwrap();
        }
    }
}

pub fn write_main<P: AsRef<std::path::Path>>(path: &P, crate_name: &str) -> std::io::Result<()> {
    let tokens = generate_main(crate_name);

    let file_name = path.as_ref().join("main.rs");

    std::fs::write(file_name, add_line_break(tokens))?;

    Ok(())
}
