use clap::Parser;
use dotenv::dotenv;
use tokio::{
    sync::oneshot,
};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

use sea_draw::{client_test::client_test, server::server};

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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("TRACE").unwrap_or_default()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let args = Args::parse();

    // Start the server
    let (stop_tx, stop_rx) = oneshot::channel();
    let server_handle = tokio::task::spawn(server(
        args.address.clone(),
        args.port,
        args.database_url.clone(),
        args.root_account_id,
        stop_rx,
    ));

    // Run the client tesst
    client_test(args.address, args.port, args.root_account_id).await?;

    // Stop the server
    stop_tx.send(()).unwrap();
    server_handle.await.unwrap()?;

    Ok(())
}
