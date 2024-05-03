pub mod error;
pub use error::{Error, Result};
pub mod parser;
pub mod templates;
pub mod writer;

mod util;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WebFrameworkEnum {
    Actix,
    Poem,
    Axum,
}

#[allow(clippy::too_many_arguments)]
pub async fn write_project<P: AsRef<std::path::Path>, T: AsRef<std::path::Path>>(
    root_path: &P,
    entities_path: &T,
    db_url: &str,
    crate_name: &str,
    sql_library: &str,
    framework: WebFrameworkEnum,
    depth_limit: Option<usize>,
    complexity_limit: Option<usize>,
) -> Result<()> {
    writer::write_cargo_toml(root_path, crate_name, sql_library, framework)?;

    let src_path = &root_path.as_ref().join("src");

    writer::write_query_root(src_path, entities_path)?;
    writer::write_lib(src_path)?;

    match framework {
        WebFrameworkEnum::Actix => crate::templates::actix::write_main(src_path, crate_name)?,
        WebFrameworkEnum::Poem => crate::templates::poem::write_main(src_path, crate_name)?,
        WebFrameworkEnum::Axum => crate::templates::axum::write_main(src_path, crate_name)?,
    }

    writer::write_env(&root_path.as_ref(), db_url, depth_limit, complexity_limit)?;

    std::process::Command::new("cargo")
        .arg("fmt")
        .current_dir(root_path)
        .spawn()?
        .wait()?;

    Ok(())
}
