use crate::{Result, TablesHashMap};
use sea_schema::postgres::{def::TableDef, discovery::SchemaDiscovery};
use sqlx::PgPool;

pub async fn explore_postgres(uri: &str) -> Result<TablesHashMap> {
    let connection = PgPool::connect(uri).await?;

    let database = uri
        .split('/')
        .last()
        .ok_or("database not specified in url")?;

    let schema = database.split("currentSchema=").last().unwrap_or("public");

    let schema = if schema.is_empty() { "public" } else { schema };

    let schema_discovery = SchemaDiscovery::new(connection, schema);

    let schema = schema_discovery.discover().await;

    let tables: TablesHashMap = schema
        .tables
        .iter()
        .map(|table: &TableDef| (table.info.name.clone(), table.write()))
        .collect();

    Ok(tables)
}
