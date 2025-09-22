use async_graphql::http::{GraphQLPlaygroundConfig, graphiql_source, playground_source};
use axum::{Extension, Router, response, routing};
use clap::Parser;
use dotenv::dotenv;
use sea_orm::Database;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use sea_draw::{backend::Backend, schema::queries_and_mutations};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Binding address at which the API will be available.
    #[clap(short, long, default_value = "127.0.0.1", env = "ADDRESS")]
    address: String,

    /// Port at which the API will be available.
    #[clap(short, long, default_value_t = 3333, env = "PORT")]
    port: u16,

    #[clap(long, env = "DATABASE_URL")]
    database_url: String,

    #[clap(long, env = "ROOT_ACCOUNT_ID")]
    root_account_id: Uuid,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("TRACE").unwrap_or_default()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    let database = Database::connect(args.database_url)
        .await
        .expect("Fail to initialize database connection");
    tracing::info!("Connected to database");

    let backend = Backend::new(database, args.root_account_id);

    tracing::info!("Starting GraphQL server");
    let schema = sea_draw::schema::schema(backend.clone()).unwrap();

    let graphql_endpoint_url = format!("http://{}:{}", args.address, args.port);
    let subscription_endpoint = format!("http://{}:{}/ws", args.address, args.port);

    let app = Router::new()
        .route(
            "/",
            routing::get(response::Html(playground_source(
                GraphQLPlaygroundConfig::new(&args.address),
            )))
            .post(queries_and_mutations),
        )
        .route(
            "/graphiql",
            routing::get(response::Html(graphiql_source(
                &graphql_endpoint_url,
                Some(&subscription_endpoint),
            ))),
        )
        // .route("/ws", routing::get(graphql_ws_handler))
        .layer(Extension(schema))
        .layer(Extension(backend));
    tracing::info!(
        "Visit GraphQL Playground at http://{}:{} or http://{}:{}/graphiql",
        args.address,
        args.port,
        args.address,
        args.port
    );
    let socketaddr: SocketAddr = (
        args.address.parse::<IpAddr>().expect("invalid ip address"),
        args.port,
    )
        .into();

    axum::serve(TcpListener::bind(&socketaddr).await.unwrap(), app).await?;

    Ok(())
}
