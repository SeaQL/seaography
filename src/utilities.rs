use async_graphql::dynamic::TypeRef;
use itertools::Itertools;
use sea_orm::ColumnType;

use crate::{BuilderContext, ActiveEnumFilterInputBuilder};

/// used to encode the primary key values of a SeaORM entity to a String
pub fn encode_cursor(values: Vec<sea_orm::Value>) -> String {
    values
        .iter()
        .map(|value| -> String {
            match value {
                sea_orm::Value::TinyInt(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
                        format!("TinyInt[{}]:{}", value.len(), value)
                    } else {
                        "TinyInt[-1]:".into()
                    }
                }
                sea_orm::Value::SmallInt(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
                        format!("SmallInt[{}]:{}", value.len(), value)
                    } else {
                        "SmallInt[-1]:".into()
                    }
                }
                sea_orm::Value::Int(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
                        format!("Int[{}]:{}", value.len(), value)
                    } else {
                        "Int[-1]:".into()
                    }
                }
                sea_orm::Value::BigInt(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
                        format!("BigInt[{}]:{}", value.len(), value)
                    } else {
                        "BigInt[-1]:".into()
                    }
                }
                sea_orm::Value::TinyUnsigned(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
                        format!("TinyUnsigned[{}]:{}", value.len(), value)
                    } else {
                        "TinyUnsigned[-1]:".into()
                    }
                }
                sea_orm::Value::SmallUnsigned(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
                        format!("SmallUnsigned[{}]:{}", value.len(), value)
                    } else {
                        "SmallUnsigned[-1]:".into()
                    }
                }
                sea_orm::Value::Unsigned(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
                        format!("Unsigned[{}]:{}", value.len(), value)
                    } else {
                        "Unsigned[-1]:".into()
                    }
                }
                sea_orm::Value::BigUnsigned(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
                        format!("BigUnsigned[{}]:{}", value.len(), value)
                    } else {
                        "BigUnsigned[-1]:".into()
                    }
                }
                sea_orm::Value::String(value) => {
                    if let Some(value) = value {
                        let value = value.as_ref();
                        format!("String[{}]:{}", value.len(), value)
                    } else {
                        "String[-1]:".into()
                    }
                }
                #[cfg(feature = "with-uuid")]
                sea_orm::Value::Uuid(value) => {
                    if let Some(value) = value {
                        let value = value.as_ref().to_string();
                        format!("Uuid[{}]:{}", value.len(), value)
                    } else {
                        "Uuid[-1]:".into()
                    }
                }
                _ => {
                    // FIXME: missing value types
                    panic!("Cannot convert type to cursor")
                }
            }
        })
        .join(",")
}

#[derive(Debug)]
pub enum DecodeMode {
    Type,
    Length,
    ColonSkip,
    Data,
}

pub fn map_cursor_values(values: Vec<sea_orm::Value>) -> sea_orm::sea_query::value::ValueTuple {
    if values.len() == 1 {
        sea_orm::sea_query::value::ValueTuple::One(values[0].clone())
    } else if values.len() == 2 {
        sea_orm::sea_query::value::ValueTuple::Two(values[0].clone(), values[1].clone())
    } else if values.len() == 3 {
        sea_orm::sea_query::value::ValueTuple::Three(
            values[0].clone(),
            values[1].clone(),
            values[2].clone(),
        )
    } else {
        panic!("seaography does not support cursors values with size greater than 3")
    }
}

