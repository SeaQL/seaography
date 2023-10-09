use async_graphql::dynamic::ValueAccessor;
use itertools::Itertools;
#[allow(unused_imports)]
use sea_orm::CursorTrait;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, Iterable, ModelTrait, PaginatorTrait,
    PrimaryKeyToColumn, QuerySelect, QueryTrait, Select,
};

use crate::{
    decode_cursor, encode_cursor, map_cursor_values, BuilderContext, Connection, Edge, PageInfo,
    PaginationInfo, PaginationInputBuilder,
};

/// used to parse pagination input object and apply it to statement
pub async fn apply_pagination<T>(
    context: &'static BuilderContext,
    db: &DatabaseConnection,
    stmt: Select<T>,
    pagination: Option<ValueAccessor<'_>>,
) -> Result<Connection<T>, sea_orm::error::DbErr>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    if let Some(pagination) = pagination {
        let pagination = pagination.object().unwrap();
        let pagination_input_builder = PaginationInputBuilder { context };

        let pagination = pagination_input_builder.parse_object(&pagination);

        if let Some(cursor_object) = pagination.cursor {
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

            if let Some(cursor) = cursor_object.cursor {
                let values = decode_cursor(&cursor)?;

                let cursor_values: sea_orm::sea_query::value::ValueTuple =
                    map_cursor_values(values);

                stmt.after(cursor_values);
            }

            let data = stmt.first(cursor_object.limit).all(db).await.unwrap();

            let has_next_page: bool = {
                let mut next_stmt = apply_stmt_cursor_by(next_stmt);

                let last_node = data.last();

                if let Some(node) = last_node {
                    let values: Vec<sea_orm::Value> = T::PrimaryKey::iter()
                        .map(|variant| node.get(variant.into_column()))
                        .collect();

                    let values = map_cursor_values(values);

                    let next_data = next_stmt.first(1).after(values).all(db).await.unwrap();

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

                    let previous_data =
                        previous_stmt.first(1).before(values).all(db).await.unwrap();

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
        } else if let Some(page_object) = pagination.page {
            let paginator = stmt.paginate(db, page_object.limit);

            let pages = paginator.num_pages().await?;

            let data = paginator.fetch_page(page_object.page).await?;

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
                    has_previous_page: page_object.page != 0,
                    has_next_page: page_object.page + 1 != pages,
                    start_cursor,
                    end_cursor,
                },
                pagination_info: Some(PaginationInfo {
                    pages,
                    current: page_object.page,
                    offset: page_object.page * page_object.limit,
                    total: pages * page_object.limit,
                }),
            })
        } else if let Some(offset_object) = pagination.offset {
            let offset = offset_object.offset;
            let limit = offset_object.limit;

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
