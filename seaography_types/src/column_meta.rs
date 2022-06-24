use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use sea_query::{ColumnDef, ColumnSpec};
use serde::{Deserialize, Serialize};

use crate::column_type::{ColumnType, map_sea_query_column_type};

#[derive(Clone, Deserialize, Serialize)]
pub struct ColumnMeta {
    pub col_name: String,
    pub col_type: ColumnType,
    pub not_null: bool,
    pub is_primary_key: bool,
}

impl ColumnMeta {
    pub fn from_column_def(col: &ColumnDef) -> Self {
        let col_specs = col.get_column_spec();

        Self {
            col_name: col.get_column_name().to_snake_case(),
            col_type: map_sea_query_column_type(col.get_column_type().unwrap()),
            not_null: col_specs.iter().any(|spec| matches!(spec, ColumnSpec::NotNull)),
            is_primary_key: col_specs.iter().any(|spec| matches!(spec, ColumnSpec::PrimaryKey)),
        }
    }

    pub fn snake_case(&self) -> String {
        self.col_name.to_snake_case()
    }

    pub fn snake_case_ident(&self) -> TokenStream {
        self.snake_case().parse().unwrap()
    }

    pub fn camel_case(&self) -> String {
        self.col_name.to_upper_camel_case()
    }

    pub fn camel_case_ident(&self) -> TokenStream {
        self.camel_case().parse().unwrap()
    }

    /**
     * Based on here
     * https://github.com/SeaQL/sea-orm/blob/master/sea-orm-codegen/src/entity/column.rs#L30
     */
    pub fn get_base_type(&self) -> TokenStream {
        #[allow(unreachable_patterns)]
        match &self.col_type {
            ColumnType::String => "String".to_owned(),
            ColumnType::Integer8 => "i8".to_owned(),
            ColumnType::Integer16 => "i16".to_owned(),
            ColumnType::Integer32 => "i32".to_owned(),
            ColumnType::Integer64 => "i64".to_owned(),
            ColumnType::Unsigned8 => "u8".to_owned(),
            ColumnType::Unsigned16 => "u16".to_owned(),
            ColumnType::Unsigned32 => "u32".to_owned(),
            ColumnType::Unsigned64 => "u64".to_owned(),
            ColumnType::Float => "f32".to_owned(),
            ColumnType::Double => "f64".to_owned(),
            ColumnType::Json => "Json".to_owned(),
            ColumnType::Date => "Date".to_owned(),
            ColumnType::Time => "Time".to_owned(),
            ColumnType::DateTime => "DateTime".to_owned(),
            ColumnType::Timestamp => "DateTimeUtc".to_owned(),
            ColumnType::TimestampWithTimeZone => "DateTimeWithTimeZone".to_owned(),
            ColumnType::Decimal => "Decimal".to_owned(),
            ColumnType::Uuid => "Uuid".to_owned(),
            ColumnType::Binary => "Vec<u8>".to_owned(),
            ColumnType::Boolean => "bool".to_owned(),
            ColumnType::Enum(name) => name.to_upper_camel_case(),
            _ => unimplemented!(),
        }
        .parse()
        .unwrap()
    }

    pub fn get_type(&self) -> TokenStream {
        let ident: TokenStream = self.get_base_type();

        match self.not_null {
            true => quote! { #ident },
            false => quote! { Option<#ident> },
        }
    }
}

impl std::fmt::Debug for ColumnMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColumnMeta")
            .field("col_name", &self.col_name)
            .field("not_null", &self.not_null)
            .field("is_primary_key", &self.is_primary_key)
            .field("col_base_type", &self.get_base_type().to_string())
            .field("col_type", &self.get_type().to_string())
            .finish()
    }
}
