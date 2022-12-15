use async_graphql::{dataloader::DataLoader, dynamic::*, Value};
use sea_orm::{prelude::*, Condition, Iterable, QueryOrder};
use seaography::heck::ToUpperCamelCase;

use crate::OrmDataloader;

pub fn schema(
    database: DatabaseConnection,
    orm_dataloader: DataLoader<OrmDataloader>,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let actor = entity_to_dynamic_graphql::<crate::entities::actor::Entity>();
    let address = entity_to_dynamic_graphql::<crate::entities::address::Entity>();
    let order_by_enum = Enum::new("OrderByEnum")
        .item(EnumItem::new("Asc"))
        .item(EnumItem::new("Desc"));

    let query = Object::new("Query").field(actor.query).field(address.query);

    let schema = Schema::build(query.type_name(), None, None)
        .register(actor.object)
        .register(actor.filter_input)
        .register(actor.order_input)
        .register(address.object)
        .register(address.filter_input)
        .register(address.order_input)
        .register(order_by_enum)
        .register(query);

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

    schema.data(database).data(orm_dataloader).finish()
}

pub struct DynamicGraphqlEntity {
    pub object: Object,
    pub query: Field,
    pub filter_input: InputObject,
    pub order_input: InputObject,
}

pub fn entity_to_dynamic_graphql<T>() -> DynamicGraphqlEntity
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let object = entity_to_object::<T>();

    let filter_input = entity_to_filter::<T>();

    let order_input = entity_to_order::<T>();

    let query = entity_to_query::<T>(&object, &filter_input, &order_input);

    DynamicGraphqlEntity {
        object,
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

pub fn entity_to_filter<T>() -> InputObject
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let name = format!(
        "{}FilterInput",
        <T as EntityName>::table_name(&T::default())
    )
    .to_upper_camel_case();

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

pub fn entity_to_order<T>() -> InputObject
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let name =
        format!("{}OrderInput", <T as EntityName>::table_name(&T::default())).to_upper_camel_case();

    T::Column::iter().fold(InputObject::new(&name), |object, column| {
        object.field(InputValue::new(
            column.as_str(),
            TypeRef::named("OrderByEnum"),
        ))
    })
}

pub fn entity_to_query<T>(
    object: &Object,
    filter_input: &InputObject,
    order_input: &InputObject,
) -> Field
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    Field::new(
        format!("{}s", <T as EntityName>::table_name(&T::default())),
        TypeRef::named_list(object.type_name()),
        move |ctx| {
            FieldFuture::new(async move {
                let filters = ctx.args.get("filters");
                let order_by = ctx.args.get("orderBy");

                let stmt = T::find();
                let stmt = apply_filters(stmt, filters);
                let stmt = apply_order(stmt, order_by);

                let database = ctx.data::<DatabaseConnection>()?;
                let data = stmt.all(database).await?;
                Ok(Some(FieldValue::list(
                    data.into_iter().map(|model| FieldValue::owned_any(model)),
                )))
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
}

#[macro_export]
macro_rules! basic_filtering_operation {
    ( $condition:expr, $column:expr, $filter:expr, $operator:ident, $type:ident ) => {
        if let Some(data) = $filter.get(stringify!($operator)) {
            let data = data.$type().expect(format!(
                "We expect the {} to be of type {}",
                stringify!($operator),
                stringify!($type)
            ).as_str());

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

pub fn apply_filters<T>(
    stmt: Select<T>,
    filters: Option<ValueAccessor>,
) -> Select<T>
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
                ColumnType::Char(_) | ColumnType::String(_) | ColumnType::Text => string_filtering_type!(condition, column, filter, string),
                ColumnType::TinyInteger
                | ColumnType::SmallInteger
                | ColumnType::Integer
                | ColumnType::BigInteger => basic_filtering_type!(condition, column, filter, i64),
                ColumnType::TinyUnsigned
                | ColumnType::SmallUnsigned
                | ColumnType::Unsigned
                | ColumnType::BigUnsigned => basic_filtering_type!(condition, column, filter, u64),
                ColumnType::Float | ColumnType::Double => basic_filtering_type!(condition, column, filter, f64),
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
        let filters = and.list().expect("We expect to find a list of FiltersInput");

        condition.add(
            filters.iter().fold(Condition::all(), |condition, filters: ValueAccessor| {
                let filters = filters.object().expect("We expect an FiltersInput object");
                condition.add(recursive_prepare_condition::<T>(filters))
            })
        )
    } else {
        condition
    };

    let condition = if let Some(or) = filters.get("or") {
        let filters = or.list().expect("We expect to find a list of FiltersInput");

        condition.add(
            filters.iter().fold(Condition::any(), |condition, filters: ValueAccessor| {
                let filters = filters.object().expect("We expect an FiltersInput object");
                condition.add(recursive_prepare_condition::<T>(filters))
            })
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
        let order_by = order_by.object().expect("We expect the entity order_by to be object type");

        T::Column::iter().fold(stmt, |stmt, column: T::Column| {
            let order = order_by.get(column.as_str());

            if let Some(order) = order {
                let order = order.enum_name().expect("We expect the order of a column to be OrderByEnum");

                match order {
                    "Asc" => stmt.order_by(column, sea_orm::Order::Asc),
                    "Desc" => stmt.order_by(column, sea_orm::Order::Desc),
                    _ => panic!("Order is not a valid OrderByEnum item")
                }
            } else {
                stmt
            }
        })
    } else {
        stmt
    }
}
