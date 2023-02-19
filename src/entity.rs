use async_graphql::{dynamic::*, Value};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use sea_orm::{prelude::*, Iterable};

use crate::{connection::*, edge::*, filter::*, order::*, query::*};

pub struct DynamicGraphqlEntity {
    pub entity_object: Object,
    pub edge_object: Object,
    pub connection_object: Object,
    pub query: Field,
    pub filter_input: InputObject,
    pub order_input: InputObject,
}

impl DynamicGraphqlEntity {
    /// Used to convert SeaORM Entity into async-graphql types
    pub fn from_entity<T>(pagination_input: &InputObject, relations: Vec<Field>) -> Self
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object = relations
            .into_iter()
            .fold(entity_to_object::<T>(), |object, field| object.field(field));

        let edge_object = Edge::<T>::to_object(&entity_object);

        let connection_object =
            Connection::<T>::entity_object_to_connection(&entity_object, &edge_object);

        let filter_input = entity_to_filter::<T>(&entity_object);

        let order_input = entity_to_order::<T>(&entity_object);

        let query = entity_to_query::<T>(
            &entity_object,
            &connection_object,
            &filter_input,
            &order_input,
            pagination_input,
        );

        DynamicGraphqlEntity {
            entity_object,
            edge_object,
            connection_object,
            query,
            filter_input,
            order_input,
        }
    }
}

pub fn entity_to_object<T>() -> Object
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    // TODO check if nullable
    let entity_object = T::Column::iter().fold(
        Object::new(<T as EntityName>::table_name(&T::default()).to_upper_camel_case()),
        |object, column: T::Column| {
            let name = column.as_str().to_lower_camel_case();

            let column_def = column.def();

            let type_name = match &column_def.get_column_type() {
                ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => TypeRef::STRING,
                ColumnType::TinyInteger
                | ColumnType::SmallInteger
                | ColumnType::Integer
                | ColumnType::BigInteger
                | ColumnType::TinyUnsigned
                | ColumnType::SmallUnsigned
                | ColumnType::Unsigned
                | ColumnType::BigUnsigned => TypeRef::INT,
                ColumnType::Float | ColumnType::Double => TypeRef::FLOAT,
                ColumnType::Decimal(_) => TypeRef::STRING,
                ColumnType::DateTime
                | ColumnType::Timestamp
                | ColumnType::TimestampWithTimeZone
                | ColumnType::Time
                | ColumnType::Date => TypeRef::STRING,
                ColumnType::Year(_) => TypeRef::INT,
                ColumnType::Interval(_, _) => TypeRef::STRING,
                ColumnType::Binary(_)
                | ColumnType::VarBinary(_)
                | ColumnType::VarBit(_)
                | ColumnType::Bit(_) => TypeRef::STRING,
                ColumnType::Boolean => TypeRef::BOOLEAN,
                ColumnType::Money(_) => TypeRef::STRING,
                ColumnType::Json | ColumnType::JsonBinary => {
                    // FIXME
                    TypeRef::STRING
                },
                ColumnType::Uuid => TypeRef::STRING,
                ColumnType::Custom(_) => {
                    // FIXME
                    TypeRef::STRING
                },
                ColumnType::Enum {
                    name: _,
                    variants: _,
                } => {
                    // FIXME
                    TypeRef::STRING
                },
                ColumnType::Array(_) => {
                    // FIXME
                    TypeRef::STRING
                },
                ColumnType::Cidr | ColumnType::Inet | ColumnType::MacAddr => TypeRef::STRING,
                _ => todo!(),
            };

            let graphql_type = if column_def.is_null() {
                TypeRef::named(type_name)
            } else {
                TypeRef::named_nn(type_name)
            };

            let field = Field::new(name, graphql_type, move |ctx| {
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
                            Some(value) => Ok(Some(Value::from(value.as_str()))),
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
                    sea_orm::sea_query::Value::Json(value) => {
                        FieldFuture::new(async move {
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoDate(value) => {
                        FieldFuture::new(async move {
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoTime(value) => {
                        FieldFuture::new(async move {
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-chrono")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
                    sea_orm::sea_query::Value::ChronoDateTime(value) => {
                        FieldFuture::new(async move {
                            // FIXME
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
                            // FIXME
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
                            // FIXME
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
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-time")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
                    sea_orm::sea_query::Value::TimeDate(value) => {
                        FieldFuture::new(async move {
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-time")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
                    sea_orm::sea_query::Value::TimeTime(value) => {
                        FieldFuture::new(async move {
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-time")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
                    sea_orm::sea_query::Value::TimeDateTime(value) => {
                        FieldFuture::new(async move {
                            // FIXME
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
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-uuid")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
                    sea_orm::sea_query::Value::Uuid(value) => {
                        FieldFuture::new(async move {
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-decimal")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-decimal")))]
                    sea_orm::sea_query::Value::Decimal(value) => {
                        FieldFuture::new(async move {
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-bigdecimal")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
                    sea_orm::sea_query::Value::BigDecimal(value) => {
                        FieldFuture::new(async move {
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "postgres-array")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
                    sea_orm::sea_query::Value::Array(array_type, value) => {
                        FieldFuture::new(async move {
                            // FIXME
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
                            // FIXME
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
                            // FIXME
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    },
                    #[allow(unreachable_patterns)]
                    _ => todo!(),
                }
            });

            object.field(field)
        },
    );

    entity_object
}
