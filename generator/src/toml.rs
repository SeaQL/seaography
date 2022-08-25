use std::collections::BTreeMap;

use seaography_types::schema_meta::SqlVersion;
use serde::Serialize;

#[derive(Serialize)]
pub struct TomlStructure {
    package: BTreeMap<String, String>,
    dependencies: BTreeMap<String, DependencyInfo>,
    #[serde(rename(serialize = "dev-dependencies"))]
    dev: BTreeMap<String, DependencyInfo>,
    workspace: WorkspaceInfo,
}

#[derive(Serialize)]
pub struct DependencyInfo {
    pub version: Option<String>,
    pub path: Option<String>,
    pub features: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct WorkspaceInfo {
    members: Vec<String>,
}

impl TomlStructure {
    ///
    /// Used to contract a new rust toml project configuration
    ///
    pub fn new(crate_name: &str, sql_version: &SqlVersion) -> Self {
        let mut package: BTreeMap<String, String> = BTreeMap::new();

        package.insert("name".into(), crate_name.into());
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
                path: None,
                version: Some("0.7.0".into()),
                features: Some(vec![
                    sqlx_driver.into(),
                    "runtime-async-std-native-tls".into(),
                ]),
            },
        );
        dependencies.insert(
            "async-graphql".into(),
            DependencyInfo {
                path: None,
                version: Some("4.0.10".into()),
                features: Some(vec!["decimal".into(), "chrono".into(), "dataloader".into()]),
            },
        );
        dependencies.insert(
            "async-graphql-poem".into(),
            DependencyInfo {
                path: None,
                version: Some("4.0.10".into()),
                features: None,
            },
        );
        dependencies.insert(
            "heck".into(),
            DependencyInfo {
                path: None,
                version: Some("0.4.0".into()),
                features: None,
            },
        );
        dependencies.insert(
            "tokio".into(),
            DependencyInfo {
                path: None,
                version: Some("1.17.0".into()),
                features: Some(vec!["macros".into(), "rt-multi-thread".into()]),
            },
        );
        dependencies.insert(
            "poem".into(),
            DependencyInfo {
                path: None,
                version: Some("1.3.29".into()),
                features: None,
            },
        );

        dependencies.insert(
            "async-trait".into(),
            DependencyInfo {
                path: None,
                version: Some("0.1.53".into()),
                features: None,
            },
        );

        dependencies.insert(
            "tracing".into(),
            DependencyInfo {
                path: None,
                version: Some("0.1.34".into()),
                features: None,
            },
        );

        dependencies.insert(
            "tracing-subscriber".into(),
            DependencyInfo {
                path: None,
                version: Some("0.3.11".into()),
                features: None,
            },
        );

        dependencies.insert(
            "itertools".into(),
            DependencyInfo {
                path: None,
                version: Some("0.10.3".into()),
                features: None,
            },
        );

        dependencies.insert(
            "seaography_derive".into(),
            DependencyInfo {
                version: None,
                features: None,
                path: Some("../../derive".into()),
            },
        );

        let mut dev: BTreeMap<String, DependencyInfo> = BTreeMap::new();
        dev.insert(
            "serde_json".into(),
            DependencyInfo {
                path: None,
                version: Some("1.0.82".into()),
                features: None,
            },
        );

        Self {
            package,
            dependencies,
            dev,
            workspace: WorkspaceInfo { members: vec![] },
        }
    }
}
