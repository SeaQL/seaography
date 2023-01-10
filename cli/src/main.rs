use clap::{Parser, ValueEnum};
use seaography_generator::write_project;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub database_url: String,

    pub crate_name: String,

    pub destination: String,

    #[clap(long)]
    pub expanded_format: Option<bool>,

    #[clap(long)]
    pub depth_limit: Option<usize>,

    #[clap(long)]
    pub complexity_limit: Option<usize>,

    #[clap(long)]
    pub ignore_tables: Option<String>,

    #[clap(long)]
    pub hidden_tables: Option<bool>,

    #[clap(short, long, value_enum, default_value_t = WebFrameworkEnum::Poem)]
    pub framework: WebFrameworkEnum,
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
            .expect(format!("There is no database name as part of the url path: {}", url).as_str())
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

    let path = std::path::Path::new(&args.destination);

    let database_url = parse_database_url(&args.database_url).unwrap();

    let (tables, sql_version) = seaography_discoverer::extract_database_metadata(&database_url)
        .await
        .unwrap();

    let sql_library = match sql_version {
        seaography_discoverer::SqlVersion::Sqlite => "sqlx-sqlite",
        seaography_discoverer::SqlVersion::Mysql => "sqlx-mysql",
        seaography_discoverer::SqlVersion::Postgres => "sqlx-postgres",
    };

    let expanded_format = args.expanded_format.unwrap_or(false);

    let ignore_tables = args
        .ignore_tables
        .unwrap_or_else(|| "seaql_migrations".into());
    let ignore_tables: Vec<&str> = ignore_tables.split(",").collect();

    let hidden_tables = args.hidden_tables.unwrap_or(true);

    let tables: std::collections::BTreeMap<
        String,
        seaography_discoverer::sea_schema::sea_query::TableCreateStatement,
    > = tables
        .into_iter()
        .filter(|(key, _)| {
            if hidden_tables {
                !key.starts_with("_")
            } else {
                true
            }
        })
        .filter(|(key, _)| {
            if !ignore_tables.is_empty() {
                !ignore_tables.contains(&key.as_str())
            } else {
                true
            }
        })
        .collect();

    let db_url = database_url.as_str();

    write_project(
        &path,
        db_url,
        &args.crate_name,
        expanded_format,
        tables,
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
}

impl From<WebFrameworkEnum> for seaography_generator::WebFrameworkEnum {
    fn from(framework: WebFrameworkEnum) -> Self {
        match framework {
            WebFrameworkEnum::Actix => seaography_generator::WebFrameworkEnum::Actix,
            WebFrameworkEnum::Poem => seaography_generator::WebFrameworkEnum::Poem,
        }
    }
}
