use async_graphql::{dataloader::DataLoader, dynamic::*, Value};
use sea_orm::{prelude::*, Iterable};
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

    let query = Object::new("Query").field(actor.query).field(address.query);

    let schema = Schema::build(query.type_name(), None, None)
        .register(actor.object)
        .register(actor.filter)
        .register(actor.order)
        .register(address.object)
        .register(address.filter)
        .register(address.order)
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
    pub filter: InputObject,
    pub order: InputObject,
}

pub fn entity_to_dynamic_graphql<T>() -> DynamicGraphqlEntity
where
    T: sea_orm::EntityTrait,
    <T as sea_orm::EntityTrait>::Model: Sync,
{
    let object = entity_to_object::<T>();

    let filter = entity_to_filter::<T>();

    let order = entity_to_order::<T>();

    let query = entity_to_query::<T>(&object, &filter, &order);

    DynamicGraphqlEntity {
        object,
        query,
        filter,
        order,
    }
}

pub fn entity_to_object<T>() -> Object
where
    T: sea_orm::EntityTrait,
    <T as sea_orm::EntityTrait>::Model: Sync,
{
    T::Column::iter().fold(
        Object::new(<T as sea_orm::EntityName>::table_name(&T::default()).to_upper_camel_case()),
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
    T: sea_orm::EntityTrait,
    <T as sea_orm::EntityTrait>::Model: Sync,
{
    let name = format!(
        "{}FilterInput",
        <T as sea_orm::EntityName>::table_name(&T::default())
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
    T: sea_orm::EntityTrait,
    <T as sea_orm::EntityTrait>::Model: Sync,
{
    let name = format!(
        "{}OrderInput",
        <T as sea_orm::EntityName>::table_name(&T::default())
    )
    .to_upper_camel_case();

    T::Column::iter().fold(InputObject::new(&name), |object, column| {

        object.field(InputValue::new(column.as_str(),TypeRef::named(TypeRef::BOOLEAN)))
    })
}

pub fn entity_to_query<T>(object: &Object, filter: &InputObject, order: &InputObject) -> Field
where
    T: sea_orm::EntityTrait,
    <T as sea_orm::EntityTrait>::Model: Sync,
{
    Field::new(
        format!("{}s", <T as sea_orm::EntityName>::table_name(&T::default())),
        TypeRef::named_list(object.type_name()),
        move |ctx| {

            FieldFuture::new(async move {
                let filters = ctx.args.try_get("filters");

                if let Ok(v) = filters {
                    v.object()?
                        .iter()
                        .for_each(|(name, val)| {
                            println!("{:?}", name);
                            println!("{:?}", val.object().unwrap().try_get("eq").unwrap().i64());
                        });
                } else {
                    println!("KEY filters is MISSING!")
                };


                let database = ctx.data::<DatabaseConnection>()?;
                let data = T::find().all(database).await?;
                Ok(Some(FieldValue::list(
                    data.into_iter().map(|model| FieldValue::owned_any(model)),
                )))
            })
        },
    )
    .argument(InputValue::new(
        "filters",
        TypeRef::named(filter.type_name()),
    ))
    .argument(InputValue::new(
        "orderBy",
        TypeRef::named(order.type_name()),
    ))
}
