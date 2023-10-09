use async_graphql::dynamic::{Field, FieldFuture, Object, TypeRef};
use async_graphql::{Error, Value};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use sea_orm::{ColumnTrait, ColumnType, EntityName, EntityTrait, IdenStatic, Iterable, ModelTrait};

/// The configuration structure for EntityObjectBuilder
pub struct EntityObjectConfig {
    /// used to format the type name of the object
    pub type_name: crate::SimpleNamingFn,
    /// used to format the name of column fields
    pub column_name: crate::ComplexNamingFn,
    /// suffix that is appended on basic version of entity type
    pub basic_type_suffix: String,
}

impl std::default::Default for EntityObjectConfig {
    fn default() -> Self {
        Self {
            type_name: Box::new(|entity_name: &str| -> String {
                entity_name.to_upper_camel_case()
            }),
            column_name: Box::new(|_entity_name: &str, column_name: &str| -> String {
                column_name.to_lower_camel_case()
            }),
            basic_type_suffix: "Basic".into(),
        }
    }
}

use crate::{BuilderContext, TypesMapHelper, GuardAction};

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

    /// used to get type name for basic version
    pub fn basic_type_name<T>(&self) -> String
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let name: String = <T as EntityName>::table_name(&T::default()).into();
        format!(
            "{}{}",
            self.context.entity_object.type_name.as_ref()(&name),
            self.context.entity_object.basic_type_suffix
        )
    }

    /// used to get column field name of entity column
    pub fn column_name<T>(&self, column: &T::Column) -> String
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

        self.basic_object::<T>(&object_name)
    }

    /// used to get the GraphQL basic object of a SeaORM entity
    pub fn basic_to_object<T>(&self) -> Object
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let object_name = self.basic_type_name::<T>();

        self.basic_object::<T>(&object_name)
    }

    /// used to create a SeaORM entity basic GraphQL object type
    fn basic_object<T>(&self, object_name: &str) -> Object
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_name = self.type_name::<T>();

        let types_map_helper = TypesMapHelper {
            context: self.context,
        };

        T::Column::iter().fold(Object::new(object_name), |object, column: T::Column| {
            let column_name = self.column_name::<T>(&column);

            let column_def = column.def();

            let type_name = match types_map_helper
                .sea_orm_column_type_to_graphql_type(column_def.get_column_type())
            {
                Some(type_name) => type_name,
                None => return object,
            };

            let graphql_type = if column_def.is_null() {
                TypeRef::named(type_name)
            } else {
                TypeRef::named_nn(type_name)
            };

            let is_enum: bool = matches!(
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

            let conversion_fn = self
                .context
                .types
                .output_conversions
                .get(&format!("{}.{}", entity_name, column_name));

            let field = Field::new(column_name, graphql_type, move |ctx| {
                let guard_flag = if let Some(guard) = guard {
                    (*guard)(&ctx)
                } else {
                    GuardAction::Allow
                };

                if let GuardAction::Block(reason) = guard_flag {
                    return FieldFuture::new(async move {
                        match reason {
                            Some(reason) => {
                                Err::<Option<()>, async_graphql::Error>(Error::new(reason))
                            }
                            None => Err::<Option<()>, async_graphql::Error>(Error::new(
                                "Field guard triggered.",
                            )),
                        }
                    });
                }

                // convert SeaQL value to GraphQL value
                // FIXME: move to types_map file
                let object = ctx
                    .parent_value
                    .try_downcast_ref::<T::Model>()
                    .expect("Something went wrong when trying to downcast entity object.");

                if let Some(conversion_fn) = conversion_fn {
                    let result = conversion_fn(&object.get(column));
                    return FieldFuture::new(async move {
                        match result {
                            Ok(value) => Ok(Some(value)),
                            // FIXME: proper error reporting
                            Err(_) => Ok(None),
                        }
                    });
                }

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
                            // FIXME: test array type
                            match value {
                                Some(value) => Ok(Some(Value::from(value.to_string()))),
                                None => Ok(None),
                            }
                        })
                    }

                    #[cfg(feature = "with-ipnetwork")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
                    sea_orm::sea_query::Value::IpNetwork(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),

                    #[cfg(feature = "with-mac_address")]
                    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
                    sea_orm::sea_query::Value::MacAddress(value) => FieldFuture::new(async move {
                        match value {
                            Some(value) => Ok(Some(Value::from(value.to_string()))),
                            None => Ok(None),
                        }
                    }),
                    #[allow(unreachable_patterns)]
                    _ => panic!("Cannot convert SeaORM value"),
                }
            });

            object.field(field)
        })
    }
}
