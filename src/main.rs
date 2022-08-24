use clap::Parser;
use seaography::{parse_database_url, Args, Result};
use seaography_discoverer::{extract_database_metadata};
use seaography_generator::inject_graphql::inject_graphql;

#[async_std::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let database_url = parse_database_url(&args.database_url)?;

    let path = std::path::Path::new(&args.destination);

    let expanded_format = true;

    let (tables, _version) = extract_database_metadata(&database_url).await?;

    let entities_hashmap = seaography_generator::sea_orm_codegen::generate_entities(tables.values().cloned().collect(), expanded_format).unwrap();

    let entities_hashmap = inject_graphql(entities_hashmap, expanded_format);

    std::fs::create_dir_all(&path.join("src/entities"))?;

    seaography_generator::sea_orm_codegen::write_entities(&path.join("src/entities"), entities_hashmap).unwrap();

    Ok(())
}
