use async_graphql::dynamic::{Field, FieldFuture, Object};
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

use crate::{ActiveEnumBuilder, BuilderContext, GuardAction, TypesMapHelper};

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
            let context = self.context;

            let column_name = self.column_name::<T>(&column);

            let column_def = column.def();

            let graphql_type = match types_map_helper.sea_orm_column_type_to_graphql_type(
                column_def.get_column_type(),
                !column_def.is_null(),
            ) {
                Some(type_name) => type_name,
                None => return object,
            };

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

            let guard = context
                .guards
                .field_guards
                .get(&format!("{}.{}", &object_name, &column_name));

            let conversion_fn = context
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

                let value = sea_query_value_to_graphql_value(
                    context,
                    object.get(column),
                    is_enum,
                    column_def.get_column_type(),
                );

                FieldFuture::new(async move { Ok(value) })
            });

            object.field(field)
        })
    }
}

fn sea_query_value_to_graphql_value(
    context: &'static BuilderContext,
    sea_query_value: sea_orm::sea_query::Value,
    is_enum: bool,
    column_type: &ColumnType,
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
        sea_orm::Value::String(value) if is_enum => value.map(|it| {
            let builder = ActiveEnumBuilder { context };

            let enum_name = match column_type {
                ColumnType::Enum { name, .. } => name.to_string(),
                _ => panic!("Expected enum column type"),
            };

            let gql_name = builder.variant_name(enum_name.as_str(), it.as_str());

            Value::from(gql_name)
        }),
        sea_orm::Value::String(value) => value.map(|it| Value::from(it.as_str())),
        sea_orm::Value::Char(value) => value.map(|it| Value::from(it.to_string())),

        #[allow(clippy::box_collection)]
        sea_orm::Value::Bytes(value) => value.map(|it| Value::from(String::from_utf8_lossy(&it))),

        #[cfg(feature = "with-postgres-array")]
        sea_orm::Value::Array(_array_value, value) => value.map(|it| {
            Value::List(
                it.into_iter()
                    .map(|item| {
                        sea_query_value_to_graphql_value(context, item, is_enum, column_type)
                            .unwrap_or(Value::Null)
                    })
                    .collect(),
            )
        }),

        #[cfg(feature = "with-json")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
        sea_orm::sea_query::Value::Json(value) => value.map(|it| Value::from(it.to_string())),

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
        sea_orm::sea_query::Value::ChronoDateTimeUtc(value) => {
            value.map(|it| Value::from(it.to_string()))
        }

        #[cfg(feature = "with-chrono")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
        sea_orm::sea_query::Value::ChronoDateTimeLocal(value) => {
            value.map(|it| Value::from(it.to_string()))
        }

        #[cfg(feature = "with-chrono")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
        sea_orm::sea_query::Value::ChronoDateTimeWithTimeZone(value) => {
            value.map(|it| Value::from(it.to_string()))
        }

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
        sea_orm::sea_query::Value::TimeDateTimeWithTimeZone(value) => {
            value.map(|it| Value::from(it.to_string()))
        }

        #[cfg(feature = "with-uuid")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
        sea_orm::sea_query::Value::Uuid(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-decimal")]
        sea_orm::sea_query::Value::Decimal(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-bigdecimal")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
        sea_orm::sea_query::Value::BigDecimal(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-ipnetwork")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
        sea_orm::sea_query::Value::IpNetwork(value) => value.map(|it| Value::from(it.to_string())),

        #[cfg(feature = "with-mac_address")]
        #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
        sea_orm::sea_query::Value::MacAddress(value) => value.map(|it| Value::from(it.to_string())),

        #[allow(unreachable_patterns)]
        _ => panic!("Cannot convert SeaORM value"),
    }
}
