use async_graphql::dynamic::{Field, FieldFuture, Object, TypeRef};
use async_graphql::{Error, Value};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use sea_orm::{ColumnTrait, ColumnType, EntityName, EntityTrait, IdenStatic, Iterable, ModelTrait};

/// The configuration structure for EntityObjectBuilder
pub struct EntityObjectConfig {
    /// used to format the type name of the object
    pub type_name: crate::SimpleNamingFn,
    /// used to format the name for the query field of the object
    pub query_entity_name: crate::SimpleNamingFn,
    /// used to format the name of column fields
    pub column_name: crate::ComplexNamingFn,
}

impl std::default::Default for EntityObjectConfig {
    fn default() -> Self {
        Self {
            type_name: Box::new(|entity_name: &str| -> String {
                entity_name.to_upper_camel_case()
            }),
            query_entity_name: Box::new(|entity_name: &str| -> String {
                entity_name.to_lower_camel_case()
            }),
            column_name: Box::new(|_entity_name: &str, column_name: &str| -> String {
                column_name.to_lower_camel_case()
            }),
        }
    }
}

use crate::{ActiveEnumBuilder, BuilderContext};

/// This builder produces the GraphQL object of a SeaORM entity
pub struct EntityObjectBuilder {
    pub context: &'static BuilderContext,
}

