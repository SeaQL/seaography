use async_graphql::{dataloader::DataLoader, dynamic::*, Value};
use sea_orm::{prelude::*, Condition, Iterable, QueryOrder};
use seaography::heck::{ToUpperCamelCase, ToLowerCamelCase};

use crate::OrmDataloader;

pub fn schema(
    database: DatabaseConnection,
    orm_dataloader: DataLoader<OrmDataloader>,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let order_by_enum = Enum::new("OrderByEnum")
        .item(EnumItem::new("Asc"))
        .item(EnumItem::new("Desc"));

    let cursor_input = InputObject::new("CursorInput")
        .field(InputValue::new(
            "cursor",
            TypeRef::named_nn(TypeRef::STRING),
        ))
        .field(InputValue::new("limit", TypeRef::named_nn(TypeRef::INT)));

    let page_input = InputObject::new("PageInput")
        .field(InputValue::new("limit", TypeRef::named_nn(TypeRef::INT)))
        .field(InputValue::new("page", TypeRef::named_nn(TypeRef::INT)));

    let pagination_input = InputObject::new("PaginationInput")
        .oneof()
        .field(InputValue::new(
            "Cursor",
            TypeRef::named(cursor_input.type_name()),
        ))
        .field(InputValue::new(
            "Pages",
            TypeRef::named(page_input.type_name()),
        ));

    let query = Object::new("Query");

    let entities = vec![
        entity_to_dynamic_graphql::<crate::entities::actor::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::address::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::category::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::city::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::country::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::customer::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::film_actor::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::film_category::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::film_text::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::film::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::inventory::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::language::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::payment::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::rental::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::staff::Entity>(&pagination_input),
        entity_to_dynamic_graphql::<crate::entities::store::Entity>(&pagination_input),
    ];

    let schema = Schema::build(query.type_name(), None, None);

    let (schema, query) = entities
        .into_iter()
        .fold((schema, query), |(schema, query), object| {
            (
                schema
                    .register(object.filter_input)
                    .register(object.order_input)
                    .register(object.edge)
                    .register(object.connection)
                    .register(object.object),
                query.field(object.query),
            )
        });

    let schema = if let Some(depth) = depth {
        schema.limit_depth(depth)
    } else {
        schema
    };

    let schema = get_filter_types()
        .into_iter()
        .fold(schema, |schema, object| schema.register(object));

    let schema = if let Some(complexity) = complexity {
        schema.limit_complexity(complexity)
    } else {
        schema
    };

    schema
        .register(PageInfo::to_object())
        .register(PaginationInfo::to_object())
        .register(cursor_input)
        .register(page_input)
        .register(pagination_input)
        .register(order_by_enum)
        .register(query)
        .data(database)
        .data(orm_dataloader)
        .finish()
}

pub struct DynamicGraphqlEntity {
    pub object: Object,
    pub edge: Object,
    pub connection: Object,
    pub query: Field,
    pub filter_input: InputObject,
    pub order_input: InputObject,
}

pub fn entity_to_dynamic_graphql<T>(pagination_input: &InputObject) -> DynamicGraphqlEntity
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let object = entity_to_object::<T>();

    let edge = Edge::<T>::to_object(&object);

    let connection = Connection::<T>::entity_object_to_connection(&object, &edge);

    let filter_input = entity_to_filter::<T>(&object);

    let order_input = entity_to_order::<T>(&object);

    let query = entity_to_query::<T>(&object, &connection, &filter_input, &order_input, pagination_input);

    DynamicGraphqlEntity {
        object,
        edge,
        connection,
        query,
        filter_input,
        order_input,
    }
}

