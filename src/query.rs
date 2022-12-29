use crate::{connection::*, edge::*, pagination::*};
use async_graphql::{dynamic::*};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use itertools::Itertools;
use sea_orm::{prelude::*, query::*, Iterable};

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
                let stmt = apply_filters(stmt, filters);
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

        let cursor_object = pagination.get("Cursor");
        let page_object = pagination.get("Pages");

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

                    next_data.len() != 0
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

                    previous_data.len() != 0
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

pub fn entity_object_relation<T, R>(name: &str) -> Field
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
    R: EntityTrait,
    T: Related<R>,
    <R as sea_orm::EntityTrait>::Model: Sync,
{
    let relation_definition = <T as Related<R>>::to();

    let name = name.to_lower_camel_case();

    let type_name: String =
        if let sea_orm::sea_query::TableRef::Table(name) = relation_definition.to_tbl {
            name.to_string()
        } else {
            // TODO look this
            "PANIC!".into()
        }
        .to_upper_camel_case();

    let field = match relation_definition.rel_type {
        sea_orm::RelationType::HasOne => {
            Field::new(name, TypeRef::named(format!("{}", type_name)), |ctx| {
                // TODO
                // dataloader applied here!
                FieldFuture::new(async move {
                    let parent: &T::Model = ctx
                        .parent_value
                        .try_downcast_ref::<T::Model>()
                        .expect("Parent should exist");

                    let stmt = <T as Related<R>>::find_related().belongs_to(parent);

                    let db = ctx.data::<DatabaseConnection>()?;

                    let data = stmt.one(db).await?;

                    if let Some(data) = data {
                        Ok(Some(FieldValue::owned_any(data)))
                    } else {
                        Ok(Some(FieldValue::NULL))
                    }
                })
            })
        }
        sea_orm::RelationType::HasMany => Field::new(
            name,
            TypeRef::named_nn(format!("{}Connection", type_name)),
            |ctx| {
                FieldFuture::new(async move {
                    // TODO
                    // each has unique query in order to apply pagination...
                    let parent: &T::Model = ctx
                        .parent_value
                        .try_downcast_ref::<T::Model>()
                        .expect("Parent should exist");

                    let stmt = <T as Related<R>>::find_related().belongs_to(parent);

                    let filters = ctx.args.get("filters");
                    let order_by = ctx.args.get("orderBy");
                    let pagination = ctx.args.get("pagination");

                    let stmt = apply_filters(stmt, filters);
                    let stmt = apply_order(stmt, order_by);

                    let db = ctx.data::<DatabaseConnection>()?;

                    let connection = apply_pagination::<R>(db, stmt, pagination).await?;

                    Ok(Some(FieldValue::owned_any(connection)))
                })
            },
        ),
    };

    let field = match relation_definition.rel_type {
        sea_orm::RelationType::HasOne => field,
        sea_orm::RelationType::HasMany => field
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