/// used to decode a String to a vector of SeaORM values
pub fn decode_cursor(s: &str) -> Result<Vec<sea_orm::Value>, sea_orm::error::DbErr> {
    let chars = s.chars();

    let mut values: Vec<sea_orm::Value> = vec![];

    let mut type_indicator = String::new();
    let mut length_indicator = String::new();
    let mut data_buffer = String::new();
    let mut length = -1;

    let mut mode: DecodeMode = DecodeMode::Type;
    for char in chars {
        match mode {
            DecodeMode::Type => {
                if char.eq(&'[') {
                    mode = DecodeMode::Length;
                } else if char.eq(&',') {
                    // SKIP
                } else {
                    type_indicator.push(char);
                }
            }
            DecodeMode::Length => {
                if char.eq(&']') {
                    mode = DecodeMode::ColonSkip;
                    length = length_indicator.parse::<i64>().unwrap();
                } else {
                    length_indicator.push(char);
                }
            }
            DecodeMode::ColonSkip => {
                // skips ':' char
                mode = DecodeMode::Data;
            }
            DecodeMode::Data => {
                if length > 0 {
                    data_buffer.push(char);
                    length -= 1;
                }

                if length <= 0 {
                    let value: sea_orm::Value = match type_indicator.as_str() {
                        "TinyInt" => {
                            if length.eq(&-1) {
                                sea_orm::Value::TinyInt(None)
                            } else {
                                sea_orm::Value::TinyInt(Some(data_buffer.parse::<i8>().unwrap()))
                            }
                        }
                        "SmallInt" => {
                            if length.eq(&-1) {
                                sea_orm::Value::SmallInt(None)
                            } else {
                                sea_orm::Value::SmallInt(Some(data_buffer.parse::<i16>().unwrap()))
                            }
                        }
                        "Int" => {
                            if length.eq(&-1) {
                                sea_orm::Value::Int(None)
                            } else {
                                sea_orm::Value::Int(Some(data_buffer.parse::<i32>().unwrap()))
                            }
                        }
                        "BigInt" => {
                            if length.eq(&-1) {
                                sea_orm::Value::BigInt(None)
                            } else {
                                sea_orm::Value::BigInt(Some(data_buffer.parse::<i64>().unwrap()))
                            }
                        }
                        "TinyUnsigned" => {
                            if length.eq(&-1) {
                                sea_orm::Value::TinyUnsigned(None)
                            } else {
                                sea_orm::Value::TinyUnsigned(Some(
                                    data_buffer.parse::<u8>().unwrap(),
                                ))
                            }
                        }
                        "SmallUnsigned" => {
                            if length.eq(&-1) {
                                sea_orm::Value::SmallUnsigned(None)
                            } else {
                                sea_orm::Value::SmallUnsigned(Some(
                                    data_buffer.parse::<u16>().unwrap(),
                                ))
                            }
                        }
                        "Unsigned" => {
                            if length.eq(&-1) {
                                sea_orm::Value::Unsigned(None)
                            } else {
                                sea_orm::Value::Unsigned(Some(data_buffer.parse::<u32>().unwrap()))
                            }
                        }
                        "BigUnsigned" => {
                            if length.eq(&-1) {
                                sea_orm::Value::BigUnsigned(None)
                            } else {
                                sea_orm::Value::BigUnsigned(Some(
                                    data_buffer.parse::<u64>().unwrap(),
                                ))
                            }
                        }
                        "String" => {
                            if length.eq(&-1) {
                                sea_orm::Value::String(None)
                            } else {
                                sea_orm::Value::String(Some(Box::new(
                                    data_buffer.parse::<String>().unwrap(),
                                )))
                            }
                        }
                        #[cfg(feature = "with-uuid")]
                        "Uuid" => {
                            if length.eq(&-1) {
                                sea_orm::Value::Uuid(None)
                            } else {
                                sea_orm::Value::Uuid(Some(Box::new(
                                    data_buffer.parse::<sea_orm::prelude::Uuid>().unwrap(),
                                )))
                            }
                        }
                        _ => {
                            // FIXME: missing value types
                            panic!("cannot encode current type")
                        }
                    };

                    values.push(value);

                    type_indicator = String::new();
                    length_indicator = String::new();
                    data_buffer = String::new();
                    length = -1;

                    mode = DecodeMode::Type;
                }
            }
        }
    }

    Ok(values)
}