impl EntityObjectBuilder {
    /// used to get type name
    pub fn type_name<T>(&self) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let name: String = <T as EntityName>::table_name(&T::default()).into();
        self.context.entity_object.type_name.as_ref()(&name)
    }

    /// used to get query field name of entity
    pub fn query_entity_name<T>(&self) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let name: String = <T as EntityName>::table_name(&T::default()).into();
        self.context.entity_object.query_entity_name.as_ref()(&name)
    }

    /// used to get column field name of entity column
    pub fn column_name<T>(&self, column: T::Column) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_name = self.type_name::<T>();
        let column_name: String = column.as_str().into();
        self.context.entity_object.column_name.as_ref()(&entity_name, &column_name)
    }

    /// used to get the GraphQL object of a SeaORM entity
    pub fn to_object<T>(&self) -> Object
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let object_name = self.type_name::<T>();
        let active_enum_builder = ActiveEnumBuilder {
            context: self.context,
        };

        T::Column::iter().fold(Object::new(&object_name), |object, column: T::Column| {
            let column_name = self.column_name::<T>(column);

            let column_def = column.def();

            // map column type to GraphQL type
            let type_name: Option<String> = match &column_def.get_column_type() {
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
                ColumnType::Decimal(_) => Some(TypeRef::STRING.into()),
                ColumnType::DateTime
                | ColumnType::Timestamp
                | ColumnType::TimestampWithTimeZone
                | ColumnType::Time
                | ColumnType::Date => Some(TypeRef::STRING.into()),
                ColumnType::Year(_) => Some(TypeRef::INT.into()),
                ColumnType::Interval(_, _) => Some(TypeRef::STRING.into()),
                ColumnType::Binary(_)
                | ColumnType::VarBinary(_)
                | ColumnType::VarBit(_)
                | ColumnType::Bit(_) => Some(TypeRef::STRING.into()),
                ColumnType::Boolean => Some(TypeRef::BOOLEAN.into()),
                ColumnType::Money(_) => Some(TypeRef::STRING.into()),
                // FIXME: json type
                ColumnType::Json | ColumnType::JsonBinary => Some(TypeRef::STRING.into()),
                ColumnType::Uuid => Some(TypeRef::STRING.into()),
                // FIXME: research what type is behind the custom type
                ColumnType::Custom(_) => Some(TypeRef::STRING.into()),
                ColumnType::Enum { name, variants: _ } => {
                    Some(active_enum_builder.type_name_from_iden(name))
                }
                // FIXME: array type
                // ColumnType::Array(_) => Some(TypeRef::STRING.into())
                ColumnType::Cidr | ColumnType::Inet | ColumnType::MacAddr => {
                    Some(TypeRef::STRING.into())
                }
                _ => None,
            };

            let type_name = if let Some(type_name) = type_name {
                type_name
            } else {
                return object;
            };

            // map if field is nullable
            let graphql_type = if column_def.is_null() {
                TypeRef::named(type_name)
            } else {
                TypeRef::named_nn(type_name)
            };

            let is_enum = matches!(
                column_def.get_column_type(),
                ColumnType::Enum {
                    name: _,
                    variants: _
                }
            );

            let guard = self
                .context
                .guards
                .field_guards
                .get(&format!("{}.{}", &object_name, &column_name));

            // convert SeaQL value to GraphQL value
            let field = Field::new(column_name, graphql_type, move |ctx| {
                let guard_flag = if let Some(guard) = guard {
                    (*guard)(&ctx)
                } else {
                    false
                };

                if guard_flag {
                    return FieldFuture::new(async move {
                        if guard_flag {
                            Err(Error::new("Field guard triggered."))
                        } else {
                            Ok(Some(Value::from(false)))
                        }
                    });
                }

                let object = ctx
                    .parent_value
                    .try_downcast_ref::<T::Model>()
                    .expect("Something went wrong when trying to downcast entity object.");

                match object.get(column) {
                    sea_orm::sea_query::Value::Bool(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::TinyInt(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::SmallInt(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::Int(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::BigInt(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::TinyUnsigned(value) => {
                        FieldFuture::new(async move {
                            match value {
                                Some(value) => Ok(Some(Value::from(value))),
                                None => Ok(None),
                            }
                        })
                    }
                    sea_orm::sea_query::Value::SmallUnsigned(value) => {
                        FieldFuture::new(async move {
                            match value {
                                Some(value) => Ok(Some(Value::from(value))),
                                None => Ok(None),
                            }
                        })
                    }
                    sea_orm::sea_query::Value::Unsigned(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::BigUnsigned(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::Float(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::Double(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value))),
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::String(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => {
                                if is_enum {
                                    Ok(Some(Value::from(
                                        value.as_str().to_upper_camel_case().to_ascii_uppercase(),
                                    )))
                                } else {
                                    Ok(Some(Value::from(value.as_str())))
                                }
                            }
                            None => Ok(None),
                        }
                    }),
                    sea_orm::sea_query::Value::Char(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),
                    #[allow(clippy::box_collection)]
                    sea_orm::sea_query::Value::Bytes(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(String::from_utf8_lossy(&value)))),
                            None => Ok(None),
                        }
                    }),
                    #[cfg(feature = "with-json")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
                    sea_orm::sea_query::Value::Json(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoDate(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoTime(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoDateTime(value) => {
                        FieldFuture::new(async move {
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoDateTimeUtc(value) => {
                        FieldFuture::new(async move {
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoDateTimeLocal(value) => {
                        FieldFuture::new(async move {
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoDateTimeWithTimeZone(value) => {
                        FieldFuture::new(async move {
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-time")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
                    sea_orm::sea_query::Value::TimeDate(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "with-time")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
                    sea_orm::sea_query::Value::TimeTime(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "with-time")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
                    sea_orm::sea_query::Value::TimeDateTime(value) => {
                        FieldFuture::new(async move {
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-time")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
                    sea_orm::sea_query::Value::TimeDateTimeWithTimeZone(value) => {
                        FieldFuture::new(async move {
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-uuid")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
                    sea_orm::sea_query::Value::Uuid(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "with-decimal")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-decimal")))]
                    sea_orm::sea_query::Value::Decimal(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "with-bigdecimal")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
                    sea_orm::sea_query::Value::BigDecimal(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "postgres-array")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
                    sea_orm::sea_query::Value::Array(array_type, value) => {
                        FieldFuture::new(async move {
                            // FIXME: array type
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-ipnetwork")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
                    sea_orm::sea_query::Value::IpNetwork(value) => {
                        FieldFuture::new(async move {
                            // FIXME: ipnet type
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-mac_address")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
                    sea_orm::sea_query::Value::MacAddress(value) => {
                        FieldFuture::new(async move {
                            // FIXME: mac type
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }
                    #[allow(unreachable_patterns)]
                    _ => todo!(),
                }
            });

            object.field(field)
        })
    }
}
