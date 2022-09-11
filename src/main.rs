use clap::Parser;
use seaography_generator::write_project;

#[async_std::main]
async fn main() {
    let args = seaography::Args::parse();

    let path = std::path::Path::new(&args.destination);

    write_project(
        &path,
        &args.database_url,
        &args.crate_name,
        args.expanded_format.unwrap_or(false),
        args.depth_limit,
        args.complexity_limit,
    )
    .await
    .unwrap();
}
