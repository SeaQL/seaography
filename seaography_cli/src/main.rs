use clap::Parser;
use seaography_cli::{parse_database_url, Args, Result, generate_orm};
use seaography_discoverer::{extract_database_metadata, extract_schema};
use seaography_generator::write_project;
use seaography_types::SchemaMeta;

#[async_std::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let database_url = parse_database_url(&args.database_url)?;

    let (tables, version) = extract_database_metadata(&database_url).await?;

    let schema: SchemaMeta = extract_schema(&database_url, &tables, &version).await?;

    let path = std::path::Path::new(&args.destination);

    std::fs::create_dir_all(&path.join("src/orm"))?;

    generate_orm(&path.join("src/orm"), tables.values().cloned().collect())?;

    write_project(&path, &schema, &args.crate_name)?;

    Ok(())
}
