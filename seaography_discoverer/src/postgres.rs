use crate::{Result, TablesHashMap};
use sea_schema::postgres::{def::TableDef, discovery::SchemaDiscovery};
use sqlx::PgPool;

pub async fn explore_postgres(uri: &String) -> Result<TablesHashMap> {
    let connection = PgPool::connect(uri).await?;

    let schema = uri
        .split("/")
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
