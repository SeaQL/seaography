use sea_schema::mysql::{discovery::SchemaDiscovery, def::TableDef};
use sqlx::MySqlPool;

use crate::TablesHashMap;

pub async fn explore_mysql(url: &String) -> TablesHashMap {
    let connection = MySqlPool::connect(url).await.unwrap();

    let database = url.split("/").last().unwrap();

    let schema_discovery = SchemaDiscovery::new(connection, database);

    let schema = schema_discovery.discover().await;

    let tables: TablesHashMap = schema
        .tables
        .iter()
        .map(|table: &TableDef| (table.info.name.clone(), table.write()))
        .collect();

    tables
}