use std::path::Path;

pub mod error;
pub use error::{Error, Result};
pub mod inject_graphql;
pub mod sea_orm_codegen;
pub mod writer;

/**
 * Most code depends from here
 * https://github.com/SeaQL/sea-orm/blob/master/sea-orm-cli/src/commands.rs
 */
pub fn parse_database_url(database_url: &str) -> Result<url::Url> {
    let url = url::Url::parse(database_url)?;

    // Make sure we have all the required url components
    //
    // Missing scheme will have been caught by the Url::parse() call
    // above
    let url_username = url.username();
    let url_host = url.host_str();

    let is_sqlite = url.scheme() == "sqlite";

    // Skip checking if it's SQLite
    if !is_sqlite {
        // Panic on any that are missing
        if url_username.is_empty() {
            return Err(Error::Error(
                "No username was found in the database url".into(),
            ));
        }
        if url_host.is_none() {
            return Err(Error::Error("No host was found in the database url".into()));
        }
    }

    //
    // Make sure we have database name
    //
    if !is_sqlite {
        // The database name should be the first element of the path string
        //
        // Throwing an error if there is no database name since it might be
        // accepted by the database without it, while we're looking to dump
        // information from a particular database
        let database_name = url
            .path_segments()
            .ok_or_else(|| {
                Error::Error(format!(
                    "There is no database name as part of the url path: {}",
                    url.as_str()
                ))
            })?
            .next()
            .unwrap();

        // An empty string as the database name is also an error
        if database_name.is_empty() {
            return Err(Error::Error(format!(
                "There is no database name as part of the url path: {}",
                url.as_str()
            )));
        }
    }

    Ok(url)
}

pub async fn write_project<P: AsRef<Path>>(
    path: &P,
    db_url: &str,
    crate_name: &str,
    expanded_format: bool,
    depth_limit: Option<usize>,
    complexity_limit: Option<usize>,
) -> Result<()> {
    let database_url = parse_database_url(db_url)?;

    let (tables, sql_version) =
        seaography_discoverer::extract_database_metadata(&database_url).await?;

    writer::write_cargo_toml(path, crate_name, &sql_version)?;

    std::fs::create_dir_all(&path.as_ref().join("src/entities"))?;

    let src_path = &path.as_ref().join("src");

    let entities_hashmap =
        sea_orm_codegen::generate_entities(tables.values().cloned().collect(), expanded_format)
            .unwrap();

    let entities_hashmap = inject_graphql::inject_graphql(entities_hashmap, expanded_format);

    writer::write_query_root(src_path, &entities_hashmap).unwrap();
    writer::write_lib(src_path)?;
    writer::write_main(src_path, crate_name)?;
    writer::write_env(&path.as_ref(), db_url, depth_limit, complexity_limit)?;

    sea_orm_codegen::write_entities(&src_path.join("entities"), entities_hashmap.clone()).unwrap();

    std::process::Command::new("cargo")
        .arg("fmt")
        .current_dir(&path)
        .spawn()?
        .wait()?;

    Ok(())
}
