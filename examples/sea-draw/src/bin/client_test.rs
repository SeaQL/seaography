use clap::Parser;
use dotenv::dotenv;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Binding address at which the API will be available.
    #[clap(short, long, default_value = "127.0.0.1", env = "ADDRESS")]
    address: String,

    /// Port at which the API will be available.
    #[clap(short, long, default_value_t = 3333, env = "PORT")]
    port: u16,

    #[clap(long, env = "ROOT_ACCOUNT_ID")]
    root_account_id: Uuid,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("TRACE").unwrap_or_else(|_| "INFO".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let args = Args::parse();
    sea_draw::client_test::client_test(args.address, args.port, args.root_account_id).await
}
