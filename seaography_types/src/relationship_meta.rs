use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::column_meta::ColumnMeta;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            false => self.dst_table.to_snake_case()
        }
    }

    pub fn snake_case_ident(&self, src: bool) -> TokenStream {
        self.snake_case(src).parse().unwrap()
    }

    pub fn camel_case(&self, src: bool) -> String {
        match src {
            true => self.src_table.to_upper_camel_case(),
            false => self.dst_table.to_upper_camel_case()
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
            false => &self.src_cols
        };

        cols.iter().any(|col: &ColumnMeta| !col.not_null)
    }

    pub fn get_optional_cols(&self, is_reverse: bool) -> Vec<bool> {
        let cols: &Vec<ColumnMeta> = match is_reverse {
            true => &self.dst_cols,
            false => &self.src_cols
        };

        cols.iter().map(|col: &ColumnMeta| !col.not_null).collect()
    }
}
