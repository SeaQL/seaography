use clap::Parser;
use dotenv::dotenv;
use sea_orm::entity::prelude::Uuid;
use tokio::sync::oneshot;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use sea_draw::server::server;

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
    server(
        args.address,
        args.port,
        args.database_url,
        args.root_account_id,
        oneshot::channel().1,
    )
    .await
}
