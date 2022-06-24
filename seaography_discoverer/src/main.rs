use clap::Parser;
use sea_schema::{sqlite::discovery::DiscoveryResult};
use seaography_types::{relationship_meta::RelationshipMeta, table_meta::TableMeta, schema_meta::SchemaMeta,
};
use seaography_discoverer::{Args, explore_sqlite, TablesHashMap, extract_relationships_meta, extract_tables_meta};



#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    let args = Args::parse();

    let tables: TablesHashMap = explore_sqlite(&args.url).await?;

    let relationships: Vec<RelationshipMeta> = extract_relationships_meta(&tables);

    let tables: Vec<TableMeta> = extract_tables_meta(&tables, &relationships);

    let schema: SchemaMeta = SchemaMeta { tables, enums: vec![] };

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    Ok(())
}
