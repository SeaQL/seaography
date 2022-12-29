use async_graphql::{dynamic::*, Value};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

impl PageInfo {
    pub fn to_object() -> Object {
        Object::new("PageInfo")
            .field(Field::new(
                "hasPreviousPage",
                TypeRef::named_nn(TypeRef::BOOLEAN),
                |ctx| {
                    FieldFuture::new(async move {
                        let cursor_page_info = ctx.parent_value.try_downcast_ref::<Self>()?;
                        Ok(Some(Value::from(cursor_page_info.has_previous_page)))
                    })
                },
            ))
            .field(Field::new(
                "hasNextPage",
                TypeRef::named_nn(TypeRef::BOOLEAN),
                |ctx| {
                    FieldFuture::new(async move {
                        let cursor_page_info = ctx.parent_value.try_downcast_ref::<Self>()?;
                        Ok(Some(Value::from(cursor_page_info.has_next_page)))
                    })
                },
            ))
            .field(Field::new(
                "startCursor",
                TypeRef::named(TypeRef::STRING),
                |ctx| {
                    FieldFuture::new(async move {
                        let cursor_page_info = ctx.parent_value.try_downcast_ref::<Self>()?;
                        let value = cursor_page_info
                            .start_cursor
                            .as_ref()
                            .map(|v| Value::from(v.as_str()))
                            .or_else(|| Some(Value::Null));
                        Ok(value)
                    })
                },
            ))
            .field(Field::new(
                "endCursor",
                TypeRef::named(TypeRef::STRING),
                |ctx| {
                    FieldFuture::new(async move {
                        let cursor_page_info = ctx.parent_value.try_downcast_ref::<Self>()?;
                        let value = cursor_page_info
                            .end_cursor
                            .as_ref()
                            .map(|v| Value::from(v.as_str()))
                            .or_else(|| Some(Value::Null));
                        Ok(value)
                    })
                },
            ))
    }
}

#[derive(Clone, Debug)]
pub struct PaginationInfo {
    pub pages: u64,
    pub current: u64,
}

impl PaginationInfo {
    pub fn to_object() -> Object {
        Object::new("PaginationInfo")
            .field(Field::new(
                "pages",
                TypeRef::named_nn(TypeRef::INT),
                |ctx| {
                    FieldFuture::new(async move {
                        let pagination_page_info = ctx.parent_value.try_downcast_ref::<Self>()?;
                        Ok(Some(Value::from(pagination_page_info.pages)))
                    })
                },
            ))
            .field(Field::new(
                "current",
                TypeRef::named_nn(TypeRef::INT),
                |ctx| {
                    FieldFuture::new(async move {
                        let pagination_page_info = ctx.parent_value.try_downcast_ref::<Self>()?;
                        Ok(Some(Value::from(pagination_page_info.current)))
                    })
                },
            ))
    }
}

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
                    panic!(
                        "cannot
                         current type"
                    )
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

// TODO parser to rust object
pub fn get_cursor_input() -> InputObject {
    InputObject::new("CursorInput")
        .field(InputValue::new("cursor", TypeRef::named(TypeRef::STRING)))
        .field(InputValue::new("limit", TypeRef::named_nn(TypeRef::INT)))
}

// TODO parser to rust object
pub fn get_page_input() -> InputObject {
    InputObject::new("PageInput")
        .field(InputValue::new("limit", TypeRef::named_nn(TypeRef::INT)))
        .field(InputValue::new("page", TypeRef::named_nn(TypeRef::INT)))
}

// TODO parser to rust object
pub fn get_pagination_input(cursor_input: &InputObject, page_input: &InputObject) -> InputObject {
    InputObject::new("PaginationInput")
        .field(InputValue::new(
            "Cursor",
            TypeRef::named(cursor_input.type_name()),
        ))
        .field(InputValue::new(
            "Pages",
            TypeRef::named(page_input.type_name()),
        ))
        .oneof()
}
