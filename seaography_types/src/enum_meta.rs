use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumMeta {
    pub enum_name: String,
    pub enum_values: Vec<String>
}