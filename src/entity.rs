use async_graphql::{dynamic::*, Value};
use heck::ToUpperCamelCase;
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
    pub fn from_entity<T>(pagination_input: &InputObject, relations: Vec<Field>) -> Self
    where
        T: EntityTrait,
        <T as EntityTrait>::Model: Sync,
    {
        let entity_object = relations.into_iter().fold(entity_to_object::<T>(), |object, field| {
            object.field(field)
        });

        let edge_object = Edge::<T>::to_object(&entity_object);

        let connection_object = Connection::<T>::entity_object_to_connection(&entity_object, &edge_object);

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
    let entity_object = T::Column::iter().fold(
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
                #[cfg(feature = "with-decimal")]
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
    );

    entity_object
}
