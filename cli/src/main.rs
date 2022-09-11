use clap::Parser;
use seaography_generator::write_project;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_parser)]
    pub database_url: String,

    #[clap(value_parser)]
    pub crate_name: String,

    #[clap(value_parser)]
    pub destination: String,

    #[clap(short, long)]
    pub expanded_format: Option<bool>,

    #[clap(short, long)]
    pub depth_limit: Option<usize>,

    #[clap(short, long)]
    pub complexity_limit: Option<usize>,
}

#[async_std::main]
async fn main() {
    let args = Args::parse();

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
