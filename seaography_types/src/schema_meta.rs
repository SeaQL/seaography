use serde::{Serialize, Deserialize};

use crate::table_meta::TableMeta;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchemaMeta {
    pub tables: Vec<TableMeta>,
    pub enums: Vec<String> // TODO for mysql, pgsql
}