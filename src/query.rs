use crate::{connection::*, edge::*, pagination::*};
use async_graphql::dynamic::*;
use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use sea_orm::{prelude::*, query::*, Iterable};

/// used to convert SeaORM Entity to async-graphql query
pub fn entity_to_query<T>(
    entity_object: &Object,
    connection_object: &Object,
    filter_input: &InputObject,
    order_input: &InputObject,
    pagination_input: &InputObject,
) -> Field
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    Field::new(
        entity_object.type_name().to_lower_camel_case(),
        TypeRef::named_nn(connection_object.type_name()),
        move |ctx| {
            FieldFuture::new(async move {
                let filters = ctx.args.get("filters");
                let order_by = ctx.args.get("orderBy");
                let pagination = ctx.args.get("pagination");

                let stmt = T::find();
                let stmt = stmt.filter(get_filter_conditions::<T>(filters));
                let stmt = apply_order(stmt, order_by);

                let db = ctx.data::<DatabaseConnection>()?;

                let connection = apply_pagination::<T>(db, stmt, pagination).await?;

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

/// used to parse order input object and apply it to statement
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
            let order = order_by.get(column.as_str().to_lower_camel_case().as_str());

            if let Some(order) = order {
                let order = order
                    .enum_name()
                    .expect("We expect the order of a column to be OrderByEnum");

                match order {
                    "ASC" => stmt.order_by(column, sea_orm::Order::Asc),
                    "DESC" => stmt.order_by(column, sea_orm::Order::Desc),
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

/// used to parse pagination input object and apply it to statement
pub async fn apply_pagination<T>(
    db: &DatabaseConnection,
    stmt: Select<T>,
    pagination: Option<ValueAccessor<'_>>,
) -> Result<Connection<T>, sea_orm::error::DbErr>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    if let Some(pagination) = pagination {
        let pagination = pagination
            .object()
            .expect("We expect the pagination to be object type");

        let cursor_object = pagination.get("cursor");
        let page_object = pagination.get("pages");
        let offset_object = pagination.get("offset");

        if let Some(cursor_object) = cursor_object {
            let cursor_object = cursor_object
                .object()
                .expect("We expect Cursor to be object");

            let limit = cursor_object
                .get("limit")
                .expect("Cursor has a mandatory limit field")
                .u64()
                .expect("Cursor limit field is should be u64");
            let cursor = cursor_object.get("cursor");

            let next_stmt = stmt.clone();
            let previous_stmt = stmt.clone();

            fn apply_stmt_cursor_by<T>(
                stmt: sea_orm::entity::prelude::Select<T>,
            ) -> sea_orm::Cursor<sea_orm::SelectModel<T::Model>>
            where
                T: EntityTrait,
                <T as EntityTrait>::Model: Sync,
            {
                let size = T::PrimaryKey::iter().fold(0, |acc, _| acc + 1);
                if size == 1 {
                    let column = T::PrimaryKey::iter()
                        .map(|variant| variant.into_column())
                        .collect::<Vec<T::Column>>()[0];
                    stmt.cursor_by(column)
                } else if size == 2 {
                    let columns = T::PrimaryKey::iter()
                        .map(|variant| variant.into_column())
                        .collect_tuple::<(T::Column, T::Column)>()
                        .unwrap();
                    stmt.cursor_by(columns)
                } else if size == 3 {
                    let columns = T::PrimaryKey::iter()
                        .map(|variant| variant.into_column())
                        .collect_tuple::<(T::Column, T::Column, T::Column)>()
                        .unwrap();
                    stmt.cursor_by(columns)
                } else {
                    panic!("seaography does not support cursors with size greater than 3")
                }
            }

            let mut stmt = apply_stmt_cursor_by(stmt);

            if let Some(cursor) = cursor {
                let cursor = cursor
                    .string()
                    .expect("Cursor value cursor should be String");

                let values = decode_cursor(cursor)?;

                let cursor_values: sea_orm::sea_query::value::ValueTuple =
                    map_cursor_values(values);

                stmt.after(cursor_values);
            }

            let data = stmt.first(limit).all(db).await.unwrap();

            let has_next_page: bool = {
                let mut next_stmt = apply_stmt_cursor_by(next_stmt);

                let last_node = data.last();

                if let Some(node) = last_node {
                    let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                        .map(|variant| node.get(variant.into_column()))
                        .collect();

                    let values = map_cursor_values(values);

                    let next_data = next_stmt.first(limit).after(values).all(db).await.unwrap();

                    !next_data.is_empty()
                } else {
                    false
                }
            };

            let has_previous_page: bool = {
                let mut previous_stmt = apply_stmt_cursor_by(previous_stmt);

                let first_node = data.first();

                if let Some(node) = first_node {
                    let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                        .map(|variant| node.get(variant.into_column()))
                        .collect();

                    let values = map_cursor_values(values);

                    let previous_data = previous_stmt
                        .first(limit)
                        .before(values)
                        .all(db)
                        .await
                        .unwrap();

                    !previous_data.is_empty()
                } else {
                    false
                }
            };

            let edges: Vec<Edge<T>> = data
                .into_iter()
                .map(|node| {
                    let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                        .map(|variant| node.get(variant.into_column()))
                        .collect();

                    let cursor: String = encode_cursor(values);

                    Edge { cursor, node }
                })
                .collect();

            let start_cursor = edges.first().map(|edge| edge.cursor.clone());
            let end_cursor = edges.last().map(|edge| edge.cursor.clone());

            Ok(Connection {
                edges,
                page_info: PageInfo {
                    has_previous_page,
                    has_next_page,
                    start_cursor,
                    end_cursor,
                },
                pagination_info: None,
            })
        } else if let Some(page_object) = page_object {
            let page_object = page_object.object().expect("We expect Pages to be object");
            let page = page_object
                .get("page")
                .map_or_else(|| Ok(0), |v| v.u64())
                .expect("PaginationInput page value should be u64");
            let limit = page_object
                .get("limit")
                .expect("Pages has a mandatory limit field")
                .u64()
                .expect("Pages limit field is should be u64");

            let paginator = stmt.paginate(db, limit);

            let pages = paginator.num_pages().await?;

            let data = paginator.fetch_page(page).await?;

            let edges: Vec<Edge<T>> = data
                .into_iter()
                .map(|node| {
                    let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                        .map(|variant| node.get(variant.into_column()))
                        .collect();

                    let cursor: String = encode_cursor(values);

                    Edge { cursor, node }
                })
                .collect();

            let start_cursor = edges.first().map(|edge| edge.cursor.clone());
            let end_cursor = edges.last().map(|edge| edge.cursor.clone());

            Ok(Connection {
                edges,
                page_info: PageInfo {
                    has_previous_page: page != 1,
                    has_next_page: page < pages - 1,
                    start_cursor,
                    end_cursor,
                },
                pagination_info: Some(PaginationInfo {
                    pages,
                    current: page,
                    offset: page * limit,
                    total: pages * limit,
                }),
            })
        } else if let Some(offset_object) = offset_object {
            let offset_object = offset_object
                .object()
                .expect("We expect Offset to be object");
            let offset = offset_object
                .get("offset")
                .map_or_else(|| Ok(0), |v| v.u64())
                .expect("OffsetInput offset value should be u64");
            let limit = offset_object
                .get("limit")
                .expect("Offset has a mandatory limit field")
                .u64()
                .expect("Offset limit field is should be u64");

            let count_stmt = stmt.clone().as_query().to_owned();

            let data = stmt.offset(offset).limit(limit).all(db).await?;

            let edges: Vec<Edge<T>> = data
                .into_iter()
                .map(|node| {
                    let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                        .map(|variant| node.get(variant.into_column()))
                        .collect();

                    let cursor: String = encode_cursor(values);

                    Edge { cursor, node }
                })
                .collect();

            let start_cursor = edges.first().map(|edge| edge.cursor.clone());
            let end_cursor = edges.last().map(|edge| edge.cursor.clone());

            let count_stmt = db.get_database_backend().build(
                sea_orm::sea_query::SelectStatement::new()
                    .expr(sea_orm::sea_query::Expr::cust("COUNT(*) AS num_items"))
                    .from_subquery(count_stmt, sea_orm::sea_query::Alias::new("sub_query")),
            );

            let total = match db.query_one(count_stmt).await? {
                Some(res) => match db.get_database_backend() {
                    sea_orm::DbBackend::Postgres => res.try_get::<i64>("", "num_items")? as u64,
                    _ => res.try_get::<i32>("", "num_items")? as u64,
                },
                None => 0,
            };

            Ok(Connection {
                edges,
                page_info: PageInfo {
                    has_previous_page: offset != 0,
                    has_next_page: offset * limit < total,
                    start_cursor,
                    end_cursor,
                },
                pagination_info: Some(PaginationInfo {
                    current: f64::ceil(offset as f64 / limit as f64) as u64,
                    pages: f64::ceil(total as f64 / limit as f64) as u64,
                    total,
                    offset,
                }),
            })
        } else {
            Err(DbErr::Type(
                "Something is wrong with the pagination input".into(),
            ))
        }
    } else {
        let data = stmt.all(db).await?;

        let edges: Vec<Edge<T>> = data
            .into_iter()
            .map(|node| {
                let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                    .map(|variant| node.get(variant.into_column()))
                    .collect();

                let cursor: String = encode_cursor(values);

                Edge { cursor, node }
            })
            .collect();

        let start_cursor = edges.first().map(|edge| edge.cursor.clone());
        let end_cursor = edges.last().map(|edge| edge.cursor.clone());

        let total = edges.len() as u64;

        Ok(Connection {
            edges,
            page_info: PageInfo {
                has_previous_page: false,
                has_next_page: false,
                start_cursor,
                end_cursor,
            },
            pagination_info: Some(PaginationInfo {
                pages: 1,
                current: 1,
                offset: 0,
                total,
            }),
        })
    }
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
            // FIXME: implement bellow methods
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
            // FIXME: implement bellow methods
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

pub fn get_filter_conditions<T>(filters: Option<ValueAccessor>) -> Condition
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    if let Some(filters) = filters {
        let filters = filters
            .object()
            .expect("We expect the entity filters to be object type");

        recursive_prepare_condition::<T>(filters)
    } else {
        Condition::all()
    }
}

pub fn recursive_prepare_condition<T>(filters: ObjectAccessor) -> Condition
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let condition = T::Column::iter().fold(Condition::all(), |condition, column: T::Column| {
        let filter = filters.get(column.as_str().to_lower_camel_case().as_str());

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
                #[cfg(feature = "with-decimal")]
                ColumnType::Decimal(_) => {
                    use std::str::FromStr;

                    let condition = if let Some(data) = filter.get("eq") {
                        let data = data
                            .string()
                            .expect("We expect the eq to be of type String");
                        let data = sea_orm::entity::prelude::Decimal::from_str(data)
                            .expect("We expect value to be Decimal");
                        condition.add(column.eq(data))
                    } else {
                        condition
                    };

                    let condition = if let Some(data) = filter.get("ne") {
                        let data = data
                            .string()
                            .expect("We expect the ne to be of type String");
                        let data = sea_orm::entity::prelude::Decimal::from_str(data)
                            .expect("We expect value to be Decimal");
                        condition.add(column.ne(data))
                    } else {
                        condition
                    };

                    let condition = if let Some(data) = filter.get("gt") {
                        let data = data
                            .string()
                            .expect("We expect the gt to be of type String");
                        let data = sea_orm::entity::prelude::Decimal::from_str(data)
                            .expect("We expect value to be Decimal");
                        condition.add(column.gt(data))
                    } else {
                        condition
                    };

                    let condition = if let Some(data) = filter.get("gte") {
                        let data = data
                            .string()
                            .expect("We expect the gte to be of type String");
                        let data = sea_orm::entity::prelude::Decimal::from_str(data)
                            .expect("We expect value to be Decimal");
                        condition.add(column.gte(data))
                    } else {
                        condition
                    };

                    let condition = if let Some(data) = filter.get("lt") {
                        let data = data
                            .string()
                            .expect("We expect the lt to be of type String");
                        let data = sea_orm::entity::prelude::Decimal::from_str(data)
                            .expect("We expect value to be Decimal");
                        condition.add(column.lt(data))
                    } else {
                        condition
                    };

                    let condition = if let Some(data) = filter.get("lte") {
                        let data = data
                            .string()
                            .expect("We expect the lte to be of type String");
                        let data = sea_orm::entity::prelude::Decimal::from_str(data)
                            .expect("We expect value to be Decimal");
                        condition.add(column.lte(data))
                    } else {
                        condition
                    };

                    condition
                }
                // ColumnType::DateTime => {
                // FIXME
                // },
                // ColumnType::Timestamp => {
                // FIXME
                // },
                // ColumnType::TimestampWithTimeZone => {
                // FIXME
                // },
                // ColumnType::Time => {
                // FIXME
                // },
                // ColumnType::Date => {
                // FIXME
                // },
                // ColumnType::Year => {
                // FIXME
                // },
                // ColumnType::Interval => {
                // FIXME
                // },
                // ColumnType::Binary => {
                // FIXME
                // },
                // ColumnType::VarBinary => {
                // FIXME
                // },
                // ColumnType::Bit => {
                // FIXME
                // },
                // ColumnType::VarBit => {
                // FIXME
                // },
                // ColumnType::Boolean => {
                // FIXME
                // },
                // ColumnType::Money(_) => {
                // FIXME
                // },
                // ColumnType::Json => {
                // FIXME
                // },
                // ColumnType::JsonBinary => {
                // FIXME
                // },
                // ColumnType::Custom(_) => {
                // FIXME
                // },
                // ColumnType::Uuid => {
                // FIXME
                // },
                ColumnType::Enum { name: _, variants } => {
                    prepare_enumeration_condition(condition, &filter, column, variants)
                }
                // ColumnType::Array(_) => {
                // FIXME
                // },
                // ColumnType::Cidr => {
                // FIXME
                // },
                // ColumnType::Inet => {
                // FIXME
                // },
                // ColumnType::MacAddr => {
                // FIXME
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

pub fn prepare_enumeration_condition<T>(
    condition: Condition,
    filter: &ObjectAccessor,
    column: T,
    variants: &Vec<std::sync::Arc<dyn Iden>>,
) -> Condition
where
    T: ColumnTrait,
{
    let extract_variant = move |input: &str| -> String {
        let variant = variants.iter().find(|variant| {
            let variant = variant
                .to_string()
                .to_upper_camel_case()
                .to_ascii_uppercase();
            variant.eq(input)
        });
        variant
            .expect("We expect to always map Enumerations.")
            .to_string()
    };

    let condition = if let Some(data) = filter.get("eq") {
        let data = data
            .enum_name()
            .expect("We expect the eq to be of type Enumeration");
        condition.add(column.eq(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("ne") {
        let data = data
            .enum_name()
            .expect("We expect the ne to be of type Enumeration");
        condition.add(column.ne(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("gt") {
        let data = data
            .enum_name()
            .expect("We expect the gt to be of type Enumeration");
        condition.add(column.gt(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("gte") {
        let data = data
            .enum_name()
            .expect("We expect the gte to be of type Enumeration");
        condition.add(column.gte(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("lt") {
        let data = data
            .enum_name()
            .expect("We expect the lt to be of type Enumeration");
        condition.add(column.lt(extract_variant(data)))
    } else {
        condition
    };

    let condition = if let Some(data) = filter.get("lte") {
        let data = data
            .enum_name()
            .expect("We expect the lte to be of type Enumeration");
        condition.add(column.lte(extract_variant(data)))
    } else {
        condition
    };

    condition
}

pub fn entity_object_relation<T, R>(name: &str, relation_definition: RelationDef) -> Field
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
    <<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
    R: EntityTrait,
    <R as sea_orm::EntityTrait>::Model: Sync,
    <<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
{
    let name = name.to_lower_camel_case();

    let type_name: String = match relation_definition.to_tbl {
        sea_orm::sea_query::TableRef::Table(table) => table.to_string(),
        sea_orm::sea_query::TableRef::TableAlias(table, _alias) => table.to_string(),
        sea_orm::sea_query::TableRef::SchemaTable(_schema, table) => table.to_string(),
        sea_orm::sea_query::TableRef::DatabaseSchemaTable(_database, _schema, table) => {
            table.to_string()
        }
        sea_orm::sea_query::TableRef::SchemaTableAlias(_schema, table, _alias) => table.to_string(),
        sea_orm::sea_query::TableRef::DatabaseSchemaTableAlias(
            _database,
            _schema,
            table,
            _alias,
        ) => table.to_string(),
        // FIXME: what if empty ?
        sea_orm::sea_query::TableRef::SubQuery(_stmt, alias) => alias.to_string(),
        sea_orm::sea_query::TableRef::ValuesList(_values, alias) => alias.to_string(),
        sea_orm::sea_query::TableRef::FunctionCall(_, alias) => alias.to_string(),
    }
    .to_upper_camel_case();

    let from_col = <T::Column as std::str::FromStr>::from_str(
        relation_definition
            .from_col
            .to_string()
            .to_snake_case()
            .as_str(),
    )
    .unwrap();

    let to_col = <R::Column as std::str::FromStr>::from_str(
        relation_definition
            .to_col
            .to_string()
            .to_snake_case()
            .as_str(),
    )
    .unwrap();

    let field = match relation_definition.is_owner {
        false => {
            Field::new(name, TypeRef::named(type_name.to_string()), move |ctx| {
                // FIXME: optimize with dataloader
                FieldFuture::new(async move {
                    let parent: &T::Model = ctx
                        .parent_value
                        .try_downcast_ref::<T::Model>()
                        .expect("Parent should exist");

                    let stmt = R::find();

                    let filter = Condition::all().add(to_col.eq(parent.get(from_col)));

                    let stmt = stmt.filter(filter);

                    let db = ctx.data::<DatabaseConnection>()?;

                    let data = stmt.one(db).await?;

                    if let Some(data) = data {
                        Ok(Some(FieldValue::owned_any(data)))
                    } else {
                        Ok(None)
                    }
                })
            })
        }
        true => Field::new(
            name,
            TypeRef::named_nn(format!("{}Connection", type_name)),
            move |ctx| {
                FieldFuture::new(async move {
                    // FIXME: optimize union queries
                    // NOTE: each has unique query in order to apply pagination...
                    let parent: &T::Model = ctx
                        .parent_value
                        .try_downcast_ref::<T::Model>()
                        .expect("Parent should exist");

                    let stmt = R::find();

                    let condition = Condition::all().add(to_col.eq(parent.get(from_col)));

                    let filters = ctx.args.get("filters");
                    let order_by = ctx.args.get("orderBy");
                    let pagination = ctx.args.get("pagination");

                    let base_condition = get_filter_conditions::<R>(filters);

                    let stmt = stmt.filter(condition.add(base_condition));
                    let stmt = apply_order(stmt, order_by);

                    let db = ctx.data::<DatabaseConnection>()?;

                    let connection = apply_pagination::<R>(db, stmt, pagination).await?;

                    Ok(Some(FieldValue::owned_any(connection)))
                })
            },
        ),
    };

    let field = match relation_definition.is_owner {
        false => field,
        true => field
            .argument(InputValue::new(
                "filters",
                TypeRef::named(format!("{}FilterInput", type_name)),
            ))
            .argument(InputValue::new(
                "orderBy",
                TypeRef::named(format!("{}OrderInput", type_name)),
            ))
            .argument(InputValue::new(
                "pagination",
                TypeRef::named("PaginationInput"),
            )),
    };

    field
}

pub fn entity_object_via_relation<T, R>(name: &str) -> Field
where
    T: Related<R>,
    T: EntityTrait,
    R: EntityTrait,
    <T as EntityTrait>::Model: Sync,
    <R as sea_orm::EntityTrait>::Model: Sync,
    <<T as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
    <<R as sea_orm::EntityTrait>::Column as std::str::FromStr>::Err: core::fmt::Debug,
{
    let to_relation_definition = <T as Related<R>>::to();
    let via_relation_definition = <T as Related<R>>::via().expect(
        "We expect this function to be used with Related that has `via` method implemented!",
    );

    let name = name.to_lower_camel_case();

    let type_name: String = match to_relation_definition.to_tbl {
        sea_orm::sea_query::TableRef::Table(table) => table.to_string(),
        sea_orm::sea_query::TableRef::TableAlias(table, _alias) => table.to_string(),
        sea_orm::sea_query::TableRef::SchemaTable(_schema, table) => table.to_string(),
        sea_orm::sea_query::TableRef::DatabaseSchemaTable(_database, _schema, table) => {
            table.to_string()
        }
        sea_orm::sea_query::TableRef::SchemaTableAlias(_schema, table, _alias) => table.to_string(),
        sea_orm::sea_query::TableRef::DatabaseSchemaTableAlias(
            _database,
            _schema,
            table,
            _alias,
        ) => table.to_string(),
        // FIXME: what if empty ?
        sea_orm::sea_query::TableRef::SubQuery(_stmt, alias) => alias.to_string(),
        sea_orm::sea_query::TableRef::ValuesList(_values, alias) => alias.to_string(),
        sea_orm::sea_query::TableRef::FunctionCall(_, alias) => alias.to_string(),
    }
    .to_upper_camel_case();

    let from_col = <T::Column as std::str::FromStr>::from_str(
        via_relation_definition
            .from_col
            .to_string()
            .to_snake_case()
            .as_str(),
    )
    .unwrap();

    let to_col = <R::Column as std::str::FromStr>::from_str(
        to_relation_definition
            .to_col
            .to_string()
            .to_snake_case()
            .as_str(),
    )
    .unwrap();

    let field = match via_relation_definition.is_owner {
        false => {
            Field::new(name, TypeRef::named(type_name.to_string()), move |ctx| {
                // FIXME: optimize by adding dataloader
                FieldFuture::new(async move {
                    let parent: &T::Model = ctx
                        .parent_value
                        .try_downcast_ref::<T::Model>()
                        .expect("Parent should exist");

                    let stmt = if let Some(_) = <T as Related<R>>::via() {
                        <T as Related<R>>::find_related()
                    } else {
                        R::find()
                    };

                    let filter = Condition::all().add(to_col.eq(parent.get(from_col)));

                    let stmt = stmt.filter(filter);

                    let db = ctx.data::<DatabaseConnection>()?;

                    let data = stmt.one(db).await?;

                    if let Some(data) = data {
                        Ok(Some(FieldValue::owned_any(data)))
                    } else {
                        Ok(None)
                    }
                })
            })
        }
        true => Field::new(
            name,
            TypeRef::named_nn(format!("{}Connection", type_name)),
            move |ctx| {
                FieldFuture::new(async move {
                    // FIXME: optimize union queries
                    // NOTE: each has unique query in order to apply pagination...
                    let parent: &T::Model = ctx
                        .parent_value
                        .try_downcast_ref::<T::Model>()
                        .expect("Parent should exist");

                    let stmt = if let Some(_) = <T as Related<R>>::via() {
                        <T as Related<R>>::find_related()
                    } else {
                        R::find()
                    };

                    let condition = Condition::all().add(to_col.eq(parent.get(from_col)));

                    let filters = ctx.args.get("filters");
                    let order_by = ctx.args.get("orderBy");
                    let pagination = ctx.args.get("pagination");

                    let base_condition = get_filter_conditions::<R>(filters);

                    let stmt = stmt.filter(condition.add(base_condition));
                    let stmt = apply_order(stmt, order_by);

                    let db = ctx.data::<DatabaseConnection>()?;

                    let connection = apply_pagination::<R>(db, stmt, pagination).await?;

                    Ok(Some(FieldValue::owned_any(connection)))
                })
            },
        ),
    };

    let field = match via_relation_definition.is_owner {
        false => field,
        true => field
            .argument(InputValue::new(
                "filters",
                TypeRef::named(format!("{}FilterInput", type_name)),
            ))
            .argument(InputValue::new(
                "orderBy",
                TypeRef::named(format!("{}OrderInput", type_name)),
            ))
            .argument(InputValue::new(
                "pagination",
                TypeRef::named("PaginationInput"),
            )),
    };

    field
}
