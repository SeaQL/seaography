use crate::{Result, TablesHashMap};
use sea_schema::postgres::{def::TableDef, discovery::SchemaDiscovery};
use sqlx::PgPool;

pub async fn explore_postgres(uri: &str) -> Result<TablesHashMap> {
    let connection = PgPool::connect(uri).await?;

    let db_uri = url::Url::parse(uri).expect("Fail to parse database URL");

    let schema = db_uri
        .query_pairs()
        .find(|(k, _)| k == "currentSchema")
        .map_or("public".to_string(), |(_, v)| v.to_string());

    let schema_discovery = SchemaDiscovery::new(connection, &schema);

    let schema = schema_discovery.discover().await;

    let tables: TablesHashMap = schema
        .tables
        .iter()
        .map(|table: &TableDef| (table.info.name.clone(), table.write()))
        .collect();

    Ok(tables)
}
