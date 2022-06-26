use clap::Parser;
use seaography_discoverer::{
    explore_mysql, explore_sqlite, extract_enums, extract_relationships_meta, extract_tables_meta,
    Args, TablesHashMap, Result, explore_postgres, utils::parse_database_url,
};
use seaography_types::{
    relationship_meta::RelationshipMeta, schema_meta::SchemaMeta, table_meta::TableMeta,
};

/**
 * Most ideas come from here
 * https://github.com/SeaQL/sea-orm/blob/master/sea-orm-cli/src/commands.rs
 */
#[async_std::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let url = parse_database_url(&args.url)?;

    let tables: TablesHashMap = match url.scheme() {
        "mysql" => {
            explore_mysql(&args.url).await?
        },
        "sqlite" => {
            explore_sqlite(&args.url).await?
        },
        "postgres" | "postgresql" | "pgsql" => {
            explore_postgres(&args.url).await?
        }
        _ => unimplemented!("{} is not supported", url.scheme()),
    };

    let relationships: Vec<RelationshipMeta> = extract_relationships_meta(&tables)?;

    let enums = extract_enums(&tables);

    let tables: Vec<TableMeta> = extract_tables_meta(&tables, &relationships);

    let schema: SchemaMeta = SchemaMeta { tables, enums };

    println!("{}", serde_json::to_string_pretty(&schema)?);

    Ok(())
}
