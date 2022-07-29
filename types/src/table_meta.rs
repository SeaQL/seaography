use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{column_meta::ColumnMeta, relationship_meta::RelationshipMeta};

/// Used to represent table metadata required by the generator crate
///
/// ```
/// use seaography_types::{ColumnMeta, TableMeta, ColumnType, RelationshipMeta};
///
/// let id_column = ColumnMeta {
///     col_name: "id".into(),
///     col_type: ColumnType::Uuid,
///     not_null: true,
///     is_primary_key: true
/// };
///
/// let name_column = ColumnMeta {
///     col_name: "name".into(),
///     col_type: ColumnType::String,
///     not_null: true,
///     is_primary_key: false
/// };
///
/// let email_column = ColumnMeta {
///     col_name: "email".into(),
///     col_type: ColumnType::String,
///     not_null: false,
///     is_primary_key: false
/// };
/// let user_id_column = ColumnMeta {
///     col_name: "user_id".into(),
///     col_type: ColumnType::Uuid,
///     not_null: true,
///     is_primary_key: false
/// };
///
/// let user_order_relationship = RelationshipMeta {
///     src_table: "orders".into(),
///     dst_table: "users".into(),
///     src_cols: vec![user_id_column],
///     dst_cols: vec![id_column.clone()]
/// };
///
/// let table_meta = TableMeta {
///     table_name: "users".into(),
///     columns: vec![id_column, name_column, email_column],
///     relations: vec![user_order_relationship]
/// };
///
/// assert_eq!(table_meta.snake_case(), "users");
/// assert_eq!(table_meta.camel_case(), "Users");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct TableMeta {
    pub table_name: String,
    pub columns: Vec<ColumnMeta>,
    pub relations: Vec<RelationshipMeta>,
}

impl TableMeta {
    pub fn snake_case(&self) -> String {
        self.table_name.to_snake_case()
    }

    pub fn snake_case_ident(&self) -> TokenStream {
        self.snake_case().parse().unwrap()
    }

    pub fn camel_case(&self) -> String {
        self.table_name.to_upper_camel_case()
    }

    pub fn camel_case_ident(&self) -> TokenStream {
        self.camel_case().parse().unwrap()
    }
}
