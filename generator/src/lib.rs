use std::path::Path;

pub mod error;
pub use error::{Error, Result};
pub mod inject_graphql;
pub mod sea_orm_codegen;
pub mod templates;
pub mod writer;

mod util;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WebFrameworkEnum {
    Actix,
    Poem,
}

pub async fn write_project<P: AsRef<Path>>(
    path: &P,
    db_url: &str,
    crate_name: &str,
    expanded_format: bool,
    tables: std::collections::BTreeMap<String, sea_query::TableCreateStatement>,
    sql_library: &str,
    framework: WebFrameworkEnum,
    depth_limit: Option<usize>,
    complexity_limit: Option<usize>,
) -> Result<()> {
    std::fs::create_dir_all(&path.as_ref().join("src/entities"))?;

    writer::write_cargo_toml(path, crate_name, &sql_library, framework)?;

    let src_path = &path.as_ref().join("src");

    let entities_hashmap =
        sea_orm_codegen::generate_entities(tables.values().cloned().collect(), expanded_format)
            .unwrap();

    let entities_hashmap = inject_graphql::inject_graphql(entities_hashmap, expanded_format);

    writer::write_query_root(src_path, &entities_hashmap).unwrap();
    writer::write_lib(src_path)?;

    match framework {
        WebFrameworkEnum::Actix => {
            crate::templates::actix::write_main(src_path, crate_name).unwrap()
        }
        WebFrameworkEnum::Poem => crate::templates::poem::write_main(src_path, crate_name).unwrap(),
    }

    writer::write_env(&path.as_ref(), db_url, depth_limit, complexity_limit)?;

    sea_orm_codegen::write_entities(&src_path.join("entities"), entities_hashmap).unwrap();

    std::process::Command::new("cargo")
        .arg("fmt")
        .current_dir(&path)
        .spawn()?
        .wait()?;

    Ok(())
}
