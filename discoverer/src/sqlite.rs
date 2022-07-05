use sea_schema::sqlite::{def::TableDef, discovery::SchemaDiscovery};
use sqlx::SqlitePool;

use crate::{Error, Result, TablesHashMap};

pub async fn explore_sqlite(url: &String) -> Result<TablesHashMap> {
    let connection = SqlitePool::connect(url).await?;

    let schema_discovery = SchemaDiscovery::new(connection);

    let schema = schema_discovery
        .discover()
        .await
        .map_err(|_| Error::Error("SqliteDiscoveryError".into()))?;

    let tables: TablesHashMap = schema
        .tables
        .iter()
        .map(|table: &TableDef| (table.name.clone(), table.write()))
        .collect();

    Ok(tables)
}
