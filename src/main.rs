use clap::Parser;
use seaography_generator::write_project;

#[async_std::main]
async fn main() {
    let args = seaography::Args::parse();

    let path = std::path::Path::new(&args.destination);

    let expanded_format = false;

    let db_url = &args.database_url;

    write_project(&path, db_url, &args.crate_name, expanded_format)
        .await
        .unwrap();
}
