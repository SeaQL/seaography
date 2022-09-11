use sea_schema::sea_query::TableCreateStatement;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SqlVersion {
    Sqlite,
    Mysql,
    Postgres,
}

pub mod sqlite;
pub use sqlite::explore_sqlite;

pub mod mysql;
pub use mysql::explore_mysql;

pub mod postgres;
pub use postgres::explore_postgres;

pub mod error;
pub use error::{Error, Result};

pub use sea_schema;

pub type TablesHashMap = BTreeMap<String, TableCreateStatement>;

pub async fn extract_database_metadata(
    database_url: &url::Url,
) -> Result<(TablesHashMap, SqlVersion)> {
    Ok(match database_url.scheme() {
        "mysql" => (
            explore_mysql(database_url.as_ref()).await?,
            SqlVersion::Mysql,
        ),
        "sqlite" => (
            explore_sqlite(database_url.as_ref()).await?,
            SqlVersion::Sqlite,
        ),
        "postgres" | "postgresql" => (
            explore_postgres(database_url.as_ref()).await?,
            SqlVersion::Postgres,
        ),
        _ => unimplemented!("{} is not supported", database_url.scheme()),
    })
}
