use async_graphql::{
    dynamic::{Field, FieldFuture, Object, ObjectAccessor},
    Value,
};
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ColumnType, EntityName, EntityTrait, IdenStatic, Iterable,
    ModelTrait, TryIntoModel,
};

use crate::{guard_error, EntityColumnId, OperationType, SeaResult, SeaographyError};

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
                if cfg!(feature = "field-snake-case") {
                    column_name.to_snake_case()
                } else {
                    column_name.to_lower_camel_case()
                }
            }),
            basic_type_suffix: "Basic".into(),
        }
    }
}

use crate::{format_variant, BuilderContext, GuardAction, TypesMapHelper};

/// This builder produces the GraphQL object of a SeaORM entity
#[derive(Copy, Clone)]
pub struct EntityObjectBuilder {
    pub context: &'static BuilderContext,
}

impl EntityObjectBuilder {
    /// used to get type name
    pub fn type_name<T>(&self) -> String
    where
        T: EntityTrait,
    {
        let name: String = <T as EntityName>::table_name(&T::default()).into();
        self.context.entity_object.type_name.as_ref()(&name)
    }

    /// used to get type name for basic version
    pub fn basic_type_name<T>(&self) -> String
    where
        T: EntityTrait,
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
    pub fn to_basic_object<T>(&self) -> Object
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
        let object_name = object_name.to_owned();

        let types_map_helper = TypesMapHelper {
            context: self.context,
        };

        T::Column::iter().fold(
            Object::new(&object_name),
            move |object, column: T::Column| {
                let object_name = object_name.clone();
                let column_name = self.column_name::<T>(&column);
                let entity_column_id = EntityColumnId::of::<T>(&column);

                let column_def = column.def();
                let graphql_type = match types_map_helper.output_type_for_column::<T>(
                    &column,
                    &entity_column_id,
                    !column_def.is_null(),
                ) {
                    Some(type_name) => type_name,
                    None => return object,
                };

                if column_def.seaography().ignore {
                    return object;
                }

                // This isn't the most beautiful flag: it's indicating whether the leaf type is an
                // enum, rather than the type itself. Ideally we'd only calculate this for the leaf
                // type itself. Could be a good candidate for refactor as this code evolves to support
                // more container types. For example, this at the very least should be recursive on
                // Array types such that arrays of arrays of enums would be resolved correctly.
                let is_enum: bool = match column_def.get_column_type() {
                    ColumnType::Enum { .. } => true,
                    #[cfg(feature = "with-postgres-array")]
                    ColumnType::Array(inner) => matches!(inner.as_ref(), ColumnType::Enum { .. }),
                    _ => false,
                };

                let conversion_fn = self
                    .context
                    .types
                    .column_options
                    .get(&entity_column_id)
                    .and_then(|options| options.output_conversion.as_ref());

                let hooks = &self.context.hooks;
                let context = self.context;

                let field = Field::new(column_name.clone(), graphql_type, move |ctx| {
                    if let GuardAction::Block(reason) =
                        hooks.field_guard(&ctx, &object_name, &column_name, OperationType::Read)
                    {
                        return FieldFuture::new(async move {
                            Err::<Option<()>, _>(guard_error(reason, "Field guard triggered."))
                        });
                    }

                    // convert SeaQL value to GraphQL value
                    let object = match ctx.parent_value.try_downcast_ref::<T::Model>() {
                        Ok(object) => object,
                        Err(_) => {
                            let object_name = object_name.clone();
                            return FieldFuture::new(async move {
                                Err::<Option<()>, _>(async_graphql::Error::new(format!(
                                    "Failed to downcast object to {object_name}"
                                )))
                            });
                        }
                    };

                    if let Some(conversion_fn) = conversion_fn {
                        let result = conversion_fn(&object.get(column));
                        return FieldFuture::new(async move { result });
                    }

                    FieldFuture::from_value(sea_query_value_to_graphql_value(
                        context,
                        object.get(column),
                        is_enum,
                    ))
                });

                object.field(field)
            },
        )
    }

    pub fn parse_object<M>(&self, object: &ObjectAccessor) -> SeaResult<M>
    where
        M: ModelTrait + Sync,
        <<M as ModelTrait>::Entity as EntityTrait>::ActiveModel: TryIntoModel<M>,
    {
        let entity_object_builder = EntityObjectBuilder {
            context: self.context,
        };
        let types_map_helper = TypesMapHelper {
            context: self.context,
        };

        let mut active_model = <<M as ModelTrait>::Entity as EntityTrait>::ActiveModel::default();

        for column in <<M as ModelTrait>::Entity as EntityTrait>::Column::iter() {
            let column_name =
                entity_object_builder.column_name::<<M as ModelTrait>::Entity>(&column);

            let value = match object.get(&column_name) {
                Some(value) => value,
                None => continue,
            };

            let value = types_map_helper
                .async_graphql_value_to_sea_orm_value::<<M as ModelTrait>::Entity>(
                    &column, &value,
                )?;

            active_model.try_set(column, value).map_err(|e| {
                let entity_name = entity_object_builder.type_name::<<M as ModelTrait>::Entity>();
                SeaographyError::TypeConversionError(
                    e.to_string(),
                    format!("{entity_name} - {column_name}"),
                )
            })?;
        }

        active_model.try_into_model().map_err(|e| {
            SeaographyError::TypeConversionError(
                self.type_name::<<M as ModelTrait>::Entity>(),
                e.to_string(),
            )
        })
    }
}

