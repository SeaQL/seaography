use async_graphql::http::{GraphQLPlaygroundConfig, graphiql_source, playground_source};
use axum::{Extension, Router, response, routing};
use sea_orm::Database;
use std::net::{IpAddr, SocketAddr};
use tokio::{net::TcpListener, sync::oneshot::Receiver};
use tracing::instrument;
use uuid::Uuid;

use crate::{backend::Backend, schema::queries_and_mutations};

#[instrument(skip_all)]
pub async fn server(
    address: String,
    port: u16,
    database_url: String,
    root_account_id: Uuid,
    stop: Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let database = Database::connect(database_url)
        .await
        .expect("Fail to initialize database connection");
    tracing::info!("Connected to database");

    let backend = Backend::new(database, root_account_id);

    tracing::info!("Starting GraphQL server");
    let schema = crate::schema::schema(backend.clone()).unwrap();

    let graphql_endpoint_url = format!("http://{}:{}", address, port);
    let subscription_endpoint = format!("http://{}:{}/ws", address, port);

    let app = Router::new()
        .route(
            "/",
            routing::get(response::Html(playground_source(
                GraphQLPlaygroundConfig::new(&address),
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
        address,
        port,
        address,
        port
    );
    let socketaddr: SocketAddr =
        (address.parse::<IpAddr>().expect("invalid ip address"), port).into();

    let axum_handle = axum::serve(TcpListener::bind(&socketaddr).await.unwrap(), app);

    tokio::select!(
        _ =  axum_handle => {
            tracing::error!("Axum task exited")
        }
        _ = stop => {
            tracing::info!("Stop signal received")
        }
    );

    Ok(())
}
