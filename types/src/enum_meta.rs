use heck::{ToSnakeCase, ToUpperCamelCase};
use serde::{Serialize, Deserialize};

/// Used to represent enumeration metadata required by the generator crate
///
/// ```
/// use sea_query::ValueTuple::One;
/// use seaography_types::EnumMeta;
///
/// let enum_meta = EnumMeta {
///     enum_name: "user_rating".into(),
///     enum_values: vec![
///         "One-Star".into(),
///         "Two-Stars".into(),
///         "Three-Stars".into()
///     ],
/// };
///
/// assert_eq!(enum_meta.camel_case(), "UserRating");
/// assert_eq!(enum_meta.snake_case(), "user_rating");
/// assert_eq!(enum_meta.enums().join(","), "OneStar,TwoStars,ThreeStars");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
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