pub(crate) fn sea_query_value_to_graphql_value(
    _context: &'static BuilderContext,
    sea_query_value: sea_orm::sea_query::Value,
    is_enum: bool,
) -> Option<Value> {
    match sea_query_value {
        sea_orm::Value::Bool(value) => value.map(Value::from),
        sea_orm::Value::TinyInt(value) => value.map(Value::from),
        sea_orm::Value::SmallInt(value) => value.map(Value::from),
        sea_orm::Value::Int(value) => value.map(Value::from),
        sea_orm::Value::BigInt(value) => value.map(Value::from),
        sea_orm::Value::TinyUnsigned(value) => value.map(Value::from),
        sea_orm::Value::SmallUnsigned(value) => value.map(Value::from),
        sea_orm::Value::Unsigned(value) => value.map(Value::from),
        sea_orm::Value::BigUnsigned(value) => value.map(Value::from),
        sea_orm::Value::Float(value) => value.map(Value::from),
        sea_orm::Value::Double(value) => value.map(Value::from),
        sea_orm::Value::String(value) if is_enum => {
            value.map(|it| Value::from(format_variant(it.as_str())))
        }
        sea_orm::Value::String(value) => value.map(|it| Value::from(it.as_str())),
        sea_orm::Value::Char(value) => value.map(|it| Value::from(it.to_string())),

        #[allow(clippy::box_collection)]
        sea_orm::Value::Bytes(value) => value.map(|it| Value::from(String::from_utf8_lossy(&it))),

        #[cfg(feature = "with-postgres-array")]
        sea_orm::Value::Array(_array_value, value) => value.map(|it| {
            Value::List(
                it.into_iter()
                    .map(|item| {
                        sea_query_value_to_graphql_value(_context, item, is_enum)
                            .unwrap_or(Value::Null)
                    })
                    .collect(),
            )
        }),

        #[cfg(feature = "with-json")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
        sea_orm::sea_query::Value::Json(value) => {
            value.map(|it| match Value::from_json(it.clone()) {
                Ok(v) => v,
                Err(_) => Value::from(it.to_string()),
            })
        }

        #[cfg(feature = "with-chrono")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
        sea_orm::sea_query::Value::ChronoDate(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-chrono")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
        sea_orm::sea_query::Value::ChronoTime(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-chrono")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
        sea_orm::sea_query::Value::ChronoDateTime(value) => {
            value.map(|it| Value::from(it.to_string()))
        }

        #[cfg(feature = "with-chrono")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
        sea_orm::sea_query::Value::ChronoDateTimeUtc(value) => value.map(|it| {
            Value::from(if _context.types.timestamp_rfc3339 {
                it.to_rfc3339()
            } else {
                it.to_string()
            })
        }),

        #[cfg(feature = "with-chrono")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
        sea_orm::sea_query::Value::ChronoDateTimeLocal(value) => value.map(|it| {
            Value::from(if _context.types.timestamp_rfc3339 {
                it.to_rfc3339()
            } else {
                it.to_string()
            })
        }),

        #[cfg(feature = "with-chrono")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
        sea_orm::sea_query::Value::ChronoDateTimeWithTimeZone(value) => value.map(|it| {
            Value::from(if _context.types.timestamp_rfc3339 {
                it.to_rfc3339()
            } else {
                it.to_string()
            })
        }),

        #[cfg(feature = "with-time")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
        sea_orm::sea_query::Value::TimeDate(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-time")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
        sea_orm::sea_query::Value::TimeTime(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-time")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
        sea_orm::sea_query::Value::TimeDateTime(value) => {
            value.map(|it| Value::from(it.to_string()))
        }

        #[cfg(feature = "with-time")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
        sea_orm::sea_query::Value::TimeDateTimeWithTimeZone(value) => value.map(|it| {
            Value::from(if _context.types.timestamp_rfc3339 {
                it.format(&time::format_description::well_known::Rfc3339)
                    .unwrap()
            } else {
                it.to_string()
            })
        }),

        #[cfg(feature = "with-uuid")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
        sea_orm::sea_query::Value::Uuid(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-decimal")]
        sea_orm::sea_query::Value::Decimal(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-bigdecimal")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
        sea_orm::sea_query::Value::BigDecimal(value) => value.map(|it| Value::from(it.to_string())),

        // #[cfg(feature = "with-ipnetwork")]
        // #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
        // sea_orm::sea_query::Value::IpNetwork(value) => value.map(|it| Value::from(it.to_string())),

        // #[cfg(feature = "with-mac_address")]
        // #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
        // sea_orm::sea_query::Value::MacAddress(value) => value.map(|it| Value::from(it.to_string())),
        #[allow(unreachable_patterns)]
        _ => None,
    }
}