pub fn entity_to_object<T>() -> Object
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    T::Column::iter().fold(
        Object::new(<T as EntityName>::table_name(&T::default()).to_upper_camel_case()),
        |object, column| {
            let field = match column.def().get_column_type() {
                ColumnType::Char(_) => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::STRING),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<char>().to_string())))
                        })
                    },
                ),
                ColumnType::String(_) => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::STRING),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<String>())))
                        })
                    },
                ),
                ColumnType::Text => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::STRING),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<String>())))
                        })
                    },
                ),
                ColumnType::TinyInteger => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::INT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<i8>())))
                        })
                    },
                ),
                ColumnType::SmallInteger => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::INT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<i16>())))
                        })
                    },
                ),
                ColumnType::Integer => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::INT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<i32>())))
                        })
                    },
                ),
                ColumnType::BigInteger => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::INT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<i64>())))
                        })
                    },
                ),
                ColumnType::TinyUnsigned => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::INT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<u8>())))
                        })
                    },
                ),
                ColumnType::SmallUnsigned => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::INT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<u16>())))
                        })
                    },
                ),
                ColumnType::Unsigned => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::INT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<u32>())))
                        })
                    },
                ),
                ColumnType::BigUnsigned => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::INT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<u64>())))
                        })
                    },
                ),
                ColumnType::Float => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::FLOAT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<f32>())))
                        })
                    },
                ),
                ColumnType::Double => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::FLOAT),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<f64>())))
                        })
                    },
                ),
                ColumnType::Decimal(_) => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::STRING),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<Decimal>().to_string())))
                        })
                    },
                ),
                // ColumnType::DateTime => {

                // },
                // ColumnType::Timestamp => {

                // },
                // ColumnType::TimestampWithTimeZone => {

                // },
                // ColumnType::Time => {

                // },
                // ColumnType::Date => {

                // },
                // ColumnType::Binary => {

                // },
                // ColumnType::TinyBinary => {

                // },
                // ColumnType::MediumBinary => {

                // },
                // ColumnType::LongBinary => {

                // },
                // ColumnType::Boolean => {

                // },
                // ColumnType::Money(_) => {

                // },
                // ColumnType::Json => {

                // },
                // ColumnType::JsonBinary => {

                // },
                // ColumnType::Custom(_) => {

                // },
                // ColumnType::Uuid => {

                // },
                // ColumnType::Enum { name, variants } => {

                // },
                // ColumnType::Array(_) => {

                // },
                _ => Field::new(
                    column.as_str(),
                    TypeRef::named_nn(TypeRef::STRING),
                    move |ctx| {
                        FieldFuture::new(async move {
                            let object = ctx.parent_value.try_downcast_ref::<T::Model>()?;
                            let value = object.get(column);
                            Ok(Some(Value::from(value.unwrap::<String>())))
                        })
                    },
                ),
            };

            object.field(field)
        },
    )
}

#[macro_export]
macro_rules! basic_filter_input_type {
    ( $name:literal, $type:expr ) => {
        InputObject::new($name)
            .field(InputValue::new("eq", TypeRef::named($type)))
            .field(InputValue::new("ne", TypeRef::named($type)))
            .field(InputValue::new("gt", TypeRef::named($type)))
            .field(InputValue::new("gte", TypeRef::named($type)))
            .field(InputValue::new("lt", TypeRef::named($type)))
            .field(InputValue::new("lte", TypeRef::named($type)))
            .field(InputValue::new("is_in", TypeRef::named_nn_list($type)))
            .field(InputValue::new("is_not_in", TypeRef::named_nn_list($type)))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
    };
}

