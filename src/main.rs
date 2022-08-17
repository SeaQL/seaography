use clap::Parser;
use seaography::{parse_database_url, Args, Result};
use seaography_discoverer::{extract_database_metadata};

#[async_std::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let database_url = parse_database_url(&args.database_url)?;

    let path = std::path::Path::new(&args.destination);

    let (tables, _version) = extract_database_metadata(&database_url).await?;

    let writer_output = seaography_generator::sea_orm_codegen::generate_entities(tables.values().cloned().collect()).unwrap();

    std::fs::create_dir_all(&path.join("src/entities"))?;

    seaography_generator::sea_orm_codegen::write_entities(&path.join("src/entities"), writer_output).unwrap();

    Ok(())
}
