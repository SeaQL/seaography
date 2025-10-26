use async_graphql::dynamic::FieldValue;
use itertools::Itertools;
use sea_orm::{sea_query::ValueTuple, DynIden, Identity};
use std::any::Any;

/// used to encode the primary key values of a SeaORM entity to a String
pub fn encode_cursor(values: ValueTuple) -> String {
    ValueTupleIter::new(&values)
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
                        format!("String[{}]:{}", value.len(), value)
                    } else {
                        "String[-1]:".into()
                    }
                }
                #[cfg(feature = "with-uuid")]
                sea_orm::Value::Uuid(value) => {
                    if let Some(value) = value {
                        let value = value.to_string();
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

#[derive(Default)]
struct ValueTupleBuilder(Option<ValueTuple>);

impl ValueTupleBuilder {
    fn push(&mut self, value: sea_orm::Value) {
        match self.0.take() {
            None => {
                self.0 = Some(ValueTuple::One(value));
            }
            Some(ValueTuple::One(a)) => {
                self.0 = Some(ValueTuple::Two(a, value));
            }
            Some(ValueTuple::Two(a, b)) => {
                self.0 = Some(ValueTuple::Three(a, b, value));
            }
            Some(ValueTuple::Three(a, b, c)) => {
                self.0 = Some(ValueTuple::Many(vec![a, b, c, value]));
            }
            Some(ValueTuple::Many(mut items)) => {
                items.push(value);
                self.0 = Some(ValueTuple::Many(items));
            }
        }
    }
}

/// used to decode a String to a vector of SeaORM values
pub fn decode_cursor(s: &str) -> Result<ValueTuple, sea_orm::DbErr> {
    let chars = s.chars();

    let mut values = ValueTupleBuilder::default();

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
                    length = length_indicator.parse::<i64>().map_err(parse_int_err)?;
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
                                sea_orm::Value::TinyInt(Some(
                                    data_buffer.parse::<i8>().map_err(parse_int_err)?,
                                ))
                            }
                        }
                        "SmallInt" => {
                            if length.eq(&-1) {
                                sea_orm::Value::SmallInt(None)
                            } else {
                                sea_orm::Value::SmallInt(Some(
                                    data_buffer.parse::<i16>().map_err(parse_int_err)?,
                                ))
                            }
                        }
                        "Int" => {
                            if length.eq(&-1) {
                                sea_orm::Value::Int(None)
                            } else {
                                sea_orm::Value::Int(Some(
                                    data_buffer.parse::<i32>().map_err(parse_int_err)?,
                                ))
                            }
                        }
                        "BigInt" => {
                            if length.eq(&-1) {
                                sea_orm::Value::BigInt(None)
                            } else {
                                sea_orm::Value::BigInt(Some(
                                    data_buffer.parse::<i64>().map_err(parse_int_err)?,
                                ))
                            }
                        }
                        "TinyUnsigned" => {
                            if length.eq(&-1) {
                                sea_orm::Value::TinyUnsigned(None)
                            } else {
                                sea_orm::Value::TinyUnsigned(Some(
                                    data_buffer.parse::<u8>().map_err(parse_int_err)?,
                                ))
                            }
                        }
                        "SmallUnsigned" => {
                            if length.eq(&-1) {
                                sea_orm::Value::SmallUnsigned(None)
                            } else {
                                sea_orm::Value::SmallUnsigned(Some(
                                    data_buffer.parse::<u16>().map_err(parse_int_err)?,
                                ))
                            }
                        }
                        "Unsigned" => {
                            if length.eq(&-1) {
                                sea_orm::Value::Unsigned(None)
                            } else {
                                sea_orm::Value::Unsigned(Some(
                                    data_buffer.parse::<u32>().map_err(parse_int_err)?,
                                ))
                            }
                        }
                        "BigUnsigned" => {
                            if length.eq(&-1) {
                                sea_orm::Value::BigUnsigned(None)
                            } else {
                                sea_orm::Value::BigUnsigned(Some(
                                    data_buffer.parse::<u64>().map_err(parse_int_err)?,
                                ))
                            }
                        }
                        "String" => {
                            if length.eq(&-1) {
                                sea_orm::Value::String(None)
                            } else {
                                sea_orm::Value::String(Some(data_buffer))
                            }
                        }
                        #[cfg(feature = "with-uuid")]
                        "Uuid" => {
                            if length.eq(&-1) {
                                sea_orm::Value::Uuid(None)
                            } else {
                                sea_orm::Value::Uuid(Some(
                                    data_buffer.parse::<sea_orm::prelude::Uuid>().map_err(|e| {
                                        sea_orm::DbErr::Type(format!("Failed to parse UUID: {e}"))
                                    })?,
                                ))
                            }
                        }
                        ty => {
                            return Err(sea_orm::DbErr::Type(format!(
                                "Unsupported type {ty} in cursor"
                            )));
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

    values
        .0
        .ok_or_else(|| sea_orm::DbErr::Type("Missing cursor value".into()))
}

#[cfg(feature = "field-pluralize")]
/// Returns unique names for singular and plural names,
/// so they can be used in different endpoints.
/// Right now a _single suffix is appended to the singular noun.
pub fn pluralize_unique(word: &str, plural: bool) -> String {
    use pluralizer::pluralize;
    let name_single = pluralize(word, 1, false);
    let name_plural = pluralize(word, 2, false);

    if plural {
        name_plural
    } else if name_single == name_plural {
        format!("{}_single", name_single)
    } else {
        name_single
    }
}

#[cfg(not(feature = "field-pluralize"))]
pub fn pluralize_unique(word: &str, _plural: bool) -> String {
    word.into()
}

fn parse_int_err(err: std::num::ParseIntError) -> sea_orm::DbErr {
    sea_orm::DbErr::Type(format!("Failed to parse integer: {err}"))
}

pub fn try_downcast_ref<'a, T: Any>(value: &'a FieldValue<'a>) -> async_graphql::Result<&'a T> {
    match value.downcast_ref::<T>() {
        Some(obj) => Ok(obj),
        None => Err(format!(
            "Cannot downcast {:?} to {}",
            value,
            std::any::type_name::<T>()
        )
        .into()),
    }
}

pub(crate) struct IdenIter<'a> {
    identity: &'a Identity,
    index: usize,
}

