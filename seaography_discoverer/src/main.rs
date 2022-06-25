use clap::Parser;
use sea_schema::{sqlite::discovery::DiscoveryResult};
use seaography_types::{relationship_meta::RelationshipMeta, table_meta::TableMeta, schema_meta::SchemaMeta,
};
use seaography_discoverer::{Args, explore_sqlite, TablesHashMap, extract_relationships_meta, extract_tables_meta, mysql::explore_mysql, extract_enums};


/**
 * Most ideas come from here
 * https://github.com/SeaQL/sea-orm/blob/master/sea-orm-cli/src/commands.rs
 */
#[async_std::main]
async fn main() -> DiscoveryResult<()> {
    let args = Args::parse();

    let tables: TablesHashMap =
    if args.url.starts_with("sqlite") {
        explore_sqlite(&args.url).await?
    } else if args.url.starts_with("mysql") {
        explore_mysql(&args.url).await
    } else {
        unreachable!()
    };

    let relationships: Vec<RelationshipMeta> = extract_relationships_meta(&tables);

    let enums = extract_enums(&tables);

    let tables: Vec<TableMeta> = extract_tables_meta(&tables, &relationships);

    let schema: SchemaMeta = SchemaMeta { tables, enums };

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    Ok(())
}
