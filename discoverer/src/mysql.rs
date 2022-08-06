use sea_schema::mysql::{def::TableDef, discovery::SchemaDiscovery};
use sqlx::MySqlPool;

use crate::{Result, TablesHashMap};

pub async fn explore_mysql(url: &str) -> Result<TablesHashMap> {
    let connection = MySqlPool::connect(url).await?;

    let schema = url
        .split('/')
        .last()
        .ok_or("schema not found in database url")?;

    let schema_discovery = SchemaDiscovery::new(connection, schema);

    let schema = schema_discovery.discover().await;

    let tables: TablesHashMap = schema
        .tables
        .iter()
        .map(|table: &TableDef| (table.info.name.clone(), table.write()))
        .collect();

    Ok(tables)
}
