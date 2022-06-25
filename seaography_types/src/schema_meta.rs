use serde::{Serialize, Deserialize};

use crate::{table_meta::TableMeta, enum_meta::EnumMeta};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchemaMeta {
    pub tables: Vec<TableMeta>,
    pub enums: Vec<EnumMeta>
}