impl<'a> IdenIter<'a> {
    pub fn new(identity: &'a Identity) -> Self {
        Self { identity, index: 0 }
    }
}

impl<'a> Iterator for IdenIter<'a> {
    type Item = &'a DynIden;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.identity {
            Identity::Unary(iden1) => {
                if self.index == 0 {
                    Some(iden1)
                } else {
                    None
                }
            }
            Identity::Binary(iden1, iden2) => match self.index {
                0 => Some(iden1),
                1 => Some(iden2),
                _ => None,
            },
            Identity::Ternary(iden1, iden2, iden3) => match self.index {
                0 => Some(iden1),
                1 => Some(iden2),
                2 => Some(iden3),
                _ => None,
            },
            Identity::Many(vec) => vec.get(self.index),
        };
        self.index += 1;
        result
    }
}

pub(crate) struct ValueTupleIter<'a> {
    value: &'a ValueTuple,
    index: usize,
}

impl<'a> ValueTupleIter<'a> {
    pub fn new(value: &'a ValueTuple) -> Self {
        Self { value, index: 0 }
    }
}

impl<'a> Iterator for ValueTupleIter<'a> {
    type Item = &'a sea_orm::Value;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.value {
            ValueTuple::One(a) => {
                if self.index == 0 {
                    Some(a)
                } else {
                    None
                }
            }
            ValueTuple::Two(a, b) => match self.index {
                0 => Some(a),
                1 => Some(b),
                _ => None,
            },
            ValueTuple::Three(a, b, c) => match self.index {
                0 => Some(a),
                1 => Some(b),
                2 => Some(c),
                _ => None,
            },
            ValueTuple::Many(vec) => vec.get(self.index),
        };
        self.index += 1;
        result
    }
}
