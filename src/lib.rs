#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_parser)]
    pub database_url: String,

    #[clap(value_parser)]
    pub crate_name: String,

    #[clap(value_parser)]
    pub destination: String,
}
