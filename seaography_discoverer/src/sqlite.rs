use sea_schema::sqlite::{discovery::{SchemaDiscovery, DiscoveryResult}, def::TableDef};
use sqlx::SqlitePool;

use crate::TablesHashMap;

pub async fn explore_sqlite(url: &String) -> DiscoveryResult<TablesHashMap> {
    let connection = SqlitePool::connect(url)
        .await
        .unwrap();

    let schema_discovery = SchemaDiscovery::new(connection);

    let schema = schema_discovery.discover().await?;

    let tables: TablesHashMap = schema
        .tables
        .iter()
        .map(|table: &TableDef| (table.name.clone(), table.write()))
        .collect();

    Ok(tables)
}