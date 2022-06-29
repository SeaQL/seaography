use heck::{ToSnakeCase, ToUpperCamelCase};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumMeta {
    pub enum_name: String,
    pub enum_values: Vec<String>
}

impl EnumMeta {
    pub fn camel_case(&self) -> String {
        self.enum_name.to_upper_camel_case()
    }

    pub fn snake_case(&self) -> String {
        self.enum_name.to_snake_case()
    }

    pub fn enums(&self) -> Vec<String> {
        self
            .enum_values
            .iter()
            .map(|enumeration| enumeration.to_upper_camel_case())
            .collect()
    }
}