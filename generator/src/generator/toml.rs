use std::{collections::BTreeMap, io, path::Path};

use seaography_types::schema_meta::SqlVersion;
use serde::Serialize;

#[derive(Serialize)]
pub struct TomlStructure {
    package: BTreeMap<String, String>,
    dependencies: BTreeMap<String, DependencyInfo>,
}

#[derive(Serialize)]
pub struct DependencyInfo {
    pub version: String,
    pub features: Option<Vec<String>>,
}

impl TomlStructure {
    /// Used to contract a new rust toml project configuration
    ///
    /// ```
    /// use seaography_generator::generator::toml::TomlStructure;
    /// use seaography_types::SqlVersion;
    ///
    /// let left = TomlStructure::new(&"generated".into(), &SqlVersion::Sqlite);
    ///
    /// let right = r#"[package]
    /// edition = '2021'
    /// name = 'generated'
    /// version = '0.1.0'
    /// [dependencies.async-graphql]
    /// version = '3.0.38'
    /// features = [
    ///     'decimal',
    ///     'chrono',
    ///     'dataloader',
    /// ]
    ///
    /// [dependencies.async-graphql-poem]
    /// version = '3.0.38'
    ///
    /// [dependencies.async-trait]
    /// version = '0.1.53'
    ///
    /// [dependencies.itertools]
    /// version = '0.10.3'
    ///
    /// [dependencies.poem]
    /// version = '1.3.29'
    ///
    /// [dependencies.sea-orm]
    /// version = '0.7.0'
    /// features = [
    ///     'sqlx-sqlite',
    ///     'runtime-async-std-native-tls',
    /// ]
    ///
    /// [dependencies.tokio]
    /// version = '1.17.0'
    /// features = [
    ///     'macros',
    ///     'rt-multi-thread',
    /// ]
    ///
    /// [dependencies.tracing]
    /// version = '0.1.34'
    ///
    /// [dependencies.tracing-subscriber]
    /// version = '0.3.11'
    /// "#;
    ///
    /// assert_eq!(toml::to_string_pretty(&left).unwrap(), right);
    /// ```
    pub fn new(crate_name: &String, sql_version: &SqlVersion) -> Self {
        let mut package: BTreeMap<String, String> = BTreeMap::new();

        package.insert("name".into(), crate_name.clone());
        package.insert("version".into(), "0.1.0".into());
        package.insert("edition".into(), "2021".into());

        let sqlx_driver = match sql_version {
            SqlVersion::Sqlite => "sqlx-sqlite",
            SqlVersion::Mysql => "sqlx-mysql",
            SqlVersion::Postgres => "sqlx-postgres",
        };

        let mut dependencies: BTreeMap<String, DependencyInfo> = BTreeMap::new();
        dependencies.insert(
            "sea-orm".into(),
            DependencyInfo {
                version: "0.7.0".into(),
                features: Some(vec![
                    sqlx_driver.into(),
                    "runtime-async-std-native-tls".into(),
                ]),
            },
        );
        dependencies.insert(
            "async-graphql".into(),
            DependencyInfo {
                version: "3.0.38".into(),
                features: Some(vec!["decimal".into(), "chrono".into(), "dataloader".into()]),
            },
        );
        dependencies.insert(
            "async-graphql-poem".into(),
            DependencyInfo {
                version: "3.0.38".into(),
                features: None,
            },
        );
        dependencies.insert(
            "tokio".into(),
            DependencyInfo {
                version: "1.17.0".into(),
                features: Some(vec!["macros".into(), "rt-multi-thread".into()]),
            },
        );
        dependencies.insert(
            "poem".into(),
            DependencyInfo {
                version: "1.3.29".into(),
                features: None,
            },
        );

        dependencies.insert(
            "async-trait".into(),
            DependencyInfo {
                version: "0.1.53".into(),
                features: None,
            },
        );

        dependencies.insert(
            "tracing".into(),
            DependencyInfo {
                version: "0.1.34".into(),
                features: None,
            },
        );

        dependencies.insert(
            "tracing-subscriber".into(),
            DependencyInfo {
                version: "0.3.11".into(),
                features: None,
            },
        );

        dependencies.insert(
            "itertools".into(),
            DependencyInfo {
                version: "0.10.3".into(),
                features: None,
            },
        );

        Self {
            package,
            dependencies,
        }
    }
}

pub fn write_cargo_toml<P: AsRef<Path>>(
    path: &P,
    crate_name: &String,
    sql_version: &SqlVersion,
) -> io::Result<()> {
    let file_path = path.as_ref().join("Cargo.toml");

    let data = TomlStructure::new(crate_name, sql_version);

    std::fs::write(file_path, toml::to_string_pretty(&data).unwrap())?;

    Ok(())
}
