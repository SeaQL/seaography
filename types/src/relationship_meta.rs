use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::column_meta::ColumnMeta;

/// Used to represent relationship metadata required by the generator crate
///
/// ```
/// use seaography_types::{ColumnMeta, RelationshipMeta, ColumnType};
///
/// let src_product_id = ColumnMeta {
///     col_name: "product_id".into(),
///     col_type: ColumnType::Uuid,
///     not_null: true,
///     is_primary_key: true
/// };
/// let src_product_item_id = ColumnMeta {
///     col_name: "product_item_id".into(),
///     col_type: ColumnType::Integer64,
///     not_null: true,
///     is_primary_key: true
/// };
///
/// let dst_id = ColumnMeta {
///     col_name: "id".into(),
///     col_type: ColumnType::Uuid,
///     not_null: true,
///     is_primary_key: true
/// };
/// let dst_item_id = ColumnMeta {
///     col_name: "item_id".into(),
///     col_type: ColumnType::Integer64,
///     not_null: true,
///     is_primary_key: true
/// };
///
/// let relation_meta = RelationshipMeta {
///     src_table: "OrderItems".into(),
///     dst_table: "ProductItems".into(),
///
///     src_cols: vec![src_product_id, src_product_item_id],
///     dst_cols: vec![dst_id, dst_item_id]
/// };
///
/// assert_eq!(relation_meta.snake_case(true), "order_items");
/// assert_eq!(relation_meta.snake_case(false), "product_items");
///
/// assert_eq!(relation_meta.camel_case(true), "OrderItems");
/// assert_eq!(relation_meta.camel_case(false), "ProductItems");
///
/// assert_eq!(relation_meta.is_reverse(&String::from("OrderItems")), false);
/// assert_eq!(relation_meta.is_reverse(&String::from("ProductItems")), true);
///
/// assert_eq!(relation_meta.is_optional(true), false);
/// assert_eq!(relation_meta.is_optional(false), false);
///
/// assert_eq!(relation_meta.get_optional_cols(true), vec![false, false]);
/// assert_eq!(relation_meta.get_optional_cols(false), vec![false, false]);
///
/// assert_eq!(relation_meta.extract_source_name(true), "id_item");
/// assert_eq!(relation_meta.extract_source_name(false), "product_product_item");
///
/// assert_eq!(relation_meta.retrieve_name(true), "id_item_order_items");
/// assert_eq!(relation_meta.retrieve_name(false), "product_product_item_product_items");
///
/// assert_eq!(relation_meta.retrieve_foreign_key(true), "IdItemOrderItemsFK");
/// assert_eq!(relation_meta.retrieve_foreign_key(false), "ProductProductItemProductItemsFK");
/// ```
///
/// ```
/// use seaography_types::{ColumnMeta, RelationshipMeta, ColumnType};
///
/// let src_column = ColumnMeta {
///     col_name: "product_id".into(),
///     col_type: ColumnType::Uuid,
///     not_null: false,
///     is_primary_key: true
/// };
///
/// let dst_column = ColumnMeta {
///     col_name: "id".into(),
///     col_type: ColumnType::Uuid,
///     not_null: true,
///     is_primary_key: true
/// };
///
/// let relation_meta = RelationshipMeta {
///     src_table: "OrderItems".into(),
///     dst_table: "ProductItems".into(),
///
///     src_cols: vec![src_column],
///     dst_cols: vec![dst_column]
/// };
///
/// assert_eq!(relation_meta.is_optional(true), false);
/// assert_eq!(relation_meta.is_optional(false), true);
///
/// assert_eq!(relation_meta.get_optional_cols(true), vec![false]);
/// assert_eq!(relation_meta.get_optional_cols(false), vec![true]);
///
/// assert_eq!(relation_meta.extract_source_name(true), "id");
/// assert_eq!(relation_meta.extract_source_name(false), "product");
///
/// assert_eq!(relation_meta.retrieve_name(true), "id_order_items");
/// assert_eq!(relation_meta.retrieve_name(false), "product_product_items");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct RelationshipMeta {
    pub src_table: String,
    pub dst_table: String,

    pub src_cols: Vec<ColumnMeta>,
    pub dst_cols: Vec<ColumnMeta>,
}

impl RelationshipMeta {
    pub fn snake_case(&self, src: bool) -> String {
        match src {
            true => self.src_table.to_snake_case(),
            false => self.dst_table.to_snake_case(),
        }
    }

    pub fn snake_case_ident(&self, src: bool) -> TokenStream {
        self.snake_case(src).parse().unwrap()
    }

    pub fn camel_case(&self, src: bool) -> String {
        match src {
            true => self.src_table.to_upper_camel_case(),
            false => self.dst_table.to_upper_camel_case(),
        }
    }

    pub fn camel_case_ident(&self, src: bool) -> TokenStream {
        self.camel_case(src).parse().unwrap()
    }

    pub fn is_reverse(&self, table_name: &String) -> bool {
        self.dst_table.eq(table_name)
    }

    pub fn is_optional(&self, is_reverse: bool) -> bool {
        let cols: &Vec<ColumnMeta> = match is_reverse {
            true => &self.dst_cols,
            false => &self.src_cols,
        };

        cols.iter().any(|col: &ColumnMeta| !col.not_null)
    }

    pub fn get_optional_cols(&self, is_reverse: bool) -> Vec<bool> {
        let cols: &Vec<ColumnMeta> = match is_reverse {
            true => &self.dst_cols,
            false => &self.src_cols,
        };

        cols.iter().map(|col: &ColumnMeta| !col.not_null).collect()
    }

    pub fn extract_source_name(&self, is_reverse: bool) -> String {
        let source_columns = if is_reverse {
            &self.dst_cols
        } else {
            &self.src_cols
        };

        source_columns
            .clone()
            .into_iter()
            .map(|column| column.snake_case())
            .map(|s: String| {
                if s.ends_with("_id") {
                    String::from(s.split_at(s.len() - 3).0)
                } else {
                    s
                }
            })
            .collect::<Vec<String>>()
            .join("_")
    }

    pub fn retrieve_name(&self, is_reverse: bool) -> String {
        let destination_entity = self.snake_case(is_reverse);

        let source_name = self.extract_source_name(is_reverse).to_snake_case();

        format!("{}_{}", source_name, destination_entity)
    }

    pub fn retrieve_foreign_key(&self, is_reverse: bool) -> String {
        let destination_entity = self.camel_case(is_reverse);

        let source_name = self.extract_source_name(is_reverse).to_upper_camel_case();

        format!("{}{}FK", source_name, destination_entity)
    }
}