pub fn get_filter_types() -> Vec<InputObject> {
    vec![
        basic_filter_input_type!("TextFilterInput", TypeRef::STRING),
        InputObject::new("StringFilterInput")
            .field(InputValue::new("eq", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("ne", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("gt", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("gte", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("lt", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("lte", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new(
                "is_in",
                TypeRef::named_nn_list(TypeRef::STRING),
            ))
            .field(InputValue::new(
                "is_not_in",
                TypeRef::named_nn_list(TypeRef::STRING),
            ))
            .field(InputValue::new("is_null", TypeRef::named(TypeRef::BOOLEAN)))
            .field(InputValue::new("contains", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new(
                "starts_with",
                TypeRef::named(TypeRef::STRING),
            ))
            .field(InputValue::new(
                "ends_with",
                TypeRef::named(TypeRef::STRING),
            ))
            .field(InputValue::new("like", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("not_like", TypeRef::named(TypeRef::STRING))),
        basic_filter_input_type!("IntegerFilterInput", TypeRef::INT),
        basic_filter_input_type!("FloatFilterInput", TypeRef::FLOAT),
        basic_filter_input_type!("BooleanFilterInput", TypeRef::BOOLEAN),
        basic_filter_input_type!("IdFilterInput", TypeRef::ID),
    ]
}

pub fn entity_to_filter<T>(entity_def: &Object) -> InputObject
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let name = format!("{}FilterInput", entity_def.type_name());

    let object = T::Column::iter().fold(InputObject::new(&name), |object, column| {
        let field = match column.def().get_column_type() {
            ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => {
                InputValue::new(column.as_str(), TypeRef::named("StringFilterInput"))
            }
            ColumnType::TinyInteger
            | ColumnType::SmallInteger
            | ColumnType::Integer
            | ColumnType::BigInteger
            | ColumnType::TinyUnsigned
            | ColumnType::SmallUnsigned
            | ColumnType::Unsigned
            | ColumnType::BigUnsigned => {
                InputValue::new(column.as_str(), TypeRef::named("IntegerFilterInput"))
            }
            ColumnType::Float | ColumnType::Double => {
                InputValue::new(column.as_str(), TypeRef::named("FloatFilterInput"))
            }
            ColumnType::Decimal(_) => {
                InputValue::new(column.as_str(), TypeRef::named("TextFilterInput"))
            }
            // ColumnType::DateTime => {

            // },
            // ColumnType::Timestamp => {

            // },
            // ColumnType::TimestampWithTimeZone => {

            // },
            // ColumnType::Time => {

            // },
            // ColumnType::Date => {

            // },
            // ColumnType::Binary => {

            // },
            // ColumnType::TinyBinary => {

            // },
            // ColumnType::MediumBinary => {

            // },
            // ColumnType::LongBinary => {

            // },
            // ColumnType::Boolean => {

            // },
            // ColumnType::Money(_) => {

            // },
            // ColumnType::Json => {

            // },
            // ColumnType::JsonBinary => {

            // },
            // ColumnType::Custom(_) => {

            // },
            // ColumnType::Uuid => {

            // },
            // ColumnType::Enum { name, variants } => {

            // },
            // ColumnType::Array(_) => {

            // },
            _ => InputValue::new(column.as_str(), TypeRef::named("TextFilterInput")),
        };

        object.field(field)
    });

    object
        .field(InputValue::new("and", TypeRef::named_nn_list(&name)))
        .field(InputValue::new("or", TypeRef::named_nn_list(&name)))
}

pub fn entity_to_order<T>(entity_def: &Object) -> InputObject
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{

    let name = format!("{}OrderInput", entity_def.type_name());

    T::Column::iter().fold(InputObject::new(&name), |object, column| {
        object.field(InputValue::new(
            column.as_str(),
            TypeRef::named("OrderByEnum"),
        ))
    })
}

pub fn entity_to_query<T>(
    object: &Object,
    connection: &Object,
    filter_input: &InputObject,
    order_input: &InputObject,
    pagination_input: &InputObject,
) -> Field
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    Field::new(
        object.type_name().to_lower_camel_case(),
        TypeRef::named_nn(connection.type_name()),
        move |ctx| {
            FieldFuture::new(async move {
                let filters = ctx.args.get("filters");
                let order_by = ctx.args.get("orderBy");

                let stmt = T::find();
                let stmt = apply_filters(stmt, filters);
                let stmt = apply_order(stmt, order_by);

                let database = ctx.data::<DatabaseConnection>()?;
                let data = stmt.all(database).await?;

                let edges: Vec<Edge<T>> = data.into_iter().map(|node: T::Model| {
                    let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                        .map(|variant| node.get(variant.into_column()))
                        .collect();

                    let cursor: String = encode_cursor(values);

                    Edge {
                        cursor,
                        node
                    }
                }).collect();

                let connection = Connection {
                    page_info: PageInfo { has_previous_page: false, has_next_page: false, start_cursor: edges.first().map(|edge| edge.cursor.clone()), end_cursor: edges.last().map(|edge| edge.cursor.clone()) },
                    pagination_info: Some(PaginationInfo {
                        current: 1,
                        pages: 1,
                    }),
                    edges
                };

                Ok(Some(FieldValue::owned_any(connection)))
            })
        },
    )
    .argument(InputValue::new(
        "filters",
        TypeRef::named(filter_input.type_name()),
    ))
    .argument(InputValue::new(
        "orderBy",
        TypeRef::named(order_input.type_name()),
    ))
    .argument(InputValue::new(
        "pagination",
        TypeRef::named(pagination_input.type_name()),
    ))
}

fn encode_cursor(values: Vec<sea_orm::Value>) -> String {
    use seaography::itertools::Itertools;

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

#[macro_export]
macro_rules! basic_filtering_operation {
    ( $condition:expr, $column:expr, $filter:expr, $operator:ident, $type:ident ) => {
        if let Some(data) = $filter.get(stringify!($operator)) {
            let data = data.$type().expect(
                format!(
                    "We expect the {} to be of type {}",
                    stringify!($operator),
                    stringify!($type)
                )
                .as_str(),
            );

            $condition.add($column.$operator(data))
        } else {
            $condition
        }
    };
}

#[macro_export]
macro_rules! basic_filtering_type {
    ( $condition:expr, $column:expr, $filter:expr, $type:ident ) => {
        {
            let condition = basic_filtering_operation!($condition, $column, $filter, eq, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, ne, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, gt, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, gte, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, lt, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, lte, $type);
            // let condition = basic_filtering_operation!(condition, $column, $filter, is_in, $type);
            // let condition = basic_filtering_operation!(condition, $column, $filter, is_not_in, $type);
            // let condition = basic_filtering_operation!(condition, $column, $filter, is_null, $type);

            condition
        }
    };
}

#[macro_export]
macro_rules! string_filtering_type {
    ( $condition:expr, $column:expr, $filter:expr, $type:ident ) => {
        {
            let condition = basic_filtering_operation!($condition, $column, $filter, eq, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, ne, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, gt, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, gte, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, lt, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, lte, $type);
            // let condition = basic_filtering_operation!(condition, $column, $filter, is_in, $type);
            // let condition = basic_filtering_operation!(condition, $column, $filter, is_not_in, $type);
            // let condition = basic_filtering_operation!(condition, $column, $filter, is_null, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, contains, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, starts_with, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, ends_with, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, like, $type);
            let condition = basic_filtering_operation!(condition, $column, $filter, not_like, $type);

            condition
        }
    };
}

pub fn apply_filters<T>(stmt: Select<T>, filters: Option<ValueAccessor>) -> Select<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    if let Some(filters) = filters {
        let filters = filters
            .object()
            .expect("We expect the entity filters to be object type");

        let condition = recursive_prepare_condition::<T>(filters);

        println!("Condition: {:?}", condition);

        stmt.filter(condition)
    } else {
        stmt
    }
}

pub fn recursive_prepare_condition<T>(filters: ObjectAccessor) -> Condition
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let condition = T::Column::iter().fold(Condition::all(), |condition, column: T::Column| {
        let filter = filters.get(column.as_str());

        if let Some(filter) = filter {
            let filter = filter
                .object()
                .expect("We expect the column filter to be object type");

            match column.def().get_column_type() {
                ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => {
                    string_filtering_type!(condition, column, filter, string)
                }
                ColumnType::TinyInteger
                | ColumnType::SmallInteger
                | ColumnType::Integer
                | ColumnType::BigInteger => basic_filtering_type!(condition, column, filter, i64),
                ColumnType::TinyUnsigned
                | ColumnType::SmallUnsigned
                | ColumnType::Unsigned
                | ColumnType::BigUnsigned => basic_filtering_type!(condition, column, filter, u64),
                ColumnType::Float | ColumnType::Double => {
                    basic_filtering_type!(condition, column, filter, f64)
                }
                ColumnType::Decimal(_) => basic_filtering_type!(condition, column, filter, string),
                // ColumnType::DateTime => {

                // },
                // ColumnType::Timestamp => {

                // },
                // ColumnType::TimestampWithTimeZone => {

                // },
                // ColumnType::Time => {

                // },
                // ColumnType::Date => {

                // },
                // ColumnType::Binary => {

                // },
                // ColumnType::TinyBinary => {

                // },
                // ColumnType::MediumBinary => {

                // },
                // ColumnType::LongBinary => {

                // },
                // ColumnType::Boolean => {

                // },
                // ColumnType::Money(_) => {

                // },
                // ColumnType::Json => {

                // },
                // ColumnType::JsonBinary => {

                // },
                // ColumnType::Custom(_) => {

                // },
                // ColumnType::Uuid => {

                // },
                // ColumnType::Enum { name, variants } => {

                // },
                // ColumnType::Array(_) => {

                // },
                _ => panic!("Type is not supported"),
            }
        } else {
            condition
        }
    });

    let condition = if let Some(and) = filters.get("and") {
        let filters = and
            .list()
            .expect("We expect to find a list of FiltersInput");

        condition.add(
            filters
                .iter()
                .fold(Condition::all(), |condition, filters: ValueAccessor| {
                    let filters = filters.object().expect("We expect an FiltersInput object");
                    condition.add(recursive_prepare_condition::<T>(filters))
                }),
        )
    } else {
        condition
    };

    let condition = if let Some(or) = filters.get("or") {
        let filters = or.list().expect("We expect to find a list of FiltersInput");

        condition.add(
            filters
                .iter()
                .fold(Condition::any(), |condition, filters: ValueAccessor| {
                    let filters = filters.object().expect("We expect an FiltersInput object");
                    condition.add(recursive_prepare_condition::<T>(filters))
                }),
        )
    } else {
        condition
    };

    condition
}

pub fn apply_order<T>(stmt: Select<T>, order_by: Option<ValueAccessor>) -> Select<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    if let Some(order_by) = order_by {
        let order_by = order_by
            .object()
            .expect("We expect the entity order_by to be object type");

        T::Column::iter().fold(stmt, |stmt, column: T::Column| {
            let order = order_by.get(column.as_str());

            if let Some(order) = order {
                let order = order
                    .enum_name()
                    .expect("We expect the order of a column to be OrderByEnum");

                match order {
                    "Asc" => stmt.order_by(column, sea_orm::Order::Asc),
                    "Desc" => stmt.order_by(column, sea_orm::Order::Desc),
                    _ => panic!("Order is not a valid OrderByEnum item"),
                }
            } else {
                stmt
            }
        })
    } else {
        stmt
    }
}

#[derive(Clone, Debug)]
pub struct Edge<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    pub cursor: String,
    pub node: T::Model
}

impl<T> Edge<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    pub fn to_object(entity_def: &Object) -> Object {
        let name = format!("{}Edge", entity_def.type_name());
        Object::new(name)
            .field(Field::new("cursor", TypeRef::named_nn(TypeRef::STRING), |ctx| {
                FieldFuture::new(async move {
                    let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
                    Ok(Some(Value::from(edge.cursor.as_str())))
                })
            }))
            .field(Field::new("node", TypeRef::named_nn(entity_def.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let edge = ctx.parent_value.try_downcast_ref::<Edge<T>>()?;
                    Ok(Some(FieldValue::borrowed_any(&edge.node)))
                })
            }))
    }
}