/// used to map from a SeaORM column type to an async_graphql type
/// None indicates that we do not support the type
pub fn map_sea_orm_column_type_to_graphql_type(context: &'static BuilderContext, ty: &ColumnType) -> Option<String> {
    let active_enum_filter_input_builder = ActiveEnumFilterInputBuilder { context, };

    match ty {
        ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => Some(TypeRef::STRING.into()),
        ColumnType::TinyInteger
        | ColumnType::SmallInteger
        | ColumnType::Integer
        | ColumnType::BigInteger
        | ColumnType::TinyUnsigned
        | ColumnType::SmallUnsigned
        | ColumnType::Unsigned
        | ColumnType::BigUnsigned => Some(TypeRef::INT.into()),
        ColumnType::Float | ColumnType::Double => Some(TypeRef::FLOAT.into()),
        ColumnType::Decimal(_) | ColumnType::Money(_) => Some(TypeRef::STRING.into()),
        ColumnType::DateTime
        | ColumnType::Timestamp
        | ColumnType::TimestampWithTimeZone
        | ColumnType::Time
        | ColumnType::Date => Some(TypeRef::STRING.into()),
        ColumnType::Year(_) => Some(TypeRef::INT.into()),
        ColumnType::Interval(_, _) => Some(TypeRef::STRING.into()),
        // FIXME: binary type
        // ColumnType::Binary(_) |
        // ColumnType::VarBinary(_) |
        // ColumnType::Bit(_) |
        // ColumnType::VarBit(_) => Some(InputValue::new(
        //     column_name,
        //     TypeRef::named(&self.context.filter_input.text_type),
        // )),
        ColumnType::Boolean => Some(TypeRef::BOOLEAN.into()),
        // FIXME: json type
        // ColumnType::Json | ColumnType::JsonBinary => Some(InputValue::new(
        //     column_name,
        //     TypeRef::named(&self.context.filter_input.text_type),
        // )),
        ColumnType::Uuid => Some(TypeRef::STRING.into()),
        ColumnType::Enum {
            name: enum_name,
            variants: _,
        } => Some(active_enum_filter_input_builder.type_name_from_iden(enum_name)),
        // FIXME: cidr, inet, mac type
        ColumnType::Cidr | ColumnType::Inet | ColumnType::MacAddr => Some(TypeRef::STRING.into()),
        // FIXME: support array types
        // ColumnType::Array(_) => {}
        // FIXME: support custom types
        // ColumnType::Custom(iden) => {}
        _ => None,
    }
}

/// used to convert a GraphQL value into SeaORM value
pub fn map_graphql_value_to_sea_orm_value(_context: &'static BuilderContext, ty: &ColumnType, value: async_graphql::dynamic::ValueAccessor) -> Option<Result<sea_orm::Value, async_graphql::Error>> {
    match ty {
        ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => Some(value.string().map(|v| v.into())),
        ColumnType::TinyInteger
        | ColumnType::SmallInteger
        | ColumnType::Integer
        | ColumnType::BigInteger
        | ColumnType::TinyUnsigned
        | ColumnType::SmallUnsigned
        | ColumnType::Unsigned
        | ColumnType::BigUnsigned => Some(value.i64().map(|v| v.into())),
        ColumnType::Float | ColumnType::Double => Some(value.f64().map(|v| v.into())),
        ColumnType::Decimal(_) | ColumnType::Money(_) => Some(value.string().map(|v| v.into())),
        ColumnType::DateTime
        | ColumnType::Timestamp
        | ColumnType::TimestampWithTimeZone
        | ColumnType::Time
        | ColumnType::Date => Some(value.string().map(|v| v.into())),
        ColumnType::Year(_) => Some(value.i64().map(|v| v.into())),
        ColumnType::Interval(_, _) => Some(value.string().map(|v| v.into())),
        // FIXME: binary type
        // ColumnType::Binary(_) |
        // ColumnType::VarBinary(_) |
        // ColumnType::Bit(_) |
        // ColumnType::VarBit(_) => Some(InputValue::new(
        //     column_name,
        //     TypeRef::named(&self.context.filter_input.text_type),
        // )),
        ColumnType::Boolean => Some(value.boolean().map(|v| v.into())),
        // FIXME: json type
        // ColumnType::Json | ColumnType::JsonBinary => Some(InputValue::new(
        //     column_name,
        //     TypeRef::named(&self.context.filter_input.text_type),
        // )),
        ColumnType::Uuid => Some(value.string().map(|v| v.into())),
        ColumnType::Enum {
            name: _enum_name,
            variants: _,
        } => Some(value.enum_name().map(|v| v.into())),
        // FIXME: cidr, inet, mac type
        ColumnType::Cidr | ColumnType::Inet | ColumnType::MacAddr => Some(value.string().map(|v| v.into())),
        // FIXME: support array types
        // ColumnType::Array(_) => {}
        // FIXME: support custom types
        // ColumnType::Custom(iden) => {}
        _ => None,
    }
}
