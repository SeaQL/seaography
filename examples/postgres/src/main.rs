use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_poem::GraphQL;
use dotenv::dotenv;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
use sea_orm::Database;
use seaography_postgres_example::*;
use std::env;

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
        .with_max_level(tracing::Level::INFO)
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