#[derive(Clone, Debug)]
pub struct Connection<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    pub page_info: PageInfo,
    pub pagination_info: Option<PaginationInfo>,
    pub edges: Vec<Edge<T>>
}

impl<T> Connection<T>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    pub fn entity_object_to_connection(entity_def: &Object, edge: &Object) -> Object
    {
        Object::new(format!("{}Connection", entity_def.type_name()))
            .field(Field::new("pageInfo", TypeRef::named_nn(PageInfo::to_object().type_name()), |ctx| {
                FieldFuture::new(async move {
                    let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                    Ok(Some(FieldValue::borrowed_any(&connection.page_info)))
                })
            }))
            .field(Field::new("paginationInfo", TypeRef::named(PaginationInfo::to_object().type_name()), |ctx| {
                FieldFuture::new(async move {
                    let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                    if let Some(value) = connection.pagination_info.as_ref().map(|v| FieldValue::borrowed_any(v)) {
                        Ok(Some(value))
                    } else {
                        Ok(FieldValue::NONE)
                    }
                })
            }))
            .field(Field::new("nodes", TypeRef::named_nn_list_nn(entity_def.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                    Ok(Some(FieldValue::list(connection.edges.iter().map(|edge: &Edge<T>| {
                        FieldValue::borrowed_any(&edge.node)
                    }))))
                })
            }))
            .field(Field::new("edges", TypeRef::named_nn_list_nn(edge.type_name()), |ctx| {
                FieldFuture::new(async move {
                    let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                    Ok(Some(FieldValue::list(connection.edges.iter().map(|edge: &Edge<T>| {
                        FieldValue::borrowed_any(edge)
                    }))))
                })
            }))
    }
}

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
                        Ok(Some(Value::from(
                            cursor_page_info.has_previous_page,
                        )))
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
                        let value = cursor_page_info.start_cursor.as_ref().map(|v| Value::from(v.as_str())).or_else(|| Some(Value::Null));
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
                        let value = cursor_page_info.end_cursor.as_ref().map(|v| Value::from(v.as_str())).or_else(|| Some(Value::Null));
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
