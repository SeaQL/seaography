use crate as seaography;
use crate::CustomOutputType;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize, CustomOutputType)]
pub struct Table {
    pub columns: Vec<Column>,
    pub primary_key: Vec<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, CustomOutputType)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: ColumnType,
    pub nullable: bool,
    pub unique: Option<bool>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, CustomOutputType)]
pub struct ColumnType {
    pub primitive: Option<String>,
    pub array: Option<Array>,
    pub enumeration: Option<Enumeration>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, CustomOutputType)]
pub struct Array {
    pub array: Box<ColumnType>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, CustomOutputType)]
pub struct Enumeration {
    pub name: String,
    pub variants: Vec<String>, // this requires `postgres-array`
}

mod inner {
    use super::*;
    use serde::de::{self, Deserializer, MapAccess, Visitor};

    impl<'de> Deserialize<'de> for ColumnType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(ColumnTypeVisitor)
        }
    }

    struct ColumnTypeVisitor;

    impl<'de> Visitor<'de> for ColumnTypeVisitor {
        type Value = ColumnType;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a primitive string, an array object, or an enumeration object")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(ColumnType {
                primitive: Some(value.to_string()),
                array: None,
                enumeration: None,
            })
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            // Deserialize into a temporary map to inspect keys
            use serde_json::Value;
            let mut value_map = serde_json::Map::new();

            while let Some((key, value)) = map.next_entry::<String, Value>()? {
                value_map.insert(key, value);
            }

            let json_value = Value::Object(value_map);

            // Try to deserialize as Array
            if let Ok(array) = Array::deserialize(json_value.clone()) {
                return Ok(ColumnType {
                    primitive: None,
                    array: Some(array),
                    enumeration: None,
                });
            }

            // Try to deserialize as Enumeration
            if let Ok(enumeration) = Enumeration::deserialize(json_value.clone()) {
                return Ok(ColumnType {
                    primitive: None,
                    array: None,
                    enumeration: Some(enumeration),
                });
            }

            Err(de::Error::custom("Unknown ColumnType variant"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deser_schema() {
        let table: Table = serde_json::from_str(
            &r#"{
          "columns": [
            {
              "name": "film_id",
              "nullable": false,
              "type": "integer"
            },
            {
              "name": "title",
              "nullable": false,
              "type": "string"
            },
            {
              "name": "last_update",
              "nullable": false,
              "type": "datetime"
            },
            {
              "name": "rating",
              "nullable": true,
              "type": {
                "name": "mpaa_rating",
                "variants": [
                  "G",
                  "NC-17",
                  "PG",
                  "PG-13",
                  "R"
                ]
              }
            },
            {
              "name": "special_features",
              "nullable": true,
              "type": {
                "array": "string"
              }
            }
          ],
          "primary_key": [
            "film_id"
          ]
        }"#,
        )
        .unwrap();

        assert_eq!(
            table,
            Table {
                columns: vec![
                    Column {
                        name: "film_id".into(),
                        nullable: false,
                        unique: None,
                        type_: ColumnType {
                            primitive: Some("integer".into()),
                            array: None,
                            enumeration: None
                        },
                        comment: None,
                    },
                    Column {
                        name: "title".into(),
                        nullable: false,
                        unique: None,
                        type_: ColumnType {
                            primitive: Some("string".into()),
                            array: None,
                            enumeration: None
                        },
                        comment: None,
                    },
                    Column {
                        name: "last_update".into(),
                        nullable: false,
                        unique: None,
                        type_: ColumnType {
                            primitive: Some("datetime".into()),
                            array: None,
                            enumeration: None
                        },
                        comment: None,
                    },
                    Column {
                        name: "rating".into(),
                        nullable: true,
                        unique: None,
                        type_: ColumnType {
                            primitive: None,
                            array: None,
                            enumeration: Some(Enumeration {
                                name: "mpaa_rating".into(),
                                variants: vec![
                                    "G".into(),
                                    "NC-17".into(),
                                    "PG".into(),
                                    "PG-13".into(),
                                    "R".into(),
                                ],
                            })
                        },
                        comment: None,
                    },
                    Column {
                        name: "special_features".into(),
                        nullable: true,
                        unique: None,
                        type_: ColumnType {
                            primitive: None,
                            array: Some(Array {
                                array: ColumnType {
                                    primitive: Some("string".into()),
                                    array: None,
                                    enumeration: None,
                                }
                                .into(),
                            }),
                            enumeration: None
                        },
                        comment: None,
                    },
                ],
                primary_key: vec!["film_id".into()],
                comment: None,
            }
        )
    }
}
