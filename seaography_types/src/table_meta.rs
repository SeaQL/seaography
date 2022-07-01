use heck::{ToUpperCamelCase, ToSnakeCase};
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};

use crate::{column_meta::ColumnMeta, relationship_meta::RelationshipMeta};

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
