use std::{collections::BTreeMap, num::ParseIntError};

use async_graphql::dynamic::{TypeRef, ValueAccessor};
use sea_orm::{ColumnTrait, ColumnType, EntityTrait};

use crate::{ActiveEnumBuilder, BuilderContext, EntityObjectBuilder, SeaResult};

pub type FnInputTypeConversion = Box<dyn Fn(&ValueAccessor) -> SeaResult<sea_orm::Value> + Send + Sync>;
pub type FnOutputTypeConversion = Box<dyn Fn(&sea_orm::sea_query::Value) -> SeaResult<async_graphql::Value> + Send + Sync>;

/// Used to provide configuration for TypesMapHelper
pub struct TypesMapConfig {
    /// used to map entity_name.column_name to a custom Type
    pub overwrites: BTreeMap<String, ConvertedType>,
    /// used to map entity_name.column_name input to a custom parser
    pub input_conversions: BTreeMap<String, FnInputTypeConversion>,
    /// used to map entity_name.column_name output to a custom formatter
    pub output_conversions: BTreeMap<String, FnOutputTypeConversion>,
    /// used to configure default time library
    pub time_library: TimeLibrary,
    /// used to configure default decimal library
    pub decimal_library: DecimalLibrary,
}

impl std::default::Default for TypesMapConfig {
    fn default() -> Self {
        Self {
            overwrites: BTreeMap::new(),
            input_conversions: BTreeMap::new(),
            output_conversions: BTreeMap::new(),

            #[cfg(all(not(feature = "with-time"), not(feature = "with-chrono")))]
            time_library: TimeLibrary::String,
            #[cfg(feature = "with-time")]
            time_library: TimeLibrary::Time,
            #[cfg(all(not(feature = "with-time"), feature = "with-chrono"))]
            time_library: TimeLibrary::Chrono,

            #[cfg(all(not(feature = "with-decimal"), not(feature = "with-bigdecimal")))]
            decimal_library: DecimalLibrary::String,
            #[cfg(feature = "with-decimal")]
            decimal_library: DecimalLibrary::Decimal,
            #[cfg(all(not(feature = "with-decimal"), feature = "with-bigdecimal"))]
            decimal_library: DecimalLibrary::BigDecimal,
        }
    }
}

/// The TypesMapHelper is used to provide type mapping for entity objects
pub struct TypesMapHelper {
    pub context: &'static BuilderContext,
}

impl TypesMapHelper {
    /// used to map a sea_orm column to the converted type target
    pub fn get_column_type<T>(&self, column: &T::Column) -> ConvertedType
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let entity_name = entity_object_builder.type_name::<T>();
        let column_name = entity_object_builder.column_name::<T>(column);

