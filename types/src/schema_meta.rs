use serde::{Deserialize, Serialize};

use crate::{enum_meta::EnumMeta, table_meta::TableMeta};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SqlVersion {
    Sqlite,
    Mysql,
    Postgres,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct SchemaMeta {
    pub tables: Vec<TableMeta>,
    pub enums: Vec<EnumMeta>,
    pub version: SqlVersion,
    pub url: String,
}
