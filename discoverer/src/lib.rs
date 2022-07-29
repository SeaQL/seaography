use sea_schema::sea_query::TableCreateStatement;
use std::collections::HashMap;

use seaography_types::{RelationshipMeta, SchemaMeta, SqlVersion, TableMeta};

pub mod sqlite;
pub use sqlite::explore_sqlite;

pub mod mysql;
pub use mysql::explore_mysql;

pub mod postgres;
pub use postgres::explore_postgres;

pub mod error;
pub use error::{Error, Result};

pub mod utils;
pub use utils::{extract_enums, extract_relationships_meta, extract_tables_meta};

pub mod test_cfg;

pub use sea_schema;

pub type TablesHashMap = HashMap<String, TableCreateStatement>;

pub async fn extract_database_metadata(
    database_url: &url::Url,
) -> Result<(TablesHashMap, SqlVersion)> {
    Ok(match database_url.scheme() {
        "mysql" => (
            explore_mysql(&database_url.to_string()).await?,
            SqlVersion::Mysql,
        ),
        "sqlite" => (
            explore_sqlite(&database_url.to_string()).await?,
            SqlVersion::Sqlite,
        ),
        "postgres" | "postgresql" | "pgsql" => (
            explore_postgres(&database_url.to_string()).await?,
            SqlVersion::Postgres,
        ),
        _ => unimplemented!("{} is not supported", database_url.scheme()),
    })
}

pub async fn extract_schema(
    database_url: &url::Url,
    tables: &TablesHashMap,
    version: &SqlVersion,
) -> Result<SchemaMeta> {
    let relationships: Vec<RelationshipMeta> = extract_relationships_meta(tables)?;

    let enums = extract_enums(tables);

    let tables: Vec<TableMeta> = extract_tables_meta(tables, &relationships);

    let schema: SchemaMeta = SchemaMeta {
        tables,
        enums,
        url: database_url.to_string(),
        version: version.clone(),
    };

    Ok(schema)
}