        self.get_column_type_helper(&entity_name, &column_name, column.def().get_column_type())
    }

    /// helper function used to determine the conversion type of a column type
    fn get_column_type_helper(
        &self,
        entity_name: &str,
        column_name: &str,
        column_type: &ColumnType,
    ) -> ConvertedType {
        let context = self.context;
        let name = format!("{}.{}", entity_name, column_name);

        if let Some(overwrite) = context.types.overwrites.get(&name) {
            return overwrite.clone();
        }

        match column_type {
            ColumnType::Char(length) => match length {
                Some(length) => {
                    if length > &1 {
                        ConvertedType::String
                    } else {
                        ConvertedType::Char
                    }
                }
                None => ConvertedType::Char,
            },
            ColumnType::String(_) => ConvertedType::String,
            ColumnType::Text => ConvertedType::String,
            ColumnType::TinyInteger => ConvertedType::TinyInteger,
            ColumnType::SmallInteger => ConvertedType::SmallInteger,
            ColumnType::Integer => ConvertedType::Integer,
            ColumnType::BigInteger => ConvertedType::BigInteger,
            ColumnType::TinyUnsigned => ConvertedType::TinyUnsigned,
            ColumnType::SmallUnsigned => ConvertedType::SmallUnsigned,
            ColumnType::Unsigned => ConvertedType::Unsigned,
            ColumnType::BigUnsigned => ConvertedType::BigUnsigned,
            ColumnType::Float => ConvertedType::Float,
            ColumnType::Double => ConvertedType::Double,
            ColumnType::Money(_) | ColumnType::Decimal(_) => match context.types.decimal_library {
                DecimalLibrary::String => ConvertedType::String,
                #[cfg(feature = "with-decimal")]
                DecimalLibrary::Decimal => ConvertedType::Decimal,
                #[cfg(feature = "with-bigdecimal")]
                DecimalLibrary::BigDecimal => ConvertedType::BigDecimal,
            },
            ColumnType::DateTime => match context.types.time_library {
                TimeLibrary::String => ConvertedType::String,
                #[cfg(feature = "with-time")]
                TimeLibrary::Time => ConvertedType::TimeDateTime,
                #[cfg(feature = "with-chrono")]
                TimeLibrary::Chrono => ConvertedType::ChronoDateTime,
            },
            ColumnType::Timestamp => match context.types.time_library {
                TimeLibrary::String => ConvertedType::String,
                #[cfg(feature = "with-time")]
                TimeLibrary::Time => ConvertedType::TimeDateTime,
                #[cfg(feature = "with-chrono")]
                TimeLibrary::Chrono => ConvertedType::ChronoDateTimeUtc,
            },
            ColumnType::TimestampWithTimeZone => match context.types.time_library {
                TimeLibrary::String => ConvertedType::String,
                #[cfg(feature = "with-time")]
                TimeLibrary::Time => ConvertedType::TimeDateTime,
                #[cfg(feature = "with-chrono")]
                TimeLibrary::Chrono => ConvertedType::ChronoDateTimeUtc,
            },
            ColumnType::Time => match context.types.time_library {
                TimeLibrary::String => ConvertedType::String,
                #[cfg(feature = "with-time")]
                TimeLibrary::Time => ConvertedType::TimeTime,
                #[cfg(feature = "with-chrono")]
                TimeLibrary::Chrono => ConvertedType::ChronoTime,
            },
            ColumnType::Date => match context.types.time_library {
                TimeLibrary::String => ConvertedType::String,
                #[cfg(feature = "with-time")]
                TimeLibrary::Time => ConvertedType::TimeDate,
                #[cfg(feature = "with-chrono")]
                TimeLibrary::Chrono => ConvertedType::ChronoDate,
            },
            ColumnType::Year(_) => ConvertedType::Integer,
            ColumnType::Interval(_, _) => ConvertedType::String,
            ColumnType::Binary(_)
            | ColumnType::VarBinary(_)
            | ColumnType::Bit(_)
            | ColumnType::VarBit(_) => ConvertedType::Bytes,
            ColumnType::Boolean => ConvertedType::Bool,

            #[cfg(not(feature = "with-json"))]
            ColumnType::Json => ConvertedType::String,
            #[cfg(feature = "with-json")]
            ColumnType::Json => ConvertedType::Json,

            // FIXME: how should we map them JsonBinary type ?
            // #[cfg(feature = "with-json")]
            // ColumnType::JsonBinary => ConvertedType::Json,
            ColumnType::JsonBinary => ConvertedType::String,

            #[cfg(not(feature = "with-uuid"))]
            ColumnType::Uuid => ConvertedType::String,
            #[cfg(feature = "with-uuid")]
            ColumnType::Uuid => ConvertedType::Uuid,
            ColumnType::Custom(name) => ConvertedType::Custom(name.to_string()),
            ColumnType::Enum {
                name,
                variants: _variants,
            } => ConvertedType::Enum(name.to_string()),

            #[cfg(not(feature = "postgres-array"))]
            ColumnType::Array(_) => ConvertedType::String,
            #[cfg(feature = "postgres-array")]
            ColumnType::Array(ty) => {
                let inner = self.map_sea_orm_type_helper(
                    entity_name,
                    &format!("{}.array", column_name),
                    column_type,
                );
                ConvertedType::Array(inner)
            }

            #[cfg(not(feature = "with-ipnetwork"))]
            ColumnType::Cidr | ColumnType::Inet => ConvertedType::String,
            #[cfg(feature = "with-ipnetwork")]
            ColumnType::Cidr | ColumnType::Inet => ConvertedType::IpNetwork,

            #[cfg(not(feature = "with-mac_address"))]
            ColumnType::MacAddr => ConvertedType::String,
            #[cfg(feature = "with-mac_address")]
            ColumnType::MacAddr => ConvertedType::MacAddress,
            _ => panic!(
                "Type mapping is not implemented for '{}.{}'",
                entity_name, column_name
            ),
        }
    }

    /// used to convert a GraphQL value into SeaORM value
    pub fn async_graphql_value_to_sea_orm_value<T>(
        &self,
        column: &T::Column,
        value: &ValueAccessor,
    ) -> SeaResult<sea_orm::Value>
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let entity_name = entity_object_builder.type_name::<T>();
        let column_name = entity_object_builder.column_name::<T>(column);

        if let Some(parser) = self
            .context
            .types
            .input_conversions
            .get(&format!("{}.{}", entity_name, column_name))
        {
            return parser.as_ref()(value);
        }

        Ok(match self.get_column_type::<T>(column) {
            ConvertedType::Bool => value.boolean().map(|v| v.into())?,
            ConvertedType::TinyInteger => {
                let value: i8 = value.i64()?.try_into()?;
                sea_orm::Value::TinyInt(Some(value))
            }
            ConvertedType::SmallInteger => {
                let value: i16 = value.i64()?.try_into()?;
                sea_orm::Value::SmallInt(Some(value))
            }
            ConvertedType::Integer => {
                let value: i32 = value.i64()?.try_into()?;
                sea_orm::Value::Int(Some(value))
            }
            ConvertedType::BigInteger => {
                let value = value.i64()?;
                sea_orm::Value::BigInt(Some(value))
            }
            ConvertedType::TinyUnsigned => {
                let value: u8 = value.u64()?.try_into()?;
                sea_orm::Value::TinyUnsigned(Some(value))
            }
            ConvertedType::SmallUnsigned => {
                let value: u16 = value.u64()?.try_into()?;
                sea_orm::Value::SmallUnsigned(Some(value))
            }
            ConvertedType::Unsigned => {
                let value: u32 = value.u64()?.try_into()?;
                sea_orm::Value::Unsigned(Some(value))
            }
            ConvertedType::BigUnsigned => {
                let value = value.u64()?;
                sea_orm::Value::BigUnsigned(Some(value))
            }
            ConvertedType::Float => {
                let value = value.f32()?;
                sea_orm::Value::Float(Some(value))
            }
            ConvertedType::Double => {
                let value = value.f64()?;
                sea_orm::Value::Double(Some(value))
            }
            ConvertedType::String | ConvertedType::Enum(_) | ConvertedType::Custom(_) => {
                let value = value.string()?;
                sea_orm::Value::String(Some(Box::new(value.to_string())))
            }
            ConvertedType::Char => {
                let value = value.string()?;
                let value: char = match value.chars().next() {
                    Some(value) => value,
                    None => return Ok(sea_orm::Value::Char(None)),
                };
                sea_orm::Value::Char(Some(value))
            }
            ConvertedType::Bytes => {
                let value = decode_hex(value.string()?)?;
                sea_orm::Value::Bytes(Some(Box::new(value)))
            }
            #[cfg(feature = "with-json")]
            ConvertedType::Json => {
                use std::str::FromStr;

                let value =
                    sea_orm::entity::prelude::Json::from_str(value.string()?).map_err(|e| {
                        crate::SeaographyError::TypeConversionError(
                            e.to_string(),
                            format!("Json - {}.{}", entity_name, column_name),
                        )
                    })?;

                sea_orm::Value::Json(Some(Box::new(value)))
            }
            #[cfg(feature = "with-chrono")]
            ConvertedType::ChronoDate => {
                let value = sea_orm::entity::prelude::ChronoDate::parse_from_str(
                    value.string()?,
                    "%Y-%m-%d",
                )
                .map_err(|e| {
                    crate::SeaographyError::TypeConversionError(
                        e.to_string(),
                        format!("ChronoDate - {}.{}", entity_name, column_name),
                    )
                })?;

                sea_orm::Value::ChronoDate(Some(Box::new(value)))
            }
            #[cfg(feature = "with-chrono")]
            ConvertedType::ChronoTime => {
                let value = sea_orm::entity::prelude::ChronoTime::parse_from_str(
                    value.string()?,
                    "%H:%M:%S",
                )
                .map_err(|e| {
                    crate::SeaographyError::TypeConversionError(
                        e.to_string(),
                        format!("ChronoTime - {}.{}", entity_name, column_name),
                    )
                })?;

                sea_orm::Value::ChronoTime(Some(Box::new(value)))
            }
            #[cfg(feature = "with-chrono")]
            ConvertedType::ChronoDateTime => {
                let value = sea_orm::entity::prelude::ChronoDateTime::parse_from_str(
                    value.string()?,
                    "%Y-%m-%d %H:%M:%S",
                )
                .map_err(|e| {
                    crate::SeaographyError::TypeConversionError(
                        e.to_string(),
                        format!("ChronoDateTime - {}.{}", entity_name, column_name),
                    )
                })?;

                sea_orm::Value::ChronoDateTime(Some(Box::new(value)))
            }
            #[cfg(feature = "with-chrono")]
            ConvertedType::ChronoDateTimeUtc => {
                use std::str::FromStr;

                let value = sea_orm::entity::prelude::ChronoDateTimeUtc::from_str(value.string()?)
                    .map_err(|e| {
                        crate::SeaographyError::TypeConversionError(
                            e.to_string(),
                            format!("ChronoDateTimeUtc - {}.{}", entity_name, column_name),
                        )
                    })?;

                sea_orm::Value::ChronoDateTimeUtc(Some(Box::new(value)))
            }
            #[cfg(feature = "with-chrono")]
            ConvertedType::ChronoDateTimeLocal => {
                use std::str::FromStr;

                let value =
                    sea_orm::entity::prelude::ChronoDateTimeLocal::from_str(value.string()?)
                        .map_err(|e| {
                            crate::SeaographyError::TypeConversionError(
                                e.to_string(),
                                format!("ChronoDateTimeLocal - {}.{}", entity_name, column_name),
                            )
                        })?;

                sea_orm::Value::ChronoDateTimeLocal(Some(Box::new(value)))
            }
            #[cfg(feature = "with-chrono")]
            ConvertedType::ChronoDateTimeWithTimeZone => {
                let value = sea_orm::entity::prelude::ChronoDateTimeWithTimeZone::parse_from_str(
                    value.string()?,
                    "%Y-%m-%d %H:%M:%S %:z",
                )
                .map_err(|e| {
                    crate::SeaographyError::TypeConversionError(
                        e.to_string(),
                        format!(
                            "ChronoDateTimeWithTimeZone - {}.{}",
                            entity_name, column_name
                        ),
                    )
                })?;

                sea_orm::Value::ChronoDateTimeWithTimeZone(Some(Box::new(value)))
            }
            #[cfg(feature = "with-time")]
            ConvertedType::TimeDate => {
                use std::str::FromStr;

                let value = sea_orm::entity::prelude::TimeDate::parse(
                    value.string()?,
                    sea_orm::sea_query::value::time_format::FORMAT_DATE,
                )
                .map_err(|e| {
                    crate::SeaographyError::TypeConversionError(
                        e.to_string(),
                        format!("TimeDate - {}.{}", entity_name, column_name),
                    )
                })?;

                sea_orm::Value::TimeDate(Some(Box::new(value)))
            }
            #[cfg(feature = "with-time")]
            ConvertedType::TimeTime => {
                use std::str::FromStr;

                let value = sea_orm::entity::prelude::TimeTime::parse(
                    value.string()?,
                    sea_orm::sea_query::value::time_format::FORMAT_TIME,
                )
                .map_err(|e| {
                    crate::SeaographyError::TypeConversionError(
                        e.to_string(),
                        format!("TimeTime - {}.{}", entity_name, column_name),
                    )
                })?;

                sea_orm::Value::TimeTime(Some(Box::new(value)))
            }
            #[cfg(feature = "with-time")]
            ConvertedType::TimeDateTime => {
                use std::str::FromStr;

                let value = sea_orm::entity::prelude::TimeDateTime::parse(
                    value.string()?,
                    sea_orm::sea_query::value::time_format::FORMAT_DATETIME,
                )
                .map_err(|e| {
                    crate::SeaographyError::TypeConversionError(
                        e.to_string(),
                        format!("TimeDateTime - {}.{}", entity_name, column_name),
                    )
                })?;

                sea_orm::Value::TimeDateTime(Some(Box::new(value)))
            }
            #[cfg(feature = "with-time")]
            ConvertedType::TimeDateTimeWithTimeZone => {
                use std::str::FromStr;
                let value = sea_orm::entity::prelude::TimeDateTimeWithTimeZone::parse(
                    value.string()?,
                    sea_orm::sea_query::value::time_format::FORMAT_DATETIME_TZ,
                )
                .map_err(|e| {
                    crate::SeaographyError::TypeConversionError(
                        e.to_string(),
                        format!("TimeDateTimeWithTimeZone - {}.{}", entity_name, column_name),
                    )
                })?;

                sea_orm::Value::TimeDateTimeWithTimeZone(Some(Box::new(value)))
            }
            #[cfg(feature = "with-uuid")]
            ConvertedType::Uuid => {
                use std::str::FromStr;

                let value =
                    sea_orm::entity::prelude::Uuid::from_str(value.string()?).map_err(|e| {
                        crate::SeaographyError::TypeConversionError(
                            e.to_string(),
                            format!("Uuid - {}.{}", entity_name, column_name),
                        )
                    })?;

                sea_orm::Value::Uuid(Some(Box::new(value)))
            }
            #[cfg(feature = "with-decimal")]
            ConvertedType::Decimal => {
                use std::str::FromStr;

                let value =
                    sea_orm::entity::prelude::Decimal::from_str(value.string()?).map_err(|e| {
                        crate::SeaographyError::TypeConversionError(
                            e.to_string(),
                            format!("Decimal - {}.{}", entity_name, column_name),
                        )
                    })?;

                sea_orm::Value::Decimal(Some(Box::new(value)))
            }
            #[cfg(feature = "with-bigdecimal")]
            ConvertedType::BigDecimal => {
                use std::str::FromStr;

                let value = sea_orm::entity::prelude::BigDecimal::from_str(value.string()?)
                    .map_err(|e| {
                        crate::SeaographyError::TypeConversionError(
                            e.to_string(),
                            format!("BigDecimal - {}.{}", entity_name, column_name),
                        )
                    })?;

                sea_orm::Value::BigDecimal(Some(Box::new(value)))
            }
            // FIXME: support array type
            #[cfg(feature = "postgres-array")]
            ConvertedType::Array(ConvertedType) => {
                let value = value.string()?;
                sea_orm::Value::String(Some(Box::new(value.to_string())))
            }
            // FIXME: support ip type
            #[cfg(feature = "with-ipnetwork")]
            ConvertedType::IpNetwork => {
                let value = value.string()?;
                sea_orm::Value::String(Some(Box::new(value.to_string())))
            }
            // FIXME: support mac type
            #[cfg(feature = "with-mac_address")]
            ConvertedType::MacAddress => {
                let value = value.string()?;
                sea_orm::Value::String(Some(Box::new(value.to_string())))
            }
        })
    }

    /// used to map from a SeaORM column type to an async_graphql type
    /// None indicates that we do not support the type
    pub fn sea_orm_column_type_to_graphql_type(&self, ty: &ColumnType) -> Option<String> {
        let active_enum_builder = ActiveEnumBuilder {
            context: self.context,
        };

        match ty {
            ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => {
                Some(TypeRef::STRING.into())
            }
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
            ColumnType::Binary(_)
            | ColumnType::VarBinary(_)
            | ColumnType::Bit(_)
            | ColumnType::VarBit(_) => Some(TypeRef::STRING.into()),
            ColumnType::Boolean => Some(TypeRef::BOOLEAN.into()),
            // FIXME: support json type
            ColumnType::Json | ColumnType::JsonBinary => None,
            ColumnType::Uuid => Some(TypeRef::STRING.into()),
            ColumnType::Enum {
                name: enum_name,
                variants: _,
            } => Some(active_enum_builder.type_name_from_iden(enum_name)),
            ColumnType::Cidr | ColumnType::Inet | ColumnType::MacAddr => {
                Some(TypeRef::STRING.into())
            }
            // FIXME: support array type
            ColumnType::Array(_) => None,
            ColumnType::Custom(_iden) => Some(TypeRef::STRING.into()),
            _ => None,
        }
    }
}

pub enum TimeLibrary {
    String,
    #[cfg(feature = "with-chrono")]
    Chrono,
    #[cfg(feature = "with-time")]
    Time,
}

pub enum DecimalLibrary {
    String,
    #[cfg(feature = "with-decimal")]
    Decimal,
    #[cfg(feature = "with-bigdecimal")]
    BigDecimal,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ConvertedType {
    Bool,
    TinyInteger,
    SmallInteger,
    Integer,
    BigInteger,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    String,
    Char,
    Bytes,
    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json,
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate,
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime,
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime,
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc,
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal,
    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone,
    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate,
    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime,
    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime,
    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone,
    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid,
    #[cfg(feature = "with-decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-decimal")))]
    Decimal,
    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal,
    #[cfg(feature = "postgres-array")]
    #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
    Array(ConvertedType),
    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork,
    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress,
    Enum(String),
    Custom(String),
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
