use clap::{Parser, ValueEnum};
use seaography_generator::write_project;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Project destination folder
    pub destination: String,

    /// SeaORM entities folder
    pub entities: String,

    /// Database URL to write in .env
    pub database_url: String,

    /// Crate name for generated project
    pub crate_name: String,

    /// Which web framework to use
    #[clap(short, long, value_enum, default_value_t = WebFrameworkEnum::Poem)]
    pub framework: WebFrameworkEnum,

    /// GraphQL depth limit
    #[clap(long)]
    pub depth_limit: Option<usize>,

    /// GraphQL complexity limit
    #[clap(long)]
    pub complexity_limit: Option<usize>,
}

/**
 * Most code depends from here
 * https://github.com/SeaQL/sea-orm/blob/master/sea-orm-cli/src/commands.rs
 */
pub fn parse_database_url(database_url: &str) -> Result<url::Url, url::ParseError> {
    let url = url::Url::parse(database_url)?;

    // Make sure we have all the required url components
    //
    // Missing scheme will have been caught by the Url::parse() call
    // above
    let url_username = url.username();
    let url_host = url.host_str();

    let is_sqlite = url.scheme() == "sqlite";

    // Skip checking if it's SQLite
    if !is_sqlite {
        // Panic on any that are missing
        if url_username.is_empty() {
            panic!("No username was found in the database url");
        }
        if url_host.is_none() {
            panic!("No host was found in the database url");
        }
    }

    //
    // Make sure we have database name
    //
    if !is_sqlite {
        // The database name should be the first element of the path string
        //
        // Throwing an error if there is no database name since it might be
        // accepted by the database without it, while we're looking to dump
        // information from a particular database
        let database_name = url
            .path_segments()
            .unwrap_or_else(|| panic!("There is no database name as part of the url path: {}", url))
            .next()
            .unwrap();

        // An empty string as the database name is also an error
        if database_name.is_empty() {
            panic!(
                "There is no database name as part of the url path: {}",
                url.as_str()
            );
        }
    }

    Ok(url)
}

#[async_std::main]
async fn main() {
    let args = Args::parse();

    let root_path = std::path::Path::new(&args.destination);

    let entities_path = std::path::Path::new(&args.entities);

    let database_url = parse_database_url(&args.database_url).unwrap();

    let sql_library = &map_sql_version(&database_url);

    let db_url = database_url.as_str();

    write_project(
        &root_path,
        &entities_path,
        db_url,
        &args.crate_name,
        sql_library,
        args.framework.into(),
        args.depth_limit,
        args.complexity_limit,
    )
    .await
    .unwrap();
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum WebFrameworkEnum {
    Actix,
    Poem,
    Axum,
}

impl From<WebFrameworkEnum> for seaography_generator::WebFrameworkEnum {
    fn from(framework: WebFrameworkEnum) -> Self {
        match framework {
            WebFrameworkEnum::Actix => seaography_generator::WebFrameworkEnum::Actix,
            WebFrameworkEnum::Poem => seaography_generator::WebFrameworkEnum::Poem,
            WebFrameworkEnum::Axum => seaography_generator::WebFrameworkEnum::Axum,
        }
    }
}

fn map_sql_version(database_url: &url::Url) -> String {
    match database_url.scheme() {
        "mysql" => String::from("sqlx-mysql"),
        "sqlite" => String::from("sqlx-sqlite"),
        "postgres" | "postgresql" => String::from("sqlx-postgres"),
        _ => unimplemented!("{} is not supported", database_url.scheme()),
    }
